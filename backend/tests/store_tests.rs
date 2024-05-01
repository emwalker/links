mod users {
    use recommendations::store::{topics, users};
    use sqlx::SqlitePool;

    #[sqlx::test(migrator = "recommendations::MIGRATOR")]
    async fn root(conn: SqlitePool) {
        let users = topics::fetch_all(
            &conn,
            Some(topics::Search {
                id: Some(topics::ROOT_ID.into()),
            }),
        )
        .await
        .unwrap();
        let user = users.first().unwrap();

        assert_eq!(user.name, "Root topic");
    }

    #[sqlx::test(migrator = "recommendations::MIGRATOR")]
    async fn create(conn: SqlitePool) {
        let owner_id = users::ROOT_ID;
        let name: String = uuid::Uuid::new_v4().into();
        let result = topics::create(
            &conn,
            &topics::CreatePayload {
                owner_id: owner_id.to_owned(),
                name: name.clone(),
            },
        )
        .await
        .unwrap();
        assert!(result.topic_id.is_some());
        assert!(result.created);
        assert!(result.errors.is_empty());

        // Fails when the topic is already present
        let result = topics::create(
            &conn,
            &topics::CreatePayload {
                name,
                owner_id: owner_id.to_owned(),
            },
        )
        .await
        .unwrap();
        assert!(result.topic_id.is_none());
        assert!(!result.created);
        assert!(result
            .errors
            .messages("name")
            .first()
            .unwrap()
            .starts_with("Topic name already taken: "));
    }
}

mod topics {
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
