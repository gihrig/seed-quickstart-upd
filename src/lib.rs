use seed::{prelude::*, *};

#[derive(Debug)]
struct Model {
    pub val: i32,
    pub html: &'static str,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
            html: include_str!("./components/counter.html"),
        }
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
    #[derive(Debug)]
    let mut nodes: Vec<Node<Msg>> = raw!(&model.html);
    log!(nodes.as_mut_slice());
    if let [decrement, counter, increment] = nodes.as_mut_slice() {
        log!(counter);
        //     decrement.add_event_handler(ev(Ev::Click, |_| Msg::Decrement));
        counter.replace_text(model.val.to_string());
        //     increment.add_event_handler(ev(Ev::Click, |_| Msg::Increment));
    }
    log!(nodes);
    nodes

    // nodes![
    //     button![
    //         class!("ml-4 mt-4 bg-green-500 text-white p-2 rounded text-2xl font-bold"),
    //         simple_ev(Ev::Click, Msg::Increment),
    //         format!("+")
    //     ],
    //     button![
    //         class!("ml-4 mt-4 bg-red-500 text-white p-2 rounded text-2xl font-bold"),
    //         simple_ev(Ev::Click, Msg::Decrement),
    //         format!("-")
    //     ],
    //     span![
    //         class!("ml-4 mt-4 bg-blue-500 text-white p-2 rounded text-2xl font-bold"),
    //         format!("{}", model.val)
    //     ],
    // ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
