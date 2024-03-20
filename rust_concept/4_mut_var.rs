fn main() {

let age = 14;   // by default all vaibles are immutable
println!("Age : {}",age);

// ager = 16;   /// cannot reassgin
/*
error[E0425]: cannot find value `ager` in this scope
 --> 4_mut_var.rs:6:1
  |
6 | ager = 16;
  | ^^^^ help: a local variable with a similar name exists: `age`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
*/

let mut name = "Rahul";  // add mut if variable needs to be mutated later in the code
println!("name : {}",name);

name = "Rahul Kumar Bhargava";
println!("name : {}",name);


}