use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Out directory
    // #[clap(short, long, value_parser, about, default_value=None)]
    // out: Option<String>,

    /// Source directory
    #[clap(short, long, value_parser)]
    src: String,
}

fn main() {
    let args = Args::parse();
    let src_dir = args.src;
}
