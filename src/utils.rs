#![allow(unused_variables)]

use std::str::FromStr;

use crate::models::quality::{Quality, QualityObject};

/// A utility function for creating download links of different qualities
///
/// ## Arguments
///
/// * `encrypted_media_url` - encrypted media url from the API
///
/// ## Returns
///
/// * `Quality` - An enum that holds the download link(s) with different qualities
pub fn create_download_links(encrypted_media_url: String) -> Quality {
    let qualities = vec![
        ("_21", "12kbps"),
        ("_48", "48kbps"),
        ("_96", "96kbps"),
        ("_160", "160kbps"),
        ("_320", "320kbps"),
    ];

    let key = b"38346591";
    let iv = b"00000000";

    // TODO: Add support for different qualities

    Quality::String(encrypted_media_url)
}

/// Utility function for creating image links of different qualities
///
/// ## Arguments
///
/// * `link` - image link from the API
///
/// ## Returns
///
/// * `Quality` - An enum that holds the image link(s) with different qualities
pub fn create_image_links(link: String) -> Quality {
    let qualities = vec!["50x50", "150x150", "500x500"];

    if qualities.iter().all(|&quality| !link.contains(quality)) {
        return Quality::String(link);
    }

    let mut image_links = Vec::new();

    for quality in qualities {
        let link = if link.contains("150x150") {
            link.replace("150x150", quality)
        } else {
            link.replace("50x50", quality)
        };

        image_links.push(QualityObject {
            quality: quality.to_string(),
            link,
        });
    }

    Quality::List(image_links)
}

/// A utility function for parsing explicit content string to boolean
///
/// ## Arguments
///
/// * `v` - explicit content string
///
/// ## Returns
///
/// * `bool` - A boolean value that indicates if the content is explicit or not
pub fn parse_explicit_content(v: String) -> bool {
    match v.as_str() {
        "1" | "true" => true,
        _ => false,
    }
}

/// A utility function for extracting the token from a link
///
/// ## Arguments
///
/// * `_type` - item type (song, album, artist)
/// * `link` - link from the API
///
/// ## Returns
///
/// * `String` - extracted token
pub fn token_from_link(_type: &str, link: &str) -> String {
    link.split(&format!("{_type}/")).collect::<Vec<_>>()[1]
        .split('/')
        .collect::<Vec<_>>()[1]
        .to_string()
}

/// A utility function for parsing string to a generic type
///
/// ## Arguments
///
/// * `from` - string to parse
///
/// ## Returns
///
/// * `T` - parsed type
pub fn parse_type<T: FromStr + Default>(from: String) -> T {
    from.parse().unwrap_or_default()
}
