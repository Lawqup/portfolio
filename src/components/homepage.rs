use super::navbar::*;
use leptos::html::Section;
use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let start = create_node_ref::<Section>(cx);
    let projects = create_node_ref::<Section>(cx);
    let about = create_node_ref::<Section>(cx);
    let contact = create_node_ref::<Section>(cx);

    window_event_listener("scroll", |_| log!("HERE"));

    view! { cx,
        <Navbar/>
        <div>
            <Start _section_ref=start/>
            <Projects _section_ref=projects/>
            <About _section_ref=about/>
            <Contact _section_ref=contact/>
        </div>
    }
}

#[component]
pub fn Start(cx: Scope, _section_ref: NodeRef<Section>) -> impl IntoView {
    view! { cx,
        <section
            ref=_section_ref
            id="start"
            class="w-screen h-screen bg-gradient-to-b from-slate-800 to-neutral-900"
        >
            <h1>"Start"</h1>
        </section>
    }
}

#[component]
pub fn Projects(cx: Scope, _section_ref: NodeRef<Section>) -> impl IntoView {
    view! { cx,
        <section
            ref=_section_ref
            id="projects"
            class="w-screen h-screen bg-gradient-to-b from-neutral-900 to-gray-700"
        >
            <h1>"Projects"</h1>
        </section>
    }
}

#[component]
pub fn About(cx: Scope, _section_ref: NodeRef<Section>) -> impl IntoView {
    view! { cx,
        <section
            ref=_section_ref
            id="about"
            class="w-screen h-screen bg-gradient-to-b from-gray-700 to-violet-900"
        >
            <h1>"About"</h1>
        </section>
    }
}
#[component]
pub fn Contact(cx: Scope, _section_ref: NodeRef<Section>) -> impl IntoView {
    view! { cx,
        <section ref=_section_ref id="contact" class="w-screen h-screen bg-violet-900">
            <h1>"Contact"</h1>
        </section>
    }
}
