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
borrowing_slice(arr, slice);

//strings

let str: &str="hello world";
let mut string: String = String::from("Hello world!");

let slice = &string[.. 6];
// slice.len();

string.push('1');
string.push_str("! Bob");
let string_after_functions_applied = string.replace("Hello", "Bye!");
println!("{}", string_after_functions_applied);


//if else

let number = 3;
if number > 0 {
    println!("Number is greater than 0")
}else if number < 0 {
    println!("Number is less than 0")
}else {
    println!("Number is 0")
}

//for loop
 
for i in 0..6{
    println!("{}",i);
}

//while loop

let mut i = 0;
while i < 4 {
    println!("{}",i);
    i = i + 1;
    if i == 3{
        println!("Exiting looop");
        break;
    }
}

//match statement

let q = 5;
match q{
    0 => println!("0"),
    1 | 2 => println!("1 or 2"),
    3..=4   => println!("3 till 4"),
    _ => println!("default"),

}

//structs

let name = String::from("Ronaldo");
let player = Player{
    name , 
    attacking: 99,
    defending: 99,
    passes: 99,
    shooting: 99,
};
player.print_player();

//traits

println!("{} {}",player.is_player(),player.won_ball());

//enums

let a: MyEnum = MyEnum::A;
let b: MyEnum = MyEnum::B(5);
let c: MyEnum = MyEnum::C{x: 1, y:2};

println!("{:?}",a);
println!("{:?}",b);
println!("{:?}",c);

if let MyEnum::B(val)=b{
    println!("{}",val);
}
if let MyEnum::C{x,y}=c{
    println!("{} {} ",x,y);
}

//vectors

let mut vec:Vec<i64> = vec![0,1,2,3,4,5,6,7];
vec.len();
vec[0];
vec.push(8);
vec.remove(0);
println!("{:?}",vec);


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

struct Player{
    name : String,
    attacking:u64,
    defending:u64,
    passes:u64,
    shooting:u64,
}

impl Player{
    fn print_player(&self){
        println!("name: {}",self.name);
        println!("attacking: {}",self.attacking);
        println!("defending: {}",self.defending);
        println!("passes: {}",self.passes);
        println!("shooting: {}",self.shooting);
    }
}

impl Ballon_Deor for Player {
    fn won_ball(&self) -> bool {
        true
    }
    fn is_player(&self) -> bool{
        true
    } 
}
trait Ballon_Deor{
    fn won_ball(&self) -> bool;
    fn is_player(&self) -> bool{
        false
    }    
}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C{x:i32, y:i32},
}