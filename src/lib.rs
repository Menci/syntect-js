#![deny(clippy::all)]

#[cfg(feature = "target_node")]
#[macro_use]
extern crate napi_derive;

#[cfg(feature = "target_wasm")]
use wasm_bindgen::prelude::*;

use syntect::highlighting::ThemeSet;
use syntect::html::{css_for_theme_with_class_style, ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[cfg_attr(feature = "target_node", napi(object))]
#[cfg_attr(feature = "target_wasm", wasm_bindgen(getter_with_clone))]
pub struct GetCssResult {
  pub css: String,
  pub error: String,
}

#[cfg_attr(feature = "target_node", napi(js_name = "getCSS"))]
#[cfg_attr(feature = "target_wasm", wasm_bindgen(js_name = "getCSS"))]
pub fn get_css(theme_data: String, class_name_prefix: String) -> GetCssResult {
  match ThemeSet::load_from_reader(&mut std::io::Cursor::new(theme_data)) {
    Ok(theme) => {
      let result: String;
      unsafe {
        let prefix = Box::into_raw(class_name_prefix.into_boxed_str());
        result =
          css_for_theme_with_class_style(&theme, ClassStyle::SpacedPrefixed { prefix: &*prefix });
        Box::from_raw(prefix);
      }

      return GetCssResult {
        css: result,
        error: "".to_string(),
      };
    }
    Err(error) => {
      return GetCssResult {
        css: "".to_string(),
        error: error.to_string(),
      };
    }
  }
}

fn get_syntax_set() -> &'static SyntaxSet {
  static mut CACHED_SYNTAX_SET: Option<SyntaxSet> = None;
  unsafe {
    if CACHED_SYNTAX_SET.is_none() {
      CACHED_SYNTAX_SET = Some(SyntaxSet::load_defaults_newlines());
    }
    return CACHED_SYNTAX_SET.as_ref().unwrap();
  }
}

#[cfg_attr(feature = "target_node", napi(object))]
#[cfg_attr(feature = "target_wasm", wasm_bindgen(getter_with_clone))]
pub struct HighlightResult {
  pub html: String,
  pub language: String,
}

#[cfg_attr(feature = "target_node", napi)]
#[cfg_attr(feature = "target_wasm", wasm_bindgen)]
pub fn highlight(code: String, language: String, prefix: String) -> HighlightResult {
  let syntax_set = get_syntax_set();

  let is_plain = language == "plain" || language == "plaintext";
  let mut syntax = syntax_set.find_syntax_plain_text();
  if !is_plain {
    match syntax_set
      .find_syntax_by_extension(&language)
      .or(syntax_set.find_syntax_by_name(&language))
    {
      Some(s) => syntax = s,
      _ => (),
    }
  }

  let result: String;
  unsafe {
    let prefix_str = Box::into_raw(String::from(prefix).into_boxed_str());
    let mut generator = ClassedHTMLGenerator::new_with_class_style(
      syntax,
      &syntax_set,
      ClassStyle::SpacedPrefixed {
        prefix: &*prefix_str,
      },
    );
    for line in LinesWithEndings::from(&code) {
      generator.parse_html_for_line_which_includes_newline(line);
    }
    result = generator.finalize();
    Box::from_raw(prefix_str);
  }

  return HighlightResult {
    html: result,
    language: syntax.name.clone(),
  };
}
