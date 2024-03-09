use sea_orm::DatabaseConnection;

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    use entity::user;
    use sea_orm::{DatabaseBackend, MockDatabase};

    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            [user::Model {
                id: "1".to_owned(),
                username: "admin".to_owned(),
                name: "admin".to_owned(),
                bio: Some("".to_owned()),
                image: Some("".to_owned()),
                path: Some("".to_owned()),
            }],
            [user::Model {
                id: "2".to_owned(),
                username: "admin".to_owned(),
                name: "admin".to_owned(),
                bio: Some("".to_owned()),
                image: Some("".to_owned()),
                path: Some("".to_owned()),
            }],
        ])
        .into_connection()
}
