 //before slice 
 fn first_word(s:&String)->usize{
    let bytes = s.as_bytes();
    for(i,&item)in bytes.iter().enumerate(){
        if item == b' '{
            return i;
    }
 }
 s.len()
}

 fn main(){
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

}

  //slice is a reference to a peace of data when we use slice we cant modifty the originall data or drop or invalidate it 
  fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // slice up to the first space
        }
    }

    &s[..] // whole string if no space found
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("First word: {}", word);

    // s.clear(); ❌ This will cause a compile-time error
}

  