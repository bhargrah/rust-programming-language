fn main() {

 // Integer

   let int_one_u8:u8 = 12;      // unsiged , typically positive value
    println!("{}",int_one_u8);

   let int_one_i8:i8 = -12;   // try putting 1000 in this 
   println!("{}",int_one_i8);

   // let int_one_i8:i8 = 1000; try putting 1000 in this
   /*
   error: literal out of range for `i8`
    --> 3_datatype.rs:3:24
    |
    3 |    let int_one_i8:i8 = 1200;   // try putting 1000 in this 
    |                        ^^^^
    |
    = note: the literal `1200` does not fit into the type `i8` whose range is `-128..=127`
    = help: consider using the type `i16` instead
    = note: `#[deny(overflowing_literals)]` on by default

    error: aborting due to 1 previous error
   */

   let int_one_i16:i16 = 12000;
   println!("{}",int_one_i16);

   let int_one_i32:i32 = 1200000;
   println!("{}",int_one_i32);

   let int_one_i64:i64 = 12454573875335735; 
   println!("{}",int_one_i64);

   let int_one_i128:i128 = 12454573875564564545656456456456335735; 
   println!("{}",int_one_i128);

   // 

   let int_arc:isize = 10_100;
   println!("{}",int_arc);

   // Float 
   let float_i32:f32 = 10.100;
   println!("{}",float_i32);

   let float_i64:f64 = 10.100843579834759348534958398539;
   println!("{}",float_i64);

   // Boolean 
   let flag:bool = true;
   println!("{}",flag);

   // Character 

   let element = 'A';
   println!("{}",element);


}