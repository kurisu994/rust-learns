use std::fmt;
use std::fmt::Formatter;

// 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
#[derive(Debug)]
pub struct Structure(pub i32);

// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Structure {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}

pub fn show() {
    // debug方式打印
    println!("This is struct `{:?}`...", Structure(3));
    // Display
    println!("This is struct: {}", Structure(3));
}
