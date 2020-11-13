/// This function adds 2 numbers.
///
/// # Example
///
/// ```
/// use test_module::add;
///
/// add(1, 2);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// #[cfg(test)]アトリビュートを指定した tests モジュールは cargo test の時だけバイナリに含まれる
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_eq!(add(2, 2), 4);
    }
}
