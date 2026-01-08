use std::collections::HashSet;

use leptos::{prelude::*, task::spawn_local};
use tower::{
    management_model::{ConfigListItem, ConfigListReq},
    tauri_web::prelude::*,
    web::component::Checkboxs,
};

use crate::{
    page::component::AppSelect,
    service::{async_delete_config, async_get_configs},
};

use super::*;

#[component]
pub fn ConfigPane() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let query_form = ConfigFormData::new();
    let configs_resource = LocalResource::new(move || async_get_configs(query_form.to_req()));
    let current_config = RwSignal::new(None);
    let current_clone = RwSignal::new(None);
    let open_add = RwSignal::new(false);
    let open_update = RwSignal::new(false);
    view! {
        <div class="flex gap-4">
            <AppSelect selected=query_form.app />
            <label class="floating-label">
                <span>名称</span>
                <input
                    type="text"
                    placeholder="配置名称"
                    class="input input-md"
                    bind:value=query_form.var_name
                />
            </label>
            <Checkboxs
                name="config_status"
                values=YesNo::all()
                checked=query_form.status
                multi=false
            />
            <button
                class="btn"
                on:click=move |_| {
                    configs_resource.refetch();
                }
            >
                查询
            </button>
            <button
                class="btn"
                on:click=move |_| {
                    query_form.reset();
                }
            >
                重置
            </button>
            <button
                class="btn"
                on:click=move |_| {
                    current_clone.set(None);
                    open_add.set(true);
                }
            >
                新增
            </button>
        </div>
        <table class="table">
            <thead>
                <tr>
                    <th></th>
                    <th>应用</th>
                    <th>名称</th>
                    <th>值</th>
                    <th>是否公开</th>
                    <th>备注</th>
                    <th>状态</th>
                    <th>创建时间</th>
                    <th>更新时间</th>
                    <th>操作</th>
                </tr>
            </thead>
            <tbody>
                <Suspense fallback=move || {
                    view! { <span class="self-center loading loading-spinner loading-xl"></span> }
                }>
                    {move || {
                        configs_resource
                            .to_view(|list| {
                                list.into_iter()
                                    .enumerate()
                                    .map(|(index, config)| {
                                        let config = config.clone();
                                        view! {
                                            <tr>
                                                <th>{index + 1}</th>
                                                <td>{config.app.label()}</td>
                                                <td>{config.var_name.clone()}</td>
                                                <td>{config.var_value.clone()}</td>
                                                <td>{config.public.label()}</td>
                                                <td>{config.remark.clone()}</td>
                                                <td>{config.status.label()}</td>
                                                <td>{from_secs_and_default(config.created)}</td>
                                                <td>
                                                    {config
                                                        .updated
                                                        .map_or(
                                                            "".to_string(),
                                                            |updated| { from_secs_and_default(updated) },
                                                        )}
                                                </td>
                                                <td class="flex gap-4">
                                                    <CloneButton current_clone open_add config=config.clone() />
                                                    <UpdateButton
                                                        current_config
                                                        open_update
                                                        config=config.clone()
                                                    />
                                                    <ConfirmDialogButton
                                                        btn="删除"
                                                        btn_class="btn-xs"
                                                        title="删除配置"
                                                        message="确认删除？"
                                                        on_ok=move || {
                                                            spawn_local(async move {
                                                                match async_delete_config(config.id).await {
                                                                    Ok(_) => app_state.success("删除成功"),
                                                                    Err(err) => app_state.error(err.to_string()),
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                            </tr>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            })
                    }}
                </Suspense>
            </tbody>
        </table>
        <Show when=move || { open_add.get() }>
            <Dialog open=open_add title="新增配置">
                <AddConfigForm open=open_add configs_resource current_clone />
            </Dialog>
        </Show>
        <Show when=move || { current_config.get().is_some() && open_update.get() }>
            <Dialog open=open_update title="修改配置">
                <UpdateConfigForm open=open_update configs_resource config=current_config />
            </Dialog>
        </Show>
    }
}

#[component]
pub fn CloneButton(
    #[prop(into)] config: Signal<ConfigListItem>,
    open_add: RwSignal<bool>,
    current_clone: RwSignal<Option<ConfigListItem>>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-xs"
            on:click=move |_| {
                current_clone.set(Some(config.get_untracked()));
                open_add.set(true)
            }
        >
            克隆
        </button>
    }
}

#[component]
pub fn UpdateButton(
    #[prop(into)] config: Signal<ConfigListItem>,
    open_update: RwSignal<bool>,
    current_config: RwSignal<Option<ConfigListItem>>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-xs"
            on:click=move |_| {
                current_config.set(Some(config.get_untracked()));
                open_update.set(true)
            }
        >
            修改
        </button>
    }
}

#[derive(Debug, Clone, Copy)]
struct ConfigFormData {
    app: RwSignal<App>,
    var_name: RwSignal<String>,
    status: RwSignal<HashSet<YesNo>>,
}

impl ConfigFormData {
    pub fn new() -> Self {
        Self {
            app: RwSignal::new(App::TowerServer),
            var_name: RwSignal::new("".to_string()),
            status: RwSignal::new(HashSet::new()),
        }
    }
    pub fn reset(&self) {
        self.app.set(App::TowerServer);
        self.var_name.set("".to_string());
        self.status.set(HashSet::new());
    }
    pub fn to_req(&self) -> ConfigListReq {
        ConfigListReq {
            app_id: self.app.get_untracked().label(),
            var_name: self.var_name.get_untracked(),
            status: self.status.get_untracked().iter().next().copied(),
        }
    }
}
