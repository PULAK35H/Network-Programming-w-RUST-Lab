fn main() {
    let a = [1,2,3];
    let b = a;
    println!("{:?} \n{:#?}",a,b);
    let y = '😎';
    println!("x={}",y)
    // println!("{:?}",b);
    // println!("{}, {}!", "Hello", "world"); // Hello, world!
    // println!("{0}, {1}!", "Hello", "world"); // Hello, world!
    // println!("{greeting}, {name}!", name="world", greeting="Hello"); // Hello, world!
    
    // println!("{:?}", [1,2,3]); // [1, 2, 3]
    // println!("{:#?}", [1,2,3]);
    // /*
    //     [
    //         1,
    //         2,
    //         3
    //     ]
    // */
    
    // // 🔎 format! macro is used to store the formatted STRING
    // let x = format!("{}, {}!", "Hello", "world");
    // println!("{}", x); // Hello, world!
    
    // let y = String::from("Hello, ") + "world!";
    // println!("{}", y); // Hello, world!
}