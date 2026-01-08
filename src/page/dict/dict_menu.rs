use leptos::{logging::log, prelude::*};
use tower::{management_model::DictListItem, web::common::WebResultExt};

use crate::{
    page::{AddDictForm, DictState, UpdateDictForm},
    service::async_get_dicts,
};
use tower::tauri_web::prelude::*;

#[component]
pub fn DictMenu(#[prop(into,default="".into())] class: String) -> impl IntoView {
    let state = expect_context::<DictState>();
    let dict_resource = LocalResource::new(move || async_get_dicts());
    view! {
        <Suspense fallback=move || {
            view! { <span class="self-center loading loading-spinner loading-xl"></span> }
        }>
            {move || {
                dict_resource
                    .to_view(|dict| {
                        view! {
                            <ul class=format!("menu rounded-box  {}", class)>
                                <li>
                                    <span>
                                        <button
                                            class="btn btn-xs"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                state.target_dict.set(None);
                                                state.open_add_dict.set(true);
                                            }
                                        >
                                            新增子项
                                        </button>
                                    </span>
                                </li>
                                {dict
                                    .children
                                    .iter()
                                    .map(|dict| {
                                        view! { <DictLi dict=dict.to_owned() /> }
                                    })
                                    .collect_view()}
                            </ul>
                        }
                            .into_any()
                    })
            }}
        </Suspense>

        <Show when=move || { state.open_add_dict.get() } fallback=|| view! {}>
            <Dialog open=state.open_add_dict title="新增字典项">
                <AddDictForm open=state.open_add_dict dict_resource />
            </Dialog>
        </Show>
        <Show
            when=move || { state.target_dict.get().is_some() && state.open_update_dict.get() }
            fallback=|| view! {}
        >
            <Dialog open=state.open_update_dict title="修改字典项">
                <UpdateDictForm open=state.open_update_dict dict_resource />
            </Dialog>
        </Show>
    }
}

#[component]
fn DictLi(#[prop(into)] dict: Signal<DictListItem>) -> impl IntoView {
    let dict = dict.get_untracked();
    if dict.children.is_empty() {
        view! {
            <li>
                <Dict data=dict />
            </li>
        }
        .into_any()
    } else {
        view! {
            <li>
                <ParentDict dict />
            </li>
        }
        .into_any()
    }
}

#[component]
fn ParentDict(#[prop(into)] dict: Signal<DictListItem>) -> impl IntoView {
    view! {
        <details open>
            <summary>
                <Dict data=dict.get_untracked() />
            </summary>
            <ul>
                {dict
                    .get_untracked()
                    .children
                    .into_iter()
                    .map(|dict| {
                        view! { <DictLi dict /> }
                    })
                    .collect_view()}
            </ul>
        </details>
    }
}

#[component]
fn Dict(#[prop(into)] data: Signal<DictListItem>) -> impl IntoView {
    let state = expect_context::<DictState>();
    view! {
        <span
            class="group"
            on:click=move |_| {
                state.current_dict.set(Some(data.get_untracked()));
            }
        >
            {data.get_untracked().dict_name}
            <span class="hidden gap-2 justify-end group-hover:flex">
                <button
                    class="btn btn-xs"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        state.target_dict.set(Some(data.get_untracked()));
                        state.open_add_dict.set(true);
                    }
                >
                    新增子项
                </button>
                <button
                    class="btn btn-xs"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        state.target_dict.set(Some(data.get_untracked()));
                        state.open_update_dict.set(true);
                    }
                >
                    修改
                </button>
            </span>
        </span>
    }
}
