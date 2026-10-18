use std::io::Write;

use xmltree::{Element, Error, XMLNode};

use crate::{ColorVersion, Format, config::Version};

pub fn render_svg<W: Write>(
    input: &str,
    color_version: ColorVersion,
    format: Format,
    board_color: &str,
    version: Version,
    writer: W
) -> Result<(), Error> {
    let mut svg = Element::parse(input.as_bytes()).unwrap();

    // Filter out all paths that aren't marked as included in this version
    svg.children.retain(|it| {
        let node = it.as_element().unwrap();

        // Only filter out path elements
        if node.name != "path" {
            return true;
        }

        // Only keep it if the classList includes the size
        let classes = node.attributes
            .get("class").unwrap()
            .split(" ")
            .collect::<Vec<_>>();

        if classes.contains(&"version-dependent") && !classes.contains(&version.to_class_string()) {
            return false;
        }

        let class_string = format.to_class_string();
        classes.contains(&class_string)
    });

    // Remove unneeded data and inject color classes
    svg.children.iter_mut().for_each(|it| {
        let XMLNode::Element(node) = it else { panic!() };

        // Again, we only want to affect <path> elements
        if node.name != "path" {
            return;
        }

        // Get the classList of this node
        let classes = node.attributes
            .get("class")
            .unwrap()
            .split(" ")
            .collect::<Vec<_>>();

        if classes.contains(&"colorable") {
            *node.attributes.get_mut("class").unwrap() = color_version.to_hoofd_class();
        } else if classes.contains(&"text") {
            *node.attributes.get_mut("class").unwrap() = color_version.to_text_class();
        }

        // Color the outline of the central icon
        else if classes.contains(&"outline") && version == Version::Infill {
            *node.attributes.get_mut("class").unwrap() = color_version.to_text_class();
        } else if classes.contains(&"outline") {
            *node.attributes.get_mut("class").unwrap() = color_version.to_hoofd_class();
        }
    });

    // Crop the viewbox to the correct size
    let target_viewbox = svg.get_child("defs").unwrap()
        .children.iter().find(|it| {
            let node = it.as_element().unwrap();

            if node.name != "viewBox" {
                return false;
            }

            let for_attr = node.attributes.get("for").unwrap();
            for_attr == format.to_class_string()
        }
    ).unwrap().as_element().unwrap().get_text().unwrap();

    svg.attributes.insert("viewBox".into(), target_viewbox.into_owned());

    let style_tag = svg
        .get_mut_child("defs").unwrap()
        .get_mut_child("style").unwrap();

    style_tag.children = vec![
        XMLNode::Text(CSS_STRING.replacen("<BOARD_COLOR>", board_color, 1))
    ];

    svg.write(writer)
}

const CSS_STRING: &str = r#"
    .board-color {
        fill: <BOARD_COLOR>;
    }

    .white {
        fill: #ffffff;
    }

    .black {
        fill: #000000;
    }
"#;

