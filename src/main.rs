use std::io::{Read, Write};
use std::fs::{File, OpenOptions, create_dir_all};
use std::process::Command;

use crate::png::render_png;
use crate::config::{get_board_color_hex, get_board_color_rgb, read_config};
use crate::svg::render_svg;

mod config;
mod png;
mod svg;

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
enum ColorVersion {
    AllWhite,
    AllBlack,
    TextBlack,
    TextWhite,
}

impl ColorVersion {
    pub fn to_text_class(self) -> String {
        match self {
            ColorVersion::AllWhite => "white",
            ColorVersion::TextWhite => "white",
            ColorVersion::AllBlack => "black",
            ColorVersion::TextBlack => "black",
        }.into()
    }

    pub fn to_hoofd_class(self) -> String {
        match self {
            ColorVersion::AllWhite => "white",
            ColorVersion::AllBlack => "black",
            ColorVersion::TextWhite => "board-color",
            ColorVersion::TextBlack => "board-color",
        }.into()
    }

    pub fn to_file_string(self, format: Format) -> String {
        match self {
            ColorVersion::AllWhite => "wit",
            ColorVersion::AllBlack => "zwart",
            ColorVersion::TextBlack => {
                if format == Format::Hoofd {
                    "kleur"
                } else {
                    "zwart_kleur"
                }
            },
            ColorVersion::TextWhite => {
                if format == Format::Hoofd {
                    "kleur"
                } else {
                    "wit_kleur"
                }
            },
        }.into()
    }
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
enum Format {
    Full,
    Small,
    Hoofd,
}

impl Format {
    pub fn to_class_string<'a>(self) -> &'a str {
        match self {
            Format::Full => "full",
            Format::Small => "small",
            Format::Hoofd => "hoofd",
        }
    }

    pub fn to_file_string(self) -> String {
        match self {
            Format::Full => "volledig",
            Format::Small => "compact",
            Format::Hoofd => "hoofd",
        }.into()
    }
}

fn write_to_file(filename: String, content: String) {
    let mut file = OpenOptions::new()
        .write(true).create(true)
        .open(&filename).unwrap();

    file.write(&mut content.as_bytes()).unwrap();
}

fn render_svg_png(
    xml: &String,
    color_versions: &[ColorVersion],
    formats: &[Format],
    basepath: &String,
    color: &String,
    draw_fat: bool,
    png_size: u32
) {
    for color_version in color_versions {
        for format in formats {
            if *format == Format::Hoofd && *color_version == ColorVersion::TextWhite {
                continue;
            }

            // SVG
            let format_string = format.to_file_string();
            let color_string = color_version.to_file_string(*format);
            let filename_svg = format!("{basepath}/logo_{format_string}_{color_string}.svg");
            let filename_png = format!("{basepath}/logo_{format_string}_{color_string}.png");

            let writer = OpenOptions::new()
                .write(true).create(true)
                .open(&filename_svg).unwrap();
            render_svg(xml, *color_version, *format, color, draw_fat, writer).unwrap();


            // PNG
            let mut svg_string = String::new();
            File::open(&filename_svg).unwrap().read_to_string(&mut svg_string).unwrap();
            render_png(&svg_string, png_size, &filename_png)
        }
    }
}

fn main() {
    dotenv::dotenv().unwrap();
    let storage_path = dotenv::var("STORAGE_PATH").unwrap();
    
    const COLOR_VERSIONS: [ColorVersion; 2] = [
        ColorVersion::TextBlack,
        ColorVersion::TextWhite
    ];

    const FORMATS: [Format; 3] = [
        Format::Full,
        Format::Small,
        Format::Hoofd,
    ];

    let mut xml = String::new();
    File::open("logo.svg").unwrap().read_to_string(&mut xml).unwrap();

    let config = read_config("config.toml".into());
    
    for (year, (color, fat)) in &config.map {
        dbg!(year);
        
        let board_color = get_board_color_hex(*color);
        create_dir_all(format!("{storage_path}/{year}")).unwrap();

        // Create SVG and PNG files
        render_svg_png(&xml, &COLOR_VERSIONS, &FORMATS, &format!("{storage_path}/{year}"), &board_color, *fat, config.png_size);

        // Create CSS and JSON files
        let buf = format!(":root {{\n  --board-color: {board_color};\n}}");
        write_to_file(format!("{storage_path}/{year}/colors.css"), buf);

        let (r,g,b) = get_board_color_rgb(*color);
        let buf = format!(r#"{{"hex": "{board_color}", "red": {r}, "green": {g}, "blue": {b}}}"#);
        write_to_file(format!("{storage_path}/{year}/colors.json"), buf);
    }

    let cp_output = Command::new("bash")
        .arg("-c")
        .arg(format!("cp {storage_path}/{}/* {storage_path}/", config.current))
        .output()
        .expect("process should execute successfully");

    if !cp_output.status.success() {
        eprintln!("Copy was not successful!!");
        dbg!(cp_output);
        panic!();
    }

    let should_draw_fat = config.map.get(&config.current).unwrap().1;
    
    render_svg_png(&xml, &[ColorVersion::AllWhite, ColorVersion::AllBlack], &FORMATS, &storage_path, &"".into(), should_draw_fat, config.png_size);
}
