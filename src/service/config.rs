use tower::{
    common::{ApiMethod, App},
    management_model::{
        AddConfigReq, ConfigListItem, ConfigListReq, ManagementResource, UpdateConfigReq,
    },
    tauri_web::prelude::*,
};

pub async fn async_get_configs(req: ConfigListReq) -> Result<Vec<ConfigListItem>, ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::Config.path(ApiMethod::List),
        req,
    )
    .await
}

pub async fn async_add_config(req: AddConfigReq) -> Result<(), ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::Config.path(ApiMethod::Insert),
        req,
    )
    .await
}

pub async fn async_update_config(req: UpdateConfigReq) -> Result<(), ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::Config.path(ApiMethod::UpdateById),
        req,
    )
    .await
}

pub async fn async_delete_config(id: i32) -> Result<(), ApiError> {
    async_http_and(
        App::TowerServer,
        "delete",
        &ManagementResource::Config.path(ApiMethod::DeleteById(id.to_string())),
        empty_req(),
    )
    .await
}
