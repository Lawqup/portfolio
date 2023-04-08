use std::time::Duration;

use super::icons::*;
use super::navbar::*;
use super::project::*;

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
        const SCROLL_BUFFER: i32 = 30;
        let scroll_y = window().scroll_y().unwrap().ceil() as i32 + SCROLL_BUFFER;
        let curr_idx = match scroll_y {
            x if (..start).contains(&x) || scroll_y == SCROLL_BUFFER => 0,
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
    const THINGS_I_BUILD: [&str; 5] = [
        "UI",
        "Webapps",
        "Databases",
        "Backends",
        "Distributed Systems",
    ];

    let next_thing_idx = create_rw_signal(cx, 0);
    let thing = create_rw_signal(cx, THINGS_I_BUILD[0].to_string());

    fn next_thing(next_thing_idx: RwSignal<usize>, thing: RwSignal<String>) {
        next_thing_idx.update(|idx| *idx = (*idx + 1) % THINGS_I_BUILD.len());

        set_timeout(
            move || next_thing(next_thing_idx, thing),
            Duration::from_millis(2500),
        );

        const HALF_ANIMATION_MILLIS: u64 = 500;

        let old_len = thing().len() as u64;
        for i in 0..old_len {
            set_timeout(
                move || {
                    thing.update(|th| {
                        th.pop();
                    })
                },
                Duration::from_millis((i + 1) * HALF_ANIMATION_MILLIS / old_len),
            );
        }

        let new_len = THINGS_I_BUILD[next_thing_idx()].len() as u64;
        for i in 0..new_len {
            set_timeout(
                move || {
                    thing.update(|th| {
                        *th += &THINGS_I_BUILD[next_thing_idx()]
                            .chars()
                            .nth(i as usize)
                            .unwrap()
                            .to_string()
                    })
                },
                Duration::from_millis(
                    HALF_ANIMATION_MILLIS + (i + 1) * HALF_ANIMATION_MILLIS / new_len,
                ),
            )
        }
    }

    next_thing(next_thing_idx, thing);

    view! { cx,
        <section
            ref=_section_ref
            id="start"
            class="w-screen h-screen bg-gradient-to-b from-slate-800 to-neutral-900 flex flex-col items-center justify-center"
        >
            <div class="text-4xl text-white font-light text-left w-96 whitespace-nowrap space-y-16">
                <div>
                    <p>"Hi, I'm " <span class="text-violet-600 font-bold">"Lawrence"</span></p>
                    <p>"I build " <span class="text-violet-600 font-bold">{thing}</span></p>
                </div>
                <DownArrow class="w-12 h-12 self-start" color="white"/>
            </div>
        </section>
    }
}

#[component]
pub fn Projects(cx: Scope, _section_ref: NodeRef<Section>) -> impl IntoView {
    view! { cx,
        <section
            ref=_section_ref
            id="projects"
            class="w-screen min-h-screen bg-neutral-900 flex justify-center items-center flex-wrap"
        >
            <ProjectCard title="Portfolio Website" src="/assets/portfolio_project.png"/>
            <ProjectCard title="Portfolio Website" src="/assets/portfolio_project.png"/>
            <ProjectCard title="Portfolio Website" src="/assets/portfolio_project.png"/>
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
