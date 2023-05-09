use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::IntersectionObserver;
use web_sys::IntersectionObserverEntry;
use web_sys::IntersectionObserverInit;

use super::about::*;
use super::contact::*;
use super::navbar::*;
use super::projects::*;
use super::start::*;

use leptos::{html::Section, *};

#[derive(Debug, Clone, PartialEq)]
enum ScrollDir {
    Up,
    Down,
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let start = create_node_ref::<Section>(cx);
    let projects = create_node_ref::<Section>(cx);
    let about = create_node_ref::<Section>(cx);
    let contact = create_node_ref::<Section>(cx);

    let (selected_idx, set_selected_idx) = create_signal(cx, 0);
    let (prev_scroll, set_prev_scroll) = create_signal(cx, window().scroll_y().unwrap());
    let (scroll_dir, set_scroll_dir) = create_signal(cx, ScrollDir::Down);

    let observer_callback: Closure<dyn Fn(Vec<IntersectionObserverEntry>)> =
        Closure::new(move |entries: Vec<IntersectionObserverEntry>| {
            if let Some(entry) = entries.last() {
                let idx = SECTIONS
                    .iter()
                    .position(|id| *id == entry.target().id())
                    .expect("Invalid element observed");
                if entry.is_intersecting() {
                    set_selected_idx(idx);
                } else if scroll_dir() == ScrollDir::Up && idx > 0 {
                    set_selected_idx(idx - 1);
                } else if idx < SECTIONS.len() - 1 {
                    set_selected_idx(idx + 1);
                }
            }
        });

    let mut options = &mut IntersectionObserverInit::new();
    options = options.root_margin("-200px");

    let observer =
        IntersectionObserver::new_with_options(observer_callback.as_ref().unchecked_ref(), options)
            .unwrap();

    window_event_listener("scroll", move |_| {
        let scroll = window().scroll_y().unwrap();
        if scroll > prev_scroll() {
            set_scroll_dir(ScrollDir::Down);
        } else if scroll < prev_scroll() {
            set_scroll_dir(ScrollDir::Up);
        }
        set_prev_scroll(scroll);
    });

    let observe = move |section_ref: NodeRef<Section>| {
        let observer = observer.clone();
        create_effect(cx, move |_| {
            if let Some(elem) = section_ref.get() {
                observer.observe(&elem);
            }
        })
    };

    observe(start);
    observe(projects);
    observe(about);
    observe(contact);

    observer_callback.forget();

    view! { cx,
        <Navbar selected_idx/>
        <div>
            <Start _section_ref=start/>
            <Projects _section_ref=projects/>
            <About _section_ref=about/>
            <Contact _section_ref=contact/>
        </div>
    }
}
