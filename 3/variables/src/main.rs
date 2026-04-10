 fn main(){
    let mut x = 5;
    println!("The value of x is :{x}");
    x = 6;
    println!("The value of x is :{x}");
 }

//constants
   
const THREE_HOURS_IN_SECONDS :U32 = 60*60*3;

//shadowing
fn main(){

   let x = 5;
   let x = x+1;
   {
      let x = x*2;
      println!("the value of x inner scope  :{x}");
   }
      println!("the value of x  :{x}");

}

 //data types 
fn main(){
   let x:f32 = 3.0;
   let y =5.0; //default f64
}
//integers 
 fn main(){
   let a:u32=20;
   let b:i32 = -5;
 }

 //operations

 fn main(){
   let addition = 5+10;
   let subtraction = 5-2;
   let product = 5*5;
   let quotient = -5/3; // result is -1
   let reminder = 15%2;
 }

 //boolean type 
 fn main(){
   let t = true;
   let f:bool = false;
 }

 //char type 
 fn main(){
   let c ='z';
   let z:char = 'Z';
   let heart_eyed_cat = 'hdhdh';
 }

 /*compound types - - tuple and array both have fised length if not we cant use it tuple is multi type 
 and array is single type with fixed length*/

 //tuple
 fn main(){
   let tup:(i32,f64,u8)=(500,2.0,1);
 }
 fn main(){
   let tup = (500,2.0,5);
   let (x,y,z)= tup; //destructuring
   println!("The Value of y:{y}");
 }

 //access by (.) - -  indexing 
 fn main(){
   let x:(i32,f64,u8)=(500,2.0,1);
   let five_hundered = x.0;
   let  two_point_zero =x.1;
   let one = x.2;
 }

 //2.Array 

 fn main(){
   let a =[1,2,3,4];
   let b:[i32;5] =[1,2,3,4,5] // with type and size(length)
   let a =[3;5];
   /*semicolon refers to two diff values or quatities mentioned like in the last case the array 
   has common element as 3 and the size is 5
 } */
fn main(){
   let a = [1, 2, 3, 4, 5];
   let first =a[0];
   let two = a[1];
}