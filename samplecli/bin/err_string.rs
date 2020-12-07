fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";
    // ?はResult型を返す関数で使える演算子
    // 「その直前の結果のResult型の値がOk(t)であればtを返し、Err(e)であればErr(e)で早期リターンして関数を終了する」
    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

    num_str
        .trim()
        .parse::<i32>()
        // Result<T,E>型に対しては、Ok(T)の場合に作用する処理とErr(E)の場合にのみ作用する処理を記述できる
        .map(|t| t * 2)
        .map_err(|e| e.to_string())
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
