use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

/// `init` is called only once, when the app is ready to run.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        val: 0,
        html: include_str!("./components/counter.html"),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    val: i32,
    html: &'static str,
}

// ------ ------
//    Update
// ------ ------

// `Debug` on `Msg` is necessary for logging many things - for instance `Node`s.
// Without `Debug`:
//  - Compilation fails on `stable` Rust ("`Msg` cannot be formatted using `{:?}`").
//  - Error message will be logged on `nightly` Rust -
//    e.g. "[<unknown> of type &alloc::vec::Vec<seed::virtual_dom::node::Node<appname::Msg>> is !Debug]".
//    The reason - it allows us to debug things that are nested in multiple levels of functions with generic `Msg`.
//    (We don't want to add general `Debug` restriction for `Msg` to make it usable also in the current `stable`.)
// (`nightly` / `stable` Rust can be chosen by many ways, for instance `rustup default stable`).
#[derive(Debug)]
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

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    // Parse HTML string into virtual nodes.
    // It's basically vector of trees because element nodes may contain children.
    let mut nodes: Vec<Node<Msg>> = raw!(&model.html);
    // See trees.
    log!(nodes);

    // Find virtual node by id and add modify it.
    update_virtual_dom_node("decrement", &mut nodes, &|node| {
        node.add_event_handler(ev(Ev::Click, |_| Msg::Decrement));
    });
    update_virtual_dom_node("increment", &mut nodes, &|node| {
        node.add_event_handler(ev(Ev::Click, |_| Msg::Increment));
    });
    update_virtual_dom_node("counter", &mut nodes, &|node| {
        node.replace_text(model.val.to_string());
    });

    // Return updated nodes to render them on the next browser animation frame.
    nodes
}

/// Recursive find & update node by id.
// `id: &str` - search for the node with this id ; (`&str` accepts string literals and string slices)
// `&mut [Node<Msg>]` - trees where we will be searching ; (`&mut [..` accepts slices and vector references)
// `&dyn Fn(&mut Node<Msg>)`
//    - Callback for node update.
//    - It's reference so `updater` propagation through recursive calls is simpler - no need to copy or clone that callback
//      when the compiler isn't smart enough to find out that it will be called only once.
//    - `dyn` is just a mark that `Fn` is trait and not another type like a struct or an enum.
//    - `Fn` trait implements all functions or closures that can be called multiple times.
//       (Possible alternative to `&dyn Fn(&mut Node<Msg>)` would be e.g. `impl FnOnce(&mut Node<Msg>) + Copy`).
fn update_virtual_dom_node(id: &str, nodes: &mut [Node<Msg>], updater: &dyn Fn(&mut Node<Msg>)) {
    // Note: Seed hasn't got dedicated API for VDOM traversing but the most things are public now,
    // so we can use many "internal" values and types

    for node in nodes {
        let el = match node {
            Node::Element(el) => el,
            _ => continue
        };
        match el.attrs.vals.get(&At::Id) {
            // (Pattern with a guard.)
            Some(AtValue::Some(el_id)) if el_id == id => {
                updater(node);
                break
            },
            // Do nothing (it will be unnecessary once guards are implemented also for `if let ..`)
            _ => ()
        }
        // Traverse through descendants.
        // (Note: Rust can't do tail-call elimination (yet), so it's relatively easy to overflow stack).
        update_virtual_dom_node(id, &mut el.children, updater);
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    // "app" is id of the root / mount element. It can be also `web_sys::Node` or another supported type.
    App::start("app", init, update, view);
}
