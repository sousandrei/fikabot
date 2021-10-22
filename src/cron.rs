use async_std::task::{sleep, JoinHandle};
use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use tracing::info;

use crate::algos;

pub fn start() -> JoinHandle<anyhow::Result<()>> {
    async_std::task::spawn(async move {
        // Every 10 seconds
        // let expression = "*/10 * * * * *";

        // Every Monday 10h30
        let expression = "* 30 10 * * 2 *";

        let schedule = Schedule::from_str(expression)?;

        let mut next = schedule.upcoming(Utc).take(1).next().unwrap();

        info!("Starting cron schedule");

        info!("now  {:?}", Utc::now());
        info!("next {:?}", next);

        loop {
            let diff = next - Utc::now();
            info!("waiting {:#?}", diff.to_std()?);

            sleep(diff.to_std()?).await;

            algos::matchmake().await?;

            next = schedule.upcoming(Utc).take(1).next().unwrap();
            info!("next {:?}", next);
        }
    })
}
