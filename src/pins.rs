use embassy_rp::gpio::{Level, Output};
use embassy_rp::Peripherals;

fn steal_p() -> Peripherals {
    unsafe { embassy_rp::Peripherals::steal() }
}

/// This is the third function. It takes the Output and "forgets" it
/// so the hardware registers aren't reset when the variable goes out of scope.
fn leak_output(out: Output) {
    // Calling forget prevents the Drop implementation from running.
    core::mem::forget(out);
}

pub fn on(num: u8) {
    let p = steal_p(); 
    match num {
        0 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_0), Level::High)) }
        1 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_1), Level::High)) }
        2 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_2), Level::High)) }
        3 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_3), Level::High)) }
        4 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_4), Level::High)) }
        5 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_5), Level::High)) }
        6 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_6), Level::High)) }
        7 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_7), Level::High)) }
        8 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_8), Level::High)) }
        9 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_9), Level::High)) }
        10 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_10), Level::High)) }
        11 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_11), Level::High)) }
        12 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_12), Level::High)) }
        13 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_13), Level::High)) }
        14 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_14), Level::High)) }
        15 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_15), Level::High)) }
        16 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_16), Level::High)) }
        17 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_17), Level::High)) }
        18 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_18), Level::High)) }
        19 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_19), Level::High)) }
        20 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_20), Level::High)) }
        21 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_21), Level::High)) }
        22 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_22), Level::High)) }
        23 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_23), Level::High)) }
        24 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_24), Level::High)) }
        25 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_25), Level::High)) }
        26 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_26), Level::High)) }
        27 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_27), Level::High)) }
        28 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_28), Level::High)) }
        29 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_29), Level::High)) }
        _ => {}
    }
}

pub fn off(num: u8) {
    let p = steal_p();
    match num {
        0 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_0), Level::Low)) }
        1 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_1), Level::Low)) }
        2 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_2), Level::Low)) }
        3 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_3), Level::Low)) }
        4 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_4), Level::Low)) }
        5 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_5), Level::Low)) }
        6 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_6), Level::Low)) }
        7 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_7), Level::Low)) }
        8 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_8), Level::Low)) }
        9 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_9), Level::Low)) }
        10 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_10), Level::Low)) }
        11 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_11), Level::Low)) }
        12 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_12), Level::Low)) }
        13 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_13), Level::Low)) }
        14 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_14), Level::Low)) }
        15 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_15), Level::Low)) }
        16 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_16), Level::Low)) }
        17 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_17), Level::Low)) }
        18 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_18), Level::Low)) }
        19 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_19), Level::Low)) }
        20 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_20), Level::Low)) }
        21 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_21), Level::Low)) }
        22 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_22), Level::Low)) }
        23 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_23), Level::Low)) }
        24 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_24), Level::Low)) }
        25 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_25), Level::Low)) }
        26 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_26), Level::Low)) }
        27 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_27), Level::Low)) }
        28 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_28), Level::Low)) }
        29 => unsafe { leak_output(Output::new(core::ptr::read(&p.PIN_29), Level::Low)) }
        _ => {}
    }
}