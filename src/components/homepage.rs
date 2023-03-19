use super::navbar::*;
use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
            <div>
              <Navbar/>
            </div>
    }
}
