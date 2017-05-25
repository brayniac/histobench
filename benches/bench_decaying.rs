#![feature(test)]
extern crate test;
extern crate histobench;

use histobench::*;

#[bench]
fn parse_increment(b: &mut test::Bencher) {
	let mut histogram = Histogram::new();
	b.iter(|| histogram.update(1));
}