#[macro_use]
extern crate criterion;
extern crate pulldown_cmark;

use criterion::Criterion;
use pulldown_cmark::{html, Options, Parser};
use std::str::from_utf8;

static CRDT_BYTES: &[u8] = include_bytes!("../third_party/xi-editor/crdt.md");

fn criterion_benchmark(c: &mut Criterion) {
    let mut full_opts = Options::empty();
    full_opts.insert(Options::ENABLE_TABLES);
    full_opts.insert(Options::ENABLE_FOOTNOTES);
    full_opts.insert(Options::ENABLE_STRIKETHROUGH);
    full_opts.insert(Options::ENABLE_TASKLISTS);
    full_opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    c.bench_function("crdt_total", |b| {
        let input = from_utf8(CRDT_BYTES).unwrap();
        let mut buf = String::with_capacity(input.len() * 3 / 2);

        b.iter(|| {
            buf.clear();
            html::push_html(&mut buf, Parser::new_ext(input, Options::empty()));
        })
    });

    c.bench_function("crdt_html", |b| {
        let input = from_utf8(CRDT_BYTES).unwrap();
        let events: Vec<_> = Parser::new_ext(input, Options::empty()).collect();
        let mut buf = String::with_capacity(input.len() * 3 / 2);

        b.iter(|| {
            buf.clear();
            html::push_html(&mut buf, events.clone().into_iter());
        })
    });

    c.bench_function("crdt_all_options_parse", |b| {
        let input = from_utf8(CRDT_BYTES).unwrap();

        b.iter(|| Parser::new_ext(input, full_opts).count())
    });

    c.bench_function("crdt_parse", |b| {
        let input = from_utf8(CRDT_BYTES).unwrap();

        b.iter(|| Parser::new_ext(input, Options::empty()).count())
    });

    c.bench_function("smart_punctuation", |b| {
        let input = r#"""'This here a real "quote"'

And -- if you're interested -- some em-dashes. Wait --- she actually said that?

Wow... Becky is so 'mean'!
"""#;

        b.iter(|| Parser::new_ext(input, full_opts).count());
    });

    c.bench_function("links_n_emphasis", |b| {
        let input = r#"""This is a [link](example.com). **Cool!**

This is a [link](example.com). **Cool!**

This is a [link](example.com). **Cool!**

This is a [link](example.com). **Cool!**
"""#;

        b.iter(|| Parser::new_ext(input, Options::empty()).count());
    });

    c.bench_function("unescapes", |b| {
        let input = "This is by far my favourite unicode code point: &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA;
        &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA;
        &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA;
        &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA;
        &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA;
        &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA;
        &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA;
        &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA; &#xAAA;";

        b.iter(|| Parser::new_ext(input, Options::empty()).count());
    });

    c.bench_function("autolinks_n_html", |b| {
        let input = "Drop me a line at <john@example.com>. <emph font-weight='BOLD'>Thanks!</emph> <otherinline>
        Drop me a line at <john@example.com>. <emph font-weight='BOLD'>Thanks!</emph> <otherinline>
        Drop me a line at <john@example.com>. <emph font-weight='BOLD'>Thanks!</emph> <otherinline>
        Drop me a line at <john@example.com>. <emph font-weight='BOLD'>Thanks!</emph> <otherinline>
        Drop me a line at <john@example.com>. <emph font-weight='BOLD'>Thanks!</emph> <otherinline>";

        b.iter(|| Parser::new_ext(input, Options::empty()).count());
    });

    c.bench_function("reuse_naive", |b| {
        let input = "This is a [link](example.com). **Cool!**";

        b.iter(|| {
            let mut total_count = 0;
            for _ in 0..100 {
                let mut parser = Parser::new(input);
                while parser.next().is_some() {
                    total_count += 1;
                }
            }
            total_count
        });
    });

    c.bench_function("reuse_new", |b| {
        let input = "This is a [link](example.com). **Cool!**";

        b.iter(|| {
            let mut parser = Parser::new(input);
            let mut total_count = 0;
            while parser.next().is_some() {
                total_count += 1;
            }
            for _ in 0..99 {
                parser.reuse(input);
                while parser.next().is_some() {
                    total_count += 1;
                }
            }
            total_count
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
