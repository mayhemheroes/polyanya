#![no_main]
use libfuzzer_sys::fuzz_target;
use polyanya::{Mesh, Vertex, Polygon};
use glam::Vec2;

fuzz_target!(|input: ((f32, f32), (f32, f32))| {
    let mesh = Mesh::new(
        vec![
            Vertex::new(Vec2::new(0., 6.), vec![0, -1]),           // 0
            Vertex::new(Vec2::new(2., 5.), vec![0, -1, 2]),        // 1
            Vertex::new(Vec2::new(5., 7.), vec![0, 2, -1]),        // 2
            Vertex::new(Vec2::new(5., 8.), vec![0, -1]),           // 3
            Vertex::new(Vec2::new(0., 8.), vec![0, -1]),           // 4
            Vertex::new(Vec2::new(1., 4.), vec![1, -1]),           // 5
            Vertex::new(Vec2::new(2., 1.), vec![1, -1]),           // 6
            Vertex::new(Vec2::new(4., 1.), vec![1, -1]),           // 7
            Vertex::new(Vec2::new(4., 2.), vec![1, -1, 2]),        // 8
            Vertex::new(Vec2::new(2., 4.), vec![1, 2, -1]),        // 9
            Vertex::new(Vec2::new(7., 4.), vec![2, -1, 4]),        // 10
            Vertex::new(Vec2::new(10., 7.), vec![2, 4, 6, -1, 3]), // 11
            Vertex::new(Vec2::new(7., 7.), vec![2, 3, -1]),        // 12
            Vertex::new(Vec2::new(11., 8.), vec![3, -1]),          // 13
            Vertex::new(Vec2::new(7., 8.), vec![3, -1]),           // 14
            Vertex::new(Vec2::new(7., 0.), vec![5, 4, -1]),        // 15
            Vertex::new(Vec2::new(11., 3.), vec![4, 5, -1]),       // 16
            Vertex::new(Vec2::new(11., 5.), vec![4, -1, 6]),       // 17
            Vertex::new(Vec2::new(12., 0.), vec![5, -1]),          // 18
            Vertex::new(Vec2::new(12., 3.), vec![5, -1]),          // 19
            Vertex::new(Vec2::new(13., 5.), vec![6, -1]),          // 20
            Vertex::new(Vec2::new(13., 7.), vec![6, -1]),          // 21
            Vertex::new(Vec2::new(1., 3.), vec![1, -1]),           // 22
        ],
        vec![
            Polygon::new(vec![0, 1, 2, 3, 4], true),           // 0
            Polygon::new(vec![5, 22, 6, 7, 8, 9], true),       // 1
            Polygon::new(vec![1, 9, 8, 10, 11, 12, 2], false), // 2
            Polygon::new(vec![12, 11, 13, 14], true),          // 3
            Polygon::new(vec![10, 15, 16, 17, 11], false),     // 4
            Polygon::new(vec![15, 18, 19, 16], true),          // 5
            Polygon::new(vec![11, 17, 20, 21], true),          // 6
        ],
    );

    let from = Vec2::new(input.0.0, input.0.1);
    let to = Vec2::new(input.1.0, input.1.1);

    mesh.path(from, to);
});