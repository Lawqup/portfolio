use leptos::*;

#[component]
pub fn Navbar(cx: Scope, selected_idx: ReadSignal<usize>) -> impl IntoView {
    // Lawrence Qupty | ---------------- | Start | Projects | About | Contact
    const BUTTONS: [&str; 4] = ["Start", "Projects", "About", "Contact"];

    view! { cx,
        <nav class="h-16 w-screen fixed top-0 z-50 pointer-events-none">
            <div class="flex-1 flex m-auto max-w-full min-h-full px-12
            justify-between items-center">
                <p class="text-violet-600 text-2xl pointer-events-auto">"Lawrence Qupty"</p>
                <div class="flex space-x-4 items-center align-middle pointer-events-auto">
                    {BUTTONS
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
    const CONTAINER: &str = "rounded-md px-8 py-1 flex w-32 group";
    const SELECTED: &str = " bg-gray-700 opacity-75";

    const TEXT: &str = "text-white text-center text-lg w-full h-full relative \
                                transition-all duration-200 group-hover:-translate-y-2";

    let get_container_class =
        move || CONTAINER.to_string() + if selected_idx() == idx { SELECTED } else { "" };
    let reset_href = move |_| {
        window()
            .location()
            .set_hash(&name.to_lowercase())
            .expect("Couldn't set hash");
    };

    view! { cx,
        <a href=format!("#{}", name.to_lowercase()) class=get_container_class on:click=reset_href>
            <span class=TEXT>{name}</span>
        </a>
    }
}
