use leptos::*;

#[component]
pub fn DownArrow(cx: Scope, class: &'static str, color: &'static str) -> impl IntoView {
    view! { cx,
        <svg viewBox="0 0 24 24" strokeWidth=1 stroke=color class=class>
            <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M19.5 13.5L12 21m0 0l-7.5-7.5M12 21V3"
            ></path>
        </svg>
    }
}
