// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn regression_test_1() {
    let original = r##"<details><summary>Testing 1..2..3..</summary>

This is a test of the details element.

</details>
"##;
    let expected = r##"<details><summary>Testing 1..2..3..</summary>
<p>This is a test of the details element.</p>
</details>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_2() {
    let original = r##"see the [many] [articles] [on] [QuickCheck].

[many]: https://medium.com/@jlouis666/quickcheck-advice-c357efb4e7e6
[articles]: http://www.quviq.com/products/erlang-quickcheck/
[on]: https://wiki.haskell.org/Introduction_to_QuickCheck1
[QuickCheck]: https://hackage.haskell.org/package/QuickCheck
"##;
    let expected = r##"<p>see the 
  <a href="https://medium.com/@jlouis666/quickcheck-advice-c357efb4e7e6">many</a> 
  <a href="http://www.quviq.com/products/erlang-quickcheck/">articles</a> 
  <a href="https://wiki.haskell.org/Introduction_to_QuickCheck1">on</a> 
  <a href="https://hackage.haskell.org/package/QuickCheck">QuickCheck</a>.
</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_3() {
    let original = r##"[![debug-stub-derive on crates.io][cratesio-image]][cratesio]
[![debug-stub-derive on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/debug_stub_derive.svg
[cratesio]: https://crates.io/crates/debug_stub_derive
[docsrs-image]: https://docs.rs/debug_stub_derive/badge.svg?version=0.3.0
[docsrs]: https://docs.rs/debug_stub_derive/0.3.0/
"##;
    let expected = r##"<p><a href="https://crates.io/crates/debug_stub_derive"><img src="https://img.shields.io/crates/v/debug_stub_derive.svg" alt="debug-stub-derive on crates.io" /></a>
<a href="https://docs.rs/debug_stub_derive/0.3.0/"><img src="https://docs.rs/debug_stub_derive/badge.svg?version=0.3.0" alt="debug-stub-derive on docs.rs" /></a></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_4() {
    let original = r##"|  Title A  |  Title B  |
| --------- | --------- |
| Content   | Content   |

|  Title A  |  Title B  |  Title C  |  Title D  |
| --------- | --------- | --------- | ---------:|
| Content   | Content   | Conent    | Content   |
"##;
    let expected = r##"<table><thead><tr><th>Title A  </th><th>Title B  </th></tr></thead><tbody>
<tr><td>Content   </td><td>Content   </td></tr>
</tbody></table>
<table><thead><tr><th>Title A  </th><th>Title B  </th><th>Title C  </th><th align="right">Title D  </th></tr></thead><tbody>
<tr><td>Content   </td><td>Content   </td><td>Conent    </td><td align="right">Content   </td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_5() {
    let original = r##"foo§__(bar)__
"##;
    let expected = r##"<p>foo§<strong>(bar)</strong></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_6() {
    let original = r##"<https://example.com> hello
"##;
    let expected = r##"<p><a href="https://example.com">https://example.com</a> hello</p>

"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_7() {
    let original = r##"[foo][bar]

<!-- foo -->
[bar]: a
"##;
    let expected = r##"<p><a href="a">foo</a></p>
<!-- foo -->
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_8() {
    let original = r##"<!-- <dl> -->
- **foo** (u8, u8)

  make something

- **bar** (u16, u16)

  make something
"##;
    let expected = r##"<!-- <dl> -->
<ul>
<li>
<p><strong>foo</strong> (u8, u8)</p>
<p>make something</p>
</li>
<li>
<p><strong>bar</strong> (u16, u16)</p>
<p>make something</p>
</li>
</ul>

"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_9() {
    let original = r##"[`
i8
`](
../../../std/primitive.i8.html
)
"##;
    let expected = r##"<p><a href="../../../std/primitive.i8.html"><code>i8</code></a></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_10() {
    let original = r##"[a]

[a]: /url (title\\*)
"##;
    let expected = r##"<p><a href="/url" title="title\*">a</a></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_11() {
    let original = r##"[a]

[a]: /url (title\))
"##;
    let expected = r##"<p><a href="/url" title="title)">a</a></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_12() {
    let original = r##"[a]

[a]: /url (title))
"##;
    let expected = r##"<p>[a]</p>
<p>[a]: /url (title))</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_13() {
    let original = r##"a <?php this is not a valid processing tag
---
b <?php but this is ?>
"##;
    let expected = r##"<h2>a &lt;?php this is not a valid processing tag</h2>
<p>b <?php but this is ?></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_14() {
    let original = r##"[a]: u\
foo
"##;
    let expected = r##"<p>foo</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_15() {
    let original = r##"\`foo`
"##;
    let expected = r##"<p>`foo`</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_16() {
    let original = r##"foo\\
bar
"##;
    let expected = r##"<p>foo\
bar</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_17() {
    let original = r##"1\. foo

1\) bar
"##;
    let expected = r##"<p>1. foo</p>
<p>1) bar</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_18() {
    let original = r##"1...

1.2.3.

1 2 3 .

1.|2.-3.

1)2)3)
"##;
    let expected = r##"<p>1...</p>
<p>1.2.3.</p>
<p>1 2 3 .</p>
<p>1.|2.-3.</p>
<p>1)2)3)</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_19() {
    let original = r##"[](<<>)
"##;
    let expected = r##"<p>[](&lt;&lt;&gt;)</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_20() {
    let original = r##"\``foo``bar`
"##;
    let expected = r##"<p>`<code>foo``bar</code></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_21() {
    let original = r##"\\`foo`
"##;
    let expected = r##"<p>\<code>foo</code></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_22() {
    let original = r##"[\\]: x

YOLO
"##;
    let expected = r##"<p>YOLO</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_23() {
    let original = r##"lorem ipsum
A | B
---|---
foo | bar
"##;
    let expected = r##"<p>lorem ipsum
A | B
---|---
foo | bar</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_24() {
    let original = r##"foo|bar  
---|---
foo|bar
"##;
    let expected = r##"<table><thead><tr><th>foo</th><th>bar</th></tr></thead>
<tbody><tr><td>foo</td><td>bar</td></tr></tbody>
</table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_25() {
    let original = r##"foo|bar\\
---|---
foo|bar
"##;
    let expected = r##"<table><thead><tr><th>foo</th><th>bar\</th></tr></thead>
<tbody><tr><td>foo</td><td>bar</td></tr></tbody>
</table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_26() {
    let original = r##"[<foo>](url)
"##;
    let expected = r##"<p><a href="url"><foo></a></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_27() {
    let original = r##"[<foo>bar</foo>](url)
"##;
    let expected = r##"<p><a href="url"><foo>bar</foo></a></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_28() {
    let original = r##"![<http://example.com>](http://example.com/logo.png)
"##;
    let expected = r##"<p><img alt="http://example.com" src="http://example.com/logo.png"></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_29() {
    let original = r##"[<http://one> <http://two>](url)
"##;
    let expected = r##"<p><a href="url"></a><a href="http://one">http://one</a> <a href="http://two">http://two</a></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_30() {
    let original = r##"Markdown | Less | Pretty
--- | --- | ---
 
some text
"##;
    let expected = r##"<table><thead><tr><th>Markdown </th><th> Less </th><th> Pretty</th></tr></thead><tbody>
</tbody></table>
<p>some text</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_31() {
    let original = r##"1. > foo
2. >
"##;
    let expected = r##"<ol>
<li>
<blockquote>
<p>foo</p>
</blockquote>
</li>
<li>
<blockquote>
</blockquote>
</li>
</ol>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_32() {
    let original = r##"[
x

]: f
"##;
    let expected = r##"<p>[
x</p>
<p>]: f</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_33() {
    let original = r##"[foo]:
"##;
    let expected = r##"<p>[foo]:</p>
"##;

    test_markdown_html(original, expected);
}