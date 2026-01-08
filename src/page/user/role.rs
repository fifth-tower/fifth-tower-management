use std::collections::HashSet;

use leptos::{logging::log, prelude::*, task::spawn_local};
use tower::{
    management_model::{ManagementResource, SetUserRolesReq},
    tauri_web::prelude::*,
    web::component::Checkbox,
};

use crate::{
    page::AddRoleForm,
    service::{async_get_roles, async_get_user_roles},
};

#[component]
pub fn RolePane(user_id: RwSignal<Option<String>>) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let checked_items = RwSignal::new(HashSet::<RoleCheckItem>::new());
    let roles_resource = LocalResource::new(move || async_get_roles());
    let user_roles_resource = LocalResource::new(move || {
        checked_items.update_untracked(|values| {
            values.clear();
        });
        let user_id = user_id.get();
        async_get_user_roles(user_id)
    });
    let open_add = RwSignal::new(false);
    view! {
        <div class="flex items-center">
            <label class="menu-title">角色</label>
            <div>
                <button
                    class="btn btn-xs"
                    on:click=move |_| {
                        user_roles_resource.refetch();
                    }
                >
                    刷新
                </button>
                <button
                    class="btn btn-xs"
                    on:click=move |_| {
                        open_add.set(true);
                    }
                >
                    新增
                </button>
                <button
                    class="btn btn-xs"
                    disabled=move || user_id.get().is_none()
                    on:click=move |_| {
                        spawn_local(async move {
                            let resp: Result<(), ApiError> = async_http_and(
                                    App::TowerServer,
                                    "put",
                                    &ManagementResource::UserRole.path(ApiMethod::Batch),
                                    SetUserRolesReq {
                                        user_id: user_id.get_untracked().unwrap(),
                                        role_ids: checked_items
                                            .get_untracked()
                                            .iter()
                                            .map(|item| item.role_id.clone())
                                            .collect(),
                                    },
                                )
                                .await;
                            tip_or(
                                resp,
                                app_state.op_tip,
                                |_| {
                                    app_state.success("设置成功。");
                                },
                            );
                        })
                    }
                >
                    保存
                </button>
            </div>
        </div>
        <Suspense fallback=move || {
            view! { <span class="self-center loading loading-spinner loading-xl"></span> }
        }>
            <fieldset class="p-4 w-full fieldset">
                <div class="flex flex-wrap gap-4">
                    {move || Suspend::new(async move {
                        let roles = roles_resource.await;
                        let user_roles = user_roles_resource.await;
                        if roles.is_err() {
                            return view! { "获取角色失败" }.into_any();
                        }
                        if user_roles.is_err() {
                            return view! { "获取用户角色失败" }.into_any();
                        }
                        let user_roles = user_roles.unwrap();
                        let roles = roles.unwrap();
                        roles
                            .iter()
                            .map(|role| {
                                let item = RoleCheckItem {
                                    role_id: role.role_id.clone(),
                                    role_name: role.role_name.clone(),
                                };
                                if user_roles.contains(&role.role_id) {
                                    checked_items
                                        .update_untracked(|values| { values.insert(item.clone()) });
                                }
                                view! {
                                    <Checkbox name="user_role" value=item checked=checked_items />
                                }
                            })
                            .collect_view()
                            .into_any()
                    })}
                </div>
            </fieldset>
        </Suspense>
        <Dialog open=open_add title="新增角色">
            <AddRoleForm open=open_add roles_resource />
        </Dialog>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RoleCheckItem {
    pub role_id: String,
    pub role_name: String,
}
impl Labelable for RoleCheckItem {
    fn label(&self) -> String {
        self.role_name.clone()
    }
}
impl ToString for RoleCheckItem {
    fn to_string(&self) -> String {
        self.role_id.clone()
    }
}
