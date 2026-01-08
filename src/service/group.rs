use tower::{
    management_model::{AddGroupReq, GroupListItem, ManagementResource},
    tauri_web::prelude::*,
};

pub async fn async_get_groups() -> Result<Vec<GroupListItem>, ApiError> {
    async_http_and(
        App::TowerServer,
        "get",
        &ManagementResource::Group.path(ApiMethod::List),
        empty_req(),
    )
    .await
}

pub async fn async_add_group(req: AddGroupReq) -> Result<(), ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::Group.path(ApiMethod::Insert),
        req,
    )
    .await
}
