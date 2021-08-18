#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{CallContext, JsObject, JsUndefined, Result};

#[cfg(all(
  not(target_env = "musl"),
  not(target_os = "android"),
  not(all(target_os = "windows", target_arch = "aarch64")),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("register", register)?;

  Ok(())
}

#[js_function]
fn register(ctx: CallContext) -> Result<JsUndefined> {
  ctx.env.get_undefined()
}
