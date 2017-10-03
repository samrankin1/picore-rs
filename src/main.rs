extern crate num_cpus;

use std::thread;
use std::sync::mpsc;
use std::ops::Range;

// maximum term of the series to sum
// 1 billion runs pretty quickly (although this obviously depends on hardware), estimate returns 3.141592652076216
const RUN_COUNT: u64 = 1000000000;

// returns a set of length 'threads' of maximally equally sized ranges containing every value in 0..n
fn get_work_intervals(n: u64, threads: usize) -> Vec<Range<u64>> {
	let per_thread_base = n / (threads as u64); // each thread gets at least this many
	let mut left_over = n % (threads as u64); // the remainder to be evenly distributed

	let mut result: Vec<Range<u64>> = Vec::new(); // store the result ranges in a vector
	let mut current_max: u64 = 0; // store the current maximum index to roll over to the next range

	for _ in 0..threads { // for each thread to assign work to

		let mut new_max = current_max + per_thread_base; // new max is the last one plus the base per thread
		if left_over > 0 { // if there are more leftover indexes to be assigned
			left_over -= 1; // subtract one
			new_max += 1; // assign this leftover to this thread
		}

		result.push(current_max..new_max); // push this result
		current_max = new_max; // prepare for the next iteration by updating the current_max
	}

	result
}

// returns the nth term of the condensed Leibniz formula for pi
// see: https://en.wikipedia.org/wiki/Leibniz_formula_for_%CF%80#Convergence
fn nth_term(n: f64) -> f64 {
	2.0 / (((4.0 * n) + 1.0) * ((4.0 * n) + 3.0))
}

fn main() {
	let worker_threads = num_cpus::get(); // create a worker thread for each available CPU
	let (tx, rx) = mpsc::channel(); // create transmit and receive channels for the worker threads

	println!("creating {} worker threads", worker_threads);
	for work_interval in get_work_intervals(RUN_COUNT, worker_threads) { // divide the work into even intervals and assign them to worker threads
		// println!("{}..{}\t({})", work_interval.start, work_interval.end, (work_interval.end - work_interval.start));

		let tx = tx.clone(); // clone the transmission channel for this thread to use
		thread::spawn(move || { // spawn a new worker thread

			let mut sum: f64 = 0.0; // start this thread's sum at 0
			for n in work_interval { // for each n value in the assigned work interval
				sum += nth_term(n as f64); // add the nth term to the local sum
			}

			tx.send(sum).ok(); // when done, transmit this partial sum
		});
	}

	let mut result_sum: f64 = 0.0; // start the total sum at 0
	for _ in 0..worker_threads { // for each worker thread created
		result_sum += rx.recv().ok().unwrap(); // retrieve a result from the receive channel and update the sum
	}

	// the result_sum estimates pi/4, so multiply to get our estimation of pi and print the result
	println!("pi ~= {}", (result_sum * 4.0));
}
