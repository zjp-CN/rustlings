// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    if (y - x).abs() > f64::EPSILON {
        // issue:
        // [clippy1.rs passed without any modifications](https://github.com/rust-lang/rustlings/issues/888)
        //
        // BTW 你可以通过
        // https://github.com/rust-lang/rust-clippy#as-a-rustc-replacement-clippy-driver
        // 即 `clippy-driver clippy1.rs` 来对单个文件（非 Cargo 项目）检测
        // if y != x {
        println!("Success!");
    }
}
