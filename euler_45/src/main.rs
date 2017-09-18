use std::collections::HashSet;

fn triangle(n: u64) -> u64{
    n * (n + 1) / 2
}

fn pentagon(n: u64) -> u64{
    n * (3 * n - 1) / 2
}

fn hexagon(n: u64) -> u64{
    n * ((2 * n) - 1)
}

fn main() {
    let mut p_reset = 165;
    let mut h_reset = 143;

    let mut h_sieve = HashSet::new();

    let mut done = false;

    loop {
        h_sieve.clear();

        for i in 1..101 {
            h_reset += 1;
            h_sieve.insert(hexagon(h_reset));
        }

        let h_max = hexagon(h_reset);
        
        loop {
            let p = pentagon(p_reset);
            if p > h_max {
                break;
            }
            if h_sieve.contains(&p) {
                println!("{} is in T, P, and H", p);
                done = true;
                break;
            }
            p_reset += 1;
        }
/*
H is a subset of T, so every H is also a T.  
That means we only need to find the next P in
H.
        loop {
            let t = triangle(t_reset);
            if t > h_max {
                break;
            }
            if p_sieve.contains(&t) {
                println!("{} is in T, P, and H", t);
                done = true;
                break;
            }
            t_reset += 1;
        }
*/
        if done {
            break;
        }
    }
}
