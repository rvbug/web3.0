# Web 3.0
#### `Entering Web 3.0 via Rust & Solana`



# Rust Basics

## Installation

1. create a virtual env (pipenv/conda). I am using conda
   * `conda create -n rust`
   * `conda activate rust`

2. visit Rust home page [https://www.rust-lang.org/learn/get-started]
   * run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   * check the installation by running.
     - which cargo
     - which rustc
   * install vscode (or editor of your choice)
      - install crates pkg (dependency in rust are managed by Cargo.toml)
      - install rust

# Introduction

3. Notes
  - 3.1 - first program
     * run `cargo new hello-world --bin` creates a folder
     * the src folder contains main.rs file
       - ! is a macro different from function (fn) `macro writes more code`
     * `cd hello-world`
     * compile by running `cargo run` or `rustc main.rs`
     * compiled code is in `target/debug/` -> ./hello-world
     
  - 3.2 variables
    * `let a = 10` - this is immutable by default, reassiging will throw error
    * `let mut a = 20` - reassign to 20
    
  - 3.3 types
    * `let var: i32 = 200` - 32 bit integer  
    *  rust is statically typed - the variable types cannot be changes
    *  `int` have signed and unsigned and are represented by `i8`, `i16`, `i32`,`i64` etc
    *  `float` represented by `f32`, `f64` etc
    *  `char` are utf8 or ascii
    *  `bool` - true or false
    `* composite types*`
       - tuples have multiple data types 
         - `let tup = (10, "hi", true)` access by `a.0` etc
         -  let (t1, t2, _ ) = tup
       - array - same data type (mutability rule gets applied here)
         - `let arr = [10, 20, 30]` access by arr[0] etc
         - `let arr = [1; 5]`  for printing 1s 5 times {:?}
         - `let arr : [f32; 5] = [1; 5]` for delaring the type
    * Conditions, loops & iterators
        - `if - else if - else`
        - `loop { if... } | loop { while... } `
        -  `for `
        -  `for y in 0..10` - prints from 0 to 9
        -  `for y in 0..=10` - prints including 10
        - `for x in arr` when arr = ['a', 'b', 'c']  
    * match - combination of switch statment + if condition
        -  `match x { }`
        -  `match (x, y) {}`
    
    
    
    
    
    
