use tower::{
    management_model::{GetUserAppsReq, ManagementResource},
    tauri_web::prelude::*,
};

pub async fn async_get_user_apps(user_id: Option<String>) -> Result<Vec<String>, ApiError> {
    if user_id.is_none() {
        return Ok(vec![]);
    }
    let user_id = user_id.unwrap();
    async_http_and(
        App::TowerServer,
        "get",
        &ManagementResource::UserApp.path(ApiMethod::List),
        GetUserAppsReq { user_id },
    )
    .await
}
