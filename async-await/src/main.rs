use futures::{executor, Future};

async fn async_add(left: i32, right: i32) -> i32 {
    println!("left: {} right: {}", left, right);
    left + right
}

async fn something_great_async_function() -> i32 {
    let ans1 = async_add(2, 3).await;
    println!("{}", ans1);
    let ans2 = async_add(3, 4).await;
    println!("{}", ans2);
    let ans3 = async_add(4, 5).await;
    println!("{}", ans3);
    let result = ans1 + ans2 + ans3;
    println!("result: {}", result);
    result
}

// async fn は impl Future の糖衣構文
// 下記のように Future を直接記述しても同じだが、見通しがよくなるメリットがあるため async/.await が使われることが多い
// fn something_great_async_function() -> impl Future<Output = i32> {
//     async {
//         let ans = async_add(2, 3).await;
//         println!("{}", ans);
//         ans
//     }
// }

async fn print_result(value: i32) {
    println!("{}", value);
}

async fn calculate() -> i32 {
    let add1 = async_add(2, 3).await;
    // 出力されない。正しくは print_result(add1).await
    print_result(add1);

    // 出力される
    let add2 = async_add(3, 4).await;
    print_result(add2).await;

    async_add(add1, add2).await
}

fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_variable = "this is outside".to_string();
    // 通常なら、ここで outside_variable の所有権がなくなりコンパイルエラーになる
    async move {
        // move キーワードを使い変数の所有権を async ブロックの中に移したのでコンパイルエラーにならない
        println!("{}", outside_variable);
    }
}

// async fn some_great_function(arg: &i32) -> i32 {
//     *arg
// }
// コンパイラは上記関数を下記のように展開
// ライフタイム 'a を持ち、戻り値が Future である関数
// fn some_great_function<'a>(arg: &'a i32) -> impl Future<Output = i32> + 'a {
//     async move {
//         *arg
//     }
// }

fn some_great_function() -> impl Future<Output = i32> {
    // let value: i32 = 5;
    // value の所有権はここで終わるのでコンパイルエラーになる
    // send_to_another_thread_with_borrowing(&value)

    // async ブロック内で value を宣言することで所有権を引き延ばせる
    async {
        let value: i32 = 5;
        send_to_another_thread_with_borrowing(&value).await
    }
}

async fn send_to_another_thread_with_borrowing(x: &i32) -> i32 {
    *x
}
fn main() {
    executor::block_on(something_great_async_function());
    executor::block_on(calculate());
}
