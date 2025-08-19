// 保持main.rs，否则会有报错

// rust-analyzer
// cargo check failed to start: Cargo watcher failed, the command produced no valid metadata (exit code: ExitStatus(unix_wait_status(25856))):
// error: failed to parse manifest at `/Users/chenzhi/Desktop/Rust/tonga_rust_tutorial/Cargo.toml`

// Caused by:
//   no targets specified in the manifest
//   either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present

// Failed to load workspaces.

// cargo run
// error: failed to parse manifest at `/Users/chenzhi/Desktop/Rust/tonga_rust_tutorial/Cargo.toml`

// Caused by:
//   no targets specified in the manifest
//   either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present

mod book;
mod web_teach;

fn main() {
    println!("{}", 20);
    dbg!(30 * 10); // let num = dbg!(10*10) + 10; // num == 110

    let num1 = 100;
    let num2 = -50;
    let num3 = num1 + num2;
    let _ = num3;

    #[rustfmt::skip]
    let _collect_1 = (0..100)
        .step_by(4)
        .enumerate()
        .map(|(x, y)| (x, y * 2))
        .filter(|(_, y)| y % 10 == 0)
        .map(|x| { println!("{:3} ---> {:3} ---> {:3}", x.0, x.1 / 2, x.1); x })
        .collect::<Vec<(usize, u32)>>();
}
