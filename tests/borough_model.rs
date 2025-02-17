// use geo_types::{Coord, LineString, MultiPolygon, Polygon};
// use pub_salesman::network_model::{Borough, BoroughGeometry};
// use shapefile::Shape;

// #[test]
// fn test_read_first_borough() -> Result<(), Box<dyn std::error::Error>> {
//     let mut shapefile_data: shapefile::Reader<
//         std::io::BufReader<std::fs::File>,
//         std::io::BufReader<std::fs::File>,
//     > = shapefile::Reader::from_path("./raw_data/polygons/London_Borough_Excluding_MHW.shp")?;
//     let mut shapefile_records: shapefile::reader::ShapeRecordIterator<
//         '_,
//         std::io::BufReader<std::fs::File>,
//         std::io::BufReader<std::fs::File>,
//         shapefile::Shape,
//         shapefile::dbase::Record,
//     > = shapefile_data.iter_shapes_and_records();

//     if let Some(Ok((shape, record))) = shapefile_records.next() {
//         let name =
//             if let Some(shapefile::dbase::FieldValue::Character(Some(s))) = record.get("NAME") {
//                 s.clone()
//             } else {
//                 panic!("Missing borough name field");
//             };

//         let geometry = match shape {
//             Shape::Polygon(polygon) => {
//                 let polygons: Vec<Polygon> = polygon
//                     .rings() // Access rings from the polygon
//                     .iter()
//                     .map(|ring| {
//                         let coords: Vec<Coord> = ring
//                             .points() // Access points from each ring
//                             .iter()
//                             .map(|point| Coord {
//                                 x: point.x, // PointZ provides x() method
//                                 y: point.y, // PointZ provides y() method
//                             })
//                             .collect();

//                         let line = LineString::from(coords);
//                         Polygon::new(line, vec![])
//                     })
//                     .collect();

//                 match polygons.len() {
//                     1 => BoroughGeometry::Single(polygons.into_iter().next().unwrap()),
//                     _ => BoroughGeometry::Multi(MultiPolygon::new(polygons)),
//                 }
//             }
//             _ => panic!("Expected PolygonZ shape"),
//         };

//         let first_borough = Borough { name, geometry };

//         println!("Borough Name: {:?}", first_borough.name);
//         match &first_borough.geometry {
//             BoroughGeometry::Single(polygon) => {
//                 println!(
//                     "First few coordinates: {:?}",
//                     polygon.exterior().0.iter().take(5).collect::<Vec<_>>()
//                 );
//             }
//             BoroughGeometry::Multi(multi_polygon) => {
//                 if let Some(first_polygon) = multi_polygon.0.first() {
//                     println!(
//                         "First few coordinates: {:?}",
//                         first_polygon
//                             .exterior()
//                             .0
//                             .iter()
//                             .take(5)
//                             .collect::<Vec<_>>()
//                     );
//                 }
//             }
//         }
//     }
//     Ok(())
// }
