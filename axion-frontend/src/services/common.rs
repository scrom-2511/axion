use std::path::PathBuf;

use dirs::home_dir;

use crate::services::errors_frontend::AxionFrontendError;

pub struct CommonService;

impl CommonService {
    pub fn get_home_dir_path_with_file(
        file_name_with_ext: &str,
    ) -> Result<PathBuf, AxionFrontendError> {
        match dirs::home_dir() {
            Some(home_dir) => return Ok(home_dir.join(file_name_with_ext)),
            None => return Err(AxionFrontendError::HomeDirNotFound),
        };
    }
}
