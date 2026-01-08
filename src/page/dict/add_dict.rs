use leptos::{prelude::*, task::spawn_local};
use tower::{
    management_model::{AddDictReq, DictListItem},
    tauri_web::prelude::*,
    web_model::common::str_trim_to_option,
};

use crate::{
    page::{component::StatusSelect, DictState},
    service::async_add_dict,
};

#[component]
pub(crate) fn AddDictForm(
    open: RwSignal<bool>,
    dict_resource: WebResult<DictListItem>,
) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let dict_state = expect_context::<DictState>();
    let form = AddDictFormData::new(
        dict_state
            .target_dict
            .get_untracked()
            .map(|target| target.dict_code),
    );
    let op_tiper = OpTiper::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let req = form.to_req();
            spawn_local(async move {
                let resp = async_add_dict(req).await;
                tip_or(
                    resp,
                    op_tiper.0,
                    |_| {
                        dict_resource.refetch();
                        open.set(false);
                        app_state.success("新增字典成功。");
                    },
                );
            });
        }>
            <fieldset class="p-4 w-full fieldset">
                <label class="label">代码</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="1"
                    maxlength="50"
                    required
                    bind:value=form.dict_code
                />
                <p class="validator-hint">必须输入</p>
                <label class="label">名称</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="1"
                    maxlength="50"
                    required
                    bind:value=form.dict_name
                />
                <p class="validator-hint">必须输入</p>
                <label class="label">上级代码</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="1"
                    maxlength="50"
                    disabled
                    required
                    prop:value=form.parent_code
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

struct AddDictFormData {
    dict_code: RwSignal<String>,
    dict_name: RwSignal<String>,
    parent_code: RwSignal<String>,
    public: RwSignal<YesNo>,
    remark: RwSignal<String>,
    status: RwSignal<YesNo>,
}
impl AddDictFormData {
    fn new(parent_code: Option<String>) -> Self {
        Self {
            dict_code: RwSignal::new("".to_string()),
            dict_name: RwSignal::new("".to_string()),
            parent_code: RwSignal::new(parent_code.unwrap_or_default()),
            public: RwSignal::new(YesNo::Yes),
            remark: RwSignal::new("".to_string()),
            status: RwSignal::new(YesNo::Yes),
        }
    }
    fn to_req(&self) -> AddDictReq {
        AddDictReq {
            dict_code: self.dict_code.get_untracked(),
            dict_name: self.dict_name.get_untracked(),
            parent_code: str_trim_to_option(self.parent_code.get_untracked()),
            public: self.public.get_untracked(),
            remark: str_trim_to_option(self.remark.get_untracked()),
            status: self.status.get_untracked(),
        }
    }
}
