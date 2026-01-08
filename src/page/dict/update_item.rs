use leptos::{prelude::*, task::spawn_local};
use tower::{
    management_model::{DictItemListItem, UpdateDictItemReq},
    tauri_web::prelude::*,
};

use crate::{page::component::StatusSelect, service::async_update_dict_item};

#[component]
pub(crate) fn UpdateItemForm(
    open: RwSignal<bool>,
    item_resource: WebResult<Vec<DictItemListItem>>,
    item: RwSignal<Option<DictItemListItem>>,
) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let form = UpdateItemFormData::new(item);
    let op_tiper = OpTiper::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let req = form.to_req();
            spawn_local(async move {
                let resp = async_update_dict_item(req).await;
                tip_or(
                    resp,
                    op_tiper.0,
                    |_| {
                        item_resource.refetch();
                        open.set(false);
                        app_state.success("修改字典项成功。");
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
                    disabled
                    required
                    prop:value=form.dict_code
                />
                <p class="validator-hint">必须输入</p>
                <label class="label">名称</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="1"
                    maxlength="50"
                    required
                    bind:value=form.item_name
                />
                <p class="validator-hint">必须输入</p>
                <label class="label">值</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="1"
                    maxlength="50"
                    required
                    bind:value=form.item_value
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

struct UpdateItemFormData {
    id: Signal<i32>,
    dict_code: Signal<String>,
    item_name: RwSignal<String>,
    item_value: RwSignal<String>,
    public: RwSignal<YesNo>,
    remark: RwSignal<String>,
    status: RwSignal<YesNo>,
}
impl UpdateItemFormData {
    fn new(item: RwSignal<Option<DictItemListItem>>) -> Self {
        let item = item.get_untracked().unwrap();
        Self {
            id: Signal::from(item.id),
            dict_code: Signal::from(item.dict_code),
            item_name: RwSignal::new(item.item_name),
            item_value: RwSignal::new(item.item_value),
            public: RwSignal::new(item.public),
            remark: RwSignal::new(item.remark.unwrap_or_default()),
            status: RwSignal::new(item.status),
        }
    }
    fn to_req(&self) -> UpdateDictItemReq {
        UpdateDictItemReq {
            id: self.id.get_untracked(),
            item_name: self.item_name.get_untracked(),
            item_value: self.item_value.get_untracked(),
            public: self.public.get_untracked(),
            remark: str_trim_to_option(self.remark.get_untracked()),
            status: self.status.get_untracked(),
        }
    }
}
