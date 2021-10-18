mod users;

use users::entities::User;
use sqlx;
use sqlx::types::Uuid;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection = "postgres://dbuser:secret@postgis:5432/database";
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(connection)
        .await?;

    let users = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", users.len());

    let mut tr = pool.begin().await?;
    sqlx::query!(
        r#"
        INSERT INTO users(
            id,
            username,
            encrypted_password,
            avatar_icon
        ) VALUES (
            $1,$2,$3,$4
        )
        "#,
        Uuid::new_v4(),
        "hogehawefrsawfrgsgeoge",
        "piyo",
        "fijfijfi"
    )
    .execute(&mut tr)
    .await?;
    
    tr.commit().await?;

    
    Ok(())
}