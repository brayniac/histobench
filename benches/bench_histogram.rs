#![feature(test)]
extern crate test;
extern crate histogram;

use histogram::*;

#[bench]
fn parse_increment(b: &mut test::Bencher) {
	let mut histogram = Histogram::new();
	b.iter(|| histogram.increment(1));
}
