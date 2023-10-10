use std::{io::BufReader, fs::File};

use xml::{EventReader, reader::XmlEvent};

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
                    println!("------ Level {} - target_path {} - name {}", target_index, target_path[target_index],  name.local_name );
                    target_index += 1;
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if target_index > 0 && name.local_name == target_path[target_index - 1] {
                    println!("------ Exit Level {} - target_path {} - name {}", target_index, target_path[target_index], name.local_name );
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


mod tests {
    use std::{fs::File, io::BufReader, path::Path};

    use xml::{reader::XmlEvent, EventReader};

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
}