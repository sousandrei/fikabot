use serde::{Deserialize, Serialize};

use crate::db::{get_values, write_values, Response};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub song: String,
}

const SHEET: &str = "users";

impl User {
    pub async fn save(&self) -> anyhow::Result<()> {
        let mut res: Response = get_values(SHEET).await?;

        res.values
            .push(vec![self.user_id.clone(), self.user_name.clone()]);

        res.values.dedup();

        write_values(SHEET, &res).await?;

        Ok(())
    }

    pub async fn _delete(user: &str) -> anyhow::Result<()> {
        let mut res: Response = get_values(SHEET).await?;

        res.values = res
            .values
            .into_iter()
            .map(|v| match v.contains(&user.to_owned()) {
                true => vec![String::new(); 3],
                false => v,
            })
            .collect();

        write_values(SHEET, &res).await?;

        Ok(())
    }

    pub async fn list() -> anyhow::Result<Vec<User>> {
        let res: Response = get_values(SHEET).await?;

        let us = res
            .values
            .into_iter()
            .map(|c| User {
                user_id: c[0].clone(),
                user_name: c[1].clone(),
                song: c[2].clone(),
            })
            .collect();

        Ok(us)
    }
}
