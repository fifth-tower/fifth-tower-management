use std::collections::HashSet;

use leptos::{prelude::*, task::spawn_local};
use tower::{
    management_model::{ManagementResource, SetUserAppsReq},
    tauri_web::prelude::*,
    web::component::Checkbox,
};

use crate::service::async_get_user_apps;

#[component]
pub fn AppPane(user_id: RwSignal<Option<String>>) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let checked_items = RwSignal::new(HashSet::<AppCheckItem>::new());
    let user_apps_resource = LocalResource::new(move || {
        checked_items.update_untracked(|values| {
            values.clear();
        });
        let user_id = user_id.get();
        async_get_user_apps(user_id)
    });
    view! {
        <div class="flex items-center">
            <label class="menu-title">应用</label>
            <div>
                <button
                    class="btn btn-xs"
                    on:click=move |_| {
                        user_apps_resource.refetch();
                    }
                >
                    刷新
                </button>
                <button
                    class="btn btn-xs"
                    disabled=move || user_id.get().is_none()
                    on:click=move |_| {
                        spawn_local(async move {
                            let resp: Result<(), ApiError> = async_http_and(
                                    App::TowerServer,
                                    "put",
                                    &ManagementResource::UserApp.path(ApiMethod::Batch),
                                    SetUserAppsReq {
                                        user_id: user_id.get_untracked().unwrap(),
                                        app_ids: checked_items
                                            .get_untracked()
                                            .iter()
                                            .map(|item| item.app_id.clone())
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
                        let user_apps = user_apps_resource.await;
                        if user_apps.is_err() {
                            return view! { "获取用户角色失败" }.into_any();
                        }
                        let user_apps = user_apps.unwrap();
                        let apps = App::all_tauri();
                        apps.iter()
                            .map(|(app, app_name)| {
                                let item = AppCheckItem {
                                    app_id: app.to_string(),
                                    app_name: app_name.clone(),
                                };
                                if user_apps.contains(&app.to_string()) {
                                    checked_items
                                        .update_untracked(|values| { values.insert(item.clone()) });
                                }
                                view! {
                                    <Checkbox name="user_app" value=item checked=checked_items />
                                }
                            })
                            .collect_view()
                            .into_any()
                    })}
                </div>
            </fieldset>
        </Suspense>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AppCheckItem {
    pub app_id: String,
    pub app_name: String,
}
impl Labelable for AppCheckItem {
    fn label(&self) -> String {
        self.app_name.clone()
    }
}
impl ToString for AppCheckItem {
    fn to_string(&self) -> String {
        self.app_id.clone()
    }
}
