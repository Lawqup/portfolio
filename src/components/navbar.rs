use leptos::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    // Lawrence Qupty | ---------------- | Start | Projects | About | Contact
    let (selected_idx, set_selected_idx) = create_signal(cx, 0);

    view! { cx,
            <nav class="bg-gray-800 h-16 flex">
              <div class="flex-1 flex m-auto max-w-full min-h-full px-12
                          justify-between items-center">
                <p class="text-violet-600 text-2xl">"Lawrence Qupty"</p>

                <div class="flex space-x-4 items-center align-middle">
                  <NavbarButton idx=0 name="Start" selected_idx=selected_idx set_selected_idx=set_selected_idx/>
                  <NavbarButton idx=1 name="Projects" selected_idx=selected_idx set_selected_idx=set_selected_idx/>
                  <NavbarButton idx=2 name="About" selected_idx=selected_idx set_selected_idx=set_selected_idx/>
                  <NavbarButton idx=3 name="Contact" selected_idx=selected_idx set_selected_idx=set_selected_idx/>
                </div>
              </div>
            </nav>
    }
}

#[component]
fn NavbarButton(
    cx: Scope,
    idx: i32,
    name: &'static str,
    selected_idx: ReadSignal<i32>,
    set_selected_idx: WriteSignal<i32>,
) -> impl IntoView {
    const CONTAINER: &'static str = "rounded-md px-8 py-1 flex w-32 group";
    const SELECTED: &'static str = " bg-gray-700";

    const TEXT: &'static str = "text-white text-center text-lg w-full h-full relative \
                                transition-all duration-200 group-hover:-translate-y-2";

    let get_container_class =
        move || CONTAINER.to_string() + if selected_idx() == idx { SELECTED } else { "" };

    let on_click = move |_| set_selected_idx(idx);

    view! {cx,
           <a href="#" class=get_container_class on:click=on_click>
             <span class=TEXT>{name}</span>
           </a>
    }
}
