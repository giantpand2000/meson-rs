use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct Config {
    pub options: Option<HashMap<String, String>>,
    pub native_file: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            options: None,
            native_file: None,
        }
    }

    pub fn options<'a>(
        &mut self,
        options: HashMap<impl AsRef<&'a str>, impl AsRef<&'a str>>,
    ) -> Self {
        let options = options
            .into_iter()
            .map(|(key, value)| (String::from(*key.as_ref()), String::from(*value.as_ref())))
            .collect::<HashMap<String, String>>();

        self.options = Some(options);
        self.clone()
    }

    pub fn native_file(&mut self, native_file: impl AsRef<Path>) -> Self {
        let native_file: PathBuf = native_file.as_ref().into();
        self.native_file = Some(native_file);
        self.clone()
    }
}
