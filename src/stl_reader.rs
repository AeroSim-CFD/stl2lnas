use crate::common::Point3D;
use crate::stl_triangle;
use std::{convert::TryInto, fs};

const POINT_BYTE_SIZE: usize = 12;
const TRIANGLE_BYTES_SIZE: usize = 50;
const HEADER_BYTES_SIZE: usize = 80;

fn read_file(filename: &str) -> Vec<u8> {
    let content = fs::read(filename).expect(format!("Unable to read {}", &filename).as_str());
    return content;
}

// I don't know how to check endianess, but this is the way that it works
fn bytes_to_u32(b: &[u8; 4]) -> u32 {
    return u32::from_be_bytes([b[3], b[2], b[1], b[0]]);
}

fn bytes_to_f32(b: &[u8; 4]) -> f32 {
    return f32::from_be_bytes([b[3], b[2], b[1], b[0]]);
}

#[allow(non_snake_case)]
fn bytes_to_point3D(b: &[u8; POINT_BYTE_SIZE]) -> Point3D {
    return Point3D {
        x: bytes_to_f32(b[..4].try_into().expect("Invalid point bytes")),
        y: bytes_to_f32(b[4..8].try_into().expect("Invalid point bytes")),
        z: bytes_to_f32(b[8..12].try_into().expect("Invalid point bytes")),
    };
}

fn number_of_triangles(stl_content: &Vec<u8>) -> u32 {
    // 80 bytes of header, then the number of triangles as u32 (4 bytes)
    let n_triangles_bytes: [u8; 4] = stl_content[HEADER_BYTES_SIZE..HEADER_BYTES_SIZE + 4]
        .try_into()
        .expect("Invalid file format (number of triangles)");
    let n_triangles = bytes_to_u32(&n_triangles_bytes);
    return n_triangles;
}

fn bytes_to_triangle(b: &[u8; TRIANGLE_BYTES_SIZE]) -> stl_triangle::TriangleSTL {
    let normal = bytes_to_point3D(b[..12].try_into().expect("Invalid file format (point 0)"));
    let point0 = bytes_to_point3D(b[12..24].try_into().expect("Invalid file format (point 1)"));
    let point1 = bytes_to_point3D(b[24..36].try_into().expect("Invalid file format (point 2)"));
    let point2 = bytes_to_point3D(b[36..48].try_into().expect("Invalid file format (normal)"));
    // 2 points are for attribute byte count
    return stl_triangle::TriangleSTL::new(point0, point1, point2, normal);
}

fn triangles_from_stl(stl_content: &Vec<u8>, n_triangles: u32) -> Vec<stl_triangle::TriangleSTL> {
    // + 4 due to triangle numbers
    let start_byte = HEADER_BYTES_SIZE + 4;
    let mut all_triangles: Vec<stl_triangle::TriangleSTL> = Vec::new();
    for i in 0..n_triangles {
        let curr_byte_idx = start_byte + TRIANGLE_BYTES_SIZE * (i as usize);
        let curr_triangle_bytes: &[u8; TRIANGLE_BYTES_SIZE] = stl_content
            [curr_byte_idx..curr_byte_idx + TRIANGLE_BYTES_SIZE]
            .try_into()
            .expect("Invalid triangle bytes");
        let triangle = bytes_to_triangle(curr_triangle_bytes);
        all_triangles.push(triangle);
    }

    return all_triangles;
}

pub fn read_stl(filename: &str) -> Vec<stl_triangle::TriangleSTL> {
    let stl_content = read_file(filename);
    let n_triangles = number_of_triangles(&stl_content);
    let triangles = triangles_from_stl(&stl_content, n_triangles);
    return triangles;
}
