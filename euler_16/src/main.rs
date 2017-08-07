extern crate num;

use num::bigint::*;
use num::FromPrimitive;
use num::Zero;

fn main() {
    let mut d: BigUint = BigUint::from_i32(2).unwrap();
    let two = BigUint::from_i32(2).unwrap();
    let ten = BigUint::from_i32(10).unwrap();
    let zero = BigUint::from_i32(0).unwrap();
    let mut power = 1;
    while power < 1000 {
        power = power + 1;
        d = d * &two;
    }
    println!("2 ^ 1000 is {}", d);
    // let mut sum: BigUint = BigUint::from_i32(0).unwrap();
    //in loop while d > 0
    //mod 10, add remainder to sum
    //shift right one digit
    let mut sum: BigUint = Zero::zero();
    while &d > &zero {
        sum = sum + (&d % &ten);
        d = &d / &ten;
        println!("{}",&d)
    }

    println!("Sum is {}", &sum)

}
