use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use axum::{
    extract::State,
    routing::{get, patch},
    Json, Router,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[derive(Debug, Clone, Serialize, Builder)]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into, strip_option), default)]
    email: Option<String>,
    #[builder(default = "32")]
    age: u8,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UserUpdate {
    age: Option<u8>,
    skills: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let console = fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(console).init();

    let user = UserBuilder::default()
        .name("Aclice")
        .email("yu@answesome.com")
        .age(32)
        .skill("Rust")
        .skill("C++")
        .build()?;

    let shared_state = Arc::new(Mutex::new(user));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app = Router::new()
        .route("/", get(user_handler))
        .route("/", patch(update_handler))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Serving on {:?}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

#[instrument]
async fn user_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    (*user.lock().unwrap()).clone().into()
}

#[instrument]
async fn update_handler(
    State(user): State<Arc<Mutex<User>>>,
    Json(userupstate): Json<UserUpdate>,
) -> Json<User> {
    let mut user = user.lock().unwrap();
    if let Some(age) = userupstate.age {
        user.age = age;
    }
    if let Some(skills) = userupstate.skills {
        user.skills = skills;
    }
    (*user).clone().into()
}
