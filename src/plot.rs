use crate::model::{Borough, BoroughGeometry, BoroughNetwork, Pub};
use geo::BoundingRect;
use plotters::prelude::*;

pub fn plot_borough_data(
    borough: &Borough,
    filtered_pubs: &[Pub],
    borough_network: &BoroughNetwork,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area
    let root = BitMapBackend::new(output_path, (2400, 2400)).into_drawing_area();
    root.fill(&WHITE)?;

    // Find the bounds of the borough for plotting
    let bounds = match &borough.geometry {
        BoroughGeometry::Single(polygon) => polygon.bounding_rect().unwrap(),
        BoroughGeometry::Multi(multi_polygon) => multi_polygon.bounding_rect().unwrap(),
    };

    // Create the coordinate spec with some padding
    let padding = (bounds.max().x - bounds.min().x) * 0.05;
    // Create chart
    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(
            bounds.min().x - padding..bounds.max().x + padding,
            bounds.min().y - padding..bounds.max().y + padding,
        )?;

    // Draw the road network
    for link in &borough_network.links {
        chart.draw_series(LineSeries::new(
            link.geometry.points().map(|p| (p.x(), p.y())),
            RED,
        ))?;
    }

    // Draw the pub points
    chart.draw_series(filtered_pubs.iter().map(|pub_data| {
        let pt = pub_data.geometry.expect("Geometry must be available");
        Circle::new((pt.x(), pt.y()), 10, BLACK.filled())
    }))?;

    // Draw the pub names to the right of each point
    chart.draw_series(filtered_pubs.iter().map(|pub_data| {
        let pt = pub_data.geometry.expect("Geometry must be available");
        Text::new(
            pub_data.name.clone(),
            (pt.x() + 5.0, pt.y()),
            ("sans-serif", 10).into_font().color(&BLACK),
        )
    }))?;
    chart.draw_series(borough_network.nodes.iter().map(|node_data| {
        Circle::new(
            (node_data.geometry.x(), node_data.geometry.y()),
            5,
            BLUE.filled(),
        )
    }))?;

    // Draw the borough boundary
    match &borough.geometry {
        BoroughGeometry::Single(polygon) => {
            chart.draw_series(LineSeries::new(
                polygon.exterior().points().map(|p| (p.x(), p.y())),
                &BLUE,
            ))?;
        }
        BoroughGeometry::Multi(multi_polygon) => {
            for polygon in multi_polygon.iter() {
                chart.draw_series(LineSeries::new(
                    polygon.exterior().points().map(|p| (p.x(), p.y())),
                    &BLUE,
                ))?;
            }
        }
    }

    root.present()?;
    Ok(())
}
