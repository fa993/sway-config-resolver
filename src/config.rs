use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::error::{path_to_error_string, SwayIOError};

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    active: bool,
    font: String,
    seen_configs: HashSet<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            active: true,
            font: "ASCII".to_string(),
            seen_configs: HashSet::new(),
        }
    }
}

impl Config {
    pub fn new(active: bool, font: String, seen_configs: HashSet<PathBuf>) -> Config {
        Config {
            active,
            font,
            seen_configs,
        }
    }

    fn push_config(&mut self, path: &Path) -> bool {
        self.seen_configs.insert(path.to_path_buf())
    }

    pub fn read_config(&mut self, path: &Path) -> Result<(), SwayIOError> {
        let r = path.canonicalize().map_err(|_| SwayIOError::PathNotFound {
            path: path_to_error_string(&path),
        })?;
        if !self.push_config(&r) {
            //Do nothing, not an error but
            return Ok(());
        }
        let f = File::open(path).map_err(|_| SwayIOError::FileOpenError {
            file_name: path_to_error_string(&path),
        })?;
        let mut yy = BufReader::new(f).lines();
        while let Some(Ok(t)) = yy.next() {
            let d = Directive::parse_directive(t.as_str())?;
            d.execute(self)?;
        }
        Ok(())
    }
}

pub enum Directive {
    Include(String),
    IncludeOne(Vec<String>),
    Font(String),
    Active(bool),
    NOP,
}

impl Directive {
    pub fn parse_directive(line: &str) -> Result<Directive, SwayIOError> {
        let trimed = line.trim();
        if trimed.starts_with('#') || trimed.is_empty() {
            Ok(Directive::NOP)
        } else if let Some(t) = trimed.strip_prefix("include_one ") {
            Ok(Directive::IncludeOne(
                t.to_string()
                    .split_whitespace()
                    .map(&str::to_string)
                    .collect(),
            ))
        } else if let Some(t) = trimed.strip_prefix("include ") {
            Ok(Directive::Include(t.to_string()))
        } else if let Some(t) = trimed.strip_prefix("font ") {
            Ok(Directive::Font(t.to_string()))
        } else {
            Err(SwayIOError::UnknownDirective {
                line: line.to_string(),
            })
        }
    }

    pub fn execute(self, conf: &mut Config) -> Result<(), SwayIOError> {
        match self {
            Self::Include(f_pattern) => {
                //first expand pattern to match file
                let po = shellexpand::full(f_pattern.as_str()).map_err(|_| {
                    SwayIOError::PathNotFound {
                        path: f_pattern.clone(),
                    }
                })?;
                for t in glob::glob(&po).map_err(|_| SwayIOError::PathNotFound {
                    path: f_pattern.clone(),
                })? {
                    let p = t.map_err(|e| SwayIOError::path_not_found(&e.path()))?;
                    if p.file_name().is_some() {
                        conf.read_config(p.as_path())?;
                    }
                }
            }
            Self::IncludeOne(dirs) => {
                for dir in dirs {
                    let po = shellexpand::full(dir.as_str())
                        .map_err(|_| SwayIOError::PathNotFound { path: dir.clone() })?;
                    let mut seen = HashSet::new();
                    for t in glob::glob(&po).map_err(|f| SwayIOError::PathNotFound {
                        path: f.msg.to_string(),
                    })? {
                        let p = t.map_err(|e| SwayIOError::PathNotFound {
                            path: path_to_error_string(&e.path()),
                        })?;
                        if p.file_name().is_some() && seen.insert(FileName { inner: p.clone() }) {
                            conf.read_config(p.as_path())?;
                        }
                    }
                }
            }
            Self::Font(t) => {
                conf.font = t;
            }
            Self::Active(t) => {
                conf.active = t;
            }
            Self::NOP => {
                //Do nothing
            }
        };
        Ok(())
    }
}

#[derive(Eq)]
struct FileName<T: AsRef<Path>> {
    inner: T,
}

impl<T: AsRef<Path>> PartialEq for FileName<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner
            .as_ref()
            .file_name()
            .eq(&other.inner.as_ref().file_name())
    }
}

impl<T: AsRef<Path>> Hash for FileName<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.as_ref().file_name().hash(state)
    }
}
