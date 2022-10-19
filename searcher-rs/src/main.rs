mod solver;
mod utils;

use ethers::{types::{U256, H512}};
use std::{thread, env};
use utils::{add_to_uint};
use solver::{Solver, MAX_ITER};

const MAX_THREADS: u64 = 10;

fn main() {
    let (caller, contract, threads_count) = get_args();
    run(caller, contract, threads_count)
}

fn run(caller: U256, contract: U256, threads_count: u64) {
    if threads_count > MAX_THREADS {
        panic!("MAX_THREADS exceeded");
    }
    let range = MAX_ITER / threads_count;
    let mut handles = Vec::new();
    for i in 0..threads_count {
        let offset = i * MAX_ITER / threads_count;
        let handle = thread::spawn(move || {
            println!("Thread {:} started ...", i);
            return solve_and_log(
                caller.clone(), 
                contract.clone(), 
                Some(offset.clone()),
                Some(offset+range)
            );
        });
        handles.push(handle);
    }
    for handle in handles {
        let result = handle.join().unwrap();
        println!("{:?}", result);
    }

}

fn solve_and_log(caller: U256, contract: U256, offset: Option<u64>, max_iter: Option<u64>) -> Option<H512> {
    let solution = Solver::new(caller, contract, offset, max_iter).solve();
    if let Some(x) = solution {
        println!("Resulting payload: {:?}", x);
        return solution;
    } else {
        println!("Not found!");
        return None
    }
}

fn get_args() -> (U256, U256, u64) {
    let args: Vec<String> = env::args().collect();
    return (
        add_to_uint(&args[1]),
        add_to_uint(&args[2]),
        args[3].parse().unwrap(),
    )
}

