use code_emplate_picker::{IMPORTS, PICKER_TEMP, TOTAL_STYLES};
use code_template::{GET_CHAR_TEMP, HEIGHT_TEMP, PAT};
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
        gen_font(font, &all_char, &mut bl)
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
        .map(|f| format!("pub mod font_{};", f))
        .collect::<Vec<String>>();

    let mod_content = import_mods.join("\n");

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

    let picker_content = [
        IMPORTS.to_string(),
        TOTAL_STYLES.replace(PAT, &max_type_idx.to_string()),
        PICKER_TEMP.replace(PAT, &picker_match.join("\n")),
    ]
    .join("\n");

    let f = File::create("output/mod.rs").unwrap();
    let mut f = BufWriter::new(f);
    f.write_all(mod_content.as_bytes()).unwrap();

    let f = File::create("output/picker.rs").unwrap();
    let mut f = BufWriter::new(f);
    f.write_all(picker_content.as_bytes()).unwrap();
}

fn name_conv(name: &str) -> String {
    name.to_lowercase()
        .replace("-", "_")
        .replace(" ", "_")
        .replace("'", "")
}

fn gen_font(font: &str, all_char: &Vec<Option<String>>, bl: &mut HashSet<String>) {
    println!("Font {}", font);
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
        return;
    }

    if let Some(empty) = NUMBER_BOUND
        .into_iter()
        .find(|b| set.get(*b as usize).unwrap().is_empty())
    {
        println!("font: {} missing number: {} -> skip", font, empty);
        bl.insert(font.to_string());
        return;
    }
    if let Some(empty) = UPPER_BOUND
        .into_iter()
        .find(|b| set.get(*b as usize).unwrap().is_empty())
    {
        println!("font: {} missing upper: {} -> skip", font, empty);
        bl.insert(font.to_string());
        return;
    }
    if let Some(empty) = LOWER_BOUND
        .into_iter()
        .find(|b| set.get(*b as usize).unwrap().is_empty())
    {
        println!("font: {} missing lower: {} -> skip", font, empty);
        bl.insert(font.to_string());
        return;
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

            let byte_array = convert_byte_array_to_byte_array_as_string(c.as_bytes());
            let byte_array_as_string: &str = byte_array.as_str();
            convert_byte_array_as_string_to_string(byte_array_as_string);
            format!(
                "const CHAR_{}: &[u8] = {};",
                char_code, byte_array_as_string
            )
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

    let merged = [
        HEIGHT_TEMP.replace(PAT, &max_height_idx.to_string()),
        chars_str.join("\n"),
        GET_CHAR_TEMP.replace(PAT, &get_char_fn.join("\n")),
    ]
    .concat();

    let path = format!("output/font_{}.rs", convention_name);

    let f = File::create(path).unwrap();
    let mut f = BufWriter::new(f);
    f.write_all(merged.as_bytes()).unwrap();
}

fn convert_byte_array_to_byte_array_as_string(bytes: &[u8]) -> String {
    let s: Vec<String> = bytes.iter().map(|b| format!("{}", b)).collect();
    let result = s.join(", ");
    format!("&[{}]", result)
}

fn convert_byte_array_as_string_to_string(s: &str) {
    // Remove &[ and ], split by commas
    let binding = s.replace("&[", "").replace("]", "");
    let parts: Vec<&str> = binding.split(',').collect();

    // Parse each part as a u8 and collect into a Vec<u8>
    let bytes: Vec<u8> = parts
        .iter()
        .map(|part| part.trim().parse::<u8>().unwrap())
        .collect();

    // Convert Vec<u8> back into a String
    let string = String::from_utf8(bytes).unwrap();

    println!("{}", string);
}
