use async_std::task::sleep;
use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use tracing::info;

use crate::algos;

pub async fn start() -> anyhow::Result<()> {
    // Every 10 seconds
    // let expression = "*/10 * * * * *";

    // Every Tuesday 10h30
    let expression = "* 30 8 * * 3 *";

    let schedule = Schedule::from_str(expression)?;

    let mut next = schedule.upcoming(Utc).take(1).next().unwrap();

    info!("Starting cron schedule");

    info!("now  {:?}", Utc::now());
    info!("next {:?}", next);

    loop {
        let diff = next - Utc::now();
        info!("waiting {:#?}", diff.to_std()?);

        sleep(diff.to_std()?).await;

        algos::song::matchmake().await?;

        next = schedule.upcoming(Utc).take(1).next().unwrap();
        info!("next {:?}", next);
    }
}
