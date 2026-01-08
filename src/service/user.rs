use tower::{
    management_model::{ManagementResource, UserListReq, UserListResp},
    tauri_web::prelude::*,
};

pub async fn async_get_paged_users(
    req: Page<UserListReq>,
) -> Result<Page<Vec<UserListResp>>, ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::User.path(ApiMethod::Page),
        req,
    )
    .await
}
