#![feature(test)]

extern crate pulldown_cmark;
extern crate test;

use pulldown_cmark::{Parser, Options};
use std::time;

fn parse_md(text: &str, opts: Options) {
    Parser::new_ext(text, opts).count();
}

const RUNS: usize = 20;

const BASE_SIZE: usize = 2000;

const GROWTH_FACTOR: usize = 10;

/// Parse times are allowed to grow by this factor greater than the
/// increase in input size.
const SLACK: f64 = 2.0;

fn measure_median_run(input: &str) -> time::Duration {
    let mut durations = (0..RUNS)
        .map(|_| {
            let before = time::Instant::now();
            parse_md(input, Options::empty());
            time::Instant::now().duration_since(before)
        })
        .collect::<Vec<_>>();
    durations.sort();
    durations[RUNS / 2]
}

fn assert_linear_scaling(short: &str, long: &str) {
    assert!(short.len() >= 500);
    assert!(long.len() > short.len());
    let scaling = long.len() as f64 / short.len() as f64;

    let short_duration_median = measure_median_run(short);
    let long_duration_median = measure_median_run(long);

    assert!(long_duration_median.as_micros() as f64 <= short_duration_median.as_micros() as f64 * scaling * SLACK);
}

fn simple_repetition_scaling_test(snip: &str) {
    let reps = BASE_SIZE / snip.len();
    let input_short = std::iter::repeat(snip).take(reps).collect::<String>();
    let input_long =  std::iter::repeat(snip).take(reps * GROWTH_FACTOR).collect::<String>();
    assert_linear_scaling(&input_short, &input_long);
}

#[bench]
fn pathological_links(_b: &mut test::Bencher) {
    simple_repetition_scaling_test("[a](<");
}

#[bench]
fn pathological_emphasis(_b: &mut test::Bencher) {
    simple_repetition_scaling_test("a***");
}

#[bench]
fn pathological_emphasis2(_b: &mut test::Bencher) {
    simple_repetition_scaling_test("a***_b__");
}

#[bench]
fn pathological_emphasis3(_b: &mut test::Bencher) {
    simple_repetition_scaling_test("[*_a");
}

#[bench]
fn pathological_strikethrough(_b: &mut test::Bencher) {
    simple_repetition_scaling_test("a***b~~");
}

fn generate_pathological_codeblocks(size: usize) -> String {
    let mut buf = String::new();
    let mut i = 1;

    while buf.len() < size {
        for _ in 0..i {
            buf.push('`');
        }
        buf.push(' ');
        i += 1;
    }

    buf
}

#[bench]
fn pathological_codeblocks1(_b: &mut test::Bencher) {
    let input_short = generate_pathological_codeblocks(BASE_SIZE);
    let input_long = generate_pathological_codeblocks(BASE_SIZE * GROWTH_FACTOR);

    assert_linear_scaling(&input_short, &input_long);
}

#[bench]
fn pathological_codeblocks2(_b: &mut test::Bencher) {
    simple_repetition_scaling_test("\\``");
}

#[bench]
fn pathological_codeblocks3(_b: &mut test::Bencher) {
    let mut input_short = std::iter::repeat("`a`").take(BASE_SIZE / 3).collect::<String>();
    input_short.push('`');
    let mut input_long = std::iter::repeat("`a`").take(BASE_SIZE * GROWTH_FACTOR / 3).collect::<String>();
    input_long.push('`');

    assert_linear_scaling(&input_short, &input_long);
}

#[bench]
fn pathological_hrules(_b: &mut test::Bencher) {
    let mut input_short = std::iter::repeat("* ").take(BASE_SIZE / 2).collect::<String>();
    input_short.push('a');
    let mut input_long = std::iter::repeat("* ").take(BASE_SIZE * GROWTH_FACTOR / 2).collect::<String>();
    input_long.push('a');

    assert_linear_scaling(&input_short, &input_long);
}

#[bench]
fn pathological_link_titles(_b: &mut test::Bencher) {
    simple_repetition_scaling_test("[ (](");
}

fn generate_pathological_codeblocks2(size: usize) -> String {
    let mut buf = String::new();
    let mut i = 1;

    while buf.len() < size {
        for _ in 0..i {
            buf.push('`');
        }
        buf.push(' ');
        i += 1;
    }
    for _ in 0..i {
        buf.push_str("*a* ");
    }

    buf
}

#[bench]
fn pathological_codeblocks4(_b: &mut test::Bencher) {
    let input_short = generate_pathological_codeblocks2(BASE_SIZE);
    let input_long = generate_pathological_codeblocks2(BASE_SIZE * GROWTH_FACTOR);

    assert_linear_scaling(&input_short, &input_long);
}
