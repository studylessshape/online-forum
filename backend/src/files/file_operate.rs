use actix_multipart::Multipart;
use actix_web::web;
use futures_util::TryStreamExt;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{error::Error, fs, io::Write};

pub struct ResourceFilePath {
    pub path: String,
}

impl ResourceFilePath {
    const IMAGE_SUFFIX: [&str; 8] = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp", "svg"];
    const TEXT_SUFFIX: [&str; 2] = ["txt", "md"];
    const BINARY_SUFFIX: [&str; 2] = ["exe", "dat"];
    const COMPRESSED_SUFFIX: [&str; 4] = ["7z", "zip", "rar", "gz"];

    const IMAGE_PATH: &str = "./static/resources/image";
    const TEXT_PATH: &str = "./static/resources/text";
    const BINARY_PATH: &str = "./static/resources/binary";
    const COMPRESSED_PATH: &str = "./static/resources/compressed";
    const UNKNOW_PATH: &str = "./static/resources/unknow";

    fn dir_check() {
        let _ = fs::create_dir_all(Self::IMAGE_PATH);
        let _ = fs::create_dir_all(Self::TEXT_PATH);
        let _ = fs::create_dir_all(Self::BINARY_PATH);
        let _ = fs::create_dir_all(Self::COMPRESSED_PATH);
        let _ = fs::create_dir_all(Self::UNKNOW_PATH);
    }

    /// `str` is path or filename or suffix
    pub fn get_type(str: String) -> Self {
        Self::dir_check();

        let suffix = Self::get_suffix(str);

        let check_suffix = |iter_suffix: &str| iter_suffix == suffix;

        if Self::IMAGE_SUFFIX.map(check_suffix).len() > 0 {
            return Self {
                path: String::from(Self::IMAGE_PATH),
            };
        }

        if Self::TEXT_SUFFIX.map(check_suffix).len() > 0 {
            return Self {
                path: String::from(Self::TEXT_PATH),
            };
        }

        if Self::BINARY_SUFFIX.map(check_suffix).len() > 0 {
            return Self {
                path: String::from(Self::BINARY_PATH),
            };
        }

        if Self::COMPRESSED_SUFFIX.map(check_suffix).len() > 0 {
            return Self {
                path: String::from(Self::COMPRESSED_PATH),
            };
        }

        return Self {
            path: String::from(Self::UNKNOW_PATH),
        };
    }

    pub fn get_suffix(str: String) -> String {
        let dot_col: Vec<_> = str.match_indices('.').collect();
        if dot_col.len() > 0 {
            String::from(&str[dot_col[dot_col.len() - 1].0..str.len()])
        } else {
            str
        }
    }
}

pub fn gen_path(file_path: String) -> String {
    let suffix = ResourceFilePath::get_suffix(file_path.clone());
    let file_type = ResourceFilePath::get_type(file_path);
    let file_name: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    format!("{}/{}{}", file_type.path, file_name, suffix)
}

pub fn get_url_path(file_path: String) -> String {
    let mut path = file_path
        .split('/')
        .filter(|str| *str != "." && *str != "static" && *str != "")
        .collect::<Vec<&str>>()
        .join("/");
    path.insert(0, '/');
    path
}

/// If save success, back the path of file on server
pub async fn save_net_file(path: String, mut multipart: Multipart) -> Result<String, impl Error> {
    // get path on server
    let save_path = gen_path(path);
    // get path with server url
    let response_url = get_url_path(save_path.clone());
    // save data
    while let Some(mut field) = multipart.try_next().await? {
        let path = save_path.clone();
        let mut file = web::block(|| fs::File::create(path)).await??;

        while let Some(chunk) = field.try_next().await? {
            file = web::block(move || file.write_all(&chunk).map(|_| file)).await??
        }
    }

    Ok(response_url) as Result<String, actix_web::error::Error>
}
