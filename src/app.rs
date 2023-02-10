use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod routes;
use routes::navbar::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <div>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/login-register" view=|cx| view! { cx, <LoginRegister/> }/>
                    <Route path="/account" view=|cx| view! { cx, <Account/> }/>
                    <Route path="/profile" view=|cx| view! { cx, <Profile/> }/>
                    <Route path="/notifications" view=|cx| view! { cx, <Notifications/> }/>
                    <Route path="/messages" view=|cx| view! { cx, <Messages/> }/>
                </Routes>
            </main>
        </Router>
        </div>
    }
}

// #[component]
// fn NavBar(cx: Scope) -> impl IntoView {
//     let (_count, _set_count,) = create_signal(cx, 0);
//     // let on_click = move |_| _set_count,.update(|count| *count += 1);
// 
//     view! {
//         cx,
//         <div>
//             // Create Account
//             <a href="/">"HOME"</a>
//             <a href="/account">"ACCOUNT"</a>
//             <a href="/login-register">"LOGIN/REGISTER"</a>
//             <a href="/profile">"PROFILE"</a>
//             <a href="/notifications">"NOTIFICATIONS"</a>
//             <a href="/messages">"MESSAGES"</a>
//         </div>
//     }
// }

#[component]
fn LoginRegister(cx: Scope) -> impl IntoView {
    let (_count, _set_count,) = create_signal(cx, 0);
    // let on_click = move |_| _set_count,.update(|count| *count += 1);

    view! {
        cx,
        <div>
            <NavBar/>
            <h1>"Login/Register"</h1>
            
        </div>
    }
}

#[component]
fn Account(cx: Scope) -> impl IntoView {
    let (_count, _set_count,) = create_signal(cx, 0);
    // let on_click = move |_| _set_count,.update(|count| *count += 1);

    view! {
        cx,
        <div>
            <NavBar/>
            <h1>"Account"</h1>

        </div>
    }
}

#[component]
fn Profile(cx: Scope) -> impl IntoView {
    let (_count, _set_count,) = create_signal(cx, 0);
    // let on_click = move |_| _set_count,.update(|count| *count += 1);

    view! {
        cx,
        <div>
            <NavBar/>
            <h1>"Profile"</h1>
            
        </div>
    }
}

#[component]
fn Notifications(cx: Scope) -> impl IntoView {
    let (_count, _set_count,) = create_signal(cx, 0);
    // let on_click = move |_| _set_count,.update(|count| *count += 1);

    view! {
        cx,
        <div>
            <NavBar/>
            <h1>"Notifications"</h1>

        </div>
    }
}

#[component]
fn Messages(cx: Scope) -> impl IntoView {
    let (_count, _set_count,) = create_signal(cx, 0);
    // let on_click = move |_| _set_count,.update(|count| *count += 1);

    view! {
        cx,
        <div>
            <NavBar/>
            <h1>"MESSAGES"</h1>

        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (_count, _set_count,) = create_signal(cx, 0);
    // let on_click = move |_| _set_count,.update(|count| *count += 1);

    

    view! { cx,
        <div>
        <NavBar/>        
            <h1>"Picto-Grammoni"</h1>

            <div>
                // feed of pics in here
                
            </div>
            // <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
