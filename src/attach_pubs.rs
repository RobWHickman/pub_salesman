use crate::model::{BoroughNetwork, NetworkLink, NetworkLinkType, NetworkNode, NetworkNodeType};
use geotypes::{LineString, Point, Line};
use std::collections::HashSet;
use Uuid;

pub fn attach_pubs(mut network: BoroughNetwork) -> BoroughNetwork {
    let pub_nodes: Vec<&NetworkNode> = network
        .nodes
        .iter()
        .filter(|node| matches!(node.node_type, NetworkNodeType::Pub))
        .collect();

    for pub_node in pub_nodes {
        if let Some((nearest_link, connection_point)) =
            find_nearest_link(&pub_node.geometry, &network.links)
        {
            let new_node_id = format!("CN_{}", Uuid::new_v4());
            let new_node = NetworkNode::new(
                new_node_id.clone(),
                NetworkNodeType::CreatedNode,
                connection_point,
            );

            let connection_link = NetworkLink::new(
                format!("CL_{}", Uuid::new_v4()),
                None,
                NetworkLinkType::CreatedLink,
                pub_node.geometry.geodesic_distance(&connection_point),
                HashSet::from([pub_node.node_id.clone(), new_node_id.clone()]),
                LineString::from(vec![
                    (pub_node.geometry.x(), pub_node.geometry.y()),
                    (connection_point.x(), connection_point.y()),
                ]),
            );

            let (link1, link2) = split_link(&nearest_link, &connection_point);

            network
                .links
                .retain(|link| link.link_id != nearest_link.link_id);
            network.links.push(link1);
            network.links.push(link2);
            network.links.push(connection_link);
            network.nodes.push(new_node);
        }
    }
    network
}

fn find_nearest_link(point: &Point<f64>, links: &[NetworkLink]) -> Option<(&NetworkLink, Point<f64>)> {
    links.iter()
        .filter_map(|link| {
            let connection = find_perpendicular_point(point, &link.geometry)?;
            Some((link, connection.point, connection.distance))
        })
        .min_by(|(_, _, dist1), (_, _, dist2)| 
            dist1.partial_cmp(dist2).unwrap_or(std::cmp::Ordering::Equal)
        )
        .map(|(link, point, _)| (link, point))
}

struct PerpendicularPoint {
    point: Point<f64>,
    distance: f64,
}

fn find_perpendicular_point(point: &Point<f64>, line: &LineString<f64>) -> Option<PerpendicularPoint> {
    line.points()
        .collect::<Vec<_>>()
        .windows(2)
        .filter_map(|window| {
            let line = Line::new(
                window[0],
                window[1]
            );
            
            let p2p = Point::new(point.x(), point.y());
            let proj = project_point_to_line(&p2p, &line)?;
            
            Some(PerpendicularPoint {
                point: proj,
                distance: point.geodesic_distance(&proj)
            })
        })
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal))
}