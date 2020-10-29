/*
// スレッドの例
use std::thread;

fn main() {
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello, world!: {}", x);
        }));
    }

    for handle in handles {
        handle.join();
    }
}
 */
/*
// 共有メモリの例
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            // lock を使って data への可変参照を得る
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        handle.join();
    }

    dbg!(data);
}
 */
// メッセージパッシングの例
use std::sync::mpsc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        // main から各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();
        // 各スレッドから main へのチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    // 各スレッドに data の値を送信
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    // 各スレッドからの結果を data に格納
    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }

    for handle in handles {
        handle.join();
    }

    dbg!(data);
}
