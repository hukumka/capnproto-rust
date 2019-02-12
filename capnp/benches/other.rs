#[macro_use]
extern crate criterion;
extern crate capnp;

use criterion::{Criterion, ParameterizedBenchmark};

use capnp::Word;

fn word_alloc_zeroed_current(len: usize) -> Vec<Word> {
    Word::allocate_zeroed_vec(len)
}

fn word_alloc_zeroed_safe(len: usize) -> Vec<Word> {
    let zero = Word { raw_content: 0 };
    vec![zero; len]
}

fn word_alloc_zeroed(length: usize) -> Vec<Word> {
    let mut result: Vec<Word> = Vec::with_capacity(length);
    unsafe {
        result.set_len(length);
        let p: *mut u8 = result.as_mut_ptr() as *mut u8;
        ::std::ptr::write_bytes(p, 0u8, length * ::std::mem::size_of::<Word>());
    }
    result
}

fn bench_word_alloc(c: &mut Criterion) {
    let sizes = vec![10000usize];
    let b = ParameterizedBenchmark::new(
        "current",
        |b, i| b.iter(|| word_alloc_zeroed_current(*i)),
        sizes,
    )
    .with_function("vec!", |b, i| b.iter(|| word_alloc_zeroed_safe(*i)))
    .with_function("unsafe", |b, i| b.iter(|| word_alloc_zeroed(*i)));

    c.bench("Word::allocate_zeroes_vec", b);
}

criterion_group!(benches, bench_word_alloc);
criterion_main!(benches);
