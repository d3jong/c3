use std::fs::{File, remove_file, remove_dir};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{prelude::*, self};
use std::path::PathBuf;
use home::home_dir;

#[inline(always)]
pub fn append_home_dir(vec:[&str; 4]) -> PathBuf {
    let mut path = PathBuf::from(format!("{}", home_dir().unwrap().to_str().unwrap()));
    for item in vec {
        path = path.join(item);
    }

    return path
    // PathBuf::from(format!("{}/{}", home_dir().unwrap().to_str().unwrap(), str))
}

#[inline(always)]
pub fn note_path(hash:&String, parent_dir: Option<PathBuf>) -> io::Result<Option<PathBuf>> {
    if hash.is_empty() {
        return Ok(None)
    }
    let parent_dir = match parent_dir {
        Some(value) => value,
        None => {
            let dir = append_home_dir([".local","share","calcurse","notes"]);
            if dir.is_file() {
                remove_file(dir.clone())?;
            }
            dir
        }
    };
    let _ = std::fs::create_dir_all(parent_dir.clone());
    Ok(Some(parent_dir.join(hash)))

}

#[inline(always)]
pub fn get_todo_path() -> io::Result<PathBuf> {
    let file = append_home_dir([".local","share","calcurse","todo"]);
    if file.is_dir() {
        remove_dir(file.clone())?;
    }
    let parentdir = file.parent().unwrap();
    let _ = std::fs::create_dir_all(parentdir)?;
    Ok(file)
}

#[inline(always)]
pub fn temp_note_path() -> PathBuf{
    let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Err(_)=>12345,
        Ok(some) => some.as_secs(),
    };
    let filename = format!("c3-note.{time}");
    let path = home_dir().unwrap().join(filename);
    path.to_path_buf()
}

#[inline(always)]
pub fn file_content(path:&PathBuf) -> io::Result<String> {
    let mut content = String::new();
    let mut file = File::open(path.as_os_str())?;
    file.read_to_string(&mut content)?;
    Ok(content)
}

