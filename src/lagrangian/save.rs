use crate::cfg::Configs;
use crate::lagrangian::triangle::LagrangianTriangle;
use crate::lagrangian::vertice::LagrangianVertice;
use crate::utils::create_folder_for_filename;
use base64;
use std::error::Error;
use std::{
    fs,
    io::{self, Write},
    path,
};

/// Save lagrangian nodes in Lagrangian Nassu (.lnas) format
///
/// # Lagrangian Nassu Format
///
/// header: "LAGRANGIAN NODES NASSU" (22 bytes)
/// min possible area: f32 (4 bytes)
/// max possible area: f32 (4 bytes)
/// number of nodes: u64 (8 bytes)
/// nodes: pos.x, pos.y, pos.z, normal.x, normal.y, normal.z, area (f32 all, 28 bytes each)
///
/// Always little endian
///
pub fn save_lagrangian_nodes_lnas(
    filename: &path::Path,
    cfg: &Configs,
    lagrangian_vertices: &Vec<LagrangianVertice>,
    lagrangian_triangles: &Vec<LagrangianTriangle>,
) -> Result<(), Box<dyn Error>> {
    create_folder_for_filename(filename)?;
    let version: String = String::from("v0.2.0");

    let vertices: Vec<u8> = lagrangian_vertices
        .iter()
        .flat_map(|nt| nt.get_le_bytes())
        .collect();

    let triangles: Vec<u8> = lagrangian_triangles
        .iter()
        .flat_map(|nt| nt.get_le_bytes())
        .collect();

    base64::encode(vertices);
    base64::encode(triangles);

    // write_to_bin_file(filename, &bytes_write)?;

    return Ok(());
}
