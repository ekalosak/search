use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use ndarray::{Array, Array2, Axis};
use ndarray_stats::LinearRegression;

#[derive(Debug, Deserialize)]
struct FitRequest {
    x: Vec<Vec<f64>>,
    y: Vec<f64>,
}

#[derive(Debug, Serialize)]
struct FitResponse {
    slope: f64,
    intercept: f64,
}

async fn fit_linear_model(request: FitRequest) -> Result<impl Reply, Rejection> {
    // Convert input data to arrays
    let x = Array2::from_shape_vec((request.x.len(), request.x[0].len()), request.x.into_iter().flatten().collect())
        .map_err(|_| warp::reject::custom(ApiError::InvalidData))?;

    let y = Array::from_vec(request.y);

    // Perform linear regression
    let lr = LinearRegression::new(x.clone(), y.clone())
        .map_err(|_| warp::reject::custom(ApiError::RegressionError))?;

    let slope = lr.slope();
    let intercept = lr.intercept();

    // Create response
    let response = FitResponse { slope, intercept };

    Ok(warp::reply::json(&response))
}

#[derive(Debug)]
enum ApiError {
    InvalidData,
    RegressionError,
}

impl warp::reject::Reject for ApiError {}

#[tokio::main]
async fn main() {
    let fit_model_route = warp::post()
        .and(warp::path("fit"))
        .and(warp::body::json())
        .and_then(fit_linear_model);

    let routes = fit_model_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
