use std::{io::{BufReader, BufWriter}, fs::{File, self}};

use regex::Regex;
use xml::{EventReader, reader::XmlEvent, EventWriter};

pub fn find_element_value<'a>(
    reader: BufReader<File>,
    target_path: &[&str],
) -> String {
    let mut target_index = 0;
    let mut include_path_value = String::new();
    let parser = EventReader::new(reader);
    
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if target_index < target_path.len() && name.local_name == target_path[target_index] {
                    // println!("------ Level {} - target_path {} - name {}", target_index, target_path[target_index],  name.local_name );
                    target_index += 1;
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if target_index > 0 && name.local_name == target_path[target_index - 1] {
                    // println!("------ Exit Level {} - target_path {} - name {}", target_index, target_path[target_index], name.local_name );
                    target_index -= 1;
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                // println!("------ Level {} - {}", target_index, text);
                if target_index == target_path.len() {
                    include_path_value = text;
                    break;
                }
            }
            _ => {}
        }
    }

    include_path_value
}

// TEMP: Use file reader and writer to write data
pub fn temp_update_element_value<'a>(
    file_path: &str,
    update_file_path: &str,
    old_value: &str,
    new_value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = fs::read_to_string(file_path).expect("Failed to read file");
    
    // replace
    let pattern = Regex::new(old_value).expect("Failed to create regex");
    content = pattern.replace(&content, new_value).to_string();

    fs::write(update_file_path, content).expect("Failed to write file");

    Ok(())
}

pub fn update_element_value<'a>(
    reader: BufReader<File>,
    writer: BufWriter<File>,
    target_path: &[&str],
    new_value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut target_index = 0;
    let mut include_path_value = String::new();
    let mut writer = EventWriter::new(writer);
    let parser = EventReader::new(reader);

    for event in parser {
        match event {
            Ok(xml::reader::XmlEvent::EndDocument) => break,
            Ok(XmlEvent::StartElement { name, mut attributes, namespace }) => {
                if target_index < target_path.len() && name.local_name == target_path[target_index] {
                    // println!("------ Level {} - target_path {} - name {}", target_index, target_path[target_index],  name.local_name );
                    target_index += 1;
                }

                let event = xml::writer::XmlEvent::StartElement  {
                    name: name.borrow(),
                    namespace: namespace.borrow(),
                    attributes: attributes.iter_mut().map(|attr| {
                        attr.value = alternating_caps(&attr.value);
                        attr.borrow()
                    }).collect(),
                };
                writer.write(event);
            }
            Ok(XmlEvent::EndElement { name }) => {
                if target_index > 0 && name.local_name == target_path[target_index - 1] {
                    // println!("------ Exit Level {} - target_path {} - name {}", target_index, target_path[target_index], name.local_name );
                    target_index -= 1;
                }

                let event = xml::writer::XmlEvent::EndElement {
                    name: Some(name.borrow()),
                };
                writer.write(event);
            }
            Ok(XmlEvent::Characters(text)) => {
                // println!("------ Level {} - {}", target_index, text);
                if target_index == target_path.len() {
                    include_path_value = text.clone();
                }

                let text = alternating_caps(&text);
                let event = xml::writer::XmlEvent::Characters(&text);
                writer.write(event);
            }
            Ok(xml::reader::XmlEvent::Comment(text)) => {
                let text = alternating_caps(&text);
                let event = xml::writer::XmlEvent::Comment(&text);
                writer.write(event);
            },
            _ => {}
        }
    }

    Ok(())
}

fn alternating_caps(text: &str) -> String {
    text.chars().enumerate()
        .map(|(i, ch)| if i&1==0 { ch.to_ascii_uppercase() } else { ch.to_ascii_lowercase() })
        .collect()
}

mod tests {
    use std::{fs::{File, read_to_string, self}, io::{BufReader, BufWriter}, path::Path};

    use regex::Regex;
    use xml::{reader::XmlEvent, EventReader};

    use crate::utils::xml_helper::update_element_value;

    use super::find_element_value;
    #[test]
    fn test_get_include_path() {
        let mdk_filepath = Path::new("/Users/asklv/TOS_Test/generated/board/Tencentos-tiny/MDK-ARM/Tencentos-tiny.uvprojx");
        let file = File::open(mdk_filepath).expect("Failed to open file");
        let reader: BufReader<File> = BufReader::new(file);

        let target_path = vec![
            "Targets",
            "Target",
            "TargetOption",
            "TargetArmAds",
            "Cads",
            "VariousControls",
            "IncludePath",
        ];

        let include_path_value = find_element_value(reader, &target_path);
        println!("IncludePath: {}", include_path_value);
    }

    #[test]
    fn test_write_include_path() {
        let mdk_filepath = Path::new("/Users/asklv/TOS_Test/generated/board/Tencentos-tiny/MDK-ARM/Tencentos-tiny.uvprojx.temp");
        let mdk_filepath_writer = Path::new("/Users/asklv/TOS_Test/generated/board/Tencentos-tiny/MDK-ARM/Tencentos-tiny.uvprojx.temp");
        let file = File::open(mdk_filepath).expect("Failed to open file");
        let file_writer = File::open(mdk_filepath_writer).expect("Failed to open file");
        let reader: BufReader<File> = BufReader::new(file);
        let writer: BufWriter<File> = BufWriter::new(file_writer);

        let target_path = vec![
            "Targets",
            "Target",
            "TargetOption",
            "TargetArmAds",
            "Cads",
            "VariousControls",
            "IncludePath",
        ];

        let _ = update_element_value(reader, writer, &target_path, "");
    }

    #[test]
    fn test_temp_write_include_path() {
        let file_path = "/Users/asklv/TOS_Test/generated/board/Tencentos-tiny/MDK-ARM/Tencentos-tiny.uvprojx.xml";
        let mut content = read_to_string(file_path).expect("Failed to read file");
        
        // replace
        let pattern = Regex::new(r"uVision Project").expect("Failed to create regex");
        content = pattern.replace(&content, "New Line 2").to_string();
    
        fs::write(file_path, content).expect("Failed to write file");
    }
}