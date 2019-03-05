#[macro_use] extern crate honggfuzz;
extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Options, html};


fn main() {
    // Here you can parse `std::env::args and 
    // setup / initialize your project

    // You have full control over the loop but
    // you're supposed to call `fuzz` ad vitam aeternam
    loop {
        // The fuzz macro gives an arbitrary object (see `arbitrary crate`)
        // to a closure-like block of code.
        // For performance reasons, it is recommended that you use the native type
        // `&[u8]` when possible.
        // Here, this slice will contain a "random" quantity of "random" data.
        fuzz!(|text: String| {
            let mut s = String::with_capacity(text.len() * 3 / 2);
            let p = Parser::new_ext(&text, Options::empty());
            html::push_html(&mut s, p);
        });
    }
}

