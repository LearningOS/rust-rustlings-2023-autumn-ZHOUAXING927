// tests9.rs
//
// rust is highly capable of sharing ffi interfaces with c/c++ and other statically compiled
// languages, and it can even link within the code itself! it makes it through the extern
// block, just like the code below.
//
// the short string after the `extern` keyword indicates which abi the externally imported
// function would follow. in this exercise, "rust" is used, while other variants exists like
// "c" for standard c abi, "stdcall" for the windows abi.
//
// the externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// if you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same abi string note. the default abi
// for rust functions is literally "rust", so if you want to link against pure rust functions,
// the whole extern term can be omitted.
//
// rust mangles symbols by default, just like c++ does. to suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// in this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// you should not modify any existing code except for adding two lines of attributes.

// I AM NOT DONE

extern "Rust" {
    //#[link_name]是一个属性元标量（attribute macro），用于在编译时设置符号链接名称
    #[link_name = "my_demo_function"]
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    //这个属性指示Rust编译器在编译时不要改变这个函数的名字。这意味着这个函数在链接时将保留其原始名字和签名。
    #[no_mangle]
    // No `extern` equals `extern "Rust"`.
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
