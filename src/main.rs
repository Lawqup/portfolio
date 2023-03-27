use crate::components::homepage::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Title text="Portfolio | Lawrence Qupty"/>
        <Router>
            <main class="relative">
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <HomePage/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}
