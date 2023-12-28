pub const IMPORTS: &str = r#"
use super::*;
"#;

pub const TOTAL_STYLES: &str = r#"
pub const TOTAL_STYLES: u32 = {$};
"#;

pub const PICKER_TEMP: &str = r#"
pub fn pick<'a>(style_idx: u32) -> (u8, Box<dyn Fn(u8) -> &'static str>) {
    match style_idx {
        {$}

        _ => panic!("unsupported style"),
    }
}

"#;
