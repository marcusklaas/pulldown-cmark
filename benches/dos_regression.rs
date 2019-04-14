#![feature(test)]

extern crate pulldown_cmark;
extern crate test;

use pulldown_cmark::{Parser, Options};
use std::time;

fn parse_md(text: &str, opts: Options) {
    Parser::new_ext(text, opts).count();
}

const RUNS: usize = 100;

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
    let input_short = std::iter::repeat(snip).take(1000).collect::<String>();
    let input_long =  std::iter::repeat(snip).take(10_000).collect::<String>();
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

// #[bench]
// fn pathological_codeblocks1(_b: &mut test::Bencher) {
//     // Note that `buf` grows quadratically with number of
//     // iterations. The point here is that the render time shouldn't
//     // grow faster than that.
//     let mut buf = String::new();
//     for i in 1..1000 {
//         for _ in 0..i {
//             buf.push('`');
//         }
//         buf.push(' ');
//     }

//     b.iter(|| render_html(&buf, Options::empty()));
// }

#[bench]
fn pathological_codeblocks2(_b: &mut test::Bencher) {
    simple_repetition_scaling_test("\\``");
}

// #[bench]
// fn pathological_codeblocks3(_b: &mut test::Bencher) {
//     let mut input = std::iter::repeat("`a`").take(4000).collect::<String>();
//     input.push('`');

//     b.iter(|| render_html(&input, Options::empty()));
// }

// #[bench]
// fn pathological_hrules(_b: &mut test::Bencher) {
//     let mut input = std::iter::repeat("* ").take(2000).collect::<String>();
//     input.push('a');

//     b.iter(|| render_html(&input, Options::empty()));
// }

#[bench]
fn pathological_link_titles(_b: &mut test::Bencher) {
    simple_repetition_scaling_test("[ (](");
}

// #[bench]
// fn advanced_pathological_codeblocks(_b: &mut test::Bencher) {
//     // Note that `buf` grows quadratically with number of
//     // iterations. The point here is that the render time shouldn't
//     // grow faster than that.
//     let size = 120;
//     let mut buf = String::new();
//     for i in 1..size {
//         for _ in 0..i {
//             buf.push('`');
//         }
//         buf.push(' ');
//     }
//     for _ in 1..(size * size) {
//         buf.push_str("*a* ");
//     }
//     eprintln!("str size: {}", buf.len());

//     b.iter(|| render_html(&buf, Options::empty()));
// }
