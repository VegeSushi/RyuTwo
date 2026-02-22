#![no_std]
#![no_main]

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
use sequential_storage::{
    map::{MapConfig, MapStorage},
};
use {defmt_rtt as _, panic_probe as _};

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
                        let test_val = "Flash Storage Working".as_bytes(); // Convert to &[u8]

                        // Store the byte slice
                        match storage.store_item(&mut data_buffer, &key, &test_val).await {
                            Ok(_) => {
                                // Fetch it back as a byte slice
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