use std::{convert::Infallible, path::PathBuf, str::FromStr, sync::Arc};

use once_cell::sync::OnceCell;
use swc::{
  common::{errors::Handler, FileName, FilePathMapping, SourceMap},
  config::{ModuleConfig, Options, SourceMapsConfig, TransformConfig},
  ecmascript::{
    ast::Module,
    parser::{Syntax, TsConfig},
  },
  Compiler, TransformOutput,
};
use swc_ecma_visit::Fold;
use swc_ecmascript::ast::ImportDecl;
use thiserror::Error;

static COMPILER: OnceCell<(Compiler, Handler, Options)> = OnceCell::new();

#[derive(Error, Debug)]
pub enum TransformError {
  #[error("File path is invalid: {0}, reason: {1}")]
  InvalidFilePath(String, Infallible),
  #[error("Transform failed: {0}")]
  TransformFailed(anyhow::Error),
}

#[inline(always)]
fn get_compiler() -> &'static (Compiler, Handler, Options) {
  COMPILER.get_or_init(|| {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Handler::with_tty_emitter(
      swc_common::errors::ColorConfig::Always,
      true,
      false,
      Some(cm.clone()),
    );
    let mut options: Options = Default::default();
    options.swcrc = false;
    options.output_path = None;
    options.source_maps = Some(SourceMapsConfig::Bool(true));
    options.is_module = true;
    options.config.module = Some(ModuleConfig::Es6);
    options.config.jsc.external_helpers = false;
    options.config.jsc.keep_class_names = true;
    let mut ts_config = TsConfig::default();
    ts_config.tsx = true;
    ts_config.dynamic_import = true;
    ts_config.dts = false;
    ts_config.import_assertions = true;
    ts_config.decorators = true;
    options.config.jsc.syntax = Some(Syntax::Typescript(ts_config));
    let mut transform_options = TransformConfig::default();
    transform_options.legacy_decorator = true;
    options.config.jsc.transform = Some(transform_options);
    (Compiler::new(cm.clone()), handler, options)
  })
}

pub fn transform(
  source: String,
  filename: &str,
) -> std::result::Result<TransformOutput, TransformError> {
  let (c, h, o) = get_compiler();
  let fm = c.cm.new_source_file(
    FileName::Real(
      PathBuf::from_str(filename)
        .map_err(|err| TransformError::InvalidFilePath(filename.to_owned(), err))?,
    ),
    source,
  );
  c.process_js_with_custom_pass(fm, h, o, EsmImportDetectFold)
    .map_err(|e| TransformError::TransformFailed(e))
}

struct EsmImportDetectFold;

impl Fold for EsmImportDetectFold {
  fn fold_import_decl(&mut self, mut i: ImportDecl) -> ImportDecl {
    println!("{:?}", i);
    i
  }
}
