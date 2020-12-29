# rust-echo-server

Rustで挑戦ネットワークプログラミングコードで理解するTCP/IP
- Software Design (ソフトウェアデザイン) 2021年1月号

## 構成
```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    └── main.rs
```

## 使い方
```
# エコーサーバのビルドと実行
/Users/yuji.hanada/.cargo/bin/cargo run --color=always
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust-echo-server`
start echo server...
incoming connection from 127.0.0.1:50047.
hello # クライアントから受信

connection closed.

# 別ターミナルでnetcatコマンドを実行
[@yuji.hanada] ~/git/rust-echo-server $ nc localhost 50000
hello # エコーサーバへ送信
hello # エコーサーバからの応答
```