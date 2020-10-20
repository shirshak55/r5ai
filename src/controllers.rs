mod action;

use serde_json::json;
use std::collections::HashMap;
use tracing::error;
use tracing::info_span;

pub async fn index(
    query_string: HashMap<String, serde_json::Value>,
    post_body: HashMap<String, serde_json::Value>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _span = info_span!("Index Route");

    let config = crate::config::get_config();
    let request = crate::request::Request::new(query_string.into(), post_body.into());

    let context = crate::context::Context::new(config, request);

    let action = action::Action::new(context)?;

    let resp = match action.name.as_str() {
        "download" => action.download().await?,
        // "get" => action.get().await?,
        // "login" => action.login().await?,
        // "logout" => action.logout().await?,
        _ => unreachable!(),
    };

    Ok(resp)
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

    error!(?err);

    let json = warp::reply::json(&json! ({
        "code": code.as_u16(),
        "message": message,
    }));

    Ok(warp::reply::with_status(json, code))
}
