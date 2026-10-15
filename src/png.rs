use std::{fs::create_dir, path::PathBuf};

use usvg::Transform;

use crate::config::BoardState;

pub fn render_pngs(config: &BoardState, svgs: Vec<(&'static str, &str)>, sizes: Vec<u32>, save_path: &PathBuf) {
    for (year, hex) in config.map.read().unwrap().iter() {
        let hex = format!("{:06x}", hex & 0xFFFFFF);
        
        let mut year_path = save_path.clone();
        year_path.push(format!("{year}"));
        create_dir(&year_path).unwrap();
        
        for (svg_data, svg_name) in &svgs {
            for size in &sizes {
                let svg_data_replaced = svg_data.replace("DEADBEEF", &hex);
                render_png(&svg_data_replaced, svg_name, *size, &year_path);
            }
        }
    }
}


fn render_png(svg_data: &str, name: &str, size: u32, save_path: &PathBuf) {
    let tree = {
        let mut opt = usvg::Options::default();
        opt.fontdb_mut().load_system_fonts();
        usvg::Tree::from_data(svg_data.as_bytes(), &opt).unwrap()
    };

    let scale_factor = size as f32 / tree.size().height();

    let pixmap_size = tree.size().scale_to_height(size as f32).unwrap().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &tree,
        Transform::from_scale(scale_factor, scale_factor),
        &mut pixmap.as_mut(),
    );

    let mut save_path = save_path.clone();
    save_path.push(format!("{name}_{size}.png"));

    dbg!(&save_path);
    pixmap.save_png(save_path).unwrap();
}
