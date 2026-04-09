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

 