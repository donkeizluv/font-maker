use code_emplate_picker::{
    IMPORTS, MAX_RANGE_TEMP, PICKER_TEMP, STYLE_ENUM_TEMP, TRY_FROM_IMPL_TEMP,
};
use code_template::{ARR_TEMP, HEIGHT_TEMP, PAT};
use convert_case::{Case, Casing};
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

        let codegen = set
            .iter()
            .map(|c| {
                if c.is_empty() {
                    return format!("\n\"\",");
                }
                return format!("\nr#\"\n{}\"#,", c);
            })
            .collect::<Vec<String>>();
        let convention_name = name_conv(font);

        let merged = [
            HEIGHT_TEMP.replace(PAT, &max_height_idx.to_string()),
            ARR_TEMP.to_string().replace(PAT, &codegen.concat()),
        ]
        .concat();

        let path = format!("output/font_{}.rs", convention_name);

        let f = File::create(path).unwrap();
        let mut f = BufWriter::new(f);
        f.write_all(merged.as_bytes()).unwrap();
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

    let style_enums = all_fonts_conv
        .iter()
        .map(|s| format!("Font {}", s.to_case(Case::UpperCamel)))
        .map(|s| s.replace(" ", ""))
        .collect::<Vec<String>>();

    let last_enum = style_enums.last().unwrap();

    let try_from_cases = style_enums
        .iter()
        .map(|s| format!("x if x == Style::{} as u32 => Ok(Style::{})", s, s))
        .collect::<Vec<String>>()
        .join(",\n")
        + ",\n";

    let picker_match = style_enums
        .iter()
        .zip(all_fonts_conv)
        .map(|(e, c)| {
            let filename = format!("font_{}", c);
            format!(
                "Style::{} => Ok(({}::HEIGHT, {}::SET))",
                e, filename, filename
            )
        })
        .collect::<Vec<String>>()
        .join(",\n");

    let picker_content = [
        IMPORTS.to_string(),
        STYLE_ENUM_TEMP.replace(PAT, &style_enums.join(",\n")),
        MAX_RANGE_TEMP.replace(PAT, &last_enum),
        TRY_FROM_IMPL_TEMP.replace(PAT, &try_from_cases),
        PICKER_TEMP.replace(PAT, &picker_match),
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
