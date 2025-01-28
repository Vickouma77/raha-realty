use sqlx::{PgPool};
use uuid::Uuid;
use crate::models::Property;

pub async fn create_property(pool: &PgPool, property: &Property) -> Result<(),  sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO properties (id, address, price, bedrooms, bathrooms)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        property.id,
        property.address,
        property.price,
        property.bedrooms,
        property.bathrooms,
    )
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_property(pool: &PgPool, property_id: Uuid) -> Result<Option<Property>, sqlx::Error> {
    sqlx::query_as!(
        Property,
        r#"
        SELECT id, address, price, bedrooms, bathrooms
        FROM properties
        WHERE id = $1
        "#,
        property_id
    )
        .fetch_optional(pool)
        .await
}

pub async fn get_all_properties(pool: &PgPool) -> Result<Vec<Property>, sqlx::Error> {
    sqlx::query_as!(
        Property,
        r#"
        SELECT id, address, price, bedrooms, bathrooms
        FROM properties
        "#
    )
        .fetch_all(pool)
        .await
}

pub async fn update_property(pool: &PgPool, property: Property) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE properties
        SET address = $2, price = $3, bedrooms = $4, bathrooms = $5
        WHERE id = $1
        "#,
        property.id,
        property.address,
        property.price,
        property.bedrooms,
        property.bathrooms
    )
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_property(pool: &PgPool, property_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM properties
        WHERE id = $1
        "#,
        property_id
    )
        .execute(pool)
        .await?;
    Ok(())
}