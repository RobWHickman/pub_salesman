use geo_types::Point;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

// rough bounds of London
const MIN_EASTING: f64 = 490000.0;
const MAX_EASTING: f64 = 570000.0;
const MIN_NORTHING: f64 = 156000.0;
const MAX_NORTHING: f64 = 203000.0;

#[derive(Debug)]
pub enum PubError {
    InvalidCoordinates(String),
}

impl fmt::Display for PubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PubError::InvalidCoordinates(msg) => write!(f, "Invalid coordinates: {}", msg),
        }
    }
}

impl Error for PubError {}

#[derive(Debug, Deserialize)]
#[serde(try_from = "PubRaw")]
pub struct Pub {
    pub name: String,
    pub address: String,
    pub geometry: Point<f64>,
}

#[derive(Debug, Deserialize)]
struct PubRaw {
    name: String,
    address1: String,
    borough_name: String,
    easting: f64,
    northing: f64,
}

impl TryFrom<PubRaw> for Pub {
    type Error = PubError;

    fn try_from(raw: PubRaw) -> Result<Self, Self::Error> {
        if !is_valid_coordinates(raw.easting, raw.northing) {
            return Err(PubError::InvalidCoordinates(format!(
                "Coordinates out of London bounds - easting: {} ({}-{}), northing: {} ({}-{})",
                raw.easting, MIN_EASTING, MAX_EASTING, raw.northing, MIN_NORTHING, MAX_NORTHING
            )));
        }

        Ok(Pub {
            name: raw.name,
            address: format!("{}, {}", raw.address1, raw.borough_name),
            geometry: Point::new(raw.easting, raw.northing),
        })
    }
}

fn is_valid_coordinates(easting: f64, northing: f64) -> bool {
    (MIN_EASTING..=MAX_EASTING).contains(&easting)
        && (MIN_NORTHING..=MAX_NORTHING).contains(&northing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_raw_pubs() -> std::io::Result<()> {
        let mut pubs_file = csv::Reader::from_path("./raw_data/pubs/Pubs.csv")?;
        let mut errors = Vec::new();

        for (index, result) in pubs_file.deserialize::<PubRaw>().enumerate() {
            if let Err(err) = result {
                errors.push(format!("Row {}: {}", index + 1, err));
            }
        }
        assert!(
            errors.is_empty(),
            "Found {} parsing errors:\n{}",
            errors.len(),
            errors.join("\n")
        );

        Ok(())
    }

    #[test]
    fn test_valid_coords() {
        assert!(is_valid_coordinates(MIN_EASTING + 1.0, MIN_NORTHING + 1.0));
        assert!(is_valid_coordinates(MAX_EASTING - 1.0, MAX_NORTHING - 1.0));
    }
    #[test]
    fn test_invalid_coords() {
        assert!(!is_valid_coordinates(MIN_EASTING - 1.0, MIN_NORTHING - 1.0));
        assert!(!is_valid_coordinates(MAX_EASTING + 1.0, MAX_NORTHING + 1.0));
    }

    #[test]
    fn test_read_pubs() -> Result<(), Box<dyn std::error::Error>> {
        let mut pubs_file = csv::Reader::from_path("./raw_data/pubs/Pubs.csv")?;

        let first_raw_pub = pubs_file
            .deserialize::<PubRaw>()
            .next()
            .ok_or("No records found")??;

        let first_pub = Pub::try_from(first_raw_pub)?;

        println!("First pub found:");
        println!("Name: {}", first_pub.name);
        println!("Address: {}", first_pub.address);
        println!("Location: {:?}", first_pub.geometry);

        Ok(())
    }

    #[test]
    fn test_invalid_pub() {
        let bad_raw_pub = PubRaw {
            name: "The Bad Pub".to_string(),
            address1: "1 bad pub road".to_string(),
            borough_name: "invalid borough".to_string(),
            easting: MAX_EASTING + 1.0,
            northing: MAX_NORTHING + 1.0,
        };

        match Pub::try_from(bad_raw_pub) {
            Err(PubError::InvalidCoordinates(_)) => (),
            Ok(_) => panic!("Expected invalid coordinates error, but conversion succeeded"),
        }
    }
}
