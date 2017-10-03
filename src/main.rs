extern crate rand;
extern crate num_cpus;

use std::thread;
use std::sync::mpsc;

use rand::Rng;

const RUN_COUNT: u32 = 100000000;

fn coprime(mut a: u64, mut b: u64) -> bool {
	while b > 0 {
		let t = b;
		b = a % b;
		a = t;
	}

	a == 1
}

fn main() {
	let (tx, rx) = mpsc::sync_channel(1000000);

	for _ in 0..(num_cpus::get() - 1) {
		let tx = tx.clone();
		thread::spawn(move || {
			for (a, b) in rand::thread_rng().gen_iter::<(u64, u64)>() {
				match tx.send(coprime(a, b)) {
					Ok(_) => continue,
					Err(_) => break
				}
			}
		});
	}

	thread::spawn(move || {
		let mut coprimes: u32 = 0;

		for _ in 0..RUN_COUNT {
			if rx.recv().ok().unwrap() {
				coprimes += 1;
			}
		}

		let n = (coprimes as f64) / (RUN_COUNT as f64);
		println!("result = '{}'", n);

		println!("pi ~= '{}'", (6.0 / n).sqrt());
	}).join().ok();

}
