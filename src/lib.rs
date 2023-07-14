use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub async fn start(rule: i32) {
    let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    let mut current_state = vec![false; canvas.client_width() as usize];
    let mut next_state: Vec<bool> = vec![false; canvas.client_width() as usize];
    let height = canvas.client_height();
    let mut current_row = 0;

    // setting starting point
    current_state[500] = true;

    while current_row < height {
        print_line(
            &context,
            &canvas,
            current_row,
            &mut current_state,
            &mut next_state,
            rule,
        );

        // sleep for 100ms after each rowW
        #[allow(unused_must_use)] {
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 100)
                    .unwrap();
            });

            wasm_bindgen_futures::JsFuture::from(promise).await;
        }

        current_row = current_row + 1;
    }
}

fn print_line(
    context: &CanvasRenderingContext2d,
    canvas: &HtmlCanvasElement,
    current_row: i32,
    current_state: &mut Vec<bool>,
    next_state: &mut Vec<bool>,
    rule: i32,
) {
    let width: i32 = canvas.client_width();

    let mut left: bool;
    let mut current: bool;
    let mut right: bool;
    let mut i: usize = 0;

    while i < width as usize {
        if current_state[i as usize] {
            context.set_fill_style(&JsValue::from_str("black"));
        } else {
            context.set_fill_style(&JsValue::from_str("white"));
        }
        
        context.fill_rect(
            i as f64,
            current_row as f64,
            1 as f64,
            1 as f64
        );

        if i == 0 {
            left = false;
            current = current_state[0];
            right = current_state[1];
        } else if i == current_state.len() - 1 {
            left = current_state[i-1];
            current = current_state[i];
            right = false;
        } else {
            left = current_state[i-1];
            current = current_state[i];
            right = current_state[i+1];
        }

        let mut n = 0;
        if left == true {
            n |= 4;
        }
        if current == true {
            n |= 2;
        }
        if right == true {
            n |= 1;
        }
                    
        next_state[i] = (rule & i32::pow(2, n)) > 0;

        i = i + 1;
    }

    // clear current state and replace with next state
    current_state.clear();
    current_state.extend(next_state.iter().cloned());
}