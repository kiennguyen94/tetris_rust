use js_sys::Reflect;
use tetris::Tetris;
use wasm_bindgen::JsValue;
use wasm_react::{c, export_components, h, hooks::use_state, props::Style, Component};

mod shape;
mod tetris;

///
/// wasm build note: wasm-opt doens't work on apple ARM. See fix:
/// https://github.com/rustwasm/wasm-pack/issues/913#issuecomment-803563120
pub struct App {
    width: u32,
    height: u32,
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(App {
            width: Reflect::get(&value, &"width".into())?
                .as_f64()
                .unwrap_or(10.0) as u32,
            height: Reflect::get(&value, &"height".into())?
                .as_f64()
                .unwrap_or(30.0) as u32,
        })
    }
}

impl Component for App {
    fn render(&self) -> wasm_react::VNode {
        let tetris = use_state(|| Tetris::new(self.width as usize, self.height as usize));

        h!(div)
            .style(
                &Style::new()
                    .display("inline-grid")
                    .grid_template(format!(
                        "repeat({}, len) / repeat({}, len)",
                        self.height, self.width
                    ))
                    .border("1px solid grey"),
            )
            .build(c![..tetris.value().iter_positions().map(|pos| {
                let typ = tetris.value().get(pos);
                h!(div).build(c![typ.unwrap_or_default()])
            })])
    }
}

export_components! {
    App
}
