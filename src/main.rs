pub mod stl_reader;
pub mod stl_triangle;
pub mod common;

fn main() {

    println!("Hello, world!");

    let filename = "teste.stl";
    let triangles = stl_reader::read_stl(filename);

}
