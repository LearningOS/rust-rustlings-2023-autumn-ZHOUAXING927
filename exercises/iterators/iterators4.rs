// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    let mut result = 1;
    (2..=num).map(|x|
        { result *= x; }
    ).collect::<Vec<_>>(); //使用 collect::<Vec<_>>() 将结果收集到一个向量中，并返回该向量作为最终结果。
    //collect::<Vec<_>>() 是 Rust 中用于将迭代器的结果收集到一个向量中的语法。它的作用是将 map 或 filter 等迭代器方法返回的迭代器结果转换为一个向量。
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
