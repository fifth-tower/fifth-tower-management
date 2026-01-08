use leptos::{prelude::*, task::spawn_local};
use tower::{
    management_model::{AddConfigReq, ConfigListItem},
    tauri_web::prelude::*,
};

use crate::{
    page::component::{AppSelect, StatusSelect},
    service::async_add_config,
};

#[component]
pub(crate) fn AddConfigForm(
    open: RwSignal<bool>,
    configs_resource: WebResult<Vec<ConfigListItem>>,
    current_clone: RwSignal<Option<ConfigListItem>>,
) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let form = AddConfigFormData::new(current_clone.get_untracked());
    let op_tiper = OpTiper::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let req = form.to_req();
            spawn_local(async move {
                let resp = async_add_config(req).await;
                tip_or(
                    resp,
                    op_tiper.0,
                    |_| {
                        configs_resource.refetch();
                        open.set(false);
                        app_state.success("新增配置成功。");
                    },
                );
            });
        }>
            <fieldset class="p-4 w-full fieldset">
                <label class="label">应用</label>
                <AppSelect selected=form.app />
                <p class="validator-hint">必须输入</p>
                <label class="label">配置名称</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="1"
                    maxlength="50"
                    required
                    bind:value=form.var_name
                />
                <p class="validator-hint">必须输入</p>
                <label class="label">配置值</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="1"
                    maxlength="50"
                    required
                    bind:value=form.var_value
                />
                <p class="validator-hint">必须输入</p>
                <label class="label">是否公共</label>
                <StatusSelect selected=form.public placeholder="是否公共" />
                <p class="validator-hint">必须输入</p>
                <label class="label">是否有效</label>
                <StatusSelect selected=form.status placeholder="是否有效" />
                <p class="validator-hint">必须输入</p>
                <label class="mt-4 mb-2 label">备注</label>
                <textarea
                    class="w-full textarea validator"
                    maxlength="500"
                    on:input:target=move |ev| { form.remark.set(ev.target().value()) }
                >
                    {move || form.remark.get()}
                </textarea>
                <OpTip content=op_tiper.0 />
                <button class="mt-4 btn btn-neutral" type="submit">
                    确定
                </button>
            </fieldset>
        </form>
    }
}

struct AddConfigFormData {
    app: RwSignal<App>,
    app_version: RwSignal<Option<String>>,
    var_name: RwSignal<String>,
    var_value: RwSignal<String>,
    public: RwSignal<YesNo>,
    remark: RwSignal<String>,
    status: RwSignal<YesNo>,
}
impl AddConfigFormData {
    fn new(clone: Option<ConfigListItem>) -> Self {
        if clone.is_none() {
            Self {
                app: RwSignal::new(App::TowerServer),
                app_version: RwSignal::new(None),
                var_name: RwSignal::new("".to_string()),
                var_value: RwSignal::new("".to_string()),
                public: RwSignal::new(YesNo::Yes),
                remark: RwSignal::new("".to_string()),
                status: RwSignal::new(YesNo::Yes),
            }
        } else {
            let clone = clone.unwrap();
            Self {
                app: RwSignal::new(clone.app),
                app_version: RwSignal::new(clone.app_version),
                var_name: RwSignal::new(clone.var_name),
                var_value: RwSignal::new(clone.var_value),
                public: RwSignal::new(clone.public),
                remark: RwSignal::new(clone.remark.unwrap_or_default()),
                status: RwSignal::new(clone.status),
            }
        }
    }
    fn to_req(&self) -> AddConfigReq {
        AddConfigReq {
            app_id: self.app.get_untracked().to_string(),
            app_version: self.app_version.get_untracked(),
            var_name: self.var_name.get_untracked(),
            var_value: self.var_value.get_untracked(),
            public: self.public.get_untracked(),
            remark: str_to_option(self.remark.get_untracked()),
            status: self.status.get_untracked(),
        }
    }
}
