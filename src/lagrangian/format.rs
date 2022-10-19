use crate::cfg::Configs;
use crate::utils::{Vec3f, Vec3u};
use base64;

use serde::{Deserialize, Serialize};
use std::string::String;

#[derive(PartialEq, Serialize, Deserialize)]
pub struct GeometryLNAS {
    pub vertices: String,
    pub triangles: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct NormalizationLNAS {
    pub size: f32,
    pub direction: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct LNAS {
    pub version: String,
    pub name: String,
    pub normalization: NormalizationLNAS,
    pub geometry: GeometryLNAS,
}

pub fn get_lnas_obj_save(
    cfg: &Configs,
    joined_vertices: &Vec<Vec3f>,
    joined_triangles: &Vec<Vec3u>,
) -> LNAS {
    let version: String = String::from("v0.3.0");

    let vertices_bytes: Vec<u8> = joined_vertices
        .iter()
        .flat_map(|v| v.to_le_bytes_as_f32())
        .collect();
    let triangles_bytes: Vec<u8> = joined_triangles
        .iter()
        .flat_map(|v| v.to_le_bytes_as_u32())
        .collect();

    let vertices_b64 = base64::encode(vertices_bytes);
    let triangles_b64 = base64::encode(triangles_bytes);

    let lnas_obj = LNAS {
        name: cfg.name.clone(),
        version: version,
        normalization: NormalizationLNAS {
            size: cfg.normalization.size,
            direction: cfg.normalization.direction.to_string(),
        },
        geometry: GeometryLNAS {
            vertices: vertices_b64,
            triangles: triangles_b64,
        },
    };
    return lnas_obj;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::*;
    use crate::lagrangian::format::get_lnas_obj_save;
    use crate::lagrangian::join::join_information;
    use crate::lagrangian::triangle::generate_lagrangian_triangles;
    use crate::lagrangian::vertice::generate_lagrangian_vertices;
    use crate::stl::reader::read_stl;

    fn get_vecs_from_geometry(geometry: &GeometryLNAS) -> (Vec<Vec3f>, Vec<Vec3u>) {
        let vertices_bytes = base64::decode(&geometry.vertices).unwrap();
        let triangles_bytes = base64::decode(&geometry.triangles).unwrap();

        let mut vertices: Vec<Vec3f> = Vec::new();
        let mut triangles: Vec<Vec3u> = Vec::new();

        // 3 element per vector, 4 bytes per element
        for i in 0..vertices_bytes.len() / 12 {
            let idx = i * 12;
            let vec_bytes: Vec<u8> = (&vertices_bytes[(idx..idx + 12)]).to_vec();
            let vert = Vec3f::from_bytes_le(&vec_bytes);
            vertices.push(vert);
        }
        for i in 0..triangles_bytes.len() / 12 {
            let idx = i * 12;
            let vec_bytes: Vec<u8> = (&triangles_bytes[(idx..idx + 12)]).to_vec();
            let triangle = Vec3u::from_bytes_le(&vec_bytes);
            triangles.push(triangle);
        }

        return (vertices, triangles);
    }

    fn check_lnas_geometry(
        geometry: &GeometryLNAS,
        joined_vertices: &Vec<Vec3f>,
        joined_triangles: &Vec<Vec3u>,
    ) {
        let (gem_vertices, gem_triangles) = get_vecs_from_geometry(geometry);

        assert_eq!(gem_vertices.len(), joined_vertices.len());
        assert_eq!(gem_triangles.len(), joined_triangles.len());

        for (i, vert) in gem_vertices.iter().enumerate() {
            if *vert != joined_vertices[i] {
                panic!(
                    "Vertice at index {} is different ({} != {})",
                    i, vert, joined_vertices[i]
                );
            }
        }

        for (i, triangle) in gem_triangles.iter().enumerate() {
            if *triangle != joined_triangles[i] {
                panic!(
                    "Vertice at index {} is different ({} != {})",
                    i, triangle, joined_triangles[i]
                );
            }
        }
    }

    #[test]
    fn check_join_info_stl_cube() {
        let filename = String::from("examples/stl/cube.stl");
        let filename_cfg = String::from("examples/convert_cube.yaml");
        let cfg = Configs::new(&filename_cfg).unwrap();

        let triangles = read_stl(&filename);
        let lagr_vertices = generate_lagrangian_vertices(&triangles);
        let lagr_triangles = generate_lagrangian_triangles(&lagr_vertices, &triangles);
        let (joined_vertices, joined_triangles) = join_information(&lagr_vertices, &lagr_triangles);
        let lnas_obj = get_lnas_obj_save(&cfg, &joined_vertices, &joined_triangles);

        check_lnas_geometry(&lnas_obj.geometry, &joined_vertices, &joined_triangles);
    }

    #[test]
    fn check_join_infocar_stl_terrain() {
        let filename = String::from("examples/stl/terrain.stl");
        let filename_cfg = String::from("examples/convert_terrain.yaml");
        let cfg = Configs::new(&filename_cfg).unwrap();

        let triangles = read_stl(&filename);
        let lagr_vertices = generate_lagrangian_vertices(&triangles);
        let lagr_triangles = generate_lagrangian_triangles(&lagr_vertices, &triangles);
        let (joined_vertices, joined_triangles) = join_information(&lagr_vertices, &lagr_triangles);
        let lnas_obj = get_lnas_obj_save(&cfg, &joined_vertices, &joined_triangles);

        check_lnas_geometry(&lnas_obj.geometry, &joined_vertices, &joined_triangles);
    }
}
