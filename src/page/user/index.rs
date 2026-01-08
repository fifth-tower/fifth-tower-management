use super::*;
use crate::service::async_get_paged_users;
use leptos::{prelude::*, task::spawn_local};
use tower::{
    management_model::{ManagementResource, ResetPasswordReq, SetStatusReq, UserListReq},
    tauri_web::prelude::*,
    web::{common::date::from_secs_and, component::Pagination},
};

#[component]
pub fn UserPane() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let page = RwSignal::new(Page::new(0, 20, UserListReq::new()));
    let query_form = UserQueryFormData::new();
    let users_resource = LocalResource::new(move || async_get_paged_users(page.get()));
    let current_user_id = RwSignal::new(None);
    move || {
        users_resource.to_view(|list| {
            view! {
                <table class="table">
                    <thead>
                        <tr>
                            <th></th>
                            <th>账号</th>
                            <th>状态</th>
                            <th>创建时间</th>
                            <th>更新时间</th>
                            <th>操作</th>
                        </tr>
                    </thead>
                    <tbody>
                        {list
                            .iter()
                            .enumerate()
                            .map(|(index, user)| {
                                let user_id: Signal<String> = Signal::from(user.user_id.clone());
                                let (status_btn, status) = if user.status == YesNo::Yes.val() {
                                    ("禁用", 1)
                                } else {
                                    ("启用", 0)
                                };
                                let status_btn: Signal<String> = Signal::from(
                                    status_btn.to_string(),
                                );
                                view! {
                                    <tr>
                                        <th>{index + 1}</th>
                                        <td>
                                            <div class="flex gap-3 items-center">
                                                <div class="avatar">
                                                    <div class="w-10 rounded-full">
                                                        <img src=to_avatar(user.avatar) />
                                                    </div>
                                                </div>
                                                <div>
                                                    <div class="font-bold">{user.username.clone()}</div>
                                                    <div class="text-sm opacity-50">
                                                        {user.nickname.clone()}
                                                    </div>
                                                </div>
                                            </div>
                                        </td>
                                        <td>
                                            {YesNo::try_from(user.status).unwrap_or(YesNo::No).label()}
                                        </td>
                                        <td>
                                            {from_secs_and(user.created, "yyyy-mm-dd hh:mi:ss")}
                                        </td>
                                        <td>
                                            {user
                                                .updated
                                                .map_or(
                                                    "".to_string(),
                                                    |updated| { from_secs_and(updated, "yyyy-mm-dd hh:mi:ss") },
                                                )}
                                        </td>
                                        <td>
                                            <button
                                                class="btn btn-xs"
                                                on:click=move |_| {
                                                    current_user_id.set(Some(user_id.get_untracked()));
                                                }
                                            >
                                                配置
                                            </button>
                                            <button
                                                class="btn btn-xs"
                                                on:click=move |_| {
                                                    spawn_local(async move {
                                                        let resp: Result<(), ApiError> = async_http_and(
                                                                App::TowerServer,
                                                                "post",
                                                                &ManagementResource::User.path("/reset_password"),
                                                                ResetPasswordReq {
                                                                    user_id: user_id.get_untracked(),
                                                                },
                                                            )
                                                            .await;
                                                        tip_or(
                                                            resp,
                                                            app_state.op_tip,
                                                            |_| {
                                                                app_state.success("重置密码成功。");
                                                            },
                                                        );
                                                    });
                                                }
                                            >
                                                重置密码
                                            </button>
                                            <button
                                                class="btn btn-xs"
                                                on:click=move |_| {
                                                    spawn_local(async move {
                                                        let resp: Result<(), ApiError> = async_http_and(
                                                                App::TowerServer,
                                                                "post",
                                                                &ManagementResource::User.path("/status"),
                                                                SetStatusReq {
                                                                    user_id: user_id.get_untracked(),
                                                                    status,
                                                                },
                                                            )
                                                            .await;
                                                        tip_or(
                                                            resp,
                                                            app_state.op_tip,
                                                            |_| {
                                                                users_resource.refetch();
                                                                app_state
                                                                    .success(
                                                                        format!("{}成功。", status_btn.get_untracked()),
                                                                    );
                                                            },
                                                        );
                                                    });
                                                }
                                            >
                                                {status_btn.get_untracked()}
                                            </button>
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
                <Pagination value=page />
                <AppPane user_id=current_user_id />
                <RolePane user_id=current_user_id />
                <GroupPane user_id=current_user_id />
                <VipLevelPane user_id=current_user_id />
            }
            .into_any()
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct UserQueryFormData {
    pub username: RwSignal<String>,
    pub nickname: RwSignal<String>,
    pub status: RwSignal<Option<YesNo>>,
}

impl UserQueryFormData {
    fn new() -> Self {
        Self {
            username: RwSignal::new("".into()),
            nickname: RwSignal::new("".into()),
            status: RwSignal::new(None),
        }
    }

    fn to_req(&self) -> UserListReq {
        UserListReq {
            username: self.username.get_untracked().trim().to_string(),
            nickname: self.nickname.get_untracked().trim().to_string(),
            status: self.status.get_untracked(),
        }
    }
}
