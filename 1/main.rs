 fn main() {
    println!("Hello, world!");
}



//println! - -  rust macro  - - - -  println - - rust function


/*compilation 
1.rustc main.rs
2..\main */

//CARGO - -  

/*
cargo - - rust package manager - -  cargo init to generate cargo.toml file
crates - - packages of code */


/*Creating a Project with Cargo

$ cargo new hello_cargo
$ cd hello_cargo

 */


/*Running cargo 
method 1 -- cargo build then .\target\debug\hello_cargo.exe

metho2 - - 
cargo run - -  - compiles and runs the code 

*/
//cargo check -- just compiles and doesnt run (executable)


/*

recap what we’ve learned so far about Cargo:

We can create a project using cargo new.
We can build a project using cargo build.
We can build and run a project in one step using cargo run.
We can build a project without producing a binary to check for errors using cargo check.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
 */
// cargo build --release - - - - 
 