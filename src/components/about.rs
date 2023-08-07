use leptos::{html::Section, *};

#[component]
pub fn About(cx: Scope, _section_ref: NodeRef<Section>) -> impl IntoView {
    view! { cx,
        <section
            ref=_section_ref
            id="About"
            class="min-h-screen bg-gradient-to-b from-neutral-900 to-violet-950 flex flex-col items-center justify-center"
        >
            <div class="text-6xl lg:text-2xl text-white font-light text-left w-5/6 lg:w-1/2 space-y-16">
                <h2 class="text-8xl lg:text-5xl font-semibold">"About me"</h2>
                <p>
                    "I'm a developer interested in a bit of everything. From frontend to distributed systems to quantum computing, my only constraint is that I need to be constantly learning."
                </p>
                <p>
                    "My effort to understand the world through computing led me to graduate from the University of Washington at 17 years old. I love how computers and computer science meet to form this messy-yet-elegant interface into our universe. While learning and challenging my mind is a huge hobby, I also spend an unreasonable amount of time bodybuilding and powerlifting."
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
