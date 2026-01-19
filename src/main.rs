use dioxus::{logger::tracing::warn, prelude::*};

const HEADER_SVG: Asset = asset!("/assets/header.svg");

const LOGO: Asset = asset!("/assets/logo.png");


const BLOG_2: Asset = asset!("/assets/blog/2.png");
const BLOG_1: Asset = asset!("/assets/blog/1.png");
const STEAM: Asset = asset!("/assets/icons/steam.svg");
const ITCHIO: Asset = asset!("/assets/icons/itchio.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {

        link {
            href: "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css",
            rel: "stylesheet",
        }
        Primary {}

    }
}

#[component]
pub fn Primary() -> Element {
    
    
    
    
    
    
    rsx! {

        Navbar {}

        News {
            title: "DANGERBALL RELEASE 1.0",
            features: "We added nothing!",
            bugfixes: "we fixed nothing!",
            img: BLOG_1,
        }
        br {}
        News {
            title: "DANGERBALL RELEASE 2.0",
            features: "everything",
            bugfixes: "the ai no longer craves cheese",
            img: BLOG_2,
        }

        // News {}
    }
}
fn class(state: Signal<bool>, class: &'static str) -> &'static str {
    if state() {
        class
    } else {
        ""
    }
}

#[component]
pub fn Navbar() -> Element {
    // navbar open and close
    let mut menu = use_signal(|| false);
    let nav_class = class(menu, "is-active");

    rsx! {

        div { class: "box",
            nav {
                aria_label: "main navigation",
                class: "navbar",
                role: "navigation",
                div { class: "navbar-brand",
                    a { class: "navbar-item", href: "https://bulma.io",

                        // figure { class: "image is-128x128",
                        //     img { src: "{LOGO}", style: "max-height: fit-content;" }

                        // }
                        // img { src: "{LOGO}" }
                        strong { style: " background-color: black; padding: 10px; color: white; border-radius: 10px;",
                            "GLASS MANIA"
                        }
                    
                    }
                    a {
                        aria_expanded: "false",
                        aria_label: "menu",
                        class: "navbar-burger",
                        "data-target": "navbarBasicExample",
                        role: "button",
                        onclick: move |_| { menu.set(!menu()) },
                        span { aria_hidden: "true" }
                        span { aria_hidden: "true" }
                        span { aria_hidden: "true" }
                        span { aria_hidden: "true" }
                    
                    }
                }

                div {
                    class: "navbar-menu {nav_class}",
                    id: "navbarBasicExample",
                    div { class: "navbar-start",
                        a { class: "navbar-item", "Home" }
                        a { class: "navbar-item", "About" }
                        div { class: "navbar-item has-dropdown is-hoverable",
                            a { class: "navbar-link", "Games" }
                            div { class: "navbar-dropdown",
                                a { class: "navbar-item", "Dangerball" }
                            
                            }
                        }
                        div { class: "navbar-item has-dropdown is-hoverable",
                            a { class: "navbar-link", "Links" }
                            div { class: "navbar-dropdown",
                                a { class: "navbar-item", "Discord" }
                                a { class: "navbar-item", "itch.io" }
                                a { class: "navbar-item", "Steam" }
                            
                            }
                        }
                    }
                    div { class: "navbar-end",
                        div { class: "navbar-item",
                            div { class: "buttons", style: "",

                                img { src: "{ITCHIO}" }

                                img { src: "{STEAM}" }
                            
                            }
                        
                        }
                    
                    }
                
                }
            }
        
        }
    }
}

#[component]
pub fn News(title: String, features: String, bugfixes: String, img: Asset) -> Element {
    rsx! {

        div { class: "container is-widescreen",
            div { class: "box",

                div { class: "content",

                    h2 { "{title}" }

                    div { class: "columns",
                        div { class: "column is-half",

                            h4 { "New Features:" }
                            p { "{features}" }

                            h4 { "Bug fixes:" }
                            p { "{bugfixes}" }
                        
                        }
                        div { class: "column",

                            figure { class: "image is-square",
                                img {
                                    src: "{img}",
                                    style: "border-radius: 4px;",
                                }
                            }
                        
                        }
                    
                    }
                    div { class: "field has-addons",
                        p { class: "control",
                            button { class: "button",

                                span { "Expand" }
                            }
                        }

                        p { class: "control",
                            button { class: "button",

                                span { "Dismiss" }
                            }
                        }
                    }
                }
            }
        }
    }
}
