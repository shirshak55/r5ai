use crate::archive::Archive;
use crate::config::Config;
use crate::context::Context;
use crate::error::Errors::{self, InvalidAction, InvalidPath};
use std::collections::HashMap;
use tracing::event;
use tracing::Level;

#[derive(Debug)]
pub struct Action {
    pub name: String,
    pub context: Context,
}

impl Action {
    const ALLOWED_ACTIONS: [&'static str; 4] = ["download", "get", "login", "logout"];

    pub fn new(context: Context) -> Result<Self, Errors> {
        event!(Level::INFO, ?context);

        let action_name = context.request.post_body.get_action_name()?;
        let is_allowed = Self::ALLOWED_ACTIONS.contains(&action_name);

        if !is_allowed {
            return Err(InvalidAction);
        }

        Ok(Self {
            name: action_name.to_owned(),
            context,
        })
    }

    pub async fn download(&self) -> Result<impl warp::Reply, Errors> {
        use warp::http::header::{CONNECTION, CONTENT_DISPOSITION, CONTENT_TYPE};

        let post_body = &self.context.request.post_body;

        let aas = post_body.get_string("as")?;
        let ttype = post_body.get_string("type")?;
        let base_href = post_body.get_string("baseHref")?;
        let hrefs = post_body.get_string("href")?;

        let archive = Archive::new(&self.context);
        let output = archive.output();

        let response = warp::http::Response::builder()
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(
                CONTENT_DISPOSITION,
                format!(r#"attachment; filename="{}""#, aas),
            )
            .header(CONNECTION, "close")
            .body("HELLO")
            .unwrap();

        Ok(response)
    }

    // pub async fn get(&self) -> Result<impl warp::Reply, Errors> {
    //     todo!()
    // }

    // pub async fn login(&self) -> Result<impl warp::Reply, Errors> {
    //     todo!()
    // }

    // pub async fn logout(&self) -> Result<impl warp::Reply, Errors> {
    //     todo!()
    // }
}
