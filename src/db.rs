use crate::User;

pub async fn add_user(user: User) {
    let db = sqlite::open("lunch.db").unwrap();

    let query = format!(
        "INSERT INTO users VALUES ('{}', '{}')",
        user.user_id, user.user_name
    );

    // TODO: properly treat this error
    db.execute(query).unwrap();
}

pub async fn del_user(user: User) {
    let db = sqlite::open("lunch.db").unwrap();

    let query = format!(
        "DELETE FROM users WHERE user_id='{}' AND user_name='{}' LIMIT 1",
        user.user_id, user.user_name
    );

    // TODO: properly treat this error
    db.execute(query).unwrap();
}

pub async fn list_users() -> Vec<User> {
    let db = sqlite::open("lunch.db").unwrap();

    let query = "SELECT * FROM 'users'";

    // TODO: properly treat this error
    let mut cursor = db.prepare(query).unwrap().cursor();

    let mut users = Vec::new();

    // TODO: cleanup this user assemblying
    while let Some(row) = cursor.next().unwrap() {
        if let [user_id, user_name] = row {
            let user = User {
                user_id: user_id.as_string().unwrap().to_string(),
                user_name: user_name.as_string().unwrap().to_string(),
            };

            users.push(user);
        }
    }

    users
}
