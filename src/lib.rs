#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]
use seed::{prelude::*, *};
use serde::{Deserialize};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counter_1: 0,
        counter_2: 0,
        my_ip: String::new(),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    counter_1: i32,
    counter_2: i32,
    my_ip: String,
}

#[derive(Deserialize, Debug)]
struct ResponsedIp {
    message: String,
    myip: String,
    xforwardedfor: String,
    ua: String,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Increment1,
    Decrement1,
    Increment2,
    Decrement2,
    GetRequest,
    FetchedIp(fetch::Result<ResponsedIp>),
}

// fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment1 => model.counter_1 += 1,
        Msg::Decrement1 => model.counter_1 -= 1,
        Msg::Increment2 => model.counter_2 += 1,
        Msg::Decrement2 => model.counter_2 -= 1,
        Msg::GetRequest => {
            orders.skip().perform_cmd({
                async {
                    Msg::FetchedIp(get_ip().await)
                }
            });
        }, Msg::FetchedIp(Ok(response_data)) => {
           model.my_ip = response_data.myip;
        }
        Msg::FetchedIp(Err(error)) => {
            log!(error);
            model.my_ip = "Error Occurred in Fetching My Ip.".to_string();
        }
    }
}

async fn get_ip() -> fetch::Result<ResponsedIp> {
    Request::new("https://webtools.agedori.com/api/myip")
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["uk-container"],
        h1![
            C!["uk-heading-medium"],
            "カウンターアプリ"
        ],
        div![
            div![model.counter_1],
            button![
                C!["uk-button uk-button-default uk-button-small"],
                ev(Ev::Click, |_| Msg::Decrement1), "減算1"
            ],
            button![
                C!["uk-button uk-button-default uk-button-small"],
                ev(Ev::Click, |_| Msg::Increment1), "加算1"
            ],
        ],
        div![
            C!["uk-divider-small"],
        ],
        div![
            div![model.counter_2],
            button![
                C!["uk-button uk-button-default uk-button-small"],
                ev(Ev::Click, |_| Msg::Decrement2), "減算2"
            ],
            button![
                C!["uk-button uk-button-default uk-button-small"],
                ev(Ev::Click, |_| Msg::Increment2), "加算2"
            ],
        ],
        div![
            C!["uk-divider-small"],
        ],
        div![
            div![model.my_ip.clone()],
            button![
                C!["uk-button uk-button-default uk-button-small"],
                ev(Ev::Click, |_| Msg::GetRequest), "IP取得"
            ],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
