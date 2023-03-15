use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlDivElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let main_elm = use_node_ref();

    {
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                // let container = document.get_element_by_id("container").unwrap();
                // let container = container.dyn_ref::<web_sys::HtmlDivElement>().unwrap();

                let drop_cb = Closure::<dyn Fn()>::new(move || {
                    web_sys::console::log_1(&"dropped".into());
                });

                // container
                //     .add_event_listener_with_callback(
                //         "drop",
                //         drop_cb.as_ref().unchecked_ref(),
                //     ).unwrap();

                document.set_ondrop(Some(drop_cb.as_ref().unchecked_ref()));
                drop_cb.forget();
            },
           (),
        );
    }


    html! {
        <div ref={main_elm} id={"container"}
            style={"height: 100vh; width: 100vw; background-color: rgba(50, 0, 150, 0.25)"}>
        </div>
    }
}

