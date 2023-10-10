// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

#[macro_use]
//#[macro_use] 是 Rust 编程语言中的一个属性，用于宏（macro）的使用者表明他们希望在代码中使用这个宏。
//当编译器遇到 #[macro_use] 时，它会检查宏定义中的 include 属性，
//并确保在使用宏的地方包含了正确的头文件。这有助于避免因缺少头文件而导致的编译错误。
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
