use std::env;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use serde::Serialize;
use plotters::prelude::*;
use ping::ping;

#[derive(Debug, Serialize)]
struct PingPlot {
    timestamps: Vec<u64>,
    round_trip_times: Vec<u64>,
}

async fn plot_ping_website(website: String) -> Result<impl Reply, Rejection> {
    // Perform ping to the website
    let mut timestamps = Vec::new();
    let mut round_trip_times = Vec::new();
    for _ in 0..10 {
        if let Ok(result) = ping(&website, 5) {
            timestamps.push(result.timestamp);
            round_trip_times.push(result.average_time as u64);
        }
    }

    // Create the plot
    let root = BitMapBackend::new("ping_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_max = *timestamps.iter().max().unwrap();
    let y_max = *round_trip_times.iter().max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Ping Plot", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(0..x_max, 0..y_max)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(LineSeries::new(
        timestamps.iter().zip(round_trip_times.iter()).map(|(&x, &y)| (x, y)),
        &RED,
    ))?;

    // Save the plot to a file
    chart.present()?;
    
    // Create the response
    let response = PingPlot {
        timestamps,
        round_trip_times,
    };

    Ok(warp::reply::json(&response))
}

#[tokio::main]
async fn main() {
    let plot_ping_route = warp::get()
        .and(warp::path("ping").and(warp::path::param()))
        .and_then(plot_ping_website);

    let routes = plot_ping_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
