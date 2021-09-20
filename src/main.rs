pub mod common;
pub mod stl_reader;
pub mod stl_triangle;

fn main() {
    let filename = "fixture/cube.stl";
    let triangles = stl_reader::read_stl(filename);
    for triangle in triangles {
        println!("{}", triangle);
    }
}
