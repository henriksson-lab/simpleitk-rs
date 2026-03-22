use simpleitk::io;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: read_write <input> <output>");
        std::process::exit(1);
    }

    let image = io::read_image(&args[1]).expect("failed to read image");
    println!("{image:?}");
    io::write_image(&image, &args[2]).expect("failed to write image");
}
