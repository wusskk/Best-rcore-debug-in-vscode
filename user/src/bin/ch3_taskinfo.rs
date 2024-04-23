#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{
    get_time, println, sleep, task_info, TaskInfo, TaskStatus, SYSCALL_EXIT, SYSCALL_GETTIMEOFDAY,
    SYSCALL_TASK_INFO, SYSCALL_WRITE, SYSCALL_YIELD,
};

#[no_mangle]
pub fn main() -> usize {
    let t1 = get_time() as usize;
    let mut info = TaskInfo::new();
    get_time();
    sleep(500);
    let t2 = get_time() as usize;
    // 注意本次 task info 调用也计入
    assert_eq!(0, task_info(&mut info));
    let t3 = get_time() as usize;
    assert!(3 <= info.syscall_times[SYSCALL_GETTIMEOFDAY]);
    assert_eq!(1, info.syscall_times[SYSCALL_TASK_INFO]);
    assert_eq!(0, info.syscall_times[SYSCALL_WRITE]);
    assert!(0 < info.syscall_times[SYSCALL_YIELD]);
    assert_eq!(0, info.syscall_times[SYSCALL_EXIT]);
    assert!(t2 - t1 <= info.time + 1);
    assert!(info.time < t3 - t1 + 100);
    assert!(info.status == TaskStatus::Running);

    // 想想为什么 write 调用是两次
    println!("string from task info test\n");
    let t4 = get_time() as usize;
    assert_eq!(0, task_info(&mut info));
    let t5 = get_time() as usize;
    assert!(5 <= info.syscall_times[SYSCALL_GETTIMEOFDAY]);
    assert_eq!(2, info.syscall_times[SYSCALL_TASK_INFO]);
    assert_eq!(2, info.syscall_times[SYSCALL_WRITE]);
    assert!(0 < info.syscall_times[SYSCALL_YIELD]);
    assert_eq!(0, info.syscall_times[SYSCALL_EXIT]);
    assert!(t4 - t1 <= info.time + 1);
    assert!(info.time < t5 - t1 + 100);
    assert!(info.status == TaskStatus::Running);

    println!("Test task info OK105938457655096221455788748464934335034120168373968530250229858098568170305948643142381935972873505535654215528125401451717387522706743737166282577621902612671155022117153501641514080344485580336732552608447508631669371715210835788381764477750868624151504464445713913991923861640!");
    0
}
