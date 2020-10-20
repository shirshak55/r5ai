/// All the error on this application binary . It must include every type of error so we can show error
/// properly to frontend

/// Error are handled in controller.rs handle_rejection function
#[derive(Debug)]
pub enum Errors {
    // Action in query is invalid.
    InvalidAction,

    // Todo use Invalid Query and Invalid post body instead of Invalid Data
    // // Error on parsing string from Query
    // InvalidQueryString,

    // // Error on parsing string from Query
    // InvalidPostBody,

    // When invalid filesystem path is supplied
    InvalidPath,

    // when invalid data is passed. Invalid Post Body or Invalid Query
    InvalidData,
}

impl warp::reject::Reject for Errors {}

impl From<Errors> for warp::Rejection {
    fn from(err: Errors) -> Self {
        warp::reject::custom(err)
    }
}
