use leptos::prelude::*;

/// Projects Page
#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="page-container">
            <div class="background-page"></div>
            <div class="page-content">
                <h1 class="page-title">"Projects"</h1>
                <div class="projects-grid">
                    <div class="project-card">
                        <h3>"Dotfiles/NixOS config"</h3>
                        <a href="#" class="project-card-link">
                            "(soon)"
                        </a>
                        <p>"My dotfiles and NixOS config."</p>
                    </div>
                    <div class="project-card">
                        <h3>"Sabana Market Site"</h3>
                        <a
                            href="https://clientes.sabanamarket.com/?ref=lunadev.online"
                            class="project-card-link"
                        >
                            "clientes.sabanamarket.com"
                        </a>
                        <p>
                            "The site for Sabana Market, my mom's business. Made possible by love, strapi, nextjs, docker and caddy"
                        </p>
                    </div>
                    <div class="project-card">
                        <h3>"This Very Website!"</h3>
                        <a
                            href="https://lunadev.online/?ref=lunadev.online"
                            class="project-card-link"
                        >
                            "lunadev.online"
                        </a>
                        <p>"This site! Made possible by love, leptos, rust, docker and caddy"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
