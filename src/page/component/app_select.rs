use leptos::prelude::*;
use tower::{tauri_web::prelude::*, web::component::TowerSelect};

#[component]
pub fn AppSelect(
    #[prop(into,default="".into())] class: Signal<String>,
    selected: RwSignal<App>,
) -> impl IntoView {
    view! { <TowerSelect class values=App::all() selected placeholder="选择应用" /> }
}
