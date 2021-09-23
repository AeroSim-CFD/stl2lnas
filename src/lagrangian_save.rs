use crate::lagrangian_node::{LagrangianNode, LAGRANGIAN_NODE_HEADER};
use csv::WriterBuilder;
use std::error::Error;
use std::{fs, path};

const HEADER_BINARY_FILE: &str = "LAGRANGIAN NODES NASSU";

fn create_folder_for_filename(filename: &path::Path) -> Result<(), Box<dyn Error>> {
    if filename.parent().is_some() {
        if filename.parent().unwrap().exists() {
            return Ok(());
        }
        fs::create_dir_all(filename.parent().unwrap())?;
    }
    return Ok(());
}

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

/// Save lagrangian nodes in binary file (extension .lnas)
///
/// # Format
///
/// header: "LAGRANGIAN NODES NASSU" (22 bytes)
/// number of nodes: u64 (8 bytes)
/// x normalization: f32 (4 bytes)
/// nodes: pos.x, pos.y, pos.z, normal.x, normal.y, normal.z, area (f32 all, 28 bytes each)
///
/// Binary is always little endian
pub fn save_lagrangian_nodes_bin(
    filename: &path::Path,
    lagrangian_nodes: &Vec<LagrangianNode>,
    x_normalization: f32,
) -> Result<(), Box<dyn Error>> {
    create_folder_for_filename(filename)?;

    let header: &[u8] = HEADER_BINARY_FILE.as_bytes();
    let number_of_nodes: [u8; 8] = lagrangian_nodes.iter().len().to_le_bytes();
    let x_normalization_b = x_normalization.to_le_bytes();
    let nodes: Vec<[u8; 28]> = lagrangian_nodes
        .iter()
        .map(|nt| nt.get_le_bytes())
        .collect();

    let mut bytes_write: Vec<u8> = header.to_vec();
    bytes_write.extend(number_of_nodes.to_vec());
    bytes_write.extend(x_normalization_b.to_vec());
    for n in nodes {
        bytes_write.extend(n.to_vec());
    }

    return Ok(());
}
