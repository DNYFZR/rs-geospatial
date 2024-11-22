// Extract Zip Archives
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub fn unzip(src:&str, dest:&str) {
  let src_path = Path::new(src);
  let file = fs::File::open(src_path).expect("Failed to open zipfile");
  let mut archive = ZipArchive::new(file).expect("Failed to access zipfile");

  for i in 0..archive.len() {
    let mut arch_file = archive.by_index(i).expect(&format!("Failed to access zipfile internal item at index {}", i)); 
    
    let path_out = match arch_file.enclosed_name() {
        Some(path) => path,
        None => continue,
    };
    
    if arch_file.is_dir() {
      fs::create_dir_all(&path_out).expect(&format!("Failed to create new directory at index {}", i));
    }

    else {
      if let Some(p) = path_out.parent() {
        if !p.exists() {
          fs::create_dir_all(p).expect("Failed to create new subdir");
        }
      }
      let full_path = PathBuf::from(String::from(format!("{}/{}", dest, path_out.to_str().expect("Failed to convert to str"))));
      let mut file_out = fs::File::create(&full_path).expect("Failed to create new file");
    
      io::copy(&mut arch_file, &mut file_out).expect("Failed to write to new file");
    }

  }
}

#[test]
fn test_unzip() {
  // remove bw extract from data dir

  // run extract on bw zip

  // confirm file now exists (again)
  assert_eq!("", "");
}