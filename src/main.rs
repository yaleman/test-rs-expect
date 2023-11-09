fn main() {
    println!("Hello, world!");
    let foo = dialoguer::Confirm::new()
        .with_prompt("Are you sure?")
        .interact();
    println!("Result: {:?}", foo);
}

#[test]
fn test_my_cmd() -> Result<(), rexpect::error::Error> {
    println!("Starting...");
    let mut p = rexpect::spawn("cargo run", Some(5000))?;
    let res = p.exp_regex(r#".*Are you sure\?.*"#)?;
    println!("Res after are you sure: {:?}", res);
    // you'll get this because it's doing the build and start:
    // ```
    // ("\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m   Compiling\u{1b}[0m test-rs-expect v0.1.0 (/Users/yaleman/Projects/test-rs-expect)\r\n\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m    Finished\u{1b}[0m dev [unoptimized + debuginfo] target(s) in 0.10s\r\n\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m     Running\u{1b}[0m `target/debug/test-rs-expect`\r\nHello, world!\r\n", "Are you sure? [y/n] \u{1b}[?25l")
    // ```
    let res = p.send("y\n")?; // you have to send the newline in test mode because it can't do the live readline checks that it can in a terminal
    println!("Res after sending y: {:?}", res);
    p.exp_regex(".*Result: Ok\\(true\\).*")?;
    println!("Res after res: {:?}", res);
    Ok(())
}
