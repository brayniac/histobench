#![feature(test)]
extern crate test;
extern crate histobench;

use histobench::*;

#[bench]
fn decaying_increment(b: &mut test::Bencher) {
	let mut histogram = Histogram::new();
	b.iter(|| histogram.update(1));
}

#[bench]
fn decaying_stddev(b: &mut test::Bencher) {
	let mut histogram = Histogram::new();
	for i in 1..1000 {
		histogram.update(i);
	}
	b.iter(|| assert!(histogram.snapshot().stddev() > 0.0));
}

#[bench]
fn decaying_p90(b: &mut test::Bencher) {
	let mut histogram = Histogram::new();
	for i in 1..1000 {
		histogram.update(i);
	}
	b.iter(|| assert!(histogram.snapshot().value(0.90) > 0));
}