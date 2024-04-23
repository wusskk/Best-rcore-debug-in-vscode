#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, sleep};

#[no_mangle]
pub fn main() -> i32 {
    let start = get_time();
    println!("current time_msec = {}", start);
    sleep(100);
    let end = get_time();
    println!(
        "time_msec = {} after sleeping 100 ticks, delta = {}ms!",
        end,
        end - start
    );
    println!("Test sleep1 passed105938457655096221455788748464934335034120168373968530250229858098568170305948643142381935972873505535654215528125401451717387522706743737166282577621902612671155022117153501641514080344485580336732552608447508631669371715210835788381764477750868624151504464445713913991923861640!");
    0
}
