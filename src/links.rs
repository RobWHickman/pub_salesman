use crate::model::{
    Borough, BoroughGeometry, BoroughNetwork, NetworkLink, NetworkLinkType, NetworkNode,
    NetworkNodeType,
};
use geo::prelude::*;
use geo_types::{Coord, Point};
use shapefile::Shape;
use std::collections::HashSet;

const SHAPEFILE_CODES: [&str; 4] = ["SP", "TL", "SU", "TQ"];
const SHAPEFILE_PATH: &str = "./raw_data/roads/";

pub fn read_road_files(borough: Borough) -> Result<BoroughNetwork, Box<dyn std::error::Error>> {
    let mut network: BoroughNetwork = BoroughNetwork::new(borough);

    for code in SHAPEFILE_CODES {
        println!("Reading data for {}", code);
        let roadlinks_file = format!("{}{}_RoadLink.shp", SHAPEFILE_PATH, code);
        let roadnodes_file = format!("{}{}_RoadNode.shp", SHAPEFILE_PATH, code);

        let mut links_reader = shapefile::Reader::from_path(&roadlinks_file)?;
        let mut nodes_reader = shapefile::Reader::from_path(&roadnodes_file)?;

        let links_records = links_reader.iter_shapes_and_records();
        let nodes_records = nodes_reader.iter_shapes_and_records();

        for record in links_records {
            if let Ok((shape, record)) = record {
                if let Some(network_link) = generate_network_link(record, shape, &network.borough) {
                    network.add_link(network_link);
                }
            }
        }

        let link_ids: HashSet<String> = network
            .links
            .iter()
            .flat_map(|link| link.connected_nodes.iter().cloned())
            .collect();

        for record in nodes_records {
            if let Ok((shape, record)) = record {
                if let Some(network_node) = generate_network_node(record, shape, &link_ids) {
                    network.add_node(network_node);
                }
            }
        }
    }

    Ok(network)
}

pub fn generate_network_node(
    node_record: shapefile::dbase::Record,
    node_shape: Shape,
    links_ids: &HashSet<String>,
) -> Option<NetworkNode> {
    let geometry: Point = match node_shape {
        Shape::PointZ(point) => Point::new(point.x, point.y),
        _ => return None,
    };

    let id = if let Some(shapefile::dbase::FieldValue::Character(Some(s))) =
        node_record.get("identifier")
    {
        s.clone()
    } else {
        return None;
    };

    if !links_ids.contains(&id) {
        return None;
    }

    Some(NetworkNode::new(id, NetworkNodeType::RoadNode, geometry))
}

pub fn generate_network_link(
    link_record: shapefile::dbase::Record,
    link_shape: Shape,
    borough: &Borough,
) -> Option<NetworkLink> {
    // Convert shape to LineString
    let geometry = match link_shape {
        Shape::PolylineZ(polyline) => {
            let coords: Vec<Coord> = polyline
                .parts()
                .iter()
                .flat_map(|part| {
                    part.iter().map(|point| Coord {
                        x: point.x,
                        y: point.y,
                    })
                })
                .collect();

            if coords.is_empty() {
                return None;
            }

            geo_types::LineString::new(coords)
        }
        _ => return None,
    };

    // Check if the line intersects with the borough using geo crate's implementation
    let intersects = match &borough.geometry {
        BoroughGeometry::Single(polygon) => geometry.intersects(polygon),
        BoroughGeometry::Multi(multi_polygon) => geometry.intersects(multi_polygon),
    };

    if !intersects {
        return None;
    }

    // Extract road information
    let id = match link_record.get("identifier") {
        Some(shapefile::dbase::FieldValue::Character(Some(s))) => s.clone(),
        _ => return None,
    };

    let name = match link_record.get("name1") {
        Some(shapefile::dbase::FieldValue::Character(s)) => s.clone(),
        _ => None,
    };

    let length = match link_record.get("length") {
        Some(shapefile::dbase::FieldValue::Numeric(Some(l))) => *l,
        _ => return None,
    };

    let start_node = match link_record.get("startNode") {
        Some(shapefile::dbase::FieldValue::Character(Some(s))) => s.clone(),
        _ => return None,
    };

    let end_node = match link_record.get("endNode") {
        Some(shapefile::dbase::FieldValue::Character(Some(s))) => s.clone(),
        _ => return None,
    };

    let mut connected_nodes = HashSet::new();
    connected_nodes.insert(start_node);
    connected_nodes.insert(end_node);

    Some(NetworkLink::new(
        id,
        name,
        NetworkLinkType::RoadLink,
        length,
        connected_nodes,
        geometry,
    ))
}
