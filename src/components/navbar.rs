use leptos::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    // Lawrence Qupty | ---------------- | Start | Projects | About | Contact
    let (selected_idx, set_selected_idx) = create_signal(cx, 1);

    const NAV_BUTTON: &'static str = "flex items-center text-gray-300 hover:bg-gray-700 \
                                      hover:text-white rounded-MD \
                                      px-6 pt-2 text-medium font-medium";

    const SELECTED: &'static str = " bg-gray-700 text-white";

    let get_button_class = move |i: i32| {
        move || NAV_BUTTON.to_string() + if selected_idx() == i { SELECTED } else { "" }
    };

    let on_click = move |i: i32| move |_| set_selected_idx(i);

    view! { cx,
            <nav class="bg-gray-800 h-16 flex">
              <div class="flex-1 flex m-auto max-w-full min-h-full px-12
                          justify-between items-center">
                <p class="text-violet-600 text-2xl">"Lawrence Qupty"</p>

                <div class="flex space-x-4 items-center">
                  <a href="#" class=get_button_class(0) on:click=on_click(0)>"Start"</a>
                  <a href="#" class=get_button_class(1) on:click=on_click(1)>"Projects"</a>
                  <a href="#" class=get_button_class(2) on:click=on_click(2)>"About"</a>
                  <a href="#" class=get_button_class(3) on:click=on_click(3)>"Contact"</a>
                </div>
              </div>
            </nav>
    }
}
