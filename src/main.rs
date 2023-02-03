extern crate rayon;
const N: usize = 1_000_000;
const W: f64 = 1f64 / N as f64;

fn f(x: f64) -> f64 {
    4f64 / (1f64 + x * x)
}

fn main() {
    // not good way
    /* let mut sum = 0.0;
     for i in 0..N {
         let x = W * (i as f64 + 0.5);
         sum += f(x);
     }
     println!("pi = {}", W * sum);*/

    //good way functional programming
    use rayon::prelude::*;
    let sum: f64 = (0..N)
        // .into_iter() //stupid because it does one by one
        .into_par_iter() //good way because it does parallel
        .map(|i| f(W * (i as f64 + 0.5)))
        .sum::<f64>();
    println!("pi = {}", W * sum);
}
