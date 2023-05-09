use super::icons::*;
use lazy_static::lazy_static;
use leptos::{html::Section, *};

use gloo_net::http::{Headers, Request};
use regex::Regex;

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
            id="Contact"
            class="h-screen bg-violet-950 flex items-center justify-center relative overflow-hidden"
        >
            <div class="absolute">
                <button
                    on:click=move |_| set_open(true)
                    disabled=submitted
                    class=move || {
                        "bg-black text-white text-4xl lg:text-2xl font-medium w-[30rem] h-[9rem] lg:w-80 lg:h-24 rounded-full flex items-center justify-around hover:bg-violet-600 transition-all duration-500 disabled:bg-violet-600 m-8"
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
                "w-[51rem] h-[68rem] lg:w-[30rem] lg:h-[40rem] min-h-fit bg-slate-800 rounded-[25.5px] lg:rounded-[20px] text-4xl lg:text-2xl font-medium p-8 text-left transition-transform absolute duration-500"
                    .to_string() + if !open() { " translate-x-[150vh]" } else { "" }
            }>
                <label class="px-8 flex">
                    "I'm"
                    <input
                        class="ml-2 text-violet-600 bg-transparent border-b-[5px] lg:border-b-2 border-violet-600 w-52 outline-none focus:border-teal-400 grow -top-0.5 relative"
                        type="text"
                        placeholder="your name"
                        name="name"
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            set_name(val);
                        }
                    />
                </label>
                <br/>
                <label class="px-8 flex">
                    "My email is"
                    <input
                        class=move || {
                            "ml-2 text-violet-600 bg-transparent border-b-[5px] lg:border-b-2 border-violet-600 w-52 outline-none focus:border-teal-400 grow -top-0.5 relative"
                                .to_string() + if email_err() { " animate-shake border-rose-600" } else { "" }
                        }
                        type="email"
                        placeholder="your email"
                        name="email"
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            set_email(val);
                        }
                        on:focus=move |_| set_email_err(false)
                    />
                </label>
                <textarea
                    class="resize-none rounded-[20px] lg:rounded-lg bg-black my-8 py-8 px-4 justify-self-center w-full h-3/5 outline-4 lg:outline-2"
                    placeholder="Your message..."
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        set_message(val);
                    }
                ></textarea>
                <div class="w-full flex flex-col items-center justify-center relative">
                    <p
                        hidden=move || !submit_err()
                        class="font-normal text-sm text-rose-600 absolute -top-6"
                    >
                        "An error occured, please try again later."
                    </p>
                    <p
                        hidden=move || !email_err()
                        class="font-normal text-sm text-rose-600 absolute -top-6"
                    >
                        "Please enter a valid email."
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
