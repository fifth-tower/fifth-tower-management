use leptos::{prelude::*, task::spawn_local};
use tower_common::{bincode_encode, user::RegisterReq};
use tower_web::component::OpTip;
use tower_web_model::{OpTipData, OpTipType, Tipable};

use crate::{service::async_register, AppState};

#[component]
pub fn RegisterForm(open: RwSignal<bool>) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let form = RegisterFormData::new();
    let op_tip = RwSignal::new(None);
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            if !form.password_valid() {
                op_tip
                    .set(
                        Some(OpTipData {
                            typ: OpTipType::Warning,
                            class: "".into(),
                            content: "两次输入的密码不一致".into(),
                        }),
                    );
                return;
            }
            let data = form.to_data();
            spawn_local(async move {
                let resp = async_register(data).await;
                match resp {
                    Ok(_) => {
                        app_state.tip(OpTipType::Success, "注册成功");
                        open.set(false);
                    }
                    Err(err) => {
                        op_tip
                            .set(
                                Some(OpTipData {
                                    typ: OpTipType::Error,
                                    class: "".into(),
                                    content: err.to_string(),
                                }),
                            );
                    }
                }
            });
        }>
            <fieldset class="p-4 w-full fieldset">
                <label class="label">用户名</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="6"
                    maxlength="16"
                    required
                    bind:value=form.username
                />
                <p class="validator-hint">"必须输入，6~16位"</p>
                <label class="label">昵称</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="3"
                    maxlength="12"
                    required
                    bind:value=form.nickname
                />
                <p class="validator-hint">"必须输入，3~12位"</p>
                <label class="label">密码</label>
                <input
                    type="password"
                    class="w-full input validator"
                    minlength="6"
                    maxlength="16"
                    pattern="^(?=.*\\d)(?=.*[a-zA-Z])(?=.*[^\\da-zA-Z\\s]).{6,16}$"
                    required
                    bind:value=form.password
                />
                <p class="validator-hint">"至少包含字母、数字、特殊字符，6~16位"</p>
                <label class="label">重复密码</label>
                <input
                    type="password"
                    class="w-full input validator"
                    minlength="6"
                    maxlength="16"
                    pattern="^(?=.*\\d)(?=.*[a-zA-Z])(?=.*[^\\da-zA-Z\\s]).{6,16}$"
                    required
                    bind:value=form.password2
                />
                <p class="validator-hint">"至少包含字母、数字、特殊字符，6~16位"</p>
                <OpTip content=op_tip />
                <button class="mt-4 btn btn-neutral" type="submit">
                    注册
                </button>
            </fieldset>
        </form>
    }
}

struct RegisterFormData {
    username: RwSignal<String>,
    password: RwSignal<String>,
    password2: RwSignal<String>,
    nickname: RwSignal<String>,
}
impl RegisterFormData {
    fn new() -> Self {
        Self {
            username: RwSignal::new("".into()),
            password: RwSignal::new("".into()),
            password2: RwSignal::new("".into()),
            nickname: RwSignal::new("".into()),
        }
    }
    fn password_valid(&self) -> bool {
        let password = self.password.get_untracked();
        let password2 = self.password2.get_untracked();
        !password.is_empty() && password == password2
    }
    fn to_data(&self) -> RegisterReq {
        let username = self.username.get_untracked();
        let password = self.password.get_untracked();
        let nickname = self.nickname.get_untracked();
        RegisterReq {
            username,
            password: bincode_encode(password),
            nickname,
            avatar: 0,
        }
    }
}
