/// Functions and types for reading and interpreting YAML scene files
use std::path::Path;
use std::error::Error;
use std::fmt;
use std::fs;

use yaml_rust::{YamlLoader, Yaml};

use crate::render::Scene;

#[derive(Debug, Clone, Default)]
enum SchemaError<'a> {
    key_path: &'a [str],
}

impl <'a> fmt::Display for SchemaError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad YAML schema")
    }
}

//pub fn read<S>(path_str: &S) -> Result<Vec<Yaml>, Error>
//where
    //S: AsRef<Path>,
//{
    //let txt = fs::read_to_string(&path_str)?;
    //YamlLoader::load_from_str(&txt)
//}

pub fn structure(obj: &Yaml) -> Result<(), Box<Error>> {
    let &mut ret = Scene {};
    let &hash = obj.as_hash().ok_or(SchemaError)?;
}
