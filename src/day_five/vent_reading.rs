use std::str::FromStr;

#[derive(Clone)]
pub struct Coordinate {
    x: u16,
    y: u16,
}

impl FromStr for Coordinate {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<u16> = s.split(',').filter_map(|s| s.parse::<u16>().ok()).collect();
        match values.len() == 2 {
            true => Ok(Coordinate {
                x: values[0],
                y: values[1],
            }),
            false => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{} is not a valid x,y coordinate", s),
            )),
        }
    }
}

pub struct VentReading {
    start: Coordinate,
    end: Coordinate,
}

impl FromStr for VentReading {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<Coordinate> = s
            .split(" -> ")
            .filter_map(|s| s.parse::<Coordinate>().ok())
            .collect();

        match coordinates.len() == 2 {
            true => Ok(VentReading {
                start: coordinates[0].clone(),
                end: coordinates[1].clone(),
            }),
            false => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{} is not a valid x1,y1 -> x2,y2 Vent Reading", s),
            )),
        }
    }
}
