use std::path::Path;

pub fn check_is_root(path: &String) -> bool {
    // Component::RootDir.as_os_str().to_str().unwrap()

    let path = Path::new(path.as_str());
    match path.parent() {
        Some(_) => false,
        None => true,
    }
}