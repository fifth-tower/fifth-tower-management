use leptos::prelude::*;
use leptos_router::{components::*, path};

use tower::{
    common::App,
    tauri_web::{
        page::component::{Header, LeftSideBar},
        AppState,
    },
    web::component::OpTip,
    web_model::MenuData,
};

use crate::page::{ConfigPane, DashboardPane, DictPane, UserPane};

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);
    state.startup();

    view! {
        <Router>
            <div class="flex gap-4">
                <LeftSideBar current_app=App::TowerManagementWebsite menus=get_menus() />
                <div class="flex relative flex-col gap-2 w-full">
                    <Header />
                    <OpTip content=state.op_tip />
                    <Routes fallback=|| "即将上线.">
                        <Route path=path!("/") view=DashboardPane />
                        <Route path=path!("/user") view=UserPane />
                        <Route path=path!("/config") view=ConfigPane />
                        <Route path=path!("/dict") view=DictPane />
                    </Routes>
                </div>
            </div>
        </Router>
    }
}

fn get_menus() -> MenuData {
    MenuData::new("/", "系统管理", icondata::AiHeartOutlined)
        .add_child_menu(MenuData::new("/", "看板", icondata::AiDashboardOutlined))
        .add_child_menu(MenuData::new("/user", "用户管理", icondata::AiUserOutlined))
        .add_child_menu(MenuData::new(
            "/config",
            "配置管理",
            icondata::AiProjectOutlined,
        ))
        .add_child_menu(MenuData::new(
            "/dict",
            "字典管理",
            icondata::AiTableOutlined,
        ))
        .to_owned()
}
