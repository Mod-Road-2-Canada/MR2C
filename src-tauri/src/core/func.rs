use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::errorwrap::Error;

pub const INSTALL: u8 = 1; // Bitwise value
pub const REMOVE: u8 = 2;

pub enum SavePosition {
    BELOW,
    ABOVE,
    BOTTOM,
}

fn newline_after_substr(string: &str, substring: &str) -> bool {
    if let Some(substring_index) = string.find(substring) {
        let substring_end = substring_index + substring.len();
        // println!("---Checking ::{} ::{}", &string, &substring);
        if let Some(_) = string[substring_end..].find('\n') {
            return true;
        }
    }

    false
}

pub fn save(
    file_name: &str,
    str_add: &str,
    str_search: &str,
    str_position: SavePosition,
    mod_tag: &str,
    mod_install_state: u8,
) -> Result<(), Error> {
    // println!("{}\n{}\n{}\n{}\n{}\n", file_name, str_add, str_search, str_position, mod_tag); // Debug
    let file_temp = format!("{}.temp", file_name);
    let mod_tag_newline = format!("\n(* {} nl *) ", mod_tag);
    let mod_tag_noeffect = format!("(* {} *) ", mod_tag);

    {
        // file only exist within scope
        let file = File::open(file_name)?;
        let out_file = File::create(&file_temp)?;

        let mut reader = BufReader::new(file);
        let mut writer = BufWriter::new(out_file);
        let mut contents = String::new();

        if mod_install_state == REMOVE {
            contents = reader
                .lines()
                .map(|line| line.unwrap())
                .filter(|line| !line.contains(&mod_tag_noeffect))
                .collect::<Vec<_>>()
                .join("\n");

            contents = contents.replace(&mod_tag_newline, "");
            // Remove the inserted newline to separate from the mod (if inserted)
        } else if mod_install_state == INSTALL {
            reader.read_to_string(&mut contents)?;
            let mut str_add = concat_string!(mod_tag_noeffect, str_add);
            str_add = str_add.replace("\n", &concat_string!("\n", mod_tag_noeffect)); // Add tag for each additional multilines

            match str_position {
                SavePosition::BOTTOM => {
                    contents = concat_string!(contents, "\n", str_add);
                }

                SavePosition::ABOVE | SavePosition::BELOW => {
                    let str_replace: String;

                    str_replace = match str_position {
                        SavePosition::ABOVE =>
                        // Case adding above
                        {
                            concat_string!(str_add, "\n", str_search)
                        }

                        SavePosition::BELOW => {
                            if newline_after_substr(&contents, &str_search) {
                                // Case when want to insert inbetween existing string
                                concat_string!(str_search, "\n", str_add, mod_tag_newline)
                            } else {
                                concat_string!(str_search, "\n", str_add)
                            }
                        }

                        _ => custombail!("SAVE: THIS SAVE POSITION WAS NOT SUPPOSED TO HAPPEN"), // ERROR
                    };

                    // Warning if search term is not found
                    match contents.matches(str_search).count() {
                        0 => custombail!("Search term not found: \"{}\"", str_search),

                        _ => contents = contents.replace(str_search, &str_replace),
                        // TODO: Design: Should there be a warning for more than once?
                    }
                }
            }
            // println!("Replace: {}", str_search);
            // println!("With: {}", str_replace);
        }

        writer.write_all(contents.as_bytes())?;
    }

    std::fs::rename(&file_temp, file_name)?;

    println!("Saved.");
    Ok(())
}
