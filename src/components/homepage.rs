use super::navbar::*;
use leptos::html::Section;
use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let start = create_node_ref::<Section>(cx);
    let projects = create_node_ref::<Section>(cx);
    let about = create_node_ref::<Section>(cx);
    let contact = create_node_ref::<Section>(cx);

    let (selected_idx, set_selected_idx) = create_signal(cx, 0);

    let detect_curr_section = move || {
        if start.get().is_none() || projects.get().is_none() || about.get().is_none() {
            return;
        }

        let start = start.get().unwrap().scroll_height();
        let projects = projects.get().unwrap().scroll_height() + start;
        let about = about.get().unwrap().scroll_height() + projects;

        // Add 30-pixel buffer to switch section a bit before
        let scroll_y = window().scroll_y().unwrap().ceil() as i32 + 30;

        let curr_idx = match scroll_y {
            x if (..start).contains(&x) => 0,
            x if (start..projects).contains(&x) => 1,
            x if (projects..about).contains(&x) => 2,
            x if (about..).contains(&x) => 3,
            _ => unreachable!(),
        };

        set_selected_idx(curr_idx)
    };

    create_effect(cx, move |_| detect_curr_section());
    window_event_listener("scroll", move |_| detect_curr_section());

    view! { cx,
        <Navbar selected_idx/>
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
        <section ref=_section_ref id="projects" class="w-screen h-screen bg-neutral-900">
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
            class="w-screen h-screen bg-gradient-to-b from-neutral-900 to-violet-900"
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
