use tower::{
    management_model::{
        GetUserVipLevelReq, GetUserVipLevelResp, ManagementResource, SaveUserVipLevelReq,
    },
    tauri_web::prelude::*,
};

pub async fn async_get_user_vip_level(
    user_id: Option<String>,
) -> Result<GetUserVipLevelResp, ApiError> {
    if user_id.is_none() {
        return Ok(GetUserVipLevelResp::default());
    }
    let req = GetUserVipLevelReq {
        user_id: user_id.unwrap(),
    };
    async_http_and(
        App::TowerServer,
        "get",
        &ManagementResource::UserVipLevel.path(ApiMethod::LoadById(req.user_id.clone())),
        req,
    )
    .await
}

pub async fn async_save_user_vip_level(req: SaveUserVipLevelReq) -> Result<(), ApiError> {
    async_http_bin_and(
        App::TowerServer,
        "post",
        &ManagementResource::UserVipLevel.path(ApiMethod::Insert),
        req,
    )
    .await
}
