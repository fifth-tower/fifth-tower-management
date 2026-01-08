use tower::{
    management_model::{GetUserRolesReq, ManagementResource},
    tauri_web::prelude::*,
};

pub async fn async_get_user_groups(user_id: Option<String>) -> Result<Vec<String>, ApiError> {
    if user_id.is_none() {
        return Ok(vec![]);
    }
    let user_id = user_id.unwrap();
    async_http_and(
        App::TowerServer,
        "get",
        &ManagementResource::UserRole.path(ApiMethod::List),
        GetUserRolesReq { user_id },
    )
    .await
}
