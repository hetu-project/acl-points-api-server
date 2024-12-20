use crate::config::Config;
use crate::jwt::jwt_handler;
use crate::storage;
use oauth2::basic::BasicClient;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Server {
    pub config: Config,
    pub store: storage::Storage,
    pub oauth: Arc<BasicClient>,
    pub jwt_handler: jwt_handler::JwtHandler,
}

impl Server {
    pub async fn new(path: PathBuf) -> Self {
        //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        //let manager = ConnectionManager::<PgConnection>::new(database_url);
        //let pg = Pool::builder().build(manager).expect("Failed to create pool.");

        let config = Config::load_config(path).unwrap();
        tracing::info!("{:?}", config);
        let store = storage::Storage::new(config.database.clone()).await;
        tracing::info!("{:?}", store);

        let secret = "my_secret_key".to_string();
        let jwt_handler = jwt_handler::JwtHandler { secret };

        Self {
            config,
            store,
            oauth: Arc::new(super::oauth::oauth_client()),
            jwt_handler,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SharedState(pub Arc<RwLock<Server>>);

impl SharedState {
    pub async fn new(path: PathBuf) -> Self {
        let server = Server::new(path).await;
        SharedState(Arc::new(RwLock::new(server)))
    }
}
