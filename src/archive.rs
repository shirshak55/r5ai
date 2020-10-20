use crate::context::Context;
use crate::error::Errors;
use crate::error::Errors::InvalidPath;
use std::ffi::OsString;

#[derive(Debug)]
pub struct Archive<'a> {
    pub context: &'a Context,

    pub base_path: &'a String,
    pub dirs: Vec<OsString>,
    pub files: Vec<OsString>,
}

impl<'a> Archive<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            base_path: &context.config.base_directory,
            dirs: Vec::new(),
            files: Vec::new(),
            context,
        }
    }

    pub fn output(&mut self, hrefs: &[&str]) -> Result<String, Errors> {
        self.add_hrefs(hrefs);

        // Todo better ignore bad dirs or files instead of failing/ returning error ?
        let files = self
            .files
            .iter()
            .map(|v| v.as_os_str().to_str().ok_or(InvalidPath))
            .collect::<Result<Vec<_>, Errors>>()?
            .join(" ");

        let dirs = self
            .dirs
            .iter()
            .map(|v| v.as_os_str().to_str().ok_or(InvalidPath))
            .collect::<Result<Vec<_>, Errors>>()?
            .join(" ");

        // Todo Fix it . It scan be security issue
        let cmd = format!(
            "cd {root_dir} && tar --no-recursion -c -- {dirs} {files}",
            root_dir = self.base_path,
            dirs = dirs,
            files = files
        );

        Ok(cmd)
    }

    fn add_hrefs(&mut self, hrefs: &[&str]) -> Result<(), Errors> {
        use std::path::Path;

        for href in hrefs {
            // TODO Normalize paths
            let meta = std::fs::metadata(Path::new(&self.context.convert_to_path(href)?));

            if let Ok(m) = meta {
                if m.is_dir() {
                    let dir = (&**href).to_owned();
                    self.files.push(dir.into());
                }

                if m.is_file() {
                    let file = (&**href).to_owned();
                    self.files.push(file.into())
                }
            }
        }

        Ok(())
    }

    // fn add_file(self: &mut Self, file: OsString) {
    //     self.files.push(file);
    // }

    // fn add_dir(self: &mut Self, dirs: Vec<OsString>) -> Result<(), Errors> {
    //     for dir in dirs {
    //         // Todo may be skip instead of retuning error?
    //         let normalized_path = self
    //             .context
    //             .convert_to_path(&dir.to_str().ok_or(InvalidPath)?);
    //     }

    //     Ok(())
    // }
}
