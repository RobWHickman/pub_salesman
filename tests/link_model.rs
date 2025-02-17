// use geo_types::{Coord, LineString};
// use pub_salesman::network_model::RoadLink;
// use shapefile::Shape::PolylineZ;
// use std::collections::HashSet;

// #[test]
// fn test_read_first_link() -> Result<(), Box<dyn std::error::Error>> {
//     let mut link_data = shapefile::Reader::from_path("./raw_data/roads/HP_RoadLink.shp")?;
//     let mut link_records = link_data.iter_shapes_and_records();

//     if let Some(Ok((shape, record))) = link_records.next() {
//         let id = if let Some(shapefile::dbase::FieldValue::Character(Some(s))) =
//             record.get("identifier")
//         {
//             s.clone()
//         } else {
//             panic!("Missing identifier field");
//         };

//         let name = if let Some(shapefile::dbase::FieldValue::Character(s)) = record.get("name1") {
//             s.clone()
//         } else {
//             None
//         };

//         let length =
//             if let Some(shapefile::dbase::FieldValue::Numeric(Some(l))) = record.get("length") {
//                 *l
//             } else {
//                 panic!("Missing length field");
//             };

//         let mut connected_nodes = HashSet::new();
//         if let Some(shapefile::dbase::FieldValue::Character(Some(s))) = record.get("startNode") {
//             connected_nodes.insert(s.clone());
//         }
//         if let Some(shapefile::dbase::FieldValue::Character(Some(s))) = record.get("endNode") {
//             connected_nodes.insert(s.clone());
//         }

//         let geometry = match shape {
//             PolylineZ(polyline) => {
//                 // Get all parts as a single LineString
//                 let coords: Vec<Coord> = polyline
//                     .parts()
//                     .iter()
//                     .flat_map(|part| {
//                         part.iter().map(|point| Coord {
//                             x: point.x,
//                             y: point.y,
//                         })
//                     })
//                     .collect();

//                 LineString::new(coords)
//             }
//             _ => panic!("Expected PolyLineZ shape"),
//         };

//         let first_link = RoadLink {
//             id,
//             name,
//             length,
//             connected_nodes,
//             geometry,
//         };

//         println!("First RoadLink: {:?}", first_link);
//     }
//     Ok(())
// }
