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
  use tempfile::TempDir;

  let src = "data/SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip";
  
  let binding = TempDir::new().expect("Upstream error : tempfile crate failed to create test dir");
  let dest = binding.path().to_str().expect("failed to extract tmp test path");
  let dest_file = format!("{}/SEPA_BATHING_WATER_POLYGONS_BNG.gpkg", &dest);
  
  // remove extract from data dir if exists already
  match fs::exists(&dest_file) {
    Ok(true) => fs::remove_file(&dest_file).expect("Error removing test file"),
    Ok(false) => (),
    Err(_) => (),
  }
  // run extract on bw zip
  unzip(src, dest);

  // confirm file now exists
  let result = fs::exists(dest_file).expect("Error checking if filepath exists");
  assert_eq!(true, result);
}