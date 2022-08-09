use std::cmp::Ordering;
use std::io;

use rand::{Rng, thread_rng};

fn main() {
    println!("请输入一个数字: ");
    let r = thread_rng().gen_range(0..=100);

    loop {
        let mut input_num = String::new();
        match io::stdin().read_line(&mut input_num) {
            Ok(_) => {}
            Err(error) => println!("error: {error}"),
        };

        let input_num = match input_num.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("请输入数字：");
                continue;
            }
        };
        match r.cmp(&input_num) {
            Ordering::Less => { println!("太大了！"); }
            Ordering::Equal => {
                println!("正确！初始数字是：{}", r);
                break;
            }
            Ordering::Greater => { println!("太小了！"); }
        }
    }
}
