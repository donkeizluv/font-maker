pub const PAT: &str = "{$}";

pub const HEIGHT_TEMP: &str = r#"pub const HEIGHT: u8 = {$};
"#;

pub const GET_CHAR_TEMP: &str = r#"
pub fn get_char(char_code: u8) -> &'static str {
    match char_code {
         {$}

        _ => panic!("unsupported character"),
    }
}    
"#;

pub const GEN_COMMENT: &str = r#"
/* 
* This file is generated using font-maker
* Do not modify manually
*/
"#;
