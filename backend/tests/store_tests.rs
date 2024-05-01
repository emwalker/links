mod users {
    use recommendations::store::users;
    use sqlx::SqlitePool;

    #[sqlx::test(migrator = "recommendations::MIGRATOR")]
    async fn root(conn: SqlitePool) {
        let users = users::fetch_all(
            &conn,
            Some(users::Search {
                id: Some(users::ROOT_ID.into()),
            }),
        )
        .await
        .unwrap();
        let user = users.first().unwrap();

        assert_eq!(user.username, "root");
        assert_eq!(user.name, "Root account");
        assert!(user.is_admin);
    }

    #[sqlx::test(migrator = "recommendations::MIGRATOR")]
    async fn create(conn: SqlitePool) {
        let username: String = uuid::Uuid::new_v4().into();
        let result = users::create(
            &conn,
            &users::CreatePayload {
                username: username.clone(),
            },
        )
        .await
        .unwrap();
        assert!(result.user_id.is_some());
        assert!(result.created);
        assert!(result.errors.is_empty());

        // Fails when the username is already present
        let result = users::create(&conn, &users::CreatePayload { username })
            .await
            .unwrap();
        assert!(result.user_id.is_none());
        assert!(!result.created);
        assert!(result
            .errors
            .messages("username")
            .first()
            .unwrap()
            .starts_with("Username already taken: "));
    }
}
