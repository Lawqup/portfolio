use leptos::*;

#[component]
pub fn DownArrow(
    cx: Scope,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    view! { cx,
        <svg
            viewBox="0 0 24 24"
            strokeWidth=1
            fill=color.unwrap_or("white")
            stroke=color.unwrap_or("white")
            class=class
        >
            <path d="m5.214 14.522s4.505 4.502 6.259 6.255c.146.147.338.22.53.22s.384-.073.53-.22c1.754-1.752 6.249-6.244 6.249-6.244.144-.144.216-.334.217-.523 0-.193-.074-.386-.221-.534-.293-.293-.766-.294-1.057-.004l-4.968 4.968v-14.692c0-.414-.336-.75-.75-.75s-.75.336-.75.75v14.692l-4.979-4.978c-.289-.289-.761-.287-1.054.006-.148.148-.222.341-.221.534 0 .189.071.377.215.52z"></path>
        </svg>
    }
}

#[component]
pub fn RightArrow(
    cx: Scope,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    view! { cx,
        <svg
            viewBox="0 0 24 24"
            strokeWidth=1
            fill=color.unwrap_or("white")
            stroke=color.unwrap_or("white")
            class=class
        >
            <path d="m14.523 18.787s4.501-4.505 6.255-6.26c.146-.146.219-.338.219-.53s-.073-.383-.219-.53c-1.753-1.754-6.255-6.258-6.255-6.258-.144-.145-.334-.217-.524-.217-.193 0-.385.074-.532.221-.293.292-.295.766-.004 1.056l4.978 4.978h-14.692c-.414 0-.75.336-.75.75s.336.75.75.75h14.692l-4.979 4.979c-.289.289-.286.762.006 1.054.148.148.341.222.533.222.19 0 .378-.072.522-.215z"></path>
        </svg>
    }
}

#[component]
pub fn LeftArrow(
    cx: Scope,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    view! { cx,
        <svg
            viewBox="0 0 24 24"
            strokeWidth=1
            fill=color.unwrap_or("white")
            stroke=color.unwrap_or("white")
            class=class
        >
            <path d="m9.474 5.209s-4.501 4.505-6.254 6.259c-.147.146-.22.338-.22.53s.073.384.22.53c1.752 1.754 6.252 6.257 6.252 6.257.145.145.336.217.527.217.191-.001.383-.074.53-.221.293-.293.294-.766.004-1.057l-4.976-4.976h14.692c.414 0 .75-.336.75-.75s-.336-.75-.75-.75h-14.692l4.978-4.979c.289-.289.287-.761-.006-1.054-.147-.147-.339-.221-.53-.221-.191-.001-.38.071-.525.215z"></path>
        </svg>
    }
}
