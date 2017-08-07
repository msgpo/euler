extern crate num;

use num::bigint::*;
use num::FromPrimitive;

fn main() {

    let mut sum: BigUint = BigUint::from_i32(1).unwrap();
    let mut acc: BigUint = BigUint::from_i32(1).unwrap();

    for interval in (2..1001).filter(|&i| i % 2 == 0) {
        let plus = BigUint::from_i32(interval).unwrap();
        for reps in 0..4 {
            //println!("Plus is {}", plus);
            sum = &sum + &plus;
            //println!("Sum is {}", sum);
            acc = &acc + &sum;
        }
    }

    println!("Sum is {}" , acc);

}
