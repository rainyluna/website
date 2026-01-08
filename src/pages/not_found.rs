use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="background-404"></div>
        <div class="center_404 text-center">
            <h1 class="text_404">"404"</h1>
            <p class="text_404">"Where are you going?"</p>
        </div>
    }
}
