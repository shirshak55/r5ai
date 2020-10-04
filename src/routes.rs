//use crate::config;
use crate::controllers;
use warp::Filter;

pub fn get_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    // let cfg = config::get_config();
    // let fs = warp::fs::dir(cfg.html_path.to_owned());

    handle_action()
        .recover(controllers::handle_rejection)
        .with(warp::trace::named("get_routes"))
}

pub fn handle_action() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::filters::query::query())
        .and_then(controllers::index)
        .with(warp::trace::named("handle_action"))
}
