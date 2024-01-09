//! xlsxwriter-rs
//! =============
//!
//! Rust binding of [libxlsxwriter](https://github.com/jmcnamara/libxlsxwriter).
//!
//! If you are looking for native rust port of libxlsxwriter, please try
//! [rust_xlsxwriter](https://crates.io/crates/rust_xlsxwriter) which is developed by original libxlsxwriter author.
//!
//! ** API of this library is not stable. **
//!
//! Supported Features
//! ------------------
//!
//! * 100% compatible Excel XLSX files.
//! * Full Excel formatting.
//! * Merged cells.
//! * Autofilters.
//! * Table.
//! * Conditional Format.
//! * Validation.
//! * Data validation and drop down lists.
//! * Worksheet PNG/JPEG images.
//! * Cell comments.
//!
//! Coming soon
//! -----------
//! * Charts.
//!
//! Examples
//! --------
//!
//! ![Result Image](https://github.com/informationsea/xlsxwriter-rs/raw/master/images/simple1.png)
//!
//! ```rust
//! use xlsxwriter::prelude::*;
//!
//! # fn main() -> Result<(), XlsxError> {
//! let workbook = Workbook::new("simple1.xlsx")?;
//!
//! let mut sheet1 = workbook.add_worksheet(None)?;
//! sheet1.write_string(0, 0, "Red text", Some(&Format::new().set_font_color(FormatColor::Red)))?;
//! sheet1.write_number(0, 1, 20., None)?;
//! sheet1.write_formula_num(1, 0, "=10+B1", None, 30.)?;
//! sheet1.write_url(
//!     1,
//!     1,
//!     "https://github.com/informationsea/xlsxwriter-rs",
//!     Some(&Format::new().set_font_color(FormatColor::Blue).set_underline(FormatUnderline::Single)),
//! )?;
//! sheet1.merge_range(2, 0, 3, 2, "Hello, world", Some(
//!     &Format::new().set_font_color(FormatColor::Green).set_align(FormatAlignment::CenterAcross)
//!                   .set_vertical_align(FormatVerticalAlignment::VerticalCenter)))?;
//!
//! sheet1.set_selection(1, 0, 1, 2);
//! sheet1.set_tab_color(FormatColor::Cyan);
//! workbook.close()?;
//! # Ok(())
//! # }
//! ```
//!
//! Please read [original libxlsxwriter document](https://libxlsxwriter.github.io/worksheet_8h.html) for description missing functions.
//! Most of this document is based on libxlsxwriter document.
//!
//! Migration Guide
//! ---------------
//!
//! ### Upgrade from prior version 0.5
//!
//! 1. Replace `use xlsxwriter::*` with `use xlsxwriter::prelude::*`
//! 2. Use [`Format::new`] instead of [`Workbook::add_format`]
//! 3. [`Format`] object's methods now return mutable reference. Please rewrite code to adopt this change.
//! 4. Some functions now return `Result<T, XlsxError>`. Please rewrite code to adopt this change.

extern crate libxlsxwriter_sys;

/// Manipulate Charts.
pub mod chart;
mod error;

/// Manipulate Formats.
pub mod format;

/// xlsxwriter prelude.
pub mod prelude;

/// Manipulate Workbook.
pub mod workbook;

/// Manipulate Worksheets.
pub mod worksheet;

use std::{ffi::CString, os::raw::c_char, pin::Pin};

use chart::*;
use error::XlsxErrorSource;
use format::*;
use worksheet::*;

pub use format::Format;
pub use workbook::Workbook;
pub use worksheet::Worksheet;

/// The error types for xlsxwriter.
#[derive(Debug)]
pub struct XlsxError {
    pub(crate) source: XlsxErrorSource,
}

fn convert_bool(value: bool) -> u8 {
    let result = if value {
        libxlsxwriter_sys::lxw_boolean_LXW_TRUE
    } else {
        libxlsxwriter_sys::lxw_boolean_LXW_FALSE
    };
    result as u8
}

fn convert_validation_bool(value: bool) -> u8 {
    let result = if value {
        libxlsxwriter_sys::lxw_validation_boolean_LXW_VALIDATION_ON
    } else {
        libxlsxwriter_sys::lxw_validation_boolean_LXW_VALIDATION_OFF
    };
    result as u8
}

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct CStringHelper {
    strings: Vec<Pin<Box<CString>>>,
}

impl CStringHelper {
    pub fn new() -> CStringHelper {
        CStringHelper {
            strings: Vec::new(),
        }
    }

    pub fn add(&mut self, s: &str) -> Result<*const c_char, XlsxError> {
        let s = Box::pin(CString::new(s)?);
        let p = s.as_ptr();
        self.strings.push(s);
        Ok(p)
    }

    pub fn add_opt(&mut self, s: Option<&str>) -> Result<*const c_char, XlsxError> {
        if let Some(s) = s {
            self.add(s)
        } else {
            Ok(std::ptr::null())
        }
    }
}

pub(crate) fn try_to_vec<I, T>(it: I) -> Result<Vec<T>, XlsxError>
where
    I: std::iter::Iterator<Item = Result<T, XlsxError>>,
{
    let mut r = Vec::new();
    for one in it {
        r.push(one?);
    }
    Ok(r)
}

/// String value or float number value
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum StringOrFloat {
    String(String),
    Float(f64),
}

impl Default for StringOrFloat {
    fn default() -> Self {
        StringOrFloat::Float(0.)
    }
}

impl StringOrFloat {
    #[must_use]
    pub fn to_string(self) -> Option<String> {
        match self {
            StringOrFloat::String(x) => Some(x),
            StringOrFloat::Float(_) => None,
        }
    }

    #[must_use]
    pub fn to_str(&self) -> Option<&str> {
        match self {
            StringOrFloat::String(x) => Some(x.as_str()),
            StringOrFloat::Float(_) => None,
        }
    }

    #[must_use]
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            StringOrFloat::String(_) => None,
            StringOrFloat::Float(x) => Some(*x),
        }
    }
}

impl From<&str> for StringOrFloat {
    fn from(val: &str) -> Self {
        StringOrFloat::String(val.to_string())
    }
}

impl From<String> for StringOrFloat {
    fn from(val: String) -> Self {
        StringOrFloat::String(val)
    }
}

impl From<f64> for StringOrFloat {
    fn from(val: f64) -> Self {
        StringOrFloat::Float(val)
    }
}

#[cfg(test)]
mod test;
