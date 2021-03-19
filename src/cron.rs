use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use tokio::{task::JoinHandle, time::sleep};

use crate::{algos, Error};

pub fn start() -> JoinHandle<Result<(), Error>> {
    tokio::spawn(async move {
        // Every 10 seconds
        // let expression = "*/10 * * * * *";

        // Every Monday 10h30
        let expression = "* 30 10 * * 2 *";

        let schedule = Schedule::from_str(expression)?;

        // TODO: properly treat this error
        let mut next = schedule.upcoming(Utc).take(1).next().unwrap();

        println!("now  {:?}", Utc::now());
        println!("next {:?}", next);

        loop {
            let diff = next - Utc::now();
            sleep(diff.to_std()?).await;

            algos::matchmake().await;

            // TODO: properly treat this error
            next = schedule.upcoming(Utc).take(1).next().unwrap();
            println!("next {:?}", next);
        }
    })
}
