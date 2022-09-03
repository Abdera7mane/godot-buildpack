use crate::GodotBuildpackMetadata;
use lazy_static::lazy_static;
use serde::Deserialize;
use regex::Regex;
use zip::result::ZipError;
use std::{fs, path::Path, io::{self, BufReader, Read}, os::unix::prelude::OpenOptionsExt};

lazy_static! {
    static ref VERSION_REGEX: Regex = Regex::new(r"^[1-4](\.\d+){1,2}$").unwrap();
    static ref TAG_REGEX: Regex = Regex::new(r"^(stable|(alpha|beta|rc)\d+)$").unwrap();
}

#[derive(Deserialize)]
pub(crate) struct GodotConfig {
    pub version: Option<String>,
    pub tag: Option<String>,
    pub mono: Option<bool>
}

impl GodotConfig {
    pub fn load(metadata: &GodotBuildpackMetadata) -> Self {
        let default = Self::from_metadata(metadata);

        match fs::read_to_string("config.godot") {
            Ok(source) => {
                let mut config: GodotConfig = toml::from_str(&source).unwrap();
                if config.version == None {
                    config.version = default.version;
                }
                if config.tag == None {
                    config.tag = default.tag
                }
                config
            },
            Err(_) => default
        }
    }

    fn from_metadata(metadata: &GodotBuildpackMetadata) -> Self {
        Self {
            version: Some(metadata.godot_version.clone()),
            tag: Some(metadata.godot_tag.clone()),
            mono: Some(metadata.godot_mono)
        }
    }

    pub fn is_valid(&self) -> bool {
            self.version.as_ref()
                .filter(|version| VERSION_REGEX.is_match(version))
                .and(self.tag.as_ref())
                .filter(|tag| TAG_REGEX.is_match(tag))
                .is_some()
    }

    pub fn print_error(&self) {
        let version = self.version.as_ref().unwrap();
        let tag = self.tag.as_ref().unwrap();
        
        if !VERSION_REGEX.is_match(&version) {
            eprint!("---> '{}' is an invalid Godot version. ", version);
            eprintln!("Valid options are (stable, alpha[n], beta[n] and rc[n])");
        }

        if !TAG_REGEX.is_match(&tag) {
            eprintln!("---> '{}' is an invalid Godot version tag", tag);
        }
    }
}

#[derive(Debug)]
pub(crate) enum DownloadError {
    RequestError(Box<ureq::Error>),
    FileCreateError(std::io::Error),
    WriteError(std::io::Error)
}

#[derive(Debug)]
pub(crate) enum UnzipError {
    FileOpenError(std::io::Error),
    FileCreateError(std::io::Error),
    StreamIOError(std::io::Error),
    ExtractError(ZipError)
}

pub(crate) fn unzip(source: &Path, destination: &Path, file_name: &str) -> Result<(), UnzipError> {
    let mut zip_file = fs::File::open(source)
        .map_err(UnzipError::FileOpenError)?;
    
    loop {
        match zip::read::read_zipfile_from_stream(&mut zip_file) {
            Ok(Some(mut file)) => {
                if file.is_dir() {
                    continue;
                };
                
                let path = match file.enclosed_name() {
                    Some(path) => path,
                    None => continue
                };
                
                match path.file_name() {
                    Some(name) => {
                        let name = name.to_str().unwrap();
                        if name.starts_with("Godot_") {
                            let dest_path = destination.join(file_name);
                            stream_write(&mut file, &dest_path)?;
                        }
                        else {

                            let sub_path = match path.to_str().unwrap().split_once('/') {
                                Some((_, sub_path)) => sub_path,
                                None => continue
                            };
                            
                            if sub_path.starts_with("GodotSharp") {
                                let dest_path = destination.join(sub_path);
                                stream_write(&mut file, &dest_path)?;
                            };

                        }
                    },
                    None => continue
                }
            }
            Ok(None) => break,
            Err(e) => {
                println!("Error encountered while reading zip: {:?}", e);
                return Err(UnzipError::ExtractError(e));
            }
        }
    }
        
    Ok(())
} 

fn stream_write(read: &mut impl Read, destination: &Path) -> Result<(), UnzipError> {
    destination.parent().and_then(
        |parent| fs::create_dir_all(parent).ok()
    );
    
    let mut dest_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o777)
        .open(destination)
        .map_err(UnzipError::FileCreateError)?;

    let mut reader = BufReader::new(read);
    io::copy(&mut reader, &mut dest_file)
        .map_err(UnzipError::StreamIOError)?;
    Ok(())
}

pub(crate) fn download(
    url: &str,
    destination: &Path,
) -> Result<(), DownloadError> {
    let mut response_reader = ureq::get(url)
        .call()
        .map_err(|err| DownloadError::RequestError(Box::new(err)))?
        .into_reader();

    let mut destination_file = fs::File::create(destination)
        .map_err(DownloadError::FileCreateError)?;

    io::copy(&mut response_reader, &mut destination_file)
        .map_err(DownloadError::WriteError)?;
    
    Ok(())
}

pub(crate) fn get_download_url(
    base: &str, version: &str, tag: &str, mono: bool, file_name: &str
) -> String {
    let mut url = format!("{}/{}", base, version);
    let mut file = format!("Godot_v{}-{}", version, tag);

    if tag != "stable" {
        url.push('/');
        url.push_str(tag);
    }
    if mono {
        url.push_str("/mono");
        file.push_str("_mono_");
        file.push_str(file_name.replace(".", "_").as_str());
    }
    else {
        file.push('_');
        file.push_str(file_name);
    }

    format!("{}/{}.zip", url, file)
}
