use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use crate::servers::ServerManager;
use crate::{AppError, User};

pub(super) async fn get_servers(
    user: User,
    State(server_manager): State<ServerManager>,
) -> Result<impl IntoResponse, AppError> {
    let servers = server_manager
        .get_servers_for_user(&user)
        .await?;
    
    Ok(Json(servers))
}