use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author="myuan", version="0.1", about="华大基因云盘批量下载器", long_about = None)]
struct Args {
    /// shared url from BGI
    #[clap(short, long, value_parser)]
    url: String,

    /// export to
    #[clap(short, long, value_parser, default_value = "./download-from-bgi")]
    export_root: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello {:?}!", args);
}
