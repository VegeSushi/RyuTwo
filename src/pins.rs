use embassy_rp::gpio::{Level, Output};
use embassy_rp::Peripherals;

/*
fn steal_p() -> &'static mut Peripherals {
    unsafe { &mut *(embassy_rp::pac::REGS_BASE as *mut Peripherals) }
}
*/

pub fn on(num: u8) {
    /*
    let mut p = unsafe { embassy_rp::Peripherals::steal() };
    match num {
        0 => unsafe { Output::new(core::ptr::read(&p.PIN_0), Level::High).set_high() }
        1 => unsafe { Output::new(core::ptr::read(&p.PIN_1), Level::High).set_high() }
        2 => unsafe { Output::new(core::ptr::read(&p.PIN_2), Level::High).set_high() }
        3 => unsafe { Output::new(core::ptr::read(&p.PIN_3), Level::High).set_high() }
        4 => unsafe { Output::new(core::ptr::read(&p.PIN_4), Level::High).set_high() }
        5 => unsafe { Output::new(core::ptr::read(&p.PIN_5), Level::High).set_high() }
        6 => unsafe { Output::new(core::ptr::read(&p.PIN_6), Level::High).set_high() }
        7 => unsafe { Output::new(core::ptr::read(&p.PIN_7), Level::High).set_high() }
        8 => unsafe { Output::new(core::ptr::read(&p.PIN_8), Level::High).set_high() }
        9 => unsafe { Output::new(core::ptr::read(&p.PIN_9), Level::High).set_high() }
        10 => unsafe { Output::new(core::ptr::read(&p.PIN_10), Level::High).set_high() }
        11 => unsafe { Output::new(core::ptr::read(&p.PIN_11), Level::High).set_high() }
        12 => unsafe { Output::new(core::ptr::read(&p.PIN_12), Level::High).set_high() }
        13 => unsafe { Output::new(core::ptr::read(&p.PIN_13), Level::High).set_high() }
        14 => unsafe { Output::new(core::ptr::read(&p.PIN_14), Level::High).set_high() }
        15 => unsafe { Output::new(core::ptr::read(&p.PIN_15), Level::High).set_high() }
        16 => unsafe { Output::new(core::ptr::read(&p.PIN_16), Level::High).set_high() }
        17 => unsafe { Output::new(core::ptr::read(&p.PIN_17), Level::High).set_high() }
        18 => unsafe { Output::new(core::ptr::read(&p.PIN_18), Level::High).set_high() }
        19 => unsafe { Output::new(core::ptr::read(&p.PIN_19), Level::High).set_high() }
        20 => unsafe { Output::new(core::ptr::read(&p.PIN_20), Level::High).set_high() }
        21 => unsafe { Output::new(core::ptr::read(&p.PIN_21), Level::High).set_high() }
        22 => unsafe { Output::new(core::ptr::read(&p.PIN_22), Level::High).set_high() }
        23 => unsafe { Output::new(core::ptr::read(&p.PIN_23), Level::High).set_high() }
        24 => unsafe { Output::new(core::ptr::read(&p.PIN_24), Level::High).set_high() }
        25 => unsafe { Output::new(core::ptr::read(&p.PIN_25), Level::High).set_high() }
        26 => unsafe { Output::new(core::ptr::read(&p.PIN_26), Level::High).set_high() }
        27 => unsafe { Output::new(core::ptr::read(&p.PIN_27), Level::High).set_high() }
        28 => unsafe { Output::new(core::ptr::read(&p.PIN_28), Level::High).set_high() }
        29 => unsafe { Output::new(core::ptr::read(&p.PIN_29), Level::High).set_high() }
        _ => {}
    }
    */
}

pub fn off(num: u8) {
    /*
    let mut p = unsafe { embassy_rp::Peripherals::steal() };
    match num {
        0 => unsafe { Output::new(core::ptr::read(&p.PIN_0), Level::Low).set_low() }
        1 => unsafe { Output::new(core::ptr::read(&p.PIN_1), Level::Low).set_low() }
        2 => unsafe { Output::new(core::ptr::read(&p.PIN_2), Level::Low).set_low() }
        3 => unsafe { Output::new(core::ptr::read(&p.PIN_3), Level::Low).set_low() }
        4 => unsafe { Output::new(core::ptr::read(&p.PIN_4), Level::Low).set_low() }
        5 => unsafe { Output::new(core::ptr::read(&p.PIN_5), Level::Low).set_low() }
        6 => unsafe { Output::new(core::ptr::read(&p.PIN_6), Level::Low).set_low() }
        7 => unsafe { Output::new(core::ptr::read(&p.PIN_7), Level::Low).set_low() }
        8 => unsafe { Output::new(core::ptr::read(&p.PIN_8), Level::Low).set_low() }
        9 => unsafe { Output::new(core::ptr::read(&p.PIN_9), Level::Low).set_low() }
        10 => unsafe { Output::new(core::ptr::read(&p.PIN_10), Level::Low).set_low() }
        11 => unsafe { Output::new(core::ptr::read(&p.PIN_11), Level::Low).set_low() }
        12 => unsafe { Output::new(core::ptr::read(&p.PIN_12), Level::Low).set_low() }
        13 => unsafe { Output::new(core::ptr::read(&p.PIN_13), Level::Low).set_low() }
        14 => unsafe { Output::new(core::ptr::read(&p.PIN_14), Level::Low).set_low() }
        15 => unsafe { Output::new(core::ptr::read(&p.PIN_15), Level::Low).set_low() }
        16 => unsafe { Output::new(core::ptr::read(&p.PIN_16), Level::Low).set_low() }
        17 => unsafe { Output::new(core::ptr::read(&p.PIN_17), Level::Low).set_low() }
        18 => unsafe { Output::new(core::ptr::read(&p.PIN_18), Level::Low).set_low() }
        19 => unsafe { Output::new(core::ptr::read(&p.PIN_19), Level::Low).set_low() }
        20 => unsafe { Output::new(core::ptr::read(&p.PIN_20), Level::Low).set_low() }
        21 => unsafe { Output::new(core::ptr::read(&p.PIN_21), Level::Low).set_low() }
        22 => unsafe { Output::new(core::ptr::read(&p.PIN_22), Level::Low).set_low() }
        23 => unsafe { Output::new(core::ptr::read(&p.PIN_23), Level::Low).set_low() }
        24 => unsafe { Output::new(core::ptr::read(&p.PIN_24), Level::Low).set_low() }
        25 => unsafe { Output::new(core::ptr::read(&p.PIN_25), Level::Low).set_low() }
        26 => unsafe { Output::new(core::ptr::read(&p.PIN_26), Level::Low).set_low() }
        27 => unsafe { Output::new(core::ptr::read(&p.PIN_27), Level::Low).set_low() }
        28 => unsafe { Output::new(core::ptr::read(&p.PIN_28), Level::Low).set_low() }
        29 => unsafe { Output::new(core::ptr::read(&p.PIN_29), Level::Low).set_low() }
        _ => {}
    }
    */
}