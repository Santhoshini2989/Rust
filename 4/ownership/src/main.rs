 fn main(){
    let mut s = String::from("hello");
    s.push_str(",world");
    println!("{s}");

 }// this string is mutated - - memory allocation!!!
 //Rust takes a different path: The memory is automatically returned once the variable that owns it goes out of scope. 
 

 fn main(){
    let s1 = String::from("hello"); //  no longer valid because we assigned to s2 - - now we only have one heap memory 
    let s2 = s1;
    println!("{s2}"); // here only we can print s2 and not s1 becasuse we made s1 invalid and allocated the s1 memory t othe s2
 }
//here ownership is transfered to s2 so s1 no longer valid
//the one who is owner at the end of the code will drop the ownership hence frees memory


 //(shallow copy - called move ) - - the ownership moves make the prev invalid

 //copy - -  copy of the stack data nothing much is here bcs we already know the size of the stack and making copy is easy

 /*  clone() method  - - deep copy - - mostly heap memory also copied so more expensive atleast we know 
         here that it takes more moemory*/
  fn main(){  
   
     let s1 = String::from("hello");
     let s2 = s1.clone();

     println!("s1 = {s1}, s2 = {s2}");
} 

//Ownership and Functions

 fn main(){
    
    let s = String::from("hello");              
    takes_ownership(s);

    let x = 5;
    make_copy(x);

 }

 fn takes_ownership(some_string:String){
   println!("{some_string}");
 }

 fn make_copy(some_integer:i32){
   println!("{some_integer}");
 }


 /*first s is the owner - - the ownership is transfered to the function 
 second x will copy it as it is a stack 

 we cant use s afterwards but can use x 

 finally in the main function only x goes out of scope
  */


  //Return Values and Scope

  fn main(){

   let s1 = gives_ownership();
   let s2 = String::from("hello");
   let s3 = take_and_gives_back(s2);
  }

  fn gives_ownership()->String{
    let some_string = String::from("yours");
    some_string
  }

  fn take_and_gives_back(a_string:String)->String{
   a_string
  }

  /*
     fn moves its retuen to s1 - s1 is owner
     s2 is owner
     s2 -fn2 -s3 s2 lost ownership
     s3 is owner
     final owners are s1 and s3 they will be droped and s2 was made invalid(moved)
   */


  //Rust does let us return multiple values using a tuple

  fn main(){
   let s1 = String::from("hello");
   let (s2,len)=calculate_length(s1);
   println!("The length of '{s2}' is {len}.");
  }
  
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
/*fist s1 is the owner then 
in s2 ----- s1- -fn (s)--s2(s2 is the owner) 
len is len 
finally s2 is the owner s1 was moved 
so s2 and len drops and goes out of scope  

*/