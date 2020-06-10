
/// a short list of extensions that shouldn't be searched
///  by content
///
/// If you feel this list should maybe be changed, contact
/// me on miaou or raise an issue.
pub fn is_known_binary(ext: &str) -> bool {
    match ext {
        "doc" | "iso" | "jpg" | "jpeg" | "ods" | "odt" | "pdf" | "png" | "ppt" | "rar" | "xls" => true,
        _ => false,
    }
}