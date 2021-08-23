#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::str;

use dog::transform;
use napi::{CallContext, Error, JsBuffer, JsObject, JsString, Result};

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
  exports.create_named_method("transform", transform_esm)?;

  Ok(())
}

#[js_function(2)]
fn transform_esm(ctx: CallContext) -> Result<JsObject> {
  let source = ctx.get::<JsBuffer>(0)?.into_value()?;
  let filename = ctx.get::<JsString>(1)?.into_utf8()?;
  let output = transform(
    String::from_utf8(source.to_owned())
      .map_err(|err| Error::new(napi::Status::InvalidArg, format!("{}", err)))?,
    filename.as_str()?,
  )
  .map_err(|err| Error::new(napi::Status::InvalidArg, format!("{:?}", err)))?;
  let mut result = ctx.env.create_object()?;
  result.set_named_property("source", source.into_raw())?;
  Ok(result)
}
