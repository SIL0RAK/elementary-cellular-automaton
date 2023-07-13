use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
fn start() {
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

    let mut current_row = 0;
    let mut current_state = vec![false; canvas.client_width() as usize];
    current_state[100] = true;
    let mut next_state = vec![false; canvas.client_width() as usize];
    let height = canvas.client_height();

    while current_row < height {
        print_line(
            &context,
            &canvas,
            current_row,
            &mut current_state,
            &mut next_state,
        );

        current_row = current_row + 1;
    }
}

fn print_line(
    context: &CanvasRenderingContext2d,
    canvas: &HtmlCanvasElement,
    current_row: i32,
    current_state: &mut Vec<bool>,
    next_state: &mut Vec<bool>,
) {

    let width: i32 = canvas.client_width();

    let mut left: bool;
    let mut current: bool;
    let mut right: bool;
    let mut i: usize = 0;

    while i < width as usize {
        if current_state[i as usize] {
            context.set_fill_style(&JsValue::from_str("red"));
        } else {
            context.set_fill_style(&JsValue::from_str("green"));
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
                    
        next_state[i] = (110 & i32::pow(2, n)) > 0;

        i = i + 1;
    }

    current_state.clear();
    current_state.extend(next_state.iter().cloned());
}