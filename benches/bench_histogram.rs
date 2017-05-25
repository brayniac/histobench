#![feature(test)]
extern crate test;
extern crate histogram;

use histogram::*;

#[bench]
fn brayniac_increment(b: &mut test::Bencher) {
	let mut histogram = Histogram::new();
	b.iter(|| histogram.increment(1));
}

#[bench]
fn brayniac_stddev(b: &mut test::Bencher) {
	let mut histogram = Histogram::new();
	for i in 1..1000 {
		let _ = histogram.increment(i);
	}
	b.iter(|| assert!(histogram.stddev().is_some()));
}

#[bench]
fn brayniac_p90(b: &mut test::Bencher) {
	let mut histogram = Histogram::new();
	for i in 1..1000 {
		let _ = histogram.increment(i);
	}
	b.iter(|| assert!(histogram.percentile(90.0).is_ok()));
}