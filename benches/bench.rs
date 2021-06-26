#![feature(test)]
extern crate test;

#[bench]
fn to_owned(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        test::black_box(
            "hello, world!".to_owned()
        )
    });
}

#[bench]
fn to_string(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        test::black_box(
            "hello, world!".to_string()
        )
    });
}
