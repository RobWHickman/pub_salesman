#[test]
fn test_read_shapefile() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = shapefile::Reader::from_path("./raw_data/roads/HP_RoadLink.shp")?;
    let mut records = reader.iter_shapes_and_records();

    if let Some(Ok((shape, record))) = records.next() {
        println!("First record: {:?}", record);
        println!("Shape type: {:?}", shape.shapetype());
        if let shapefile::Shape::PolylineZ(polyline_z) = shape {
            if let Some(first_part) = polyline_z.parts().get(0) {
                println!(
                    "First few coordinates of first part: {:?}",
                    &first_part[..first_part.len().min(5)]
                );
            }
        }
    }
    Ok(())
}

#[test]
fn test_read_nodefile() -> Result<(), Box<dyn std::error::Error>> {
    let mut node_data = shapefile::Reader::from_path("./raw_data/roads/HP_RoadNode.shp")?;
    let mut node_records: shapefile::reader::ShapeRecordIterator<
        '_,
        std::io::BufReader<std::fs::File>,
        std::io::BufReader<std::fs::File>,
        shapefile::Shape,
        shapefile::dbase::Record,
    > = node_data.iter_shapes_and_records();

    if let Some(Ok((points, record))) = node_records.next() {
        println!("First record: {:?}", record);
        println!("Shape type: {:?}", points.shapetype());
        if let shapefile::Shape::PointZ(point_z) = points {
            println!("First node coordinates: {:?}, {:?}", point_z.x, point_z.y)
        }
    }
    Ok(())
}
