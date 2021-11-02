use std::env;
use std::fs;


fn main() -> std::io::Result<()> {
    // get arguments
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 3 {
        println!("引数を2個指定してください");
        std::process::exit(1);
    }

    let src = &argv[1]; // source file
    let dest = &argv[2]; // destination file
    fs::copy(src, dest)?; // copy to destination
    Ok(())
}
