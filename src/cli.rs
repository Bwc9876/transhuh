use clap::clap_derive::Parser;

use crate::geom::PointOfInterest;

#[derive(Parser)]
#[command(name="transhuh", author, version, about, long_about = None)]
pub struct Cli {
    #[arg(
        short = 'u',
        long = "api-url",
        help = "URL for the TransLoc API, should end with `JSONPRelay.svc`"
    )]
    pub api_base: String,
    #[arg(short = 'r', long = "route", help = "Route of the bus to look for")]
    pub route_id: usize,
    #[arg(
        short = 'p',
        long = "poi",
        help = "A point of interest that will be checked, format: `label:latitude,longitude`"
    )]
    pub points: Vec<PointOfInterest>,
}
