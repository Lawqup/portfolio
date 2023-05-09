use super::icons::*;
use leptos::{html::Section, *};
use std::time::Duration;

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

    let reset_href = move |_| {
        window()
            .location()
            .set_hash("Projects")
            .expect("Couldn't set hash");
    };
    view! { cx,
        <section
            ref=_section_ref
            id="Start"
            class="h-screen bg-gradient-to-b from-slate-800 to-neutral-900 flex flex-col items-center justify-center"
        >
            <div class="text-6xl lg:text-4xl text-white font-light text-left w-[46rem] lg:w-96 whitespace-nowrap space-y-16">
                <div>
                    <p>"Hi, I'm " <span class="text-violet-600 font-bold">"Lawrence"</span></p>
                    <p>"I build " <span class="text-violet-600 font-bold">{thing}</span></p>
                </div>
                <a href="#projects" on:click=reset_href>
                    <DownArrow class="w-28 h-28 mt-24 lg:w-12 lg:h-12 lg:mt-12" color="white"/>
                </a>
            </div>
        </section>
    }
}
