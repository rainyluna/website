use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">
                <h1
                    class="overlay-text"
                    style="text-align: poisition: relative; top: 15%; left: 5%;"
                >
                    "Hi!"
                </h1>

                <h2
                    class="overlay-text"
                    style="text-align: poisition: relative; top: 25%; left: 5%;"
                >
                    "Welcome to my site"
                    <br />
                    "I'm Luna, a Rust and"
                    <br />
                    "   web developer"
                    <br />
                    "(learning still ofc)"
                    <br />
                    "and sysadmin"
                </h2>

                <h2 class="overlay-text" style="text-align: right; top: 25%; right: 5%;">
                    "I like making silly"
                    <br />
                    "art like this, and"
                    <br />
                    "other cool stuff"
                    <br />
                    <br />
                    "Just UwU :3"
                </h2>

                <div class="background-home"></div>

                <img src="/public/faces.webp" class="foreground-art" alt="Artistic overlay" />
            </div>

        </ErrorBoundary>
    }
}
