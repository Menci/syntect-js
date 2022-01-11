#[cfg(feature = "target_node")]
fn main() {
  extern crate napi_build;
  napi_build::setup();
}

#[cfg(feature = "target_wasm")]
fn main() {}
