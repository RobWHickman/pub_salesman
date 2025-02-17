use crate::network_model::{Borough, BoroughGeometry};
use geo_types::{Coord, LineString, MultiPolygon, Polygon};
use shapefile::Shape;

const SHAPEFILE_PATH: &str = "./raw_data/polygons/London_Borough_Excluding_MHW.shp";

pub fn read_borough_file(borough_name: &str) -> Result<Borough, Box<dyn std::error::Error>> {
    let mut shapefile_reader = shapefile::Reader::from_path(SHAPEFILE_PATH)?;
    let mut shapefile_records = shapefile_reader.iter_shapes_and_records();

    while let Some(Ok((shape, record))) = shapefile_records.next() {
        if let Some(shapefile::dbase::FieldValue::Character(Some(name))) = record.get("NAME") {
            if name == borough_name {
                let geometry = match shape {
                    Shape::Polygon(polygon) => {
                        let polygons: Vec<Polygon> = polygon
                            .rings()
                            .iter()
                            .map(|ring| {
                                let coords: Vec<Coord> = ring
                                    .points()
                                    .iter()
                                    .map(|point| Coord {
                                        x: point.x,
                                        y: point.y,
                                    })
                                    .collect();

                                let line = LineString::from(coords);
                                Polygon::new(line, vec![])
                            })
                            .collect();

                        match polygons.len() {
                            1 => BoroughGeometry::Single(polygons.into_iter().next().unwrap()),
                            _ => BoroughGeometry::Multi(MultiPolygon::new(polygons)),
                        }
                    }
                    _ => continue,
                };

                return Ok(Borough {
                    name: name.clone(),
                    geometry,
                });
            }
        }
    }

    Err(format!("Borough '{}' not found", borough_name).into())
}
