use geo_types::{LineString, MultiPolygon, Point, Polygon};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct Pub {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    address1: String,
    borough_name: String,
    #[serde(skip)]
    pub geometry: Option<Point<f64>>,
}

impl Pub {
    pub fn full_address(&self) -> String {
        format!("{}, {}", self.address1, self.borough_name)
    }

    pub fn add_geometry(&mut self, x: f64, y: f64) {
        self.geometry = Some(Point::new(x, y));
    }
}

#[derive(Debug)]
pub struct RoadNode {
    pub id: String,
    pub geometry: Point<f64>,
}

#[derive(Debug)]
pub struct RoadLink {
    pub id: String,
    pub name: Option<String>,
    pub length: f64,
    pub connected_nodes: HashSet<String>,
    pub geometry: LineString<f64>,
}

#[derive(Debug)]
pub enum BoroughGeometry {
    Single(Polygon),
    Multi(MultiPolygon),
}

#[derive(Debug)]
pub struct Borough {
    pub name: String,
    pub geometry: BoroughGeometry,
}

#[derive(Debug, Clone, Copy)]
pub enum NetworkNodeType {
    Pub,
    RoadNode,
    CreatedNode,
}

#[derive(Debug)]
pub struct NetworkNode {
    pub node_id: String,
    pub node_type: NetworkNodeType,
    pub geometry: Point<f64>,
}

impl NetworkNode {
    pub fn new(id: String, node_type: NetworkNodeType, geometry: Point<f64>) -> Self {
        NetworkNode {
            node_id: id,
            node_type,
            geometry,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NetworkLinkType {
    RoadLink,
    CreatedLink,
}

#[derive(Debug)]
pub struct NetworkLink {
    pub link_id: String,
    pub link_name: Option<String>,
    pub link_type: NetworkLinkType,
    pub link_length: f64,
    pub connected_nodes: HashSet<String>,
    pub geometry: LineString<f64>,
}

impl NetworkLink {
    pub fn new(
        id: String,
        name: Option<String>,
        link_type: NetworkLinkType,
        length: f64,
        connected_nodes: HashSet<String>,
        geometry: LineString<f64>,
    ) -> Self {
        NetworkLink {
            link_id: id,
            link_name: name,
            link_type,
            link_length: length,
            connected_nodes,
            geometry,
        }
    }
}

#[derive(Debug)]
pub struct BoroughNetwork {
    pub borough: Borough,
    pub nodes: Vec<NetworkNode>,
    pub links: Vec<NetworkLink>,
}

impl BoroughNetwork {
    pub fn new(borough: Borough) -> Self {
        BoroughNetwork {
            borough,
            nodes: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: NetworkNode) {
        self.nodes.push(node);
    }

    pub fn add_link(&mut self, link: NetworkLink) {
        self.links.push(link);
    }
}
