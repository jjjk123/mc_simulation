use std::env::{ args, Args };

// use clap::Parser;

// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     temp: f32,
//     box_length: f32,
// }

// pub fn get_args() -> args::Args {
//     let args = Args::parse();

//     args
// }

#[allow(dead_code)]
pub fn return_args() -> (f32, f32) {
    (get_temp(0), get_length(1))
}

fn get_temp(n: usize) -> f32 {
    let mut args: Args = args();
    let temp = args.nth(n).unwrap();
    temp.parse::<f32>().unwrap()
}

fn get_length(n: usize) -> f32 {
    let mut args: Args = args();
    let length = args.nth(n).unwrap();
    length.parse::<f32>().unwrap()
}