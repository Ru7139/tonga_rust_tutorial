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

    let a: u64 = (1..=10)
        .step_by(1)
        .filter(|x| x % 2 == 1)
        .into_iter()
        .enumerate()
        .map(|(index, value): (usize, u64)| {
            let num = value.pow(index as u32);
            println!("{}, {}, {}", index, value, num);
            num
        })
        .sum();

    let a_ox = min_sum_3x3_matrix(a);
    for row in a_ox.iter() {
        println!("{:?}", row);
    }

    Ok(())
}

fn min_sum_3x3_matrix(n: u64) -> [[u64; 3]; 3] {
    factorize(n)
        .into_iter()
        .fold(vec![1; 9], |mut acc, p| {
            let min_index = acc
                .iter()
                .enumerate()
                .min_by_key(|&(_, &val)| val)
                .map(|(i, _)| i)
                .unwrap();
            acc[min_index] *= p;
            acc
        })
        .chunks(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn factorize(mut n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut d = 2;

    while d * d <= n {
        while n % d == 0 {
            result.push(d);
            n /= d;
        }
        d += 1;
    }

    if n > 1 {
        result.push(n);
    }

    result
}

// fn main() {
//     let n = 360;
//     let mat = min_sum_3x3_matrix(n);
//     for row in mat.iter() {
//         println!("{:?}", row);
//     }
// }
