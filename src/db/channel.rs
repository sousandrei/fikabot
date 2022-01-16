use serde::{Deserialize, Serialize};

use crate::db::{get_values, write_values, Response};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Channel {
    pub channel_id: String,
    pub channel_name: String,
}

const SHEET: &str = "channels";

impl Channel {
    pub async fn save(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut res: Response = get_values(config, SHEET).await?;

        res.values
            .push(vec![self.channel_id.clone(), self.channel_name.clone()]);

        res.values.dedup();

        write_values(config, SHEET, &res).await?;

        Ok(())
    }

    pub async fn delete(config: &crate::Config, channel: &str) -> anyhow::Result<()> {
        let mut res: Response = get_values(config, SHEET).await?;

        res.values = res
            .values
            .into_iter()
            .map(|v| match v.contains(&channel.to_owned()) {
                true => vec![String::new(); 2],
                false => v,
            })
            .collect();

        write_values(config, SHEET, &res).await?;

        Ok(())
    }

    pub async fn list(config: &crate::Config) -> anyhow::Result<Vec<Channel>> {
        let res: Response = get_values(config, SHEET).await?;

        let cs = res
            .values
            .into_iter()
            .map(|c| Channel {
                channel_id: c[0].clone(),
                channel_name: c[1].clone(),
            })
            .collect();

        Ok(cs)
    }
}
