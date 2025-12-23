use crate::servers::ServerManager;
use crate::{AppError, User};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use smol_str::SmolStr;
use common::status::ServerStatus;

#[derive(serde::Deserialize)]
pub(super) struct Filters {
    game: Option<SmolStr>,
}

pub(super) async fn get_servers(
    user: User,
    State(server_manager): State<ServerManager>,
    filters: Query<Filters>,
) -> Result<impl IntoResponse, AppError> {
    let servers = server_manager.get_servers_for_user(&user).await?;
    let filtered = if let Some(ref filter) = filters.game {
        servers.into_iter().filter(|s| {
            match s {
                ServerStatus::Factorio(_) => filter == "Factorio",
                ServerStatus::Generic(_) => filter == "Generic",
            }
        }).collect()
    } else {
        servers
    };

    Ok(Json(filtered))
}
