// pub mod p_03_install;
//
// 这个官方网址可以看到一些详细的介绍
// https://www.rust-lang.org/

// 官方安装教程
// https://www.rust-lang.org/zh-CN/tools/install
// 根据默认的方式安装（MSVC｜GNU）
//
// rustup update
// rustup self uninstall
//
// mac os上使用curl

// cargo new [project_name]
// cd ls
// .gitignore
// src/main.rs
// src/cargo.toml
// cargo run

// pub mod p_04_rustrover_analyzer;
//
// jetbrains官网
// https://www.jetbrains.com/
// https://www.jetbrains.com/rust/

// 如果使用 vs code
// 安装 rust-analyzer 插件来获得inlay hint

#[test]
fn example_program() {
    println!("Hello, world!");
}

#[test]
fn test_read_input() {
    // fn read_input(reader: std::io::BufRead) -> String
    // this is wrong, the trait can not be the params directly
    fn read_input<R: std::io::BufRead>(reader: R) -> String {
        let mut lines = reader.lines();
        lines.next().unwrap().unwrap().trim().to_string()
    }

    let input_data = b"hello world\n";
    let cursor = std::io::Cursor::new(&input_data[..]);
    let result = read_input(cursor);
    assert_eq!(result, "hello world");
}
