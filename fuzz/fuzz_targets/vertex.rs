#![no_main]
use libfuzzer_sys::fuzz_target;
use glam::Vec2;
use polyanya::Vertex;

fuzz_target!(|input: (f32, f32, Vec<isize>)| {
    Vertex::new(Vec2::new(input.0, input.1), input.2);
});