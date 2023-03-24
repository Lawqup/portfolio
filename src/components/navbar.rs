use leptos::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    // Lawrence Qupty | ---------------- | Start | Projects | About | Contact
    const BUTTONS: [&'static str; 4] = ["Start", "Projects", "About", "Contact"];

    let (selected_idx, set_selected_idx) = create_signal(cx, 0);

    view! { cx,
        <nav class="h-16 w-screen fixed top-0 z-9999">
            <div class="flex-1 flex m-auto max-w-full min-h-full px-12
                                                justify-between items-center">
                <p class="text-violet-600 text-2xl">"Lawrence Qupty"</p>
                <div class="flex space-x-4 items-center align-middle">
                    {BUTTONS
                        .into_iter()
                        .enumerate()
                        .map(|(idx, name)| {
                            view! { cx,
                                <NavbarButton
                                    idx=idx
                                    name=name
                                    selected_idx=selected_idx
                                    set_selected_idx=set_selected_idx
                                />
                            }
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
    set_selected_idx: WriteSignal<usize>,
) -> impl IntoView {
    const CONTAINER: &'static str = "rounded-md px-8 py-1 flex w-32 group";
    const SELECTED: &'static str = " bg-gray-700";

    const TEXT: &'static str = "text-white text-center text-lg w-full h-full relative \
                                transition-all duration-200 group-hover:-translate-y-2";

    let get_container_class =
        move || CONTAINER.to_string() + if selected_idx() == idx { SELECTED } else { "" };

    let on_click = move |_| set_selected_idx(idx);

    view! { cx,
        <a href=format!("#{}", name.to_lowercase()) class=get_container_class on:click=on_click>
            <span class=TEXT>{name}</span>
        </a>
    }
}
