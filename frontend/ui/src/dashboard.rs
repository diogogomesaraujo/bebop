use dioxus::prelude::*;

/// Const for the Account Section CSS.
const MAIN_CSS: Asset = asset!("assets/styling/main.css");

/// Account dashboard component that is shown in the main page.
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            id: "page",
            Header{}
            div { style: "display: inline-block; margin-bottom: 14px;" }
            NewTransaction{}
            div { style: "display: inline-block; margin-bottom: 14px;" }
            Transactions{}
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        div {
            id: "header",
            div {
                id: "circle",
                style: "display: inline-block; margin-right: 14px;"
            }

            div {
                style: "display: flex; flex-direction: column;",
                a { id:"h2", style: "font-weight: bold;", "Dashboard" }
                div { id:"secondary", a { "Hi " strong { "Diogo" } ", welcome back!" } }
            }
        }
    }
}

#[component]
fn NewTransaction() -> Element {
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "START TRANSACTION" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Wallet:" }
                input {
                    id: "input",
                    // value: "{input_text}",
                    // oninput: move |event| input_text.set(event.value())
                }
            }
            div { style: "display: inline-block; margin-bottom: 14px;" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Amount (XNO):" }
                input {
                    id: "input",
                    // value: "{input_text}",
                    // oninput: move |event| input_text.set(event.value())
                }
            }
        }
    }
}

#[component]
fn Transactions() -> Element {
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "TRANSACTIONS" }
            div {
                id: "column-section",
                div {
                    id: "transaction",

                }
            }
        }
    }
}
