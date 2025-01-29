mod common;

use actix_web::{test, web, App};
use bigdecimal::BigDecimal;
use property_service::{handlers, models::Property};
use uuid::Uuid;
use common::{setup_database, teardown_database};

#[actix_rt::test]
async fn test_create_property() {
    let pool = setup_database().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/properties", web::post().to(handlers::create_property)),
    )
        .await;

    let property = Property {
        id: Uuid::new_v4(),
        address: "123 Main St".to_string(),
        price: BigDecimal::try_from(500000.0).unwrap(),
        bedrooms: 3,
        bathrooms: 2,
    };

    let req = test::TestRequest::post()
        .uri("/properties")
        .set_json(&property)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    teardown_database(pool).await;
}

#[actix_rt::test]
async fn test_get_property() {
    let pool = setup_database().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/properties/{id}", web::get().to(handlers::get_property)),
    )
        .await;

    let property = Property {
        id: Uuid::new_v4(),
        address: "456 Elm St".to_string(),
        price: BigDecimal::try_from(300000.0).unwrap(),
        bedrooms: 2,
        bathrooms: 1,
    };

    // Insert a property into the database
    sqlx::query!(
        r#"
        INSERT INTO properties (id, address, price, bedrooms, bathrooms)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        property.id,
        property.address,
        property.price,
        property.bedrooms,
        property.bathrooms
    )
        .execute(&pool)
        .await
        .expect("Failed to insert property");

    let req = test::TestRequest::get()
        .uri(&format!("/properties/{}", property.id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: Property = test::read_body_json(resp).await;
    assert_eq!(body.id, property.id);
    assert_eq!(body.address, property.address);

    teardown_database(pool).await;
}

#[actix_rt::test]
async fn test_get_all_properties() {
    let pool = setup_database().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/properties", web::get().to(handlers::get_all_properties)),
    )
        .await;

    let property1 = Property {
        id: Uuid::new_v4(),
        address: "123 Main St".to_string(),
        price: BigDecimal::try_from(500000.0).unwrap(),
        bedrooms: 3,
        bathrooms: 2,
    };

    let property2 = Property {
        id: Uuid::new_v4(),
        address: "456 Elm St".to_string(),
        price: BigDecimal::try_from(300000.0).unwrap(),
        bedrooms: 2,
        bathrooms: 1,
    };

    // Insert properties into the database
    sqlx::query!(
        r#"
        INSERT INTO properties (id, address, price, bedrooms, bathrooms)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        property1.id,
        property1.address,
        property1.price,
        property1.bedrooms,
        property1.bathrooms
    )
        .execute(&pool)
        .await
        .expect("Failed to insert property1");

    sqlx::query!(
        r#"
        INSERT INTO properties (id, address, price, bedrooms, bathrooms)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        property2.id,
        property2.address,
        property2.price,
        property2.bedrooms,
        property2.bathrooms
    )
        .execute(&pool)
        .await
        .expect("Failed to insert property2");

    let req = test::TestRequest::get().uri("/properties").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: Vec<Property> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 2);

    teardown_database(pool).await;
}