 fn main(){
    let s1 = String::from("hello");         //ownership
    let (s2,len) = calculate_length(s1);
    println!("the length of '{s2}' is {len}.");

 }

 fn calculate_length(s:String)->(String,usize){
    let length = s.len();
    (s,length)
 }

 /*reference --  we just  point to the address using reference so no ownership borrowed - unlike pointers reference 
    will point to a valid value of a particular type for the life of that reference.*/

 fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s:&String)->usize{
 s.len()
}

/*The opposite of referencing by using & is dereferencing, 
which is accomplished with the dereference operator, */

//immutable referencw -- - we cant change the value here''
fn main() {
    let s = String::from("hello");

    change(&s);
}
                                        //doesnt work 
fn change(some_string: &String) {
    some_string.push_str(", world");
}

//Mutable references
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {   //we borrow & mut s 
    some_string.push_str(", world");
} 

/*Mutable references have one big restriction
we cannot borrow s as mutable more than once at a time
 */
    fn main(){
           let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;                            

    println!("{r1}, {r2}");
  }
/*
 this doesnt work - bcause cant modifty the same data more than once at the same time 
 */
fn main(){
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{r1}");
                                   // this works as the reference is not being used at the same time and sope of first one ended
    let r2 = &mut s;
    println!("{r2}");
}

//by creating a scope we an use mmultiple reference 

fn main(){
     let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}


// We also cannot have a mutable reference while we have an immutable one to the same value.
fn main(){
        let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM           //doesnt work  so no normal reference and mutable reference allowed at the smae time

    println!("{r1}, {r2}, and {r3}");

}

//we we use after the modification it works 
 fn main(){
     let mut  s =String::from("hello");
     let s1 = &s;
     let s2 = &s;
     
     
     println!("{s1},{s2}");
     let s3 =yoo(&mut s);        //works fine   bcs scope of s1 and s2 ended at println!
     println!("{s3}")
 }
 
 fn yoo(x:&String)->usize{
     x.len()
 }

 //another example

     fn main(){
            let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

     }

     // ***Dangling References
     fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

//explanatiuon
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!

  //solution
  fn no_dangle() -> String {
    let s = String::from("hello");

    s
}