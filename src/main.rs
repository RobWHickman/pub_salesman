use clap::Parser;
use geo::prelude::Contains;
use pub_salesman::boroughs::read_borough_file;
use pub_salesman::links::read_road_files;
use pub_salesman::network_model::{BoroughGeometry, BoroughNetwork, NetworkNode, NetworkNodeType};
use pub_salesman::pub_model::Pub;
// use pub_salesman::plot::plot_borough_data;

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

    let mut pubs_file = csv::Reader::from_path("./raw_data/pubs/Pubs.csv")?;

    let filtered_pubs: Vec<Pub> = pubs_file
        .deserialize::<Pub>()
        .filter_map(Result::ok)
        .filter_map(|pub_data| match &borough.geometry {
            BoroughGeometry::Single(polygon) => {
                if polygon.contains(&pub_data.geometry) {
                    Some(pub_data)
                } else {
                    None
                }
            }
            BoroughGeometry::Multi(multi_polygon) => {
                if multi_polygon.contains(&pub_data.geometry) {
                    Some(pub_data)
                } else {
                    None
                }
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
        let node = NetworkNode::new(
            pub_item.name.clone(),
            NetworkNodeType::Pub,
            pub_item.geometry,
        );
        borough_network.add_node(node);
    }

    // plot_borough_data(
    //     &borough_network.borough,
    //     &filtered_pubs,
    //     &borough_network,
    //     "borough_visualization.png",
    // )?;

    Ok(())
}
