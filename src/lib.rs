#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counter_1: 0,
        counter_2: 0
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    counter_1: i32,
    counter_2: i32
}


// ------ ------
//    Update
// ------ ------

enum Msg {
    Increment1,
    Decrement1,
    Increment2,
    Decrement2,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment1 => model.counter_1 += 1,
        Msg::Decrement1 => model.counter_1 -= 1,
        Msg::Increment2 => model.counter_2 += 1,
        Msg::Decrement2 => model.counter_2 -= 1,
    }
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
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
