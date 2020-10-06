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

    pub fn output(&self) -> String {
        // Todo Fix it . It scan be security issue
        let cmd = format!(
            "cd {root_dir} && tar --no-recursion -c -- {dirs} {files}",
            root_dir = self.base_path,
            dirs = "",
            files = ""
        );

        todo!()
    }

    fn add_file(self: &mut Self, file: OsString) {
        self.files.push(file);
    }

    fn add_dir(self: &mut Self, dirs: Vec<OsString>) -> Result<(), Errors> {
        for dir in dirs {
            // Todo may be skip instead of retuning error?
            let normalized_path = self
                .context
                .convert_to_path(&dir.to_str().ok_or(InvalidPath)?);
        }

        Ok(())
    }

    fn add_hrefs(&self, hrefs: Vec<String>) {
        use std::path::Path;

        // for href in hrefs {
        //     let normalized_path = href;
        //     // Todo Normalize paths
        //     let rfile_name = Path::new(normalized_path.into()).file_name();

        //     // if let Ok(filename) = rfile_name {

        //     //     if(self.is_managed_path(normalized_path)){
        //     //         let path = self.convert_to_path(normalized_path);
        //     //         let archived_file = ;

        //     //         let meta -
        //     //     };

        //     // }
        // }

        todo!()
    }
}
