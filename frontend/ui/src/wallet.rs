use dioxus::prelude::*;
use routes::*;

/// Const for the Account Section CSS.
const MAIN_CSS: Asset = asset!("assets/styling/main.css");

/// Account dashboard component that is shown in the main page.
#[component]
pub fn Wallet() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            id: "page",
            Header{}
            div { style: "display: inline-block; margin-bottom: 14px;" }
            Balance{}
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
                a { id:"h2", style: "font-weight: bold;", "Shelby Company Ltd." }
                div { id:"secondary", a { "5 Participants" } }
            }
        }
    }
}

/// Balance component that goes inside the account component.
#[component]
fn Balance() -> Element {
    let account = "nano_19kqrk7taqnprmy1hcchpkdcpfqnpm7knwdhn9qafhd7b94s99ofngf5ent1";

    let balance_future = use_resource(|| async { get_account_balance(account).await });
    let balance_info: AccountBalanceResponse = match &*balance_future.read_unchecked() {
        Some(res) => (*res).clone(),
        None => AccountBalanceResponse::new(),
    };

    let nano_price_future = use_resource(|| async { get_nano_price_euro().await });
    let nano_price = match &*nano_price_future.read_unchecked() {
        Some(res) => (*res).clone(),
        None => NanoPriceResponse {
            nano: Some(NanoPriceEuro { eur: Some(0.) }),
        },
    };

    let balance_nano = match balance_info.balance_nano {
        Some(nano) => match nano.parse::<f32>() {
            Ok(nano) => nano,
            Err(_) => 0.,
        },
        None => 0.,
    };

    let pending_nano = match balance_info.pending_nano {
        Some(nano) => nano,
        None => String::from("0.0"),
    };

    let nano_price = match nano_price.nano {
        Some(nano) => match nano.eur {
            Some(price) => price,
            None => 0.,
        },
        None => 0.,
    };

    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "TOTAL BALANCE" }
            div {
                style: "display: inline-block; margin-bottom: 36px;",
                div {
                    id: "fill-card",
                    span { id: "sub-heading" , "XNO" }
                    strong { id: "h1" , {balance_nano.clone().to_string()} }
                }
                div {
                    id: "fill-card",
                    span { id: "secondary" , "~EUR" }
                    div {
                        id: "secondary" ,
                        strong { id: "sub-heading" , {format!("{:.2}", nano_price * balance_nano)} {"€"} }
                    }
                }
            }
            div {
                div {
                    id: "container",
                    style: "display: inline-block; margin-bottom: 14px;",
                    div {
                        id: "fill-card",
                        span { id: "secondary", "PENDING" }
                    }
                }
                div {
                    id: "container",
                    div {
                        id: "fill-card",
                        span { style: "display: inline-block; padding-right: 10px; align-items: center;", "XNO" }
                        strong { id: "sub-heading" , {pending_nano} }
                    }
                }
            }
        }
    }
}
