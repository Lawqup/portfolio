use leptos::{
    ev::MouseEvent,
    html::{Section, A},
    *,
};

#[component]
pub fn Projects(cx: Scope, _section_ref: NodeRef<Section>) -> impl IntoView {
    view! { cx,
        <section
            ref=_section_ref
            id="Projects"
            class="min-h-screen bg-neutral-900 flex justify-center items-center flex-wrap relative"
        >
            <ProjectCard
                title="Rust-nn"
                src="/assets/rust-nn.png"
                link="https://github.com/lawqup/rust-nn"
            />
            <ProjectCard
                title="Vision"
                src="/assets/vision_logo.png"
                link="https://www.vision.eco/"
            />
            <ProjectCard
                title="Fit Social"
                src="/assets/fitsocial_image.png"
                link="https://github.com/Monacr/FitSocial"
            />
            <ProjectCard
                title="Husky Navigation Services"
                src="/assets/hnav_logo.png"
                link="https://huskynavigation.azurewebsites.net"
            />
        </section>
    }
}

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
        if window()
            .match_media("(pointer: coarse)")
            .expect("Could not execute media query")
            .unwrap()
            .matches()
        {
            return;
        }

        let card = card_ref.get().unwrap().get_bounding_client_rect();
        let dx = 0.2 * (ev.client_x() as f64 - card.left() - card.width() / 2.0);
        let dy = 0.2 * (ev.client_y() as f64 - card.top() - card.height() / 2.0);

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
            class="w-[40rem] h-[64rem] lg:w-80 lg:h-[32rem] transition-all duration-100 ease-linear overflow-hidden bg-violet-600 rounded-[15px] lg:rounded-lg m-16 px-4 py-2 flex justify-around space-y-8 flex-col border-4 lg:border-2 border-white border-solid hover:bg-rose-600 before:content-[''] before:bg-gradient-to-l before:from-white before:w-full before:h-full before:-left-full before:transition-all before:duration-500 before:absolute hover:before:left-full"
            on:mousemove=magnetize
            on:mouseleave=unmagnetize
            href=link
            ref=card_ref
            target="_blank"
        >
            <img
                class="w-full aspect-square overflow-hidden rounded-[15px] lg:rounded-lg border-4 lg:border-2 border-white border-solid bg-gray-600"
                alt=""
                src=src
                on:load=move |_| set_loading(false)
                class:animate-pulse=loading
            />
            <p class="text-4xl lg:text-2xl text-white font-light pb-10">{title}</p>
        </a>
    }
}
