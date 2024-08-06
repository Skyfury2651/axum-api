use super::auth;
use crate::middleware::auth as auth_middleware;
use crate::routes::{ profile, register };
use crate::state::auth_state::AuthState;
use crate::state::token_state::TokenState;
use crate::state::user_state::UserState;
use axum::routing::{ get, IntoMakeService };
use axum::{ middleware, Router };
use tower::ServiceBuilder;
use std::sync::Arc;
use crate::config::database::Database;

pub fn routes(db_conn: Arc<Database>) -> IntoMakeService<Router> {
    let merged_router = {
        let auth_state = AuthState::new(&db_conn);
        let user_state = UserState::new(&db_conn);
        let token_state = TokenState::new(&db_conn);
        auth::routes()
            .with_state(auth_state)
            .merge(register::routes().with_state(user_state))
            .merge(
                profile
                    ::routes()
                    .layer(
                        ServiceBuilder::new().layer(
                            middleware::from_fn_with_state(token_state, auth_middleware::auth)
                        )
                    )
            )
            .merge(
                Router::new().route(
                    "/health",
                    get(|| async { "Healthy..." })
                )
            )
    };

    let app_router: Router = Router::new().nest("/api", merged_router);
    app_router.into_make_service()
}
