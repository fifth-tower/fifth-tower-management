use super::*;
use leptos::prelude::*;
use tower::management_model::DictListItem;

#[component]
pub fn DictPane() -> impl IntoView {
    let state = DictState::new();
    provide_context(state);
    view! {
        <div class="flex gap-4">
            <DictMenu class="flex-none w-56" />
            <Show when=move || { state.current_dict.get().is_some() } fallback=|| view! {}>
                <ItemTable />
            </Show>
        </div>
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DictState {
    pub current_dict: RwSignal<Option<DictListItem>>,
    pub open_add_dict: RwSignal<bool>,
    pub open_update_dict: RwSignal<bool>,
    pub target_dict: RwSignal<Option<DictListItem>>,
}
impl DictState {
    pub fn new() -> Self {
        Self {
            current_dict: RwSignal::new(None),
            open_add_dict: RwSignal::new(false),
            open_update_dict: RwSignal::new(false),
            target_dict: RwSignal::new(None),
        }
    }
}
