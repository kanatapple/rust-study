/* 所有権ごと渡してしまっている例
fn main() {
    let important_data = "Hello World!".to_string();

    // 所有権も渡してしまってるので、関数実行後のimportant_dataの所有権がない
    calc_data(important_data);

    println!("{}", important_data);
}

fn calc_data(data: String) {
    println!("{}", data);
}
 */

/* 呼び出し元に所有権を返す例
fn main() {
    let mut important_data = "Hello World!".to_string();

    important_data = calc_data(important_data);

    println!("{}", important_data);
}

fn calc_data(data: String) -> String {
    println!("{}", data);
    // 呼び出し元に所有権を返す
    data
}
 */

/* 借用の例
fn main() {
    let important_data = "Hello World!".to_string();

    calc_data(&important_data);

    println!("{}", important_data);
}

fn calc_data(data: &String) {
    println!("{}", data);
}
 */

/* 不変な参照渡しは複数あっていい
fn main() {
    let x = 5;

    let y = &x; // 1回目の不変な参照渡し
    let z = &x; // 2回目の不変な参照渡し

    dbg!(x);
    dbg!(y);
    dbg!(z);
}
 */

/* 可変な参照渡しは複数あってはダメな例と不変な参照渡しと可変な参照渡しは同時に存在できない例
fn main() {
    let mut x = 5;
    {
        let y = &mut x; // 1回目の可変な参照渡し
        let z = &mut x; // 2回目の可変な参照渡し (エラー)

        dbg!(y);
        dbg!(z);
    }

    {
        let y = &x; // 不変な参照渡し
        let z = &mut x; // 可変な参照渡し

    }
}
 */

/* 参照は所有者のライフタイムより長く存在できない例
fn main() {
    let y;
    {
        let x = 5;
        y = &x;
        dbg!(x);
    }
    dbg!(y);
}
 */
