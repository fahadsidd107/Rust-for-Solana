fn main() {
    //Simple example of hello world
    println!("Hello to the world!");

    //printing variables

    let x = 5;
    let y = 10;
    println!("The value of x is: {}", x);
    println!("The value of x and y is: {} {}", x, y);

    //data types

    //unsigned int
    //u8,u8,u16,u32,u64
    let unsigned: u8 = 10;

    //signed int
    //i8,i8,i16,i32,i64
    let signed: i8 = -10;

    //floating point
    //f32,f64
    let floating: f32 = 10.1;

    //boolean
    let boolean: bool = true;

    println!(
        "unsigned: {} ,signed: {} , floating: {} ,boolean: {}",
        unsigned, signed, floating, boolean
    );

    //char
    let letter = "Fahad Siddiqui";
    let emoji = "\u{1F600}";

    println!("letter: {} ,emoji: {}", letter, emoji);

    //array
    let array:[u8;5] = [1, 2, 3, 4, 5];
    let array2:[u16;8] = [100,200,300,400,500,500,600,700];

    println!("array: {:?}", array);
    println!("array2: {:?}", array2);
    println!("index : {},length: {}" ,array[0] , array.len());

//tuple 
    let tuple: (i32, f64, u8,bool) = (500, 6.4, 1,false);
    let tuple2=(3,2);
    println!("tuple: {:?}", tuple);
    println!("tuple2: {:?}", tuple2);
    println!("first {},second {},third {}, fourth {}", tuple.0,tuple.1,tuple.2,tuple.3);

//destructuring

let (a,b,c,d) =tuple;
println!("a: {} ,b: {} ,c: {} ,d: {} ",a,b,c,d);

println!("{}",is_even(3));

//mutiblity

let mut x = 5;
x = 6;
println!("x: {}", x);


//slicing 

let arr = [0,1,2,3,4,5,6,7,8,9,10];
let slicedArray = &arr[2..5];
println!("slicedArray: {:?}", slicedArray);

let slice = &arr[1..3];
borrowing_slice(arr, slice)
}

pub fn is_even(num: u8)->bool {
    let digit:u8 = num % 2;
    digit == 0
}

fn borrowing_slice(arr: [u8;11], slice: &[u8]) {
    println!("arr: {:?}", arr);
    println!("slice: {:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}",slice[0],slice[1]);
}

