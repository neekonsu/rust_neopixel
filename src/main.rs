#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; /* implements default panic due to #![no_std] macro */

#[entry] /* Manual entrypoint annotation due to #![no_main] macro */
fn main() -> !
{
    loop {}
}
