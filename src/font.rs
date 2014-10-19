use libc;
use std;

pub struct Options {
  pub opaque: *mut libc::c_void
}

impl Options {
  pub fn create() -> Options {
    unsafe {
      let foreign_result = cairo_font_options_create();
      return Options { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_font_options_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn merge(&mut self, other: &mut Options) {
    unsafe {
      cairo_font_options_merge(self.opaque, other.opaque);
    }
  }

  pub fn hash(&mut self) -> libc::c_ulong {
    unsafe {
      let foreign_result = cairo_font_options_hash(self.opaque);
      return foreign_result;
    }
  }

  pub fn equal(&mut self, other: &mut Options) -> bool {
    unsafe {
      let foreign_result = cairo_font_options_equal(self.opaque, other.opaque);
      return foreign_result;
    }
  }

  pub fn set_antialias(&mut self, antialias: super::antialias::Antialias) {
    unsafe {
      cairo_font_options_set_antialias(self.opaque, antialias);
    }
  }

  pub fn get_antialias(&mut self) -> super::antialias::Antialias {
    unsafe {
      let foreign_result = cairo_font_options_get_antialias(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_subpixel_order(&mut self, subpixel_order: subpixel_order::SubpixelOrder) {
    unsafe {
      cairo_font_options_set_subpixel_order(self.opaque, subpixel_order);
    }
  }

  pub fn get_subpixel_order(&mut self) -> subpixel_order::SubpixelOrder {
    unsafe {
      let foreign_result = cairo_font_options_get_subpixel_order(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_hint_style(&mut self, hint_style: hint_style::HintStyle) {
    unsafe {
      cairo_font_options_set_hint_style(self.opaque, hint_style);
    }
  }

  pub fn get_hint_style(&mut self) -> hint_style::HintStyle {
    unsafe {
      let foreign_result = cairo_font_options_get_hint_style(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_hint_metrics(&mut self, hint_metrics: hint_metrics::HintMetrics) {
    unsafe {
      cairo_font_options_set_hint_metrics(self.opaque, hint_metrics);
    }
  }

  pub fn get_hint_metrics(&mut self) -> hint_metrics::HintMetrics {
    unsafe {
      let foreign_result = cairo_font_options_get_hint_metrics(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_font_options_create() -> *mut libc::c_void;
  fn cairo_font_options_status(self_arg: *mut libc::c_void) -> super::Status;
  fn cairo_font_options_merge(self_arg: *mut libc::c_void, other: *mut libc::c_void);
  fn cairo_font_options_hash(self_arg: *mut libc::c_void) -> libc::c_ulong;
  fn cairo_font_options_equal(self_arg: *mut libc::c_void, other: *mut libc::c_void) -> bool;
  fn cairo_font_options_set_antialias(self_arg: *mut libc::c_void, antialias: super::antialias::Antialias);
  fn cairo_font_options_get_antialias(self_arg: *mut libc::c_void) -> super::antialias::Antialias;
  fn cairo_font_options_set_subpixel_order(self_arg: *mut libc::c_void, subpixel_order: subpixel_order::SubpixelOrder);
  fn cairo_font_options_get_subpixel_order(self_arg: *mut libc::c_void) -> subpixel_order::SubpixelOrder;
  fn cairo_font_options_set_hint_style(self_arg: *mut libc::c_void, hint_style: hint_style::HintStyle);
  fn cairo_font_options_get_hint_style(self_arg: *mut libc::c_void) -> hint_style::HintStyle;
  fn cairo_font_options_set_hint_metrics(self_arg: *mut libc::c_void, hint_metrics: hint_metrics::HintMetrics);
  fn cairo_font_options_get_hint_metrics(self_arg: *mut libc::c_void) -> hint_metrics::HintMetrics;
}

impl std::clone::Clone for Options {
  fn clone(&self) -> Options {
    unsafe {
      let foreign_result = cairo_font_options_copy(self.opaque);
      return Options { opaque: foreign_result as *mut libc::c_void };
    }
  }
}

extern {
  fn cairo_font_options_copy(self_arg: *mut libc::c_void) -> *mut libc::c_void;
}

impl std::ops::Drop for Options {
  fn drop(&mut self) {
    unsafe {
      cairo_font_options_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_font_options_destroy(self_arg: *mut libc::c_void);
}


pub struct FontFace {
  pub opaque: *mut libc::c_void
}

impl FontFace {
  pub fn create_toy(family: &str, slant: slant::Slant, weight: weight::Weight) -> FontFace {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_toy_font_face_create(family.to_c_str().unwrap() as *mut i8, slant, weight);
      return FontFace { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn toy_get_family(&mut self) -> std::string::String {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_toy_font_face_get_family(self.opaque);
      return std::c_str::CString::new(foreign_result as *const i8, false).as_str().unwrap().to_owned();
    }
  }

  pub fn toy_get_slant(&mut self) -> slant::Slant {
    unsafe {
      let foreign_result = cairo_toy_font_face_get_slant(self.opaque);
      return foreign_result;
    }
  }

  pub fn toy_get_weight(&mut self) -> slant::Slant {
    unsafe {
      let foreign_result = cairo_toy_font_face_get_weight(self.opaque);
      return foreign_result;
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_font_face_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_type(&mut self) -> font_type::FontType {
    unsafe {
      let foreign_result = cairo_font_face_get_type(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_reference_count(&mut self) -> libc::c_uint {
    unsafe {
      let foreign_result = cairo_font_face_get_reference_count(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_toy_font_face_create(family: *mut i8, slant: slant::Slant, weight: weight::Weight) -> *mut libc::c_void;
  fn cairo_toy_font_face_get_family(self_arg: *mut libc::c_void) -> *mut i8;
  fn cairo_toy_font_face_get_slant(self_arg: *mut libc::c_void) -> slant::Slant;
  fn cairo_toy_font_face_get_weight(self_arg: *mut libc::c_void) -> slant::Slant;
  fn cairo_font_face_status(self_arg: *mut libc::c_void) -> super::Status;
  fn cairo_font_face_get_type(self_arg: *mut libc::c_void) -> font_type::FontType;
  fn cairo_font_face_get_reference_count(self_arg: *mut libc::c_void) -> libc::c_uint;
}

impl std::clone::Clone for FontFace {
  fn clone(&self) -> FontFace {
    unsafe {
      let foreign_result = cairo_font_face_reference(self.opaque);
      return FontFace { opaque: foreign_result as *mut libc::c_void };
    }
  }
}

extern {
  fn cairo_font_face_reference(self_arg: *mut libc::c_void) -> *mut libc::c_void;
}

impl std::ops::Drop for FontFace {
  fn drop(&mut self) {
    unsafe {
      cairo_font_face_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_font_face_destroy(self_arg: *mut libc::c_void);
}


pub struct ScaledFont {
  pub opaque: *mut libc::c_void
}

impl ScaledFont {
  pub fn create(font_face: &mut FontFace, font_matrix: &mut super::matrix::Matrix, ctm: &mut super::matrix::Matrix, options: &mut Options) -> ScaledFont {
    unsafe {
      let foreign_result = cairo_scaled_font_create(font_face.opaque, font_matrix, ctm, options.opaque);
      return ScaledFont { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_scaled_font_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn extents(&mut self) -> FontExtents {
    unsafe {
      let mut extents:FontExtents = std::intrinsics::init();
      cairo_scaled_font_extents(self.opaque, &mut extents);
      return extents;
    }
  }

  pub fn text_extents(&mut self, utf8: &str) -> TextExtents {
    unsafe {
      use std::c_str::ToCStr;
      let mut extents:TextExtents = std::intrinsics::init();
      cairo_scaled_font_text_extents(self.opaque, utf8.to_c_str().unwrap() as *mut i8, &mut extents);
      return extents;
    }
  }

  pub fn glyph_extents(&mut self, glyphs: &[Glyph]) -> TextExtents {
    unsafe {
      let mut extents:TextExtents = std::intrinsics::init();
      cairo_scaled_font_glyph_extents(self.opaque, glyphs.as_ptr() as *mut Glyph, glyphs.len() as libc::c_int, &mut extents);
      return extents;
    }
  }

  pub fn get_font_face(&mut self) -> FontFace {
    unsafe {
      let foreign_result = cairo_scaled_font_get_font_face(self.opaque);
      return FontFace { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn get_font_options(&mut self, options: &mut FontExtents) {
    unsafe {
      cairo_scaled_font_get_font_options(self.opaque, options);
    }
  }

  pub fn get_font_matrix(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut font_matrix:super::matrix::Matrix = std::intrinsics::init();
      cairo_scaled_font_get_font_matrix(self.opaque, &mut font_matrix);
      return font_matrix;
    }
  }

  pub fn get_ctm(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut ctm:super::matrix::Matrix = std::intrinsics::init();
      cairo_scaled_font_get_ctm(self.opaque, &mut ctm);
      return ctm;
    }
  }

  pub fn get_scale_matrix(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut scale_matrix:super::matrix::Matrix = std::intrinsics::init();
      cairo_scaled_font_get_scale_matrix(self.opaque, &mut scale_matrix);
      return scale_matrix;
    }
  }

  pub fn get_type(&mut self) -> font_type::FontType {
    unsafe {
      let foreign_result = cairo_scaled_font_get_type(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_reference_count(&mut self) -> libc::c_uint {
    unsafe {
      let foreign_result = cairo_scaled_font_get_reference_count(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_scaled_font_create(font_face: *mut libc::c_void, font_matrix: *mut super::matrix::Matrix, ctm: *mut super::matrix::Matrix, options: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_scaled_font_status(self_arg: *mut libc::c_void) -> super::Status;
  fn cairo_scaled_font_extents(self_arg: *mut libc::c_void, extents: *mut FontExtents);
  fn cairo_scaled_font_text_extents(self_arg: *mut libc::c_void, utf8: *mut i8, extents: *mut TextExtents);
  fn cairo_scaled_font_glyph_extents(self_arg: *mut libc::c_void, glyphs: *mut Glyph, glyphs: libc::c_int, extents: *mut TextExtents);
  fn cairo_scaled_font_get_font_face(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_scaled_font_get_font_options(self_arg: *mut libc::c_void, options: *mut FontExtents);
  fn cairo_scaled_font_get_font_matrix(self_arg: *mut libc::c_void, font_matrix: *mut super::matrix::Matrix);
  fn cairo_scaled_font_get_ctm(self_arg: *mut libc::c_void, ctm: *mut super::matrix::Matrix);
  fn cairo_scaled_font_get_scale_matrix(self_arg: *mut libc::c_void, scale_matrix: *mut super::matrix::Matrix);
  fn cairo_scaled_font_get_type(self_arg: *mut libc::c_void) -> font_type::FontType;
  fn cairo_scaled_font_get_reference_count(self_arg: *mut libc::c_void) -> libc::c_uint;
}

impl std::clone::Clone for ScaledFont {
  fn clone(&self) -> ScaledFont {
    unsafe {
      let foreign_result = cairo_scaled_font_reference(self.opaque);
      return ScaledFont { opaque: foreign_result as *mut libc::c_void };
    }
  }
}

extern {
  fn cairo_scaled_font_reference(self_arg: *mut libc::c_void) -> *mut libc::c_void;
}

impl std::ops::Drop for ScaledFont {
  fn drop(&mut self) {
    unsafe {
      cairo_scaled_font_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_scaled_font_destroy(self_arg: *mut libc::c_void);
}


pub struct Glyph {
  index: libc::c_ulong,
  x: f64,
  y: f64
}

pub struct Cluster {
  num_bytes: libc::c_int,
  num_glyphs: libc::c_int
}

pub struct FontExtents {
  ascent: f64,
  descent: f64,
  height: f64,
  max_x_advance: f64,
  max_y_advance: f64
}

pub struct TextExtents {
  x_bearing: f64,
  y_bearing: f64,
  width: f64,
  height: f64,
  max_x_advance: f64,
  max_y_advance: f64
}

pub mod cluster_flags {
  #[repr(i32)]
  pub enum ClusterFlags {
    Forwards = 0,
    Backwards = 1
  }
}

pub mod font_type {
  #[repr(i32)]
  pub enum FontType {
    Toy = 0,
    FT = 1,
    Win32 = 2,
    Quartz = 3,
    User = 4
  }
}

pub mod slant {
  #[repr(i32)]
  pub enum Slant {
    Normal = 0,
    Italic = 1,
    Oblique = 2
  }
}

pub mod weight {
  #[repr(i32)]
  pub enum Weight {
    Normal = 0,
    Bold = 1
  }
}

pub mod subpixel_order {
  #[repr(i32)]
  pub enum SubpixelOrder {
    Default = 0,
    RGB = 1,
    BGR = 2,
    VRGB = 3,
    VBGR = 4
  }
}

pub mod hint_style {
  #[repr(i32)]
  pub enum HintStyle {
    Default = 0,
    None = 1,
    Slight = 2,
    Medium = 3,
    Full = 4
  }
}

pub mod hint_metrics {
  #[repr(i32)]
  pub enum HintMetrics {
    Default = 0,
    Off = 1,
    On = 2
  }
}
