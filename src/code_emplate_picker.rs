pub const IMPORTS: &str = r#"
use anchor_lang::prelude::*;
use crate::error::CustomProgramError;
use super::*;
"#;

pub const MAX_RANGE_TEMP: &str = r#"
pub const MAX_RANGE: u32 = {$};
"#;

pub const PICKER_TEMP: &str = r#"
pub fn pick<'a>(style_idx: u32) -> Result<(u8, &'a [&'static str])> {
    match style_idx {
        {$}
    }
}
"#;
