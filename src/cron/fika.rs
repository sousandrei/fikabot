use async_std::task::sleep;
use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use tracing::{error, info};

use crate::algos;

pub async fn start() -> anyhow::Result<()> {
    // Every 10 seconds
    // let expression = "*/10 * * * * *";

    // Every Monday 10h30
    let expression = "0 30 8 * * 2 *";

    let schedule = Schedule::from_str(expression)?;

    let mut next = schedule.upcoming(Utc).take(1).next().unwrap();

    info!("Starting cron schedule");

    info!("now  {:?}", Utc::now());
    info!("next {:?}", next);

    loop {
        let diff = next - Utc::now();
        info!("waiting {:#?}", diff.to_std()?);

        sleep(diff.to_std()?).await;

        if let Err(e) = algos::fika::matchmake().await {
            error!("Error on matchmaking: {}", e);
        };

        next = schedule.upcoming(Utc).take(1).next().unwrap();
        info!("next {:?}", next);
    }
}
