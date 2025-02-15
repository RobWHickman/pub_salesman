use pub_salesman::model::Pub;

#[test]
fn test_read_first_pub() -> Result<(), Box<dyn std::error::Error>> {
    let mut pubs_file = csv::Reader::from_path("./data/pubs/pubs_cutdown.csv")?;

    let pub_data: Pub = pubs_file
        .deserialize()
        .next()
        .expect("Failed to read pub")?;

    assert!(!pub_data.name.is_empty());
    assert!(pub_data.latitude >= -90.0 && pub_data.latitude <= 90.0);
    assert!(pub_data.longitude >= -180.0 && pub_data.longitude <= 180.0);
    assert!(!pub_data.full_address().is_empty());

    println!(
        "First pub: {}, at ({}, {}), address: {}",
        pub_data.name,
        pub_data.latitude,
        pub_data.longitude,
        pub_data.full_address()
    );

    Ok(())
}
