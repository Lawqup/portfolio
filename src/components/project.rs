use leptos::*;

#[component]
pub fn ProjectCard(cx: Scope, title: &'static str, src: &'static str) -> impl IntoView {
    let link = "/projects/".to_string() + &title.to_string().to_lowercase().replace(' ', "-");

    view! { cx,
        <a
            class="w-80 h-[32rem] bg-violet-600 rounded-lg px-4 py-2 flex justify-around space-y-8 flex-col border-2 border-white border-solid"
            href=link
        >
            <img class="rounded-lg border-2 border-white border-solid" src=src/>
            <h1 class="text-2xl text-white font-light pb-10">{title}</h1>
        </a>
    }
}
