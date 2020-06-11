use futures::executor::block_on;
use async_programming::blocking::compute_blocking;
use async_programming::non_blocking::compute_non_blocking;

fn main() {
    println!("Result of blocking asynchronous code:" );
    block_on(compute_blocking());
    println!();
    println!("Result of non-blocking asynchronous code:" );
    block_on(compute_non_blocking());
}