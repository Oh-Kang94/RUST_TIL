use axum::{
    extract::Extension,
    http::{header, Method},
    routing::{get, post},
    Router,
};


let app = Router::new()
        .route("/", get(controllers::info::route_info))
        .route("/login", post(controllers::auth::login))
        .route("/register", post(controllers::auth::register))
        .route("/", post(controllers::auth::refresh_token))
        .route("/user_profile", get(controllers::user::user_profile))
        .layer(cors)
        .layer(Extension(pool));
        