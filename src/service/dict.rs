use tower::{
    common::{ApiMethod, App},
    management_model::{AddDictReq, DictListItem, ManagementResource, UpdateDictReq},
    tauri_web::prelude::*,
};

pub async fn async_get_dicts() -> Result<DictListItem, ApiError> {
    async_http_and(
        App::TowerServer,
        "get",
        &ManagementResource::Dict.path(ApiMethod::List),
        empty_req(),
    )
    .await
    .map(|dicts| list_to_tree(dicts))
}

pub async fn async_add_dict(req: AddDictReq) -> Result<(), ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::Dict.path(ApiMethod::Insert),
        req,
    )
    .await
}

pub async fn async_update_dict(req: UpdateDictReq) -> Result<(), ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::Dict.path(ApiMethod::UpdateById),
        req,
    )
    .await
}
