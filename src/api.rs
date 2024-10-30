use anyhow::Context;
use serde::Deserialize;

use crate::prelude::*;

#[allow(unused)] // idc
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MapVehiclePoint {
    /// How fast the vehicle is heading
    ground_speed: f64,
    /// Current facing degrees relative to north?
    heading: u16,
    /// Whether this bus is behind schedule
    is_delayed: bool,
    /// Whether the bus is currently on the route line
    is_on_route: bool,
    /// Lat.
    pub latitude: f64,
    /// Long.
    pub longitude: f64,
    /// Bus Name
    name: String,
    /// ID of the route this bus is on
    #[serde(alias = "RouteID")]
    pub route_id: usize,
    /// ???
    seconds: usize,
    /// Timestamp of when the bus last reported its location
    time_stamp: String,
    /// Numeric ID of this bus
    #[serde(alias = "VehicleID")]
    vehicle_id: usize,
}

const VEHICLE_POINTS_ENDPOINT: &str = "/GetMapVehiclePoints?isPublicMap=true";

pub struct Api {
    vehicle_points_endpoint: String,
}

impl Api {
    pub fn new(api_base: &str) -> Self {
        Self {
            vehicle_points_endpoint: format!(
                "{}{}",
                api_base.trim_start_matches('/'),
                VEHICLE_POINTS_ENDPOINT
            ),
        }
    }

    pub fn fetch_vehicle_points(&self) -> Result<Vec<MapVehiclePoint>> {
        reqwest::blocking::get(&self.vehicle_points_endpoint)
            .context("Failed to GET vehicle points from API")?
            .json()
            .context("Failed to parse response from vehicle points API")
    }
}
