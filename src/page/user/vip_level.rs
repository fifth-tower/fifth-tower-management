use leptos::prelude::*;
use tower::{tauri_web::prelude::*, web::common::date::from_secs_and_default};

use crate::{page::SetVipLevelForm, service::async_get_user_vip_level};

#[component]
pub fn VipLevelPane(user_id: RwSignal<Option<String>>) -> impl IntoView {
    let user_vip_level_resource =
        LocalResource::new(move || async_get_user_vip_level(user_id.get()));
    let open_vip = RwSignal::new(false);
    view! {
        <div class="flex items-center">
            <label class="menu-title">会员等级</label>
            <div>
                <button
                    class="btn btn-xs"
                    on:click=move |_| {
                        user_vip_level_resource.refetch();
                    }
                >
                    刷新
                </button>
                <button
                    class="btn btn-xs"
                    disabled=move || user_id.get().is_none()
                    on:click=move |_| {
                        open_vip.set(true);
                    }
                >
                    升级
                </button>
            </div>
        </div>
        <Suspense fallback=move || {
            view! { <span class="self-center loading loading-spinner loading-xl"></span> }
        }>
            <fieldset class="p-4 w-full fieldset">
                <div class="flex flex-wrap gap-4">
                    {move || {
                        user_vip_level_resource
                            .to_view(|vip| {
                                view! {
                                    <span>{vip.level_code.to_string()}</span>
                                    {vip
                                        .status
                                        .eq(&YesNo::Yes)
                                        .then(|| {
                                            Some(
                                                view! {
                                                    <span>{from_secs_and_default(vip.expried)}" 过期"</span>
                                                },
                                            )
                                        })}
                                }
                                    .into_any()
                            })
                    }}
                </div>
            </fieldset>
        </Suspense>
        <Dialog open=open_vip title="升级vip">
            <SetVipLevelForm open=open_vip user_vip_level_resource user_id />
        </Dialog>
    }
}
