use crate::lagrangian::format::LNAS;
use crate::utils::create_folder_for_filename;
use std::error::Error;
use std::{fs, path};

use serde_yaml;

pub fn save_lnas(filename: &path::Path, lnas_obj: &LNAS) -> Result<(), Box<dyn Error>> {
    create_folder_for_filename(filename)?;
    let file = fs::File::create(filename)?;
    serde_yaml::to_writer(file, &lnas_obj)?;
    print!("saving");

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::*;
    use crate::lagrangian::format::get_lnas_obj_save;
    use crate::lagrangian::join::join_information;
    use crate::lagrangian::triangle::generate_lagrangian_triangles;
    use crate::lagrangian::vertice::generate_lagrangian_vertices;
    use crate::stl::surfaces::get_surfaces;

    #[test]
    fn check_join_info_stl_cube() {
        let filename_cfg = String::from("examples/convert_cube.yaml");
        let cfg = Configs::new(&filename_cfg).unwrap();

        let (triangles, surfaces) = get_surfaces(&cfg.stl.files);
        let lagr_vertices = generate_lagrangian_vertices(&triangles);
        let lagr_triangles = generate_lagrangian_triangles(&lagr_vertices, &triangles);
        let (joined_vertices, joined_triangles) = join_information(&lagr_vertices, &lagr_triangles);
        let lnas_obj = get_lnas_obj_save(&cfg, &joined_vertices, &joined_triangles, &surfaces);

        let folder_path = path::Path::new(&cfg.output.folder);
        let lnas_filename = folder_path.join(format!("{}.lnas", cfg.name));

        save_lnas(&lnas_filename, &lnas_obj).unwrap();
    }

    #[test]
    fn check_join_info_stl_terrain() {
        let filename_cfg = String::from("examples/convert_terrain.yaml");
        let cfg = Configs::new(&filename_cfg).unwrap();

        let (triangles, surfaces) = get_surfaces(&cfg.stl.files);
        let lagr_vertices = generate_lagrangian_vertices(&triangles);
        let lagr_triangles = generate_lagrangian_triangles(&lagr_vertices, &triangles);
        let (joined_vertices, joined_triangles) = join_information(&lagr_vertices, &lagr_triangles);
        let lnas_obj = get_lnas_obj_save(&cfg, &joined_vertices, &joined_triangles, &surfaces);

        let folder_path = path::Path::new(&cfg.output.folder);
        let lnas_filename = folder_path.join(format!("{}.lnas", cfg.name));

        save_lnas(&lnas_filename, &lnas_obj).unwrap();
    }

    #[test]
    fn check_join_info_stl_terrain_cube() {
        let filename_cfg = String::from("examples/convert_cube_plane.yaml");
        let cfg = Configs::new(&filename_cfg).unwrap();

        let (triangles, surfaces) = get_surfaces(&cfg.stl.files);
        let lagr_vertices = generate_lagrangian_vertices(&triangles);
        let lagr_triangles = generate_lagrangian_triangles(&lagr_vertices, &triangles);
        let (joined_vertices, joined_triangles) = join_information(&lagr_vertices, &lagr_triangles);
        let lnas_obj = get_lnas_obj_save(&cfg, &joined_vertices, &joined_triangles, &surfaces);

        let folder_path = path::Path::new(&cfg.output.folder);
        let lnas_filename = folder_path.join(format!("{}.lnas", cfg.name));

        save_lnas(&lnas_filename, &lnas_obj).unwrap();
    }
}
