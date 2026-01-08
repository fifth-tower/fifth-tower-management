use std::collections::HashSet;

use leptos::prelude::*;
use tower::{tauri_web::prelude::*, web::component::Checkboxs};

#[component]
pub fn StatusCheckbox(
    #[prop(into)] name: String,
    #[prop(into,default="".into())] class: Signal<String>,
    checked: RwSignal<HashSet<YesNo>>,
) -> impl IntoView {
    view! { <Checkboxs class name values=YesNo::all() checked multi=false /> }
}
