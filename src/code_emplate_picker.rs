pub const IMPORTS: &str = r#"
use anchor_lang::prelude::*;
use crate::error::CustomProgramError;
use super::*;
"#;

pub const STYLE_ENUM_TEMP: &str = r#"
enum Style {
    {$}
}
"#;

pub const TRY_FROM_IMPL_TEMP: &str = r#"
impl TryFrom<u32> for Style {
    type Error = CustomProgramError;

    fn try_from(v: u32) -> std::result::Result<Self, Self::Error> {
        match v {
            {$}
            _ => Err(CustomProgramError::InvalidStyle.into()),
        }
    }
}
"#;

pub const MAX_RANGE_TEMP: &str = r#"
pub fn get_max_range() -> usize {
    Style::{$} as usize
}
"#;

pub const PICKER_TEMP: &str = r#"
pub fn pick(style: u32) -> Result<(u8, CHARSET)> {
    match Style::try_from(style)? {
        {$}
    }
}
"#;
