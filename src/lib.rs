use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = Math)]
  fn random() -> f64;
}

#[wasm_bindgen]
pub struct Example {
  my_number: f64,
  my_vec: Vec<f32>,
}

#[wasm_bindgen]
impl Example {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Example {
    log("instantiating new Example");

    Example {
      my_number: random(),
      my_vec: vec![0.1, 0.2, 9.56],
    }
  }

  #[wasm_bindgen(getter)]
  pub fn my_number(&self) -> f64 {
    self.my_number
  }

  #[wasm_bindgen(setter)]
  pub fn set_my_number(&mut self, my_number: f64) {
    self.my_number = my_number;
  }

  #[wasm_bindgen(getter)]
  pub fn my_vec(&self) -> Float32Array {
    unsafe { Float32Array::view(&self.my_vec) }
  }

  #[wasm_bindgen(setter)]
  pub fn set_my_vec(&mut self, my_vec: Float32Array) {
    for i in 0..self.my_vec.len() {
      self.my_vec[i] = my_vec.at(i.try_into().unwrap()).unwrap_or(0.);
    }
  }

  pub fn sum_my_vec(&self) -> f32 {
    self.my_vec.iter().sum()
  }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
