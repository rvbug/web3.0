//this is a comment

fn main() {
    
    println!("Hello world from Rust!");

    let var = 498;
     println!("the value of var is {} ", var);
     var = 100;
     println!("value of var is now {} ", var);

    //mutuable - not required
     let mut mutvar = 100;
      println!("the mutable value is {}", mutvar);

    //signed int
    let signed_int : i8 = 127; // -128 to 127
    println!("{}", signed_int);
    
    //unsigned int
    let usigned : u8 = 129;
    println!("{}",usigned);
    
    //char
    let ch = "char";
    println!("{}", ch);
    
    //boolean
    let b = true //false

    //tuple
    let t = (10, "hi", true);
    println!("{}", t.0);
    
    //array
    let arr: [f32; 5] = [1.0; 5];
    println!("{:?}", arr);
    //mutuable array
    let mut arr1 = [0];
    arr1[0] = 1;
    println!("{}", arr[0]);
    
    let age = 20;
    if age >= 100{
        println!("Congratulations @ {} ", age)
    }
    else if age < 100  && age > 18 {
        println!("Enjoy life @ {} ", age)
    }
    else {
        println!("Not old enough to vote @ {} ", age)
    }   
    
    
    //loops with for
    let mut y = 10;
    loop {
        if y > 5000 {
            break;
        }
        y = y* 2;
        println!("{}", y); 
    }
    println!("completed...");
    
    //loop with while
     loop {
        while y <= 500 {
            y = y * 2;
            println!("{}", y);
        }   
    }
    
    // prints from 0 to 9 only
    for x in 0..10 {
        println!("{ \n }", x);
    }
    
    //print all inclusive
    for x in 0..=10 {
        println!("{ \n }", x);
    }
    
    //for loops and iterators
    let x = ['a', 'b', 'c'];
    for i in x {
        println!("{}", i);
    }
    
    let v = 'a';
    match v {
        'a' => println!("value = {}", v),
        'b' => println!("value = {}", v),
        _   => println!("rest of the value"),
    }
   
    //non-exhaustive patterns
   let x = true;
   let y = false;
   match (x, y) {
       (true, true) => println!("x is true y is false"),
       _ => println!("all other combination")
   }
    
}
