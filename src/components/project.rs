use leptos::{ev::MouseEvent, html::A, *};

#[component]
pub fn ProjectCard(
    cx: Scope,
    title: &'static str,
    src: &'static str,
    link: &'static str,
) -> impl IntoView {
    let card_ref = create_node_ref::<A>(cx);
    let (loading, set_loading) = create_signal(cx, true);

    let magnetize = move |ev: MouseEvent| {
        let card = card_ref.get().unwrap().get_bounding_client_rect();

        let dx = 0.2 * (ev.client_x() as f64 - card.left() - card.width() / 2.0);
        let dy = 0.2 * (ev.client_y() as f64 - card.top() - card.height() / 2.0);
        log!("{} {}", ev.client_x(), card.left());

        card_ref
            .get()
            .unwrap()
            .style()
            .set_property("transform", &format!("translate({dx}px, {dy}px)"))
            .expect("Property could not be updated");
    };
    let unmagnetize = move |_| {
        card_ref
            .get()
            .unwrap()
            .style()
            .set_property("transform", "translate(0px, 0px)")
            .expect("Property could not be updated");
    };

    view! { cx,
        <a
            class="w-80 h-[32rem] transition-all duration-100 ease-linear overflow-hidden bg-violet-600 rounded-lg m-16 px-4 py-2 flex justify-around space-y-8 flex-col border-2 border-white border-solid hover:bg-rose-600 before:content-[''] before:bg-gradient-to-l before:from-white before:w-full before:h-full before:-left-full before:transition-all before:duration-500 before:absolute hover:before:left-full blur-[2px] hover:blur-none"
            on:mousemove=magnetize
            on:mouseleave=unmagnetize
            href=link
            ref=card_ref
        >
            <img
                class="w-full aspect-square overflow-hidden rounded-lg border-2 border-white border-solid bg-gray-600"
                alt=""
                src=src
                on:load=move |_| set_loading(false)
                class:animate-pulse=loading
            />
            <h1 class="text-2xl text-white font-light pb-10">{title}</h1>
        </a>
    }
}
