use tower::{
    management_model::{AddRoleReq, ManagementResource, RoleListItem},
    tauri_web::prelude::*,
};

pub async fn async_get_roles() -> Result<Vec<RoleListItem>, ApiError> {
    async_http_and(
        App::TowerServer,
        "get",
        &ManagementResource::Role.path(ApiMethod::List),
        empty_req(),
    )
    .await
}

pub async fn async_add_role(req: AddRoleReq) -> Result<(), ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::Role.path(ApiMethod::Insert),
        req,
    )
    .await
}
