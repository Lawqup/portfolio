use leptos::*;

pub const SECTIONS: [&str; 4] = ["Start", "Projects", "About", "Contact"];

#[component]
pub fn Navbar(cx: Scope, selected_idx: ReadSignal<usize>) -> impl IntoView {
    // Lawrence Qupty | ---------------- | Start | Projects | About | Contact

    view! { cx,
        <nav class="h-16 w-screen fixed top-0 z-50 lg:pointer-events-none">
            <div class="flex-1 flex my-4 max-w-full min-h-full px-12
            justify-between items-center flex-col space-y-4 lg:flex-row lg:my-0">
                <h1 class="text-violet-600 text-2xl pointer-events-auto text-left w-auto hidden lg:inline-block">
                    "Lawrence Qupty"
                </h1>
                <div class="flex pointer-events-auto w-full lg:w-auto justify-around">
                    {SECTIONS
                        .into_iter()
                        .enumerate()
                        .map(|(idx, name)| {
                            view! { cx, <NavbarButton idx name selected_idx/> }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </nav>
    }
}

#[component]
fn NavbarButton(
    cx: Scope,
    idx: usize,
    name: &'static str,
    selected_idx: ReadSignal<usize>,
) -> impl IntoView {
    const CONTAINER: &str = "rounded-md px-8 py-3 lg:py-1 flex w-52 lg:w-32 group";
    const SELECTED: &str = " bg-gray-700 opacity-75";

    let get_container_class =
        move || CONTAINER.to_string() + if selected_idx() == idx { SELECTED } else { "" };
    let reset_href = move |_| {
        window()
            .location()
            .set_hash(name)
            .expect("Couldn't set hash");
    };

    view! { cx,
        <a href=format!("#{name}") class=get_container_class on:click=reset_href>
            <span class="text-white text-center text-3xl lg:text-lg w-full h-full relative
            transition-all duration-200 group-hover:-translate-y-2">{name}</span>
        </a>
    }
}
