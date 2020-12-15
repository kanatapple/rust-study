use std::fmt;

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Num(cause) => write!(f, "Parse Error: {}", cause),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(cause: std::io::Error) -> Self {
        Self::Io(cause)
    }
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";
    // ?はResult型を返す関数で使える演算子
    // 「その直前の結果のResult型の値がOk(t)であればtを返し、Err(e)であればErr(e)で早期リターンして関数を終了する」
    let num_str = std::fs::read_to_string(path).map_err(MyError::from)?;

    num_str
        .trim()
        .parse::<i32>()
        // Result<T,E>型に対しては、Ok(T)の場合に作用する処理とErr(E)の場合にのみ作用する処理を記述できる
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e))
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}