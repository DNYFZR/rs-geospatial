// General utilities
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use rusqlite::Connection;

#[derive(Debug)]
pub struct SqliteMaster {
    db_type:String,
    db_name:String,
    tbl_name:String,
    root_page:i64,
    sql:String,
}

pub fn query_schema(database: &str, table:&str) -> Vec<SqliteMaster> {
  //  Connect to the DB
  let conn = Connection::open(database)
      .expect("failed to connect to DB");

  // Execute query
  let sql = format!("SELECT * FROM sqlite_master WHERE type='table' AND name='{}' ORDER BY name;", &table);
  let mut engine = conn.prepare(&sql).expect("SQL prep error");
  let mut rows = engine.query([]).expect("Row query failed");
  let mut data = vec![];

  while let Some(row) = rows.next().expect("while error") {
      let tmp_n= SqliteMaster {
          db_type: row.get(0).expect("failed to get row"),
          db_name: row.get(1).expect("failed to get row"),
          tbl_name: row.get(2).expect("failed to get row"),
          root_page: row.get(3).expect("failed to get row"),
          sql: row.get(4).expect("failed to get row"),
      };
      
      data.push(tmp_n);
  }
  return data;
     
}

pub fn path_to_str(path:&Path) -> &str {
  match path.to_str() {
    Some(val) => val,
    None => ""    
  }
}

pub fn get_filename(path:&Path) -> &str {
  if path.is_file() {
      return path
          .to_str()
          .expect("failed to convert to str")
          .split("/")
          .last()
          .expect("Error extracting filename from path"); 
  } else {
    return "not-a-file";
  }
}

pub fn list_dir(path:&Path) -> Vec<String> {
if path.is_dir() {
  return path.read_dir()
  .expect("failed to read tmp dir")
  .map(|file| {
      match file {
          Ok(file) => {
            match file.file_name().clone().to_str() {
                Some(file_str) => String::from(file_str),
                None => String::new(),
            }
          },
          Err(_) => String::new(),
      }
  }).collect::<Vec<String>>();
} else {
  return vec![String::from("not-a-directory"), ];
}
}

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

// Testing

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
