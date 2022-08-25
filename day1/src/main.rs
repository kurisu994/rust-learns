use crate::simple::simple::simple_display;
use crate::complex::complex_display;

mod simple;
mod complex;

// 单行注释，注释内容直到行尾。

/*
* 块注释，注释内容一直到结束分隔符
*/
/// 文档注释 这是main方法
///  支持 Markdown
/// # 主题
///
/// ```
/// // 在文档注释中，你可以书写代码块
/// // 如果向 `rustdoc` 传递 --test 参数，它还会帮你测试注释文档中的代码！
/// println!("Hello, world2!");
/// ```
fn main() {
    simple_display();
    complex_display();
}

