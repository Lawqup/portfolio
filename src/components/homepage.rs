use super::navbar::*;
use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Navbar/>
        <div>
            <section id="start">
                <Start/>
            </section>
            <section id="projects">
                <Projects/>
            </section>
            <section id="about">
                <About/>
            </section>
            <section id="contact">
                <Contact/>
            </section>
        </div>
    }
}

#[component]
pub fn Start(cx: Scope) -> impl IntoView {
    view! { cx,
            <div class="bg-slate-800 w-screen h-screen">
              <h1>"Start"</h1>
            </div>
    }
}

#[component]
pub fn Projects(cx: Scope) -> impl IntoView {
    view! { cx,
            <div class="bg-neutral-900 w-screen h-screen">
              <h1>"Projects"</h1>
            </div>
    }
}

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
            <div class="bg-gray-900 w-screen h-screen">
              <h1>"About"</h1>
            </div>
    }
}
#[component]
pub fn Contact(cx: Scope) -> impl IntoView {
    view! { cx,
            <div class="bg-black w-screen h-screen">
              <h1>"Contact"</h1>
            </div>
    }
}
