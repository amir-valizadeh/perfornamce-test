#![allow(dead_code)]
use std::time::Instant;
const MAX: i64 = 3_00_000;

// fn main() {
//     let start_time = Instant::now();

//     let mut primes: Vec<i64> = Vec::<i64>::with_capacity(30_000);
//     let mut is_prime;
//     for _num in 2..MAX {
//         is_prime = true;
//         let mut _i = 0;
//         for _prime in primes.iter() {
//             if _num % _prime == 0 {
//                 is_prime = false;
//                 break;
//             }
//             _i += 1;
//         }
//         if is_prime {
//             primes.push(_num)
//         }
//     }
//     let end_time = start_time.elapsed();

//     println!("{} - {:?}", primes.len(), end_time)
// }

// fn main() {
//    
//     let x=3_00_000;
//     let mut a= 0;
//     for i in 2..x{
//         if is_prime(i){
//             a+=1
//         }
//     }
//      //function calling
//     println!("{}", a);
// }
// 
// fn is_prime(n: i64) -> bool {
//     if n <= 1 {
//         return false;
//     }
//     for a in 2..((n/2) as i64) {
//         if n % a == 0 {
//             return false; // if it is not the last statement you need to use `return`
//         }
//     }
//    return true // last value to return
// } 



fn main() {
    let start_time = Instant::now();

    let mut count = 0;
    let mut is_prime;
    for c in 2..MAX {
        is_prime = true;
        for inner_c in 2..(c / 2 + 1) {
            if c % inner_c == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            count += 1;
        }
    }
    let end_time = start_time.elapsed();
    println!("{} - {:?}", count, end_time);
}
