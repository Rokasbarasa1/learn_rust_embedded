#![deny(unsafe_code)]
#![no_std]
#![no_main] 

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry] 
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50_u16;

    let mut counter: usize = 0;
    let mut skip: bool = false;
    loop{
        if skip{

            skip = false;
            delay.delay_ms(half_period);
            //This actualy makes the number counter - 1 ,but 
            //i dont have to check for negative then. 
            counter = (counter + 1) % 8;
            panic!("Hello, world!");
        }else{
            leds[counter].on().ok();

            delay.delay_ms(half_period);

            let counter_temp: usize;
            counter_temp = (counter + 7) % 8;
            leds[counter_temp].off().ok();

            skip = true;
        }
    }
}