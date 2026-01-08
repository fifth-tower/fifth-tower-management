use tower::{
    common::{ApiMethod, App},
    management_model::{
        AddDictItemReq, DictItemListItem, DictItemListReq, ManagementResource, UpdateDictItemReq,
    },
    tauri_web::prelude::*,
};

pub async fn async_get_dict_items(
    dict_code: Option<String>,
) -> Result<Vec<DictItemListItem>, ApiError> {
    if dict_code.is_none() {
        return Ok(vec![]);
    }
    async_http_and(
        App::TowerServer,
        "get",
        &ManagementResource::DictItem.path(ApiMethod::List),
        DictItemListReq {
            dict_code: dict_code.unwrap(),
        },
    )
    .await
}

pub async fn async_add_dict_item(req: AddDictItemReq) -> Result<(), ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::DictItem.path(ApiMethod::Insert),
        req,
    )
    .await
}

pub async fn async_update_dict_item(req: UpdateDictItemReq) -> Result<(), ApiError> {
    async_http_and(
        App::TowerServer,
        "post",
        &ManagementResource::DictItem.path(ApiMethod::UpdateById),
        req,
    )
    .await
}
