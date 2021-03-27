use std::path::PathBuf;

pub fn dir_exists(path: &PathBuf) -> bool {
    path.exists()
}

#[cfg(test)]
mod tests {
    use crate::dir_exists;
    use std::path::PathBuf;
    use tempdir::TempDir;

    fn create_temporary_directory() -> TempDir {
        TempDir::new("tmp_today_tests_").expect("Could not create temporary directory")
    }

    fn clean_up(temp_dir: TempDir) {
        let path_buf = temp_dir.path().to_owned();
        let error_message = format!("Could not remove temporary directory: {:?}", path_buf);
        temp_dir.close().expect(&error_message);
    }

    #[test]
    fn dir_exits_returns_false_when_no_dir() {
        assert_eq!(
            dir_exists(&PathBuf::from("some_non_existing_directory")),
            false
        );
    }

    #[test]
    fn dir_exists_returns_true_when_dir_exists() {
        let temp_dir = create_temporary_directory();
        assert_eq!(dir_exists(&temp_dir.path().to_owned()), true);
        clean_up(temp_dir);
    }
}
