use csv::{Writer, WriterBuilder};
use std::{fs, path, io};
use std::error::Error;
use crate::lagrangian_node::{LagrangianNode, LAGRANGIAN_NODE_HEADER};

const HEADER_BINARY_FILE: &str = "LAGRANGIAN NODES NASSU";

fn create_folder_for_filename(filename: &path::Path) -> Result<(), Box<dyn Error>>{
    if filename.parent().is_some()  {
        if filename.parent().unwrap().exists() {
            return Ok(());
        }
        fs::DirBuilder::new()
            .recursive(true)
            .create(filename.parent().unwrap())?;
    }
    return Ok(());
}

pub fn save_lagrangian_nodes_csv(filename: &path::Path, lagrangian_nodes: &Vec<LagrangianNode>) 
    -> Result<(), Box<dyn Error>>{

    create_folder_for_filename(filename)?;

    let mut wrt = WriterBuilder::new()
        .has_headers(false)
        .flexible(false)
        .from_path(filename)?;

    // Write header
    wrt.write_record(&LAGRANGIAN_NODE_HEADER)?;

    for ln in lagrangian_nodes.iter(){
        wrt.serialize(ln)?;
    }

    return Ok(());
}

/// Save lagrangian nodes in binary file (extension .lnas)
/// 
/// # Format
/// 
/// Binary is always little endian
/// header: "LAGRANGIAN NODES NASSU" (22 bytes)
/// number of nodes: u64 (8 bytes)
/// nodes: pos.x, pos.y, pos.z, normal.x, normal.y, normal.z, area (f32 all, 28 bytes each)
pub fn save_lagrangian_nodes_bin(filename: &path::Path, lagrangian_nodes: &Vec<LagrangianNode>)
    -> Result<(), Box<dyn Error>>{

    create_folder_for_filename(filename)?;

    let header: &[u8]= HEADER_BINARY_FILE.as_bytes();
    let number_of_nodes: [u8; 8] = lagrangian_nodes.iter().len().to_le_bytes();
    let nodes: Vec<[u8; 28]> = lagrangian_nodes.iter().map(
        |nt| nt
    )

    return Ok(());
}