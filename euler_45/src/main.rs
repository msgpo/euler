use std::collections::HashSet;

fn triangle(n: u64) -> u64{
    n * (n + 1) / 2
}

fn pentagon(n: u64) -> u64{
    n * (3 * n - 1) / 2
}

fn hexagon(n: u64) -> u64{
    n * (2 * n - 1)
}

fn main() {
    let mut t_reset = 285;
    let mut p_reset = 165;
    let mut h_reset = 143;

    let mut h_sieve = HashSet::new();
    let mut p_sieve = HashSet::new();

    let mut done = false;

    loop {
        h_sieve.clear();
        p_sieve.clear();

        for i in 1..101 {
            h_reset += i;
            h_sieve.insert(hexagon(h_reset));
        }

        let h_max = hexagon(h_reset);
        if h_max > 1533800000 {
            println!("There is a bug.  Answer exceeded");
            println!("Size of H sieve is {}", h_sieve.len());
            break;
        }
        
        loop {
            let p = pentagon(p_reset);
            println!("p is now {}", p);
            if p > h_max {
                break;
            }
            if h_sieve.contains(&p) {
                p_sieve.insert(p);
            }
            p_reset += 1;
        }
        if p_sieve.len() > 0 {
            println!("h_max is now {}", h_max);
            println!("p sieve size is now {}", p_sieve.len());
        }

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

        if done {
            break;
        }
    }
}
