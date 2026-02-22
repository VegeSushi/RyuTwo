#![no_std]
#![no_main]

mod pins;

use embedded_alloc::TlsfHeap as Heap;
use defmt::{info, panic, unwrap};
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, Instance, InterruptHandler};
use embassy_usb::UsbDevice;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use static_cell::StaticCell;
use core::ops::Range;
use embassy_rp::flash::{Flash};
use embedded_storage_async::nor_flash::MultiwriteNorFlash;
use embassy_time;
use wartcl::{empty, Env, FlowChange};
use sequential_storage::{
    map::{MapConfig, MapStorage},
};
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

// The offset from the start of FLASH (matching 0x101C0000 in memory.x)
// Offset from 0x10000000 (Start of Flash chip)
const STORAGE_BASE: u32 = 0x1C0000; 
// Entire 256K region for the Map
const MAP_FLASH_RANGE: Range<u32> = STORAGE_BASE..(STORAGE_BASE + 0x40000);

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 32 * 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        
        // We use addr_of_mut! to get a raw pointer without creating an intermediate reference.
        // This satisfies the new safety requirements for mutable statics.
        unsafe {
            let ptr = core::ptr::addr_of_mut!(HEAP_MEM) as usize;
            ALLOCATOR.init(ptr, HEAP_SIZE);
        }
    }

    let p = embassy_rp::init(Default::default());

    let flash = Flash::<_, _, { 2 * 1024 * 1024 }>::new(p.FLASH, p.DMA_CH0);
    let flash = embassy_embedded_hal::adapter::BlockingAsync::new(flash);

    let driver = Driver::new(p.USB, Irqs);

    let config = {
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some("RyuTwo");
        config.product = Some("RyuTwo-Firmware-Serial");
        config.serial_number = Some("002");
        config.max_power = 100;
        config.max_packet_size_0 = 64;
        config
    };

    let mut builder = {
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

        embassy_usb::Builder::new(
            driver,
            config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [],
            CONTROL_BUF.init([0; 64]),
        )
    };

    let mut class = {
        static STATE: StaticCell<State> = StaticCell::new();
        let state = STATE.init(State::new());
        CdcAcmClass::new(&mut builder, state, 64)
    };

    let usb = builder.build();

    let mut map_storage = MapStorage::new(
        flash,
        const { MapConfig::new(MAP_FLASH_RANGE) },
        sequential_storage::cache::KeyPointerCache::<4, u8, 8>::new(),
    );

    unwrap!(spawner.spawn(usb_task(usb)));

    loop {
        class.wait_connection().await;
        info!("Connected");
        let _ = handle_commands(&mut class, &mut map_storage).await;
        info!("Disconnected");
    }
}

type MyUsbDriver = Driver<'static, USB>;
type MyUsbDevice = UsbDevice<'static, MyUsbDriver>;

#[embassy_executor::task]
async fn usb_task(mut usb: MyUsbDevice) -> ! {
    usb.run().await
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn handle_commands<'d, T: Instance + 'd, E: defmt::Format>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    storage: &mut MapStorage<u8, impl MultiwriteNorFlash<Error = E>, impl sequential_storage::cache::KeyCacheImpl<u8>>,
) -> Result<(), Disconnected> {
    let mut line_buf = [0u8; 64];
    let mut pos = 0;

    loop {
        let mut read_buf = [0u8; 64];
        let n = class.read_packet(&mut read_buf).await?;
        
        for &byte in &read_buf[..n] {
            // Check for newline (Enter key)
            if byte == b'\n' || byte == b'\r' {
                if pos > 0 {
                    let command = &line_buf[..pos];
                    
                    if command.starts_with(b"ping") {
                        class.write_packet(b"pong\r\n").await?;
                    } else if command.starts_with(b"write") {
                        // Usage: Send "write" + 1 byte key + \n
                        // The Pico responds with "READY", then waits for text + \0
                        let key = command.get(5).cloned().unwrap_or(0);
                        let _ = class.write_packet(b"READY\r\n").await;

                        let mut data_buf = [0u8; 1024]; 
                        let mut data_pos = 0;
                        
                        // Stream text until null terminator
                        'stream: loop {
                            let mut rx_buf = [0u8; 64];
                            let n = class.read_packet(&mut rx_buf).await?;
                            for &b in &rx_buf[..n] {
                                if b == b'\0' || data_pos >= data_buf.len() { break 'stream; }
                                data_buf[data_pos] = b;
                                data_pos += 1;
                            }
                        }

                        let mut scratch = [0u8; 1024];
                        match storage.store_item(&mut scratch, &key, &&data_buf[..data_pos]).await {
                            Ok(_) => { let _ = class.write_packet(b"STORED\r\n").await; }
                            Err(_) => { let _ = class.write_packet(b"STORAGE_ERR\r\n").await; }
                        }
                    } else if command.starts_with(b"read") {
                        // Usage: Send "read" + 1 byte key + \n
                        let key = command.get(4).cloned().unwrap_or(0);
                        let mut fetch_buf = [0u8; 1024];
                        
                        match storage.fetch_item::<&[u8]>(&mut fetch_buf, &key).await {
                            Ok(Some(data)) => {
                                let _ = class.write_packet(data).await;
                                let _ = class.write_packet(b"\r\n").await;
                            }
                            _ => { let _ = class.write_packet(b"NOT_FOUND\r\n").await; }
                        }
                    } else if command.starts_with(b"format") {
                        match storage.erase_all().await {
                            Ok(_) => {
                                let _ = class.write_packet(b"formatted\r\n").await;
                            }
                            Err(_e) => {
                                let _ = class.write_packet(b"error\r\n").await;
                            }
                        }
                    } else if command.starts_with(b"test_flash") {
                        let mut data_buffer = [0u8; 64];
                        let key = 5u8;
                        let test_val = "Flash Storage Working".as_bytes(); 

                        match storage.store_item(&mut data_buffer, &key, &test_val).await {
                            Ok(_) => {
                                match storage.fetch_item::<&[u8]>(&mut data_buffer, &key).await {
                                    Ok(Some(returned_bytes)) => {
                                        class.write_packet(b"Verified: ").await?;
                                        class.write_packet(returned_bytes).await?;
                                        class.write_packet(b"\r\n").await?;
                                    }
                                    _ => { class.write_packet(b"Fetch failed\r\n").await?; }
                                }
                            }
                            Err(_) => { class.write_packet(b"Store failed\r\n").await?; }
                        }
                    } else if command.starts_with(b"run") {
                        let key = command.get(3).cloned().unwrap_or(0);
                        let mut fetch_buf = [0u8; 1024]; // Buffer to hold the script from flash
    
                        match storage.fetch_item::<&[u8]>(&mut fetch_buf, &key).await {
                            Ok(Some(code)) => {
                                // Found the script! Now execute it.
                                run_tcl(class, code).await;
                            }
                            Ok(None) => {
                                let _ = class.write_packet(b"SCRIPT_NOT_FOUND\r\n").await;
                            }
                            Err(_) => {
                                let _ = class.write_packet(b"FLASH_READ_ERR\r\n").await;
                            }
                        }
                    } else {
                        class.write_packet(b"Unknown\r\n").await?;
                    }
                    pos = 0; // Reset buffer for next command
                }
            } else {
                // Add byte to buffer if there's space
                if pos < line_buf.len() {
                    line_buf[pos] = byte;
                    pos += 1;
                }
            }
        }
    }
}

async fn run_tcl<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    code: &[u8],
) {
    let mut tcl = Env::default();

    // Command: sleep <ms>
    tcl.register(b"sleep", 0, |_, args| {
        if let Some(arg) = args.get(1) {
            if let Ok(ms_str) = core::str::from_utf8(arg) {
                if let Ok(ms) = ms_str.parse::<u64>() {
                    embassy_time::block_for(embassy_time::Duration::from_millis(ms));
                }
            }
        }
        Ok(empty())
    });

    // Command: gpio <idx> <1/0>
    tcl.register(b"gpio", 0, |_, args| {
        if args.len() >= 3 {
            let idx_res = core::str::from_utf8(&args[1]).ok().and_then(|s| s.parse::<u8>().ok());
            let action = core::str::from_utf8(&args[2]).ok();
            
            if let (Some(idx), Some(act)) = (idx_res, action) {
                match act {
                    "high" | "1" => pins::on(idx),  // No 'p' needed!
                    "low" | "0" => pins::off(idx), // No 'p' needed!
                    _ => return Err(FlowChange::Error),
                }
            }
        }
        Ok(empty())
    });

    tcl.register(b"put", 0, |_, args| {
        if let Some(arg) = args.get(1) {
            return Ok(arg.to_vec().into()); 
        }
        Ok(empty())
    });

    match tcl.eval(code) {
        Ok(result) => {
            if !result.is_empty() {
                let _ = class.write_packet(&result).await;
                let _ = class.write_packet(b"\r\n").await;
            }
        }
        Err(FlowChange::Error) => { let _ = class.write_packet(b"Tcl Error\r\n").await; }
        Err(FlowChange::Return(val)) => {
            let _ = class.write_packet(b"Returned: ").await;
            let _ = class.write_packet(&val).await;
            let _ = class.write_packet(b"\r\n").await;
        }
        _ => { let _ = class.write_packet(b"Unexpected flow change\r\n").await; }
    }
}