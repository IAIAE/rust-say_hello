//!
//! 这个是头部的说明
//!
pub fn say_hello() {
    println!("hello world");
}

/// 这个是测试用函数
/// # Examples
/// ```
/// use say_hello;
/// assert_eq!(3, say_hello::add_one(2));
/// ```
///
pub fn add_one(a: i32) -> i32 {
    a + 1
}