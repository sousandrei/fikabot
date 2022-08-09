use futures_util::TryStreamExt;
use sqlx::Row;

use super::DbConnection;

#[derive(Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
}

impl Channel {
    pub async fn find_all(conn: &DbConnection) -> anyhow::Result<Vec<Channel>> {
        let mut rows = sqlx::query("SELECT * FROM channels").fetch(conn);

        let mut channels = Vec::new();

        while let Some(row) = rows.try_next().await? {
            let channel = Channel {
                id: row.get(0),
                name: row.get(1),
            };

            channels.push(channel);
        }

        Ok(channels)
    }

    pub async fn _find(conn: &DbConnection) -> anyhow::Result<Channel> {
        let row = sqlx::query("SELECT * FROM channels")
            .fetch_one(conn)
            .await?;

        let channel = Channel {
            id: row.get(0),
            name: row.get(1),
        };

        Ok(channel)
    }

    pub async fn save(self, conn: &DbConnection) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO channels (id, name) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE name = ?",
        )
        .bind(&self.id)
        .bind(&self.name)
        .bind(&self.name)
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn delete(conn: &DbConnection, id: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM channels WHERE id = ?")
            .bind(id)
            .execute(conn)
            .await?;

        Ok(())
    }
}
