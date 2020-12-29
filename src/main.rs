use std::{
    error,
    io::{self, prelude::*},
    net, str, thread,
};

// ①ローカルマシン（127.0.0.1）の50000ポートで接続を待ち受ける
// ②接続の確立要求がきたら新しくスレッドを起動し、ハンドラを呼び出す
// ③クライアントから送られてきたデータを読み取り、そのまま送り返す
fn main() -> Result<(), Box<dyn error::Error>> {
    println!("start echo server...");
    // (1) ソケットの生成とローカルアドレスへの紐付け
    let listener = net::TcpListener::bind("127.0.0.1:50000")?; // ?演算子 -> Resultをunwrap
    loop {
        let (stream, _) = listener.accept()?; // (2) 接続の待ち受け

        // 複数のクライアントに対処するため新しいスレッドを起動します。
        // 1つのクライアントとやりとりを行う専用のスレッドを用意することで、
        // サーバはクライアントの接続要求と、接続済みクライアントとのメッセージを複数扱うことができます。
        thread::spawn(move || { handler(stream).unwrap(); });
    }
}

// クライアントが接続しにきたときの処理
fn handler(mut stream: net::TcpStream) -> Result<(), Box<dyn error::Error>> {
    println!("incoming connection from {}.", stream.peer_addr()?);
    loop {
        let mut reader = io::BufReader::new(&stream);
        let mut buf = vec![];
        match reader.read_until(b'\n', &mut buf)? {
            0 => {
                println!("connection closed.");
                return Ok(());
            }
            n => {
                println!("{}", str::from_utf8(&buf[..n])?);
                stream.write_all(&buf[..n])?;// (4) ソケットへの書き込み
            }
        }
    }
}