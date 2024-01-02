use code_emplate_picker::{IMPORTS, PICKER_TEMP, TOTAL_STYLES};
use code_template::{GEN_COMMENT, GET_CHAR_TEMP, HEIGHT_TEMP, PAT};
use figrs::{Figlet, FigletOptions};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufWriter, Write},
    ops::RangeInclusive,
};

use crate::config::ALL_FONTS;

const MAX_ASCII: u8 = 126;

const SPACE: u8 = 32;
const NUMBER_BOUND: RangeInclusive<u8> = 48..=57;
const UPPER_BOUND: RangeInclusive<u8> = 65..=90;
const LOWER_BOUND: RangeInclusive<u8> = 97..=122;

mod code_emplate_picker;
mod code_template;
mod config;

fn main() {
    // clean output
    fs::remove_dir_all("output").unwrap();
    fs::create_dir("output").unwrap();

    let all_char = (0..=MAX_ASCII)
        .into_iter()
        .map(|c| {
            if c == SPACE
                || NUMBER_BOUND.contains(&c)
                || UPPER_BOUND.contains(&c)
                || LOWER_BOUND.contains(&c)
            {
                return Some(std::str::from_utf8(&[c]).unwrap().to_string());
            }
            None
        })
        .collect::<Vec<Option<String>>>();
    let mut bl: HashSet<String> = HashSet::new();

    for font in ALL_FONTS {
        let opt = FigletOptions {
            font: font.to_string(),
            ..FigletOptions::default()
        };
        let set = all_char
            .clone()
            .into_iter()
            .map(|c| match c {
                Some(code) => Figlet::text(code, opt.clone()).unwrap().text,
                None => "".to_string(),
            })
            .collect::<Vec<String>>();

        // check if all required bounds have char
        if let Some(empty) = [SPACE]
            .into_iter()
            .find(|b| set.get(*b as usize).unwrap().is_empty())
        {
            println!("font: {} missing SPACE: {} -> skip", font, empty);
            bl.insert(font.to_string());
            continue;
        }

        if let Some(empty) = NUMBER_BOUND
            .into_iter()
            .find(|b| set.get(*b as usize).unwrap().is_empty())
        {
            println!("font: {} missing number: {} -> skip", font, empty);
            bl.insert(font.to_string());
            continue;
        }
        if let Some(empty) = UPPER_BOUND
            .into_iter()
            .find(|b| set.get(*b as usize).unwrap().is_empty())
        {
            println!("font: {} missing upper: {} -> skip", font, empty);
            bl.insert(font.to_string());
            continue;
        }
        if let Some(empty) = LOWER_BOUND
            .into_iter()
            .find(|b| set.get(*b as usize).unwrap().is_empty())
        {
            println!("font: {} missing lower: {} -> skip", font, empty);
            bl.insert(font.to_string());
            continue;
        }

        let max_char = set.iter().map(|c| c.len()).max().unwrap();

        if max_char >= 400 {
            println!("warning! font: {font} max size is: {max_char}");
        }

        let max_height_idx = set
            .iter()
            .map(|c| {
                c.split("\n")
                    .filter(|c| !c.is_empty())
                    .collect::<Vec<&str>>()
                    .len()
            })
            .max()
            .unwrap();

        let chars_str = set
            .iter()
            .enumerate()
            .map(|(char_code, c)| {
                if c.is_empty() {
                    return "".to_owned();
                }
                format!("const CHAR_{}: &str = r#\"\n{}\n\"#;", char_code, c)
            })
            .filter(|c| !c.is_empty())
            .collect::<Vec<String>>();

        let get_char_fn = set
            .iter()
            .enumerate()
            .map(|(char_code, c)| {
                if c.is_empty() {
                    return "".to_owned();
                }
                format!("{char_code} => CHAR_{char_code},")
            })
            .filter(|c| !c.is_empty())
            .collect::<Vec<String>>();
        let convention_name = name_conv(font);

        let font_codegen = [
            GEN_COMMENT.to_string(),
            HEIGHT_TEMP.replace(PAT, &max_height_idx.to_string()),
            chars_str.join("\n"),
            GET_CHAR_TEMP.replace(PAT, &get_char_fn.join("\n")),
        ]
        .concat();

        let path = format!("output/font_{}.rs", convention_name);

        let f = File::create(path).unwrap();
        let mut f = BufWriter::new(f);
        f.write_all(font_codegen.as_bytes()).unwrap();
    }
    let bl_filter_fonts = ALL_FONTS
        .into_iter()
        .filter(|f| !bl.contains(f.to_owned()))
        .collect::<Vec<&str>>();

    let all_fonts_conv = bl_filter_fonts
        .iter()
        .map(|f| name_conv(f))
        .collect::<Vec<String>>();

    let import_mods = all_fonts_conv
        .iter()
        .map(|f| format!("mod font_{};", f))
        .collect::<Vec<String>>();

    let mod_codegen = vec![
        vec![GEN_COMMENT.to_string(), "pub mod picker;".to_string()],
        import_mods,
    ]
    .concat()
    .join("\n");

    let max_type_idx = all_fonts_conv.len() - 1;

    let picker_match = all_fonts_conv
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            let filename = format!("font_{}", c);
            format!("{idx} => ({filename}::HEIGHT, Box::new(|idx: u8| {filename}::get_char(idx))),")
        })
        .collect::<Vec<String>>();
    // picker_match.push("\n _ => Err(CustomProgramError::InvalidStyle.into()),".to_string());

    let picker_codegen = [
        GEN_COMMENT.to_string(),
        IMPORTS.to_string(),
        TOTAL_STYLES.replace(PAT, &max_type_idx.to_string()),
        PICKER_TEMP.replace(PAT, &picker_match.join("\n")),
    ]
    .join("\n");

    let f = File::create("output/mod.rs").unwrap();
    let mut f = BufWriter::new(f);
    f.write_all(mod_codegen.as_bytes()).unwrap();

    let f = File::create("output/picker.rs").unwrap();
    let mut f = BufWriter::new(f);
    f.write_all(picker_codegen.as_bytes()).unwrap();
}

fn name_conv(name: &str) -> String {
    name.to_lowercase()
        .replace("-", "_")
        .replace(" ", "_")
        .replace("'", "")
}
