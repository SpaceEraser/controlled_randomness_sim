use rand::{thread_rng, Rng, RngCore, SeedableRng};
use rand_pcg::Pcg64Mcg;
use rayon::prelude::*;

fn sim_threaded(p: f64, n: u64) -> f64 {
    let total_loops: u64 = (0..n)
        .into_par_iter()
        .map(|_| {
            let mut l = 0;
            let mut cp = p;
            loop {
                l += 1;
                if thread_rng().gen::<f64>() > cp {
                    break;
                }
                cp *= p;
            }
            return l;
        })
        .sum();
    return n as f64 / total_loops as f64;
}

fn sim_repeated<R: RngCore>(p: f64, n: u64, rng: &mut R) -> f64 {
    let mut l = 0;
    for _ in 0..n {
        let mut cp = p;
        loop {
            l += 1;
            if rng.gen::<f64>() > cp {
                break;
            }
            cp *= p;
        }
    }
    return n as f64 / l as f64;
}

fn sim<R: RngCore>(p: f64, n: u64, rng: &mut R) -> f64 {
    let mut l = 0;
    let mut c = 0;
    let mut cp = p;

    loop {
        if rng.gen::<f64>() <= cp {
            cp *= p;
        } else {
            cp = p;
            c += 1;
            if l >= n {
                break;
            }
        }
        l += 1;
    }
    return c as f64 / l as f64;
}

fn sim_closed(p: f64, n: u64) -> f64 {
    let ex: f64 = (1..n)
        .map(|j| (j as f64) * p.powi((j * (j - 1) / 2) as i32) * (1.0 - p.powi(j as i32)))
        .sum();
    return 1.0 / ex;
}

fn find_p(target: f64, n: u64, eps: f64) -> Result<f64, f64> {
    let mut a: f64 = 0.0;
    let mut b: f64 = 1.0;
    let mut cp = 0.0;
    for _ in 0..n {
        cp = (a + b) / 2.0;
        let ct = sim_closed(cp, 5000);
        if (ct - target).abs() <= eps {
            return Ok(cp);
        }
        if ct < target {
            b = cp;
        } else {
            a = cp;
        }
    }
    return Err(cp);
}

fn main() {
    let mut rng = Pcg64Mcg::from_entropy();
    let p = find_p(0.99, 100, 0.001);
    dbg!(p);
    dbg!(sim(p.unwrap_or_else(|e| e), 10_000_000, &mut rng));
    dbg!(sim_repeated(p.unwrap_or_else(|e| e), 10_000_000, &mut rng));
    for i in 1..=6 {
        println!(
            "10^{} loops = {}",
            i,
            sim_closed(p.unwrap_or_else(|e| e), 10u64.pow(i))
        );
    }
}
