use std::collections::HashSet;

use leptos::{prelude::*, task::spawn_local};
use tower::{
    management_model::{ManagementResource, SetUserGroupsReq},
    tauri_web::prelude::*,
    web::component::Checkbox,
};

use crate::{
    page::AddGroupForm,
    service::{async_get_groups, async_get_user_groups},
};

#[component]
pub fn GroupPane(user_id: RwSignal<Option<String>>) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let checked_items = RwSignal::new(HashSet::<GroupCheckItem>::new());
    let groups_resource = LocalResource::new(move || async_get_groups());
    let user_groups_resource = LocalResource::new(move || {
        checked_items.update_untracked(|values| {
            values.clear();
        });
        let user_id = user_id.get();
        async_get_user_groups(user_id)
    });
    let open_add = RwSignal::new(false);
    view! {
        <div class="flex items-center">
            <label class="menu-title">用户组</label>
            <div>
                <button
                    class="btn btn-xs"
                    on:click=move |_| {
                        user_groups_resource.refetch();
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
                                    &ManagementResource::UserGroup.path(ApiMethod::Batch),
                                    SetUserGroupsReq {
                                        user_id: user_id.get_untracked().unwrap(),
                                        group_ids: checked_items
                                            .get_untracked()
                                            .iter()
                                            .map(|item| item.group_id.clone())
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
                        let groups = groups_resource.await;
                        let user_groups = user_groups_resource.await;
                        if groups.is_err() {
                            return view! { "获取角色失败" }.into_any();
                        }
                        if user_groups.is_err() {
                            return view! { "获取用户角色失败" }.into_any();
                        }
                        let user_groups = user_groups.unwrap();
                        let groups = groups.unwrap();
                        groups
                            .iter()
                            .map(|group| {
                                let item = GroupCheckItem {
                                    group_id: group.group_id.clone(),
                                    group_name: group.group_name.clone(),
                                };
                                if user_groups.contains(&group.group_id) {
                                    checked_items
                                        .update_untracked(|values| { values.insert(item.clone()) });
                                }
                                view! {
                                    <Checkbox name="user_group" value=item checked=checked_items />
                                }
                            })
                            .collect_view()
                            .into_any()
                    })}
                </div>
            </fieldset>
        </Suspense>
        <Dialog open=open_add title="新增角色">
            <AddGroupForm open=open_add groups_resource />
        </Dialog>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GroupCheckItem {
    pub group_id: String,
    pub group_name: String,
}
impl Labelable for GroupCheckItem {
    fn label(&self) -> String {
        self.group_name.clone()
    }
}
impl ToString for GroupCheckItem {
    fn to_string(&self) -> String {
        self.group_id.clone()
    }
}

struct SetGroupsFormData {
    user_id: Signal<Option<String>>,
    group_ids: RwSignal<Vec<String>>,
}
impl SetGroupsFormData {
    fn new(user_id: Signal<Option<String>>) -> Self {
        Self {
            user_id,
            group_ids: RwSignal::new(vec![]),
        }
    }
    fn to_req(&self) -> SetUserGroupsReq {
        SetUserGroupsReq {
            user_id: self.user_id.get_untracked().unwrap(),
            group_ids: self.group_ids.get_untracked(),
        }
    }
}
