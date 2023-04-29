use gloo_net::http::Headers;
use gloo_net::http::Request;
use regex::Regex;
use std::time::Duration;

use super::icons::*;
use super::navbar::*;
use super::project::*;

use lazy_static::lazy_static;
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
            class="w-screen min-h-screen bg-gradient-to-b from-neutral-900 to-violet-950 flex flex-col items-center justify-center"
        >
            <div class="text-2xl text-white font-light text-left w-1/2 space-y-16">
                <h1 class="text-5xl font-semibold">"About me"</h1>
                <p>
                    "I'm a developer interested in a bit of everything. From frontend to distributed systems to quantum computing, my only constraint is that I need to be constantly learning."
                </p>
                <p>
                    "My effort to understand the world through computing led me to graduating from the University of Washington at 18 years old. I love how computers and computer science meet to form this messy-yet-elegant interface into our universe. While learning and challenging my mind is a huge hobby, I also spend an unreasonable amount of time bodybuiling and powerlifting."
                </p>
                <p>
                    "You can find a copy of my resume "
                    <a
                        class="text-violet-500 font-semibold"
                        href="/assets/qupty_resume.pdf"
                        target="_blank"
                    >
                        "here"
                    </a> "."
                </p>
            </div>
        </section>
    }
}
#[component]
pub fn Contact(cx: Scope, _section_ref: NodeRef<Section>) -> impl IntoView {
    let (open, set_open) = create_signal(cx, false);
    let (name, set_name) = create_signal(cx, String::new());
    let (email, set_email) = create_signal(cx, String::new());
    let (message, set_message) = create_signal(cx, String::new());
    let (email_err, set_email_err) = create_signal(cx, false);
    let (submit_err, set_submit_err) = create_signal(cx, false);
    let (submitted, set_submitted) = create_signal(cx, false);

    let handle_send = move |_| {
        spawn_local(async move {
            lazy_static! {
                static ref EMAIL_RE: Regex =
                    Regex::new(r"^\w+([-+.']\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
            }

            if !EMAIL_RE.is_match(&email()) {
                set_email_err(true);
                return;
            }

            let headers = Headers::new();
            headers.append("Content-Type", "application/json");
            headers.append("Accept", "application/json");

            let body = format!(
                r#"{{"name":"{}", "email":"{}", "message":"{}"}}"#,
                name(),
                email(),
                message()
            );
            let res = Request::post("https://formsubmit.co/ajax/lawrencequp@gmail.com")
                .headers(headers)
                .body(body)
                .send()
                .await;

            if res.is_err() || res.unwrap().status() != 200 {
                set_submit_err(true);
            } else {
                set_submit_err(false);
                set_open(false);
                set_submitted(true);
            }
        })
    };

    view! { cx,
        <section
            ref=_section_ref
            id="contact"
            class="w-screen min-h-screen bg-violet-950 flex items-center justify-center relative overflow-hidden"
        >
            <div class="absolute">
                <button
                    on:click=move |_| set_open(true)
                    disabled=submitted
                    class=move || {
                        "bg-black text-white text-2xl font-medium w-80 h-24 rounded-full flex items-center justify-around hover:bg-violet-600 transition-all duration-500 disabled:bg-violet-600 m-8"
                            .to_string() + if open() { " -translate-x-[150vh]" } else { "" }
                    }
                >
                    {move || {
                        if submitted() {
                            view! { cx,
                                "Message sent"
                                <></>
                            }
                        } else {
                            view! { cx,
                                "Contact me"
                                <RightArrow class="w-12 h-12"/>
                            }
                        }
                    }}
                </button>
                <p class=move || {
                    "text-2xl font-light transition-all duration-300 delay-500".to_string()
                        + if submitted() { " opacity-100" } else { " opacity-0" }
                }>"I'll get back to you soon!"</p>
            </div>
            <div class=move || {
                "w-[30rem] h-[40rem] min-h-fit bg-slate-800 rounded-[20px] text-2xl font-medium p-8 text-left transition-transform absolute duration-500"
                    .to_string() + if !open() { " translate-x-[150vh]" } else { "" }
            }>
                <label class="px-8">
                    "I'm"
                    <input
                        class="ml-2 text-violet-600 bg-transparent border-b-2 border-violet-600 w-52 outline-none focus:border-teal-400"
                        type="text"
                        placeholder="your name"
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            set_name(val);
                        }
                    />
                </label>
                <br/>
                <label class="px-8">
                    "My email is"
                    <input
                        class=move || {
                            "ml-2 text-violet-600 bg-transparent border-b-2 border-violet-600 w-52 outline-none focus:border-teal-400"
                                .to_string() + if email_err() { " animate-shake border-rose-600" } else { "" }
                        }
                        type="text"
                        placeholder="your email"
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            set_email(val);
                        }
                        on:focus=move |_| set_email_err(false)
                    />
                </label>
                <textarea
                    class="resize-none rounded-lg bg-black my-8 p-8 justify-self-center w-full h-3/5"
                    placeholder="Your message..."
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        set_message(val);
                    }
                ></textarea>
                <div class="w-full flex flex-col items-center justify-center">
                    <p
                        class:hidden=move || !submit_err()
                        class="font-normal text-sm text-rose-600 pb-2"
                    >
                        "An error occured, please try again later."
                    </p>
                    <button
                        class="bg-black w-56 h-20 rounded-full flex items-center justify-around hover:bg-violet-600 transition duration-500 disabled:opacity-75 disabled:bg-gray-700"
                        disabled=move || name().is_empty() || email().is_empty() || message().is_empty()
                        on:click=handle_send
                    >
                        <LeftArrow class="w-12 h-12"/>
                        "Send"
                    </button>
                </div>
            </div>
        </section>
    }
}
