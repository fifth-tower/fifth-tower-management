use leptos::prelude::*;

#[component]
pub fn Blank(
    #[prop(into)] app_id: String,
    #[prop(into)] src: String,
    #[prop(into, optional)] width: MaybeProp<u32>,
    #[prop(into, optional)] height: MaybeProp<u32>,
    #[prop(default=Dir::Record)] dir: Dir,
) -> impl IntoView {
    view! {
        <div class="rounded-xl shadow-sm rounded-box" />
        <div class="alert alert-info alert-success alert-warning alert-error badge badge-xs badge-neutral badge-info badge-warning" />
        <div class="mask mask-star-2 rating rating-xs mask-heart" />
        <div class="filter filter-reset" />
        <div class="mask-squircle mask-heart mask-hexagon mask-hexagon-2 mask-decagon mask-diamond mask-square mask-circle" />
        <div class="theme-controller btn btn-sm btn-block btn-ghost btn-neutral btn-circle btn-xs btn-block btn-link" />
        <div class="dropdown dropdown-start dropdown-end dropdown-center dropdown-content dropdown-hover dropdown-top" />
        <div class="menu menu-title tooltip tooltip-right menu-active" />
        <div class="absolute relative bottom-1 top-2 right-2 p-2 p-4 py-4 mt-4 mr-4 w-20 w-24 w-32 w-36 w-64 w-full h-screen lg:w-56 z-1 w-100 w-160 max-2xl max-3xl" />
        <div class="fieldset label textarea validator validator-hint input" />
        <div class="loading loading-spinner loading-lg" />
        <div class="navbar-start navbar-end" />
        <div class="text-base text-lg font-bold text-left text-right" />
        <div class="toggle toggle-xs" />
        <div class="checkbox select" />
        <div class="radial-progress" />
        <div class="modal modal-open modal-box" />
        <div class="hidden lg:hidden sm:max-lg:hidden" />
        <div class="text-white bg-red-400 bg-red-500 bg-orange-400 bg-base-100 bg-base-200" />
        <div class="flex justify-start justify-center justify-between items-center" />
    }
}
