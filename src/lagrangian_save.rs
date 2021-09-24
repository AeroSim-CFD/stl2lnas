use crate::lagrangian_node::{LagrangianNode, LAGRANGIAN_NODE_HEADER};
use crate::utils::create_folder_for_filename;
use csv::WriterBuilder;
use std::error::Error;
use std::{
    fs,
    io::{self, Write},
    path,
};

const HEADER_BINARY_FILE: &str = "LAGRANGIAN NODES NASSU";

pub fn save_lagrangian_nodes_csv(
    filename: &path::Path,
    lagrangian_nodes: &Vec<LagrangianNode>,
) -> Result<(), Box<dyn Error>> {
    create_folder_for_filename(filename)?;

    let mut wrt = WriterBuilder::new()
        .has_headers(false)
        .flexible(false)
        .from_path(filename)?;

    // Write header
    wrt.write_record(&LAGRANGIAN_NODE_HEADER)?;

    for ln in lagrangian_nodes.iter() {
        wrt.serialize(ln)?;
    }
    wrt.flush()?;

    return Ok(());
}

fn write_to_bin_file(filename: &path::Path, v: &Vec<u8>) -> Result<(), io::Error> {
    let mut file = fs::File::create(filename)?;
    // Write a slice of bytes to the file
    file.write_all(v)?;
    return Ok(());
}

/// Save lagrangian nodes in Lagrangian Nassu (.lnas) format
///
/// # Lagrangian Nassu Format
///
/// header: "LAGRANGIAN NODES NASSU" (22 bytes)
/// number of nodes: u64 (8 bytes)
/// min possible area: f32 (4 bytes)
/// max possible area: f32 (4 bytes)
/// nodes: pos.x, pos.y, pos.z, normal.x, normal.y, normal.z, area (f32 all, 28 bytes each)
///
/// Always little endian
///
pub fn save_lagrangian_nodes_lnas(
    filename: &path::Path,
    lagrangian_nodes: &Vec<LagrangianNode>,
    min_area: f32,
    max_area: f32,
) -> Result<(), Box<dyn Error>> {
    create_folder_for_filename(filename)?;

    let header: &[u8] = HEADER_BINARY_FILE.as_bytes();
    let number_of_nodes: [u8; 8] = lagrangian_nodes.iter().len().to_le_bytes();
    let nodes: Vec<[u8; 28]> = lagrangian_nodes
        .iter()
        .map(|nt| nt.get_le_bytes())
        .collect();

    let mut bytes_write: Vec<u8> = header.to_vec();
    bytes_write.extend(number_of_nodes.to_vec());
    bytes_write.extend(min_area.to_le_bytes().to_vec());
    bytes_write.extend(max_area.to_le_bytes().to_vec());
    for n in nodes {
        bytes_write.extend(n.to_vec());
    }

    write_to_bin_file(filename, &bytes_write)?;

    return Ok(());
}
