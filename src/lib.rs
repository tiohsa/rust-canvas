use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
struct TableCell {
    x: f64,
    y: f64,
    height: f64,
    width: f64,
    text: String,
}

impl TableCell {
    fn is_inside(&self, x: f64, y: f64) -> bool {
        self.x <= x && x <= (self.x + self.width) && self.y <= y && y <= (self.y + self.height)
    }
}

struct Table {
    x: f64,
    y: f64,
    height: f64,
    width: f64,
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    start();

    Ok(())
}

pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    let red = JsValue::from_str("rgba(255, 0, 0, 1.0)");
    let green = JsValue::from_str("rgba(0, 255, 0, 1.0)");
    let blue = JsValue::from_str("rgba(0, 0, 255, 1.0)");

    let row_num = 50;
    let column_num = 144;
    let width = 20.0;
    let height = 40.0;
    let offset_x = canvas.offset_left();
    let offset_y = canvas.offset_top();

    let table = Table {
        x: 0.0,
        y: 0.0,
        height: height * row_num as f64,
        width: width * column_num as f64,
    };
    canvas.set_width(table.width as u32);
    canvas.set_height(table.height as u32);

    let mut cells = Vec::new();
    for row_index in 0..row_num {
        let y = height * row_index as f64;
        for column_index in 0..column_num {
            let x = width * column_index as f64;
            let style = match (column_index % 3 + row_index) % 3 {
                0 => &red,
                1 => &green,
                _ => &blue,
            };

            context.set_fill_style(&style);
            context.move_to(x, y);
            context.fill_rect(x, y, width, height);
            let text = format!("{}", column_index % 3 + row_index % 3);
            cells.push(TableCell {
                x: x,
                y: y,
                height,
                width,
                text,
            });
        }
    }

    let info = document
        .get_element_by_id("info")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()?;
    let info = Rc::new(info);
    let cells = Rc::new(cells);
    // mouseover event
    {
        let info = info.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x();
            let y = event.offset_y();
            let cell_x = x / width as i32;
            let cell_y = y / height as i32;
            let index = cell_x as i32 + cell_y as i32 * column_num;
            if let Some(cell) = cells.get(index as usize) {
                if cell.is_inside(x as f64, y as f64) {
                    let pointer_x = cell.x as i32 + offset_x;
                    let pointer_y = cell.y as i32 + offset_y - info.client_height();
                    info.style()
                        .set_property("left", &format!("{}px", pointer_x))
                        .unwrap();
                    info.style()
                        .set_property("top", &format!("{}px", pointer_y))
                        .unwrap();
                    info.set_inner_html(&cell.text);
                    info.style().set_property("visibility", "visible").unwrap();
                } else {
                    info.style().set_property("visibility", "hidden").unwrap();
                }
            } else {
                info.style().set_property("visibility", "hidden").unwrap();
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    // mouseout event
    {
        let info = info.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            info.style().set_property("visibility", "hidden").unwrap();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseout", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
