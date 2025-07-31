use ferris_says::say;

use std::io::{stdout,BufWriter};

fn main(){
    let stdout = stdout();
     let message = String::from("hello this is my first Rust code!");
     let width =message.chars().count();

     let mut writer = BufWriter::new(stdout.lock());
     say(&message,width,&mut writer).unwrap();
}


// Cargo: the Rust build tool and package manager

/*
build your project with cargo build
run your project with cargo run
test your project with cargo test
build documentation for your project with cargo doc
publish a library to crates.io with cargo publish

