// use geo_types::Point;
// use pub_salesman::network_model::RoadNode;
// use shapefile::Shape;

// #[test]
// fn test_read_first_node() -> Result<(), Box<dyn std::error::Error>> {
//     let mut node_data = shapefile::Reader::from_path("./raw_data/roads/HP_RoadNode.shp")?;
//     let mut node_records = node_data.iter_shapes_and_records();

//     if let Some(Ok((shape, record))) = node_records.next() {
//         let geometry = match shape {
//             Shape::PointZ(point) => Point::new(point.x, point.y),
//             _ => panic!("Expected a Point shape"),
//         };

//         let id = if let Some(shapefile::dbase::FieldValue::Character(Some(s))) =
//             record.get("identifier")
//         {
//             s.clone()
//         } else {
//             panic!("Missing identifier field");
//         };

//         let first_node = RoadNode { id, geometry };

//         println!("First RoadNode: {:?}", first_node);
//     }
//     Ok(())
// }
