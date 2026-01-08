use leptos::{prelude::*, task::spawn_local};
use tower::{
    management_model::{AddGroupReq, GroupListItem},
    tauri_web::prelude::*,
};

use crate::service::async_add_group;

#[component]
pub fn AddGroupForm(
    open: RwSignal<bool>,
    groups_resource: WebResult<Vec<GroupListItem>>,
) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let form = AddGroupFormData::new();
    let op_tiper = OpTiper::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let req = form.to_req();
            spawn_local(async move {
                let resp = async_add_group(req).await;
                tip_or(
                    resp,
                    op_tiper.0,
                    |_| {
                        groups_resource.refetch();
                        open.set(false);
                        app_state.success("保存用户组成功。");
                    },
                );
            });
        }>
            <fieldset class="p-4 w-full fieldset">
                <label class="label">用户组编码</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="6"
                    required
                    bind:value=form.group_id
                />
                <p class="validator-hint">必须输入</p>
                <label class="label">用户组名称</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="6"
                    required
                    bind:value=form.group_name
                />
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

struct AddGroupFormData {
    group_id: RwSignal<String>,
    group_name: RwSignal<String>,
    remark: RwSignal<String>,
}
impl AddGroupFormData {
    fn new() -> Self {
        Self {
            group_id: RwSignal::new("".into()),
            group_name: RwSignal::new("".into()),
            remark: RwSignal::new("".into()),
        }
    }
    fn to_req(&self) -> AddGroupReq {
        AddGroupReq {
            group_id: self.group_id.get_untracked(),
            group_name: self.group_name.get_untracked(),
            remark: Some(self.remark.get_untracked()),
        }
    }
}
