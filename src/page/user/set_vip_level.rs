use leptos::{prelude::*, task::spawn_local};
use tower::{
    common::dict::VipLevel,
    management_model::{GetUserVipLevelResp, SaveUserVipLevelReq},
    tauri_web::prelude::*,
};

use crate::service::async_save_user_vip_level;

#[component]
pub fn SetVipLevelForm(
    open: RwSignal<bool>,
    user_id: RwSignal<Option<String>>,
    user_vip_level_resource: WebResult<GetUserVipLevelResp>,
) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let form = SetVipLevelFormData::new();
    let op_tiper = OpTiper::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            spawn_local(async move {
                let req = form.to_req(user_id);
                let resp = async_save_user_vip_level(req).await;
                tip_or(
                    resp,
                    op_tiper.0,
                    |_| {
                        user_vip_level_resource.refetch();
                        open.set(false);
                        app_state.success("升级成功。");
                    },
                );
            });
        }>
            <fieldset class="p-4 w-full fieldset">
                <label class="label">等级</label>
                <select
                    on:change:target=move |ev| {
                        form.vip_level.set(ev.target().value().try_into().unwrap());
                    }
                    prop:value=move || form.vip_level.get().to_string()
                >
                    {VipLevel::all()
                        .iter()
                        .filter(|i| VipLevel::Vip0.ne(i))
                        .map(|i| {
                            view! { <option value=i.to_string()>{i.to_string()}</option> }
                        })
                        .collect_view()}
                </select>
                <p class="validator-hint">必须输入</p>
                <label class="label">激活时长</label>
                <span class="flex gap-4">
                    <span>
                        <select
                            on:change:target=move |ev| {
                                form.months.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=move || form.months.get().to_string()
                        >
                            {(0..13)
                                .map(|i| {
                                    view! { <option value=i>{i}</option> }
                                })
                                .collect_view()}
                        </select>
                        月
                    </span>
                    <span>
                        <select
                            on:change:target=move |ev| {
                                form.days.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=move || form.days.get().to_string()
                        >
                            {(0..15)
                                .map(|i| {
                                    view! { <option value=i>{i}</option> }
                                })
                                .collect_view()}
                        </select>
                        日
                    </span>
                </span>
                <OpTip content=op_tiper.0 />
                <button class="mt-4 btn btn-neutral" type="submit">
                    确定
                </button>
            </fieldset>
        </form>
    }
}

#[derive(Debug, Clone, Copy)]
struct SetVipLevelFormData {
    vip_level: RwSignal<VipLevel>,
    months: RwSignal<u32>,
    days: RwSignal<u64>,
}
impl SetVipLevelFormData {
    fn new() -> Self {
        Self {
            vip_level: RwSignal::new(VipLevel::default()),
            months: RwSignal::new(0),
            days: RwSignal::new(0),
        }
    }
    fn to_req(&self, user_id: RwSignal<Option<String>>) -> SaveUserVipLevelReq {
        SaveUserVipLevelReq {
            user_id: user_id.get_untracked().unwrap(),
            vip_level: self.vip_level.get_untracked(),
            months: self.months.get_untracked(),
            days: self.days.get_untracked(),
        }
    }
}
