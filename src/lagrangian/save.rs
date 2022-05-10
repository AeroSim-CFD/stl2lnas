use crate::cfg::Configs;
use crate::utils::create_folder_for_filename;
use crate::utils::{Vec3f, Vec3u};
use base64;
use std::error::Error;
use std::{fs, path};

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::string::String;

#[derive(PartialEq, Serialize, Deserialize)]
pub struct GeometryLNAS {
    pub vertices: String,
    pub triangles: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct LNAS {
    pub version: String,
    pub name: String,
    pub normalization_x: f32,
    pub geometry: GeometryLNAS,
}

pub fn save_lnas(
    filename: &path::Path,
    cfg: &Configs,
    joined_vertices: &Vec<Vec3f>,
    joined_triangles: &Vec<Vec3u>,
) -> Result<(), Box<dyn Error>> {
    let version: String = String::from("v0.2.0");

    let vertices_bytes: Vec<u8> = joined_vertices
        .iter()
        .flat_map(|v| v.to_le_bytes_as_f32())
        .collect();
    let triangles_bytes: Vec<u8> = joined_triangles
        .iter()
        .flat_map(|v| v.to_le_bytes_as_f32())
        .collect();

    let vertices_b64 = base64::encode(vertices_bytes);
    let triangles_b64 = base64::encode(triangles_bytes);

    let lnas_save = LNAS {
        name: cfg.name.clone(),
        version: version,
        normalization_x: cfg.conversion.normalization_x,
        geometry: GeometryLNAS {
            vertices: vertices_b64,
            triangles: triangles_b64,
        },
    };

    create_folder_for_filename(filename)?;
    let file = fs::File::create(filename)?;
    serde_yaml::to_writer(file, &lnas_save)?;

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::*;
    use crate::lagrangian::format::join_information;
    use crate::lagrangian::triangle::generate_lagrangian_triangles;
    use crate::lagrangian::vertice::generate_lagrangian_vertices;
    use crate::stl::reader::read_stl;

    #[test]
    fn check_join_info_stl_cube() {
        let filename = String::from("examples/stl/cube.stl");
        let filename_cfg = String::from("examples/convert_cube.yaml");

        let triangles = read_stl(&filename);
        let lagr_vertices = generate_lagrangian_vertices(&triangles);
        let lagr_triangles = generate_lagrangian_triangles(&lagr_vertices, &triangles);
        let (joined_vertices, joined_triangles) = join_information(&lagr_vertices, &lagr_triangles);

        let cfg = Configs::new(&filename_cfg).unwrap();
        let folder_path = path::Path::new(&cfg.output.folder);
        let lnas_filename = folder_path.join(format!("{}.lnas", cfg.name));

        save_lnas(&lnas_filename, &cfg, &joined_vertices, &joined_triangles).unwrap();
    }

    #[test]
    fn check_join_infocar_stl_terrain() {
        let filename = String::from("examples/stl/terrain.stl");
        let filename_cfg = String::from("examples/convert_terrain.yaml");

        let triangles = read_stl(&filename);
        let lagr_vertices = generate_lagrangian_vertices(&triangles);
        let lagr_triangles = generate_lagrangian_triangles(&lagr_vertices, &triangles);
        let (joined_vertices, joined_triangles) = join_information(&lagr_vertices, &lagr_triangles);

        let cfg = Configs::new(&filename_cfg).unwrap();
        let folder_path = path::Path::new(&cfg.output.folder);
        let lnas_filename = folder_path.join(format!("{}.lnas", cfg.name));

        save_lnas(&lnas_filename, &cfg, &joined_vertices, &joined_triangles).unwrap();
    }
}
