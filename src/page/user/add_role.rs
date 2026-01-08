use leptos::{prelude::*, task::spawn_local};
use tower::{
    management_model::{AddRoleReq, RoleListItem},
    tauri_web::prelude::*,
};

use crate::service::async_add_role;

#[component]
pub fn AddRoleForm(
    open: RwSignal<bool>,
    roles_resource: WebResult<Vec<RoleListItem>>,
) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let form = AddRoleFormData::new();
    let op_tiper = OpTiper::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let req = form.to_req();
            spawn_local(async move {
                let resp = async_add_role(req).await;
                tip_or(
                    resp,
                    op_tiper.0,
                    |_| {
                        roles_resource.refetch();
                        open.set(false);
                        app_state.success("保存角色成功。");
                    },
                );
            });
        }>
            <fieldset class="p-4 w-full fieldset">
                <label class="label">角色编码</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="6"
                    required
                    bind:value=form.role_id
                />
                <p class="validator-hint">必须输入</p>
                <label class="label">角色名称</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="6"
                    required
                    bind:value=form.role_name
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

struct AddRoleFormData {
    role_id: RwSignal<String>,
    role_name: RwSignal<String>,
    remark: RwSignal<String>,
}
impl AddRoleFormData {
    fn new() -> Self {
        Self {
            role_id: RwSignal::new("".into()),
            role_name: RwSignal::new("".into()),
            remark: RwSignal::new("".into()),
        }
    }
    fn to_req(&self) -> AddRoleReq {
        AddRoleReq {
            role_id: self.role_id.get_untracked(),
            role_name: self.role_name.get_untracked(),
            remark: Some(self.remark.get_untracked()),
        }
    }
}
