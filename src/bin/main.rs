use clap::Parser;


fn main() {
    let opts = projector_rust::options::ProjectorOpts::parse();
    println!("{:?}", opts);
}