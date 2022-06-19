use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut file = File::create("../../../test.txt")?;

    for x in 0..10000 {
        file.write_fmt(format_args!("12345678{:04}\n", x))?;
    }

    Ok(())
}
