#![no_main]
use libfuzzer_sys::fuzz_target;
use polyanya::Polygon;

fuzz_target!(|input: (&[u8], bool)| {
    Polygon::new(input.0.iter().map(|x| *x as u32).collect::<Vec<_>>(), input.1);
});