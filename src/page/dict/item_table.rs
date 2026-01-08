use crate::{
    page::{AddItemForm, DictState, UpdateItemForm},
    service::async_get_dict_items,
};
use leptos::prelude::*;
use tower::tauri_web::prelude::*;

#[component]
pub fn ItemTable() -> impl IntoView {
    let state = expect_context::<DictState>();
    let item_resource = LocalResource::new(move || {
        let dict_code = state.current_dict.get().map(|dict| dict.dict_code);
        async_get_dict_items(dict_code)
    });
    let open_add = RwSignal::new(false);
    let open_update = RwSignal::new(false);
    let current_item = RwSignal::new(None);
    view! {
        <div class="flex flex-col gap-4">
            <div class="flex justify-between items-center">
                <span>{move || state.current_dict.get().unwrap().dict_name}</span>
                <button
                    class="btn btn-sm"
                    on:click=move |_| {
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
                        view! {
                            <span class="self-center loading loading-spinner loading-xl"></span>
                        }
                    }>
                        {move || {
                            item_resource
                                .to_view(|list| {
                                    list.into_iter()
                                        .enumerate()
                                        .map(|(index, item)| {
                                            let item = item.clone();
                                            view! {
                                                <tr>
                                                    <th>{index + 1}</th>
                                                    <td>{item.item_name.clone()}</td>
                                                    <td>{item.item_value.clone()}</td>
                                                    <td>{item.public.label()}</td>
                                                    <td>{item.remark.clone()}</td>
                                                    <td>{item.status.label()}</td>
                                                    <td>{from_secs_and_default(item.created)}</td>
                                                    <td>
                                                        {item
                                                            .updated
                                                            .map_or(
                                                                "".to_string(),
                                                                |updated| { from_secs_and_default(updated) },
                                                            )}
                                                    </td>
                                                    <td>
                                                        <button
                                                            class="btn btn-xs"
                                                            on:click=move |_| {
                                                                current_item.set(Some(item.clone()));
                                                                open_update.set(true)
                                                            }
                                                        >
                                                            修改
                                                        </button>
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
        </div>
        <Show when=move || { open_add.get() } fallback=|| view! {}>
            <Dialog open=open_add title="新增字典项">
                <AddItemForm open=open_add item_resource />
            </Dialog>
        </Show>
        <Show
            when=move || { current_item.get().is_some() && open_update.get() }
            fallback=|| view! {}
        >
            <Dialog open=open_update title="修改字典项">
                <UpdateItemForm open=open_update item_resource item=current_item />
            </Dialog>
        </Show>
    }
}
