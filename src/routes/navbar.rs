use leptos::{component, view, IntoView, Scope};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    let (_count, _set_count,) = create_signal(cx, 0);
    // let on_click = move |_| _set_count,.update(|count| *count += 1);

    view! {
        cx,
        <div>
            // Create Account
            <a href="/">"HOME"</a>
            <a href="/account">"ACCOUNT"</a>
            <a href="/login-register">"LOGIN/REGISTER"</a>
            <a href="/profile">"PROFILE"</a>
            <a href="/notifications">"NOTIFICATIONS"</a>
            <a href="/messages">"MESSAGES"</a>
        </div>
    }
}

