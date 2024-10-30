use std::str::FromStr;

use anyhow::Context;

use crate::{api::MapVehiclePoint, prelude::*};

#[derive(Debug, Clone)]
pub struct PointOfInterest {
    pub label: String,
    latitude: f64,
    longitude: f64,
}

/// Lat, Long
type Cord = (f64, f64);

fn contains(poi: Cord, vehicle: Cord, range: f64) -> bool {
    let lat_range = (poi.0 - range)..(poi.0 + range);
    let long_range = (poi.1 - range)..(poi.1 + range);
    lat_range.contains(&vehicle.0) && long_range.contains(&vehicle.1)
}

impl PointOfInterest {
    pub fn vehicle_within_range(&self, vehicle: &MapVehiclePoint, range: f64) -> bool {
        contains(
            (self.latitude, self.longitude),
            (vehicle.latitude, vehicle.longitude),
            range,
        )
    }
}

impl FromStr for PointOfInterest {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (label, cords) = value
            .split_once(":")
            .context("Invalid syntax (expected `label:lat,long`)")?;
        let (lat, long) = cords
            .split_once(",")
            .context("Invalid syntax (expected `label:lat,long`")?;

        Ok(Self {
            label: label.to_string(),
            latitude: lat.parse().context("Latitude is not a valid number")?,
            longitude: long.parse().context("Longitude is not a valid number")?,
        })
    }
}
