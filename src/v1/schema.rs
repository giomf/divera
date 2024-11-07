pub mod response {
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize)]
    pub struct User {
        pub user_id: i64,
        pub user_cluster_relation_id: i64,
        pub cluster_id: i64,
        pub status_id: i64,
        pub firstname: String,
        pub lastname: String,
    }
}
