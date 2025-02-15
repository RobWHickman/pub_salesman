use clap::Parser;
use geo::prelude::Contains;
use geo_types::Point;
use proj::Proj;
use pub_salesman::boroughs::read_borough_file;
use pub_salesman::links::read_road_files;
use pub_salesman::model::{BoroughGeometry, BoroughNetwork, NetworkNode, NetworkNodeType, Pub};
use pub_salesman::plot::plot_borough_data;

#[derive(Parser)]
#[command(name = "Borough Finder")]
#[command(version = "1.0")]
#[command(about = "Finds a borough in London by name", long_about = None)]
struct Cli {
    borough_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let borough = read_borough_file(&args.borough_name)?;

    println!("Borough found: {:?}", borough.name);
    let to_borough_crs = Proj::new_known_crs("EPSG:4326", "EPSG:27700", None)?;

    let mut pubs_file = csv::Reader::from_path("./raw_data/pubs/Pubs.csv")?;

    let filtered_pubs: Vec<Pub> = pubs_file
        .deserialize()
        .filter_map(|result| result.ok())
        .filter_map(|mut pub_data: Pub| {
            if let Ok((x, y)) = to_borough_crs.convert((pub_data.longitude, pub_data.latitude)) {
                let pub_location = Point::new(x, y);
                if match &borough.geometry {
                    BoroughGeometry::Single(polygon) => polygon.contains(&pub_location),
                    BoroughGeometry::Multi(multi_polygon) => multi_polygon.contains(&pub_location),
                } {
                    pub_data.add_geometry(x, y);
                    Some(pub_data)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    println!("Pubs in {}: {:?}", borough.name, filtered_pubs.len());

    let mut borough_network: BoroughNetwork = read_road_files(borough)?;

    println!(
        "Roads in {}: {} roads, {} nodes",
        borough_network.borough.name,
        borough_network.links.len(),
        borough_network.nodes.len()
    );

    for pub_item in &filtered_pubs {
        if let Some(pub_point) = pub_item.geometry {
            let node = NetworkNode::new(pub_item.name.clone(), NetworkNodeType::Pub, pub_point);
            borough_network.add_node(node);
        }
    }

    plot_borough_data(
        &borough_network.borough,
        &filtered_pubs,
        &borough_network,
        "borough_visualization.png",
    )?;

    Ok(())
}
