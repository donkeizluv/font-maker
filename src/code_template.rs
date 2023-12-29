pub const PAT: &str = "{$}";

pub const HEIGHT_TEMP: &str = r#"pub const HEIGHT: u8 = {$};
"#;

pub const GET_CHAR_TEMP: &str = r#"
pub fn get_char(char_code: u8) -> &'static [u8] {
    match char_code {
         {$}

        _ => panic!("usupported character"),
    }
}    
"#;
