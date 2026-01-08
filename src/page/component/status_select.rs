use leptos::prelude::*;
use tower::{tauri_web::prelude::*, web::component::TowerSelect};

#[component]
pub fn StatusSelect(
    #[prop(into,default="".into())] class: Signal<String>,
    selected: RwSignal<YesNo>,
    #[prop(into, optional)] placeholder: Option<String>,
) -> impl IntoView {
    view! {
        <TowerSelect
            class=class.get_untracked()
            values=YesNo::all()
            selected
            placeholder=placeholder.unwrap_or_default()
        />
    }
}
