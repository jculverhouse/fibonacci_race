use cached::proc_macro::cached;
use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};

#[test]
fn test_each_version() {
    assert_eq!(backtrace_fib(20), 6765);
    assert_eq!(backtrace_memo_fib(&mut HashMap::new(), 20), 6765);
    assert_eq!(dynamic_fib(20), 6765);
    assert_eq!(cached_fib(20), 6765);
    assert_eq!(cached_dynamic_fib(20), 6765);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} n (positive integer to solve slow way)", args[0]);
        return;
    }

    let fib_num = args[1].parse::<u128>().unwrap();
    println!("\nThe first time solving will be the slowest\n");
    solve_each(fib_num);
    println!("What about solving it a second or third time, anyone faster this time?\n");
    solve_each(fib_num);
    solve_each(fib_num);
}

fn solve_each(fib_num: u128) {
    let now = Instant::now();
    let _ = backtrace_fib(fib_num);
    let elapsed = now.elapsed();
    print_results(fib_num, "simple backtracing/recursion", elapsed);

    let now = Instant::now();
    let _ = backtrace_memo_fib(&mut HashMap::new(), fib_num);
    let elapsed = now.elapsed();
    print_results(fib_num, "backtracing/recursion with memoization", elapsed);

    let now = Instant::now();
    let _ = dynamic_fib(fib_num);
    let elapsed = now.elapsed();
    print_results(fib_num, "dynamic programming with memoization", elapsed);

    let now = Instant::now();
    let _ = cached_fib(fib_num);
    let elapsed = now.elapsed();
    print_results(fib_num, "cached function", elapsed);

    let now = Instant::now();
    let _ = cached_dynamic_fib(fib_num);
    let elapsed = now.elapsed();
    print_results(fib_num, "cached dynamic function", elapsed);

    println!();
}

fn print_results(fib_num: u128, desc: &str, elapsed: Duration) {
    println!(
        "  Solving fib:{} with {:50} took {:>15} ns",
        fib_num,
        desc,
        elapsed.as_nanos()
    );
}

fn backtrace_fib(fib_num: u128) -> u128 {
    if fib_num == 0 || fib_num == 1 {
        return fib_num;
    }
    backtrace_fib(fib_num - 1) + backtrace_fib(fib_num - 2)
}

fn backtrace_memo_fib(memo: &mut HashMap<u128, u128>, fib_num: u128) -> u128 {
    match memo.get(&fib_num).map(|answer| answer.clone()) {
        Some(result) => result,
        None => {
            let result = match fib_num {
                0 | 1 => fib_num,
                n => backtrace_memo_fib(memo, n - 1) + backtrace_memo_fib(memo, n - 2),
            };
            memo.insert(fib_num, result.clone());
            result
        }
    }
}

fn dynamic_fib(fib_num: u128) -> u128 {
    let mut memo = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);
    match fib_num {
        0 | 1 => {} // already set
        n => {
            for i in 2..=n {
                let result = *memo.get(&(i - 1)).unwrap() + *memo.get(&(i - 2)).unwrap();
                memo.insert(i, result);
            }
        }
    };
    *memo.get(&fib_num).unwrap()
}

#[cached(size = 100)]
fn cached_fib(fib_num: u128) -> u128 {
    if fib_num == 0 || fib_num == 1 {
        return fib_num;
    }
    cached_fib(fib_num - 1) + cached_fib(fib_num - 2)
}

#[cached(size = 100)]
fn cached_dynamic_fib(fib_num: u128) -> u128 {
    if fib_num == 0 || fib_num == 1 {
        return fib_num;
    }
    for i in 2..=fib_num {
        let _ = cached_fib(i);
    }
    cached_fib(fib_num - 1) + cached_fib(fib_num - 2)
}
