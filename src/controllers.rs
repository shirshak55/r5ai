use crate::config::Config;
use serde_json::json;
use serde_json::Value;
use serde_json::Value::Null;
use std::collections::HashMap;
use tracing::event;
use tracing::Level;
use warp::reject;

#[derive(Debug)]
struct Action {
    pub name: String,
    pub queries: HashMap<String, Value>,
    pub config: &'static Config,
}

impl Action {
    const ALLOWED_ACTIONS: [&'static str; 4] = ["download", "get", "login", "logout"];

    pub fn new(
        queries: HashMap<String, serde_json::Value>,
        config: &'static Config,
    ) -> Result<Self, warp::Rejection> {
        let action = queries.get("action");
        let action_name = match action {
            Some(serde_json::Value::String(ss)) => ss,
            _ => return Err(reject::custom(InvalidAction)),
        };

        let is_allowed = Self::ALLOWED_ACTIONS.contains(&action_name.as_str());

        if !is_allowed {
            return Err(reject::custom(InvalidAction));
        }

        event!(Level::INFO, ?queries);

        Ok(Self {
            name: action_name.to_owned(),
            queries,
            config,
        })
    }

    pub async fn get(&self) -> serde_json::Value {
        json!({"action": self.name})
    }

    pub async fn download(&self) -> serde_json::Value {
        let aas = self.queries.get("as");
        let ttype = self.queries.get("type");
        let base_href = self.queries.get("baseHref");
        let hrefs = self.queries.get("href").unwrap_or("");

        json!({"action": self.name})
    }

    pub async fn login(&self) -> serde_json::Value {
        json!({})
    }

    pub async fn logout(&self) -> serde_json::Value {
        json!({})
    }
}
#[derive(Debug)]
struct InvalidAction;

impl warp::reject::Reject for InvalidAction {}

pub async fn index(
    queries: HashMap<String, serde_json::Value>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let config = crate::config::get_config();
    let action = Action::new(queries, config)?;

    match action.name.as_str() {
        "download" => action.download().await,
        "get" => action.get().await,
        "login" => action.login().await,
        "logout" => action.logout().await,
        _ => unreachable!(),
    };
}

// Central Error Handling
pub async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::reply::Reply, std::convert::Infallible> {
    use warp::http::StatusCode;

    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found. 404 Error";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Server Error 500";
    }

    // TODO Remove dbg!
    dbg!(err);

    let json = warp::reply::json(&json! ({
        "code": code.as_u16(),
        "message": message,
    }));

    Ok(warp::reply::with_status(json, code))
}
