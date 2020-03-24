use seed::{prelude::*, *};

struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self { val: 0 }
    }
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.val += 1,
        Msg::Decrement => model.val -= 1,
    }
}

fn view(model: &Model) -> impl View<Msg> {
    nodes![
        button![
            class!("ml-4 mt-4 bg-green-500 text-white p-2 rounded text-2xl font-bold"),
            simple_ev(Ev::Click, Msg::Increment),
            format!("+")
        ],
        button![
            class!("ml-4 mt-4 bg-red-500 text-white p-2 rounded text-2xl font-bold"),
            simple_ev(Ev::Click, Msg::Decrement),
            format!("-")
        ],
        span![
            class!("ml-4 mt-4 bg-blue-500 text-white p-2 rounded text-2xl font-bold"),
            format!("{}", model.val)
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
