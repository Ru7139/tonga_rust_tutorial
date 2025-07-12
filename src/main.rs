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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!("Hello");

    let a: u32 = (1..=10)
        .step_by(1)
        .into_iter()
        .enumerate()
        .map(|(index, value): (usize, u32)| {
            let num = value.pow(index as u32);
            println!("{}", num);
            num
        })
        .sum();
    dbg!(a);

    Ok(())
}
