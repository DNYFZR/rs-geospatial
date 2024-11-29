// General utilities
use std::path::Path;
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

