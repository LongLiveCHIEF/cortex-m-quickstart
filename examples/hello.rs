//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
  rtt_init_print!();

    loop {
      rprintln!("Hello, world!")
    }
}
