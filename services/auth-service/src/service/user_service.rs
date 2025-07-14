use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::model::user::{RegisterUser, LoginData, User};
use crate::utils::jwt::create_jwt;
use std::error::Error;

pub async fn register_user(
    data: RegisterUser,
    db: &PgPool,
) -> Result<User, Box<dyn Error>> {
    let hashed = hash(&data.password, DEFAULT_COST)?;
    
    let record = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        data.username,
        data.email,
        hashed
    )
    .fetch_one(db)
    .await?;

    Ok(User {
        id: record.id,
        username: data.username,
        email: data.email,
    })
}

pub async fn login_user(
    data: LoginData,
    db: &PgPool,
) -> Result<(String, User), Box<dyn Error>> {
    let user = sqlx::query!(
        "SELECT id, username, email, password FROM users WHERE email = $1",
        data.email
    )
    .fetch_optional(db)
    .await?;

    let user = match user {
        Some(user) => user,
        None => return Err("Invalid credentials".into()),
    };

    if !verify(&data.password, &user.password)? {
        return Err("Invalid credentials".into());
    }

    let token = create_jwt(&user.email)?;

    let user_data = User {
        id: user.id,
        username: user.username,
        email: user.email,
    };

    Ok((token, user_data))
}

pub async fn get_user_by_email(
    email: &str,
    db: &PgPool,
) -> Result<Option<User>, Box<dyn Error>> {
    let result = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE email = $1",
        email
    )
    .fetch_optional(db)
    .await?;

    Ok(result)
}

pub async fn get_all_users(db: &PgPool) -> Result<Vec<User>, Box<dyn Error>> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users"
    )
    .fetch_all(db)
    .await?;

    Ok(users)
}
