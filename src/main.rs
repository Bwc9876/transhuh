mod api;
mod cli;
mod geom;

mod prelude {
    use anyhow::Error as AError;
    use std::result::Result as StdResult;
    pub type Result<T = (), E = AError> = StdResult<T, E>;
}

use std::{collections::HashMap, hash::RandomState, time::Duration};

use anyhow::Context;
use api::Api;
use clap::Parser;
use notify_rust::Notification;
use prelude::*;

fn show_error_notif(e: anyhow::Error) -> Result {
    Notification::new()
        .summary("TransLoc Error")
        .icon("gpxsee")
        .body(&format!("Encountered Error: {e}\n\nWill not notify of future errors until a successful request."))
        .show().context("Failed to show notification").map(|_| ())
}

const POLL_INTERVAL: Duration = Duration::from_secs(5);
const RANGE_EPSILON: f64 = 0.001;

fn main() -> Result {
    let cli = cli::Cli::parse();
    let api = Api::new(&cli.api_base);

    let mut had_error = false;
    let mut notified_for_poi = HashMap::<_, _, RandomState>::from_iter(
        cli.points.iter().map(|p| (p.label.clone(), false)),
    );

    loop {
        match api.fetch_vehicle_points() {
            Ok(vehicles) => {
                had_error = false;

                let vehicles = vehicles
                    .into_iter()
                    .filter(|v| v.route_id == cli.route_id)
                    .collect::<Vec<_>>();

                for poi in cli.points.iter() {
                    let bus_in_range = vehicles
                        .iter()
                        .any(|v| poi.vehicle_within_range(v, RANGE_EPSILON));
                    let notified_previously = notified_for_poi.get_mut(&poi.label).unwrap();

                    if bus_in_range && !*notified_previously {
                        let notif = Notification::new()
                            .summary(&format!("The bus is at {}!", poi.label))
                            .icon("gpxsee")
                            .show()
                            .context("Failed to show notif for POI");
                        if let Err(why) = notif {
                            show_error_notif(why)?;
                            had_error = true;
                            continue;
                        }
                    }

                    *notified_previously = bus_in_range;
                }
            }
            Err(why) if !had_error => {
                show_error_notif(why)?;
                had_error = true;
            }
            _ => {}
        }

        std::thread::sleep(POLL_INTERVAL);
    }
}
