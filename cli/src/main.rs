#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Result;
use std::time::Instant;

use ansi_term::Colour;
use rayon::prelude::*;

#[cfg(all(not(target_env = "musl"), not(debug_assertions)))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Debug, Deserialize, Clone)]
struct PackageJson {
  name: String,
  version: String,
  main: Option<String>,
  module: Option<String>,
  #[serde(rename(deserialize = "type"))]
  module_type: Option<String>,
}

fn main() -> Result<()> {
  let now = Instant::now();
  let _package_map: HashMap<String, PackageJson> = walkdir::WalkDir::new("node_modules")
    .max_depth(3)
    .into_iter()
    .collect::<Vec<walkdir::Result<walkdir::DirEntry>>>()
    .into_par_iter()
    .filter(|p| {
      p.as_ref()
        .map(|d| d.file_name() == OsStr::new("package.json"))
        .unwrap_or(false)
    })
    .filter_map(|d| d.ok())
    .filter_map(|entry| {
      let path = entry.path();
      if let Ok(f) = File::open(path) {
        if let Ok(package_json) = serde_json::from_reader::<File, PackageJson>(f) {
          if package_json.main.is_none() && package_json.module_type == Some("module".to_owned()) {
            println!(
              "[{}] is esm only",
              Colour::Red.bold().paint(package_json.name.clone())
            );
            Some((package_json.name.clone(), package_json))
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      }
    })
    .collect();
  let elapsed = now.elapsed();
  println!("{} ms", elapsed.as_nanos() / 1000000);
  Ok(())
}
