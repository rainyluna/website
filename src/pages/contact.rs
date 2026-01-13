use leptos::prelude::*;

/// Contact Page
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <div class="page-container">
            <div class="background-page"></div>
            <div class="page-content">
                <h1 class="page-title">"Contact"</h1>
                <div class="contact-text">
                    <p>"Reach me:"</p>
                    <a class="contact-link" href="mailto:luna@uwu.chat">
                        "Email (click)"
                    </a>

                </div>
            </div>
        </div>
    }
}
