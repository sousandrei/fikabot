use futures_util::TryStreamExt;
use sqlx::Row;

use super::DbConnection;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub song: String,
}

impl User {
    pub async fn find_all(conn: &DbConnection) -> anyhow::Result<Vec<User>> {
        let mut rows = sqlx::query("SELECT * FROM users").fetch(conn);

        let mut users = Vec::new();

        while let Some(row) = rows.try_next().await? {
            let user = User {
                id: row.get(0),
                name: row.get(1),
                song: row.get(2),
            };

            users.push(user);
        }

        Ok(users)
    }

    pub async fn _find(conn: &DbConnection) -> anyhow::Result<User> {
        let row = sqlx::query("SELECT * FROM users").fetch_one(conn).await?;

        let user = User {
            id: row.get(0),
            name: row.get(1),
            song: row.get(2),
        };

        Ok(user)
    }

    pub async fn save(self, conn: &DbConnection) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO users (id, name, song) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE name = ?, song = ?")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.song)
            .bind(&self.name)
            .bind(&self.song)
            .execute(conn)
            .await?;

        Ok(())
    }
}
