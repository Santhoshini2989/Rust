 fn main(){
    let number =3;

    if number < 5 {
        println!("cpndition was true");
    }else{
        println!("cpndition was false");

    }
 }

 //otehr than 0

 fn main(){
    let numebr = 3;
    if number !=0 {
        println!("number was something other than 0");
    }
 }
//multiple conditions
 fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

//using if in a let statement

fn main(){
    let condition = true;
    let number = if condition{5}else{6};
    println!("the value of the number is :{number}");
}
//loops - - loop while and for 
fn main(){
    loop{
        println!("again!");
    }
}

 // returning values form loops

 fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

    
    //Disambiguating with Loop Labels
 fn main(){
    let mut count = 0;
    'counting_loop:{
        println!("count={count}");
        let mut remaining = 10;

        loop{
           println!("remaining ={remaining}");
           if remaining = = 9{
            break;
           }
           if count ==2{
            break 'counting_loop;
           }
           remaining -=1;
        
        }
        count +=1;
    
    }
     println!("final count:{count}");
 }
        

        //while  --  WHILE THE CONDITION IS TRUE IT EXECUTES ORELSE EXITSS

        fn main(){
            let mut number =3;

            while number!=0{
                println!("{number}");
            number -=1;
            }
            println!("LIFTOFF!!!");
        }
 
        //Looping Through a Collection with for - -  this is error prone 

        fn main(){
            let a =[10,20,30,40,50];
            let mut index=0;
            while index <5{
                println!("teh value is :{},a[index]");
                index +=1;
            }
        }
//for 
 fn main(){
    let a =[10,20,30,40,50];
    for element in a {
          println!("the value is: {element}");
    }
 }

 fn main(){
    for number in (1..4).rev(){
        println!("{number}!");

    }
    println!("LIFTOFF!!!");
 }
