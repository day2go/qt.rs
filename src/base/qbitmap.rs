// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpixmap::QPixmap;
use super::qsize::QSize;
use super::qmatrix::QMatrix;
use super::qstring::QString;
use super::qtransform::QTransform;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QBitmap::NewQBitmap(const QPixmap & );
  fn _ZN7QBitmapC1ERK7QPixmap(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QBitmap::NewQBitmap(const QSize & );
  fn _ZN7QBitmapC1ERK5QSize(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QBitmap::NewQBitmap(int w, int h);
  fn _ZN7QBitmapC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> i32;
  // proto: void QBitmap::FreeQBitmap();
  fn _ZN7QBitmapD0Ev() -> i32;
  // proto: void QBitmap::swap(QBitmap & other);
  fn _ZN7QBitmap4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QBitmap QBitmap::transformed(const QMatrix & );
  fn _ZNK7QBitmap11transformedERK7QMatrix(arg0: *const c_void) -> i32;
  // proto: void QBitmap::clear();
  fn _ZN7QBitmap5clearEv() -> i32;
  // proto: void QBitmap::NewQBitmap(const QString & fileName, const char * format);
  fn _ZN7QBitmapC1ERK7QStringPKc(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: void QBitmap::NewQBitmap();
  fn _ZN7QBitmapC1Ev(qthis: *mut c_void) -> i32;
  // proto: QBitmap QBitmap::transformed(const QTransform & matrix);
  fn _ZNK7QBitmap11transformedERK10QTransform(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QBitmap)=1
pub struct QBitmap {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBitmap {
  pub fn NewQBitmap<T: QBitmap_NewQBitmap>(value: T) -> QBitmap {
    let rsthis = value.NewQBitmap();
    return rsthis;
    // return 1;
  }
}

pub trait QBitmap_NewQBitmap {
  fn NewQBitmap(self) -> QBitmap;
}

// proto: void QBitmap::NewQBitmap(const QPixmap & );
impl<'a> /*trait*/ QBitmap_NewQBitmap for (&'a  QPixmap) {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1ERK7QPixmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QBitmapC1ERK7QPixmap(qthis, arg0)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QBitmap::NewQBitmap(const QSize & );
impl<'a> /*trait*/ QBitmap_NewQBitmap for (&'a  QSize) {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1ERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QBitmapC1ERK5QSize(qthis, arg0)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QBitmap::NewQBitmap(int w, int h);
impl<'a> /*trait*/ QBitmap_NewQBitmap for (i32, i32) {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QBitmapC1Eii(qthis, arg0, arg1)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBitmap {
  pub fn FreeQBitmap<T: QBitmap_FreeQBitmap>(&mut self, value: T) -> i32 {
    value.FreeQBitmap(self);
    return 1;
  }
}

pub trait QBitmap_FreeQBitmap {
  fn FreeQBitmap(self, this: &mut QBitmap) -> i32;
}

// proto: void QBitmap::FreeQBitmap();
impl<'a> /*trait*/ QBitmap_FreeQBitmap for () {
  fn FreeQBitmap(self, this: &mut QBitmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapD0Ev()};
    unsafe {_ZN7QBitmapD0Ev()};
    return 1;
  }
}

impl /*struct*/ QBitmap {
  pub fn swap<T: QBitmap_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QBitmap_swap {
  fn swap(self, this: &mut QBitmap) -> i32;
}

// proto: void QBitmap::swap(QBitmap & other);
impl<'a> /*trait*/ QBitmap_swap for (&'a mut QBitmap) {
  fn swap(self, this: &mut QBitmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmap4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QBitmap4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitmap {
  pub fn transformed<T: QBitmap_transformed>(&mut self, value: T) -> i32 {
    value.transformed(self);
    return 1;
  }
}

pub trait QBitmap_transformed {
  fn transformed(self, this: &mut QBitmap) -> i32;
}

// proto: QBitmap QBitmap::transformed(const QMatrix & );
impl<'a> /*trait*/ QBitmap_transformed for (&'a  QMatrix) {
  fn transformed(self, this: &mut QBitmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBitmap11transformedERK7QMatrix()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QBitmap11transformedERK7QMatrix(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitmap {
  pub fn clear<T: QBitmap_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QBitmap_clear {
  fn clear(self, this: &mut QBitmap) -> i32;
}

// proto: void QBitmap::clear();
impl<'a> /*trait*/ QBitmap_clear for () {
  fn clear(self, this: &mut QBitmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmap5clearEv()};
    unsafe {_ZN7QBitmap5clearEv()};
    return 1;
  }
}

// proto: void QBitmap::NewQBitmap(const QString & fileName, const char * format);
impl<'a> /*trait*/ QBitmap_NewQBitmap for (&'a  QString, &'a  String) {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1ERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN7QBitmapC1ERK7QStringPKc(qthis, arg0, arg1)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QBitmap::NewQBitmap();
impl<'a> /*trait*/ QBitmap_NewQBitmap for () {
  fn NewQBitmap(self) -> QBitmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitmapC1Ev()};
    unsafe {_ZN7QBitmapC1Ev(qthis)};
    let rsthis = QBitmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QBitmap QBitmap::transformed(const QTransform & matrix);
impl<'a> /*trait*/ QBitmap_transformed for (&'a  QTransform) {
  fn transformed(self, this: &mut QBitmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBitmap11transformedERK10QTransform()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QBitmap11transformedERK10QTransform(arg0)};
    return 1;
  }
}

