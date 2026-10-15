use std::{ffi::OsStr, fs::{create_dir, remove_dir_all}, path::{Component, Path, PathBuf}};

use dotenv;
use rocket::{Request, State, fs::{FileServer, Options}, http::{ContentType, Status}, response::Redirect};
#[macro_use] extern crate rocket;

use macros::make_route_svg;
use config::*;
use png::*;

mod config;
mod png;

#[get("/<board>/colors.css")]
fn get_colors_css(board: u32, state: &State<BoardState>) -> Option<(ContentType, String)> {
    Some((
        ContentType::CSS,
        include_str!("templates/colors.css").replace("DEADBEEF", &get_board_color_hex(state, board)?)
    ))
}

#[get("/<board>/colors.json")]
fn get_colors_json(board: u32, state: &State<BoardState>) -> Option<(ContentType, String)> {
    let hex = get_board_color_hex(state, board)?;
    let (r,g,b) = get_board_color_rgb(state, board)?;

    // TODO make this with serde
    Some((
        ContentType::JSON,
        format!("{{\"hex\": \"#{hex}\", \"red\": {r}, \"green\": {g}, \"blue\": {b}}}\n")
    ))
}

#[catch(default)]
fn internal_server_error(status: Status, _req: &Request) -> String {
    format!("{status}")
}

// Try to find path in /<current_board>/<path>
#[get("/<path..>", rank = 1000)]
fn rerouter(path: PathBuf, state: &State<BoardState>) -> Option<Redirect> {
    let curr_str = state.current.to_string();
    let component = Component::Normal(OsStr::new(&curr_str));
    if path.clone().components().next() == Some(component) {
        return None;
    }

    let res = path.to_str().unwrap();
    Some(Redirect::temporary(format!("/{}/{res}", state.current)))
}

make_route_svg!(logo_hoofd);
make_route_svg!(logo_small_white);
make_route_svg!(logo_small_black);
make_route_svg!(logo_full);

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().unwrap();
    let path = dotenv::var("STORAGE_PATH").unwrap();

    // Ensure directory is completely empty
    if Path::new(&path).exists() {
        remove_dir_all(&path).unwrap();
    }
    create_dir(&path).unwrap();

    let save_path = PathBuf::from(&path);
    let config = read_config("config.toml".into());

    render_pngs(
        &config,
        vec![
            (svg_data_logo_hoofd, "logo_hoofd"),
            (svg_data_logo_small_white, "logo_small_white"),
            (svg_data_logo_small_black, "logo_small_black"),
            (svg_data_logo_full, "logo_full")
        ],
        config.sizes.to_vec(),
        &save_path,
    );

    rocket::build().manage(config).mount("/", routes![
        get_colors_css,
        get_colors_json,
        get_logo_hoofd_svg,
        get_logo_small_black_svg,
        get_logo_small_white_svg,
        get_logo_full_svg,
        rerouter
    ]).register("/", catchers![
        internal_server_error,
    ]).mount("/", FileServer::new(&path, Options::default()))
}
