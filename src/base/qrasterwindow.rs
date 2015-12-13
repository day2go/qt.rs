// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwindow::QWindow;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QRasterWindow::NewQRasterWindow(QWindow * parent);
  fn _ZN13QRasterWindowC1EP7QWindow(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QRasterWindow::metaObject();
  fn _ZNK13QRasterWindow10metaObjectEv() -> i32;
  // proto: void QRasterWindow::NewQRasterWindow(const QRasterWindow & );
  fn _ZN13QRasterWindowC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QRasterWindow)=1
pub struct QRasterWindow {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRasterWindow {
  pub fn NewQRasterWindow<T: QRasterWindow_NewQRasterWindow>(value: T) -> QRasterWindow {
    let rsthis = value.NewQRasterWindow();
    return rsthis;
    // return 1;
  }
}

pub trait QRasterWindow_NewQRasterWindow {
  fn NewQRasterWindow(self) -> QRasterWindow;
}

// proto: void QRasterWindow::NewQRasterWindow(QWindow * parent);
impl<'a> /*trait*/ QRasterWindow_NewQRasterWindow for (&'a mut QWindow) {
  fn NewQRasterWindow(self) -> QRasterWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QRasterWindowC1EP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QRasterWindowC1EP7QWindow(qthis, arg0)};
    let rsthis = QRasterWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRasterWindow {
  pub fn metaObject<T: QRasterWindow_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QRasterWindow_metaObject {
  fn metaObject(self, this: &mut QRasterWindow) -> i32;
}

// proto: const QMetaObject * QRasterWindow::metaObject();
impl<'a> /*trait*/ QRasterWindow_metaObject for () {
  fn metaObject(self, this: &mut QRasterWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QRasterWindow10metaObjectEv()};
    unsafe {_ZNK13QRasterWindow10metaObjectEv()};
    return 1;
  }
}

// proto: void QRasterWindow::NewQRasterWindow(const QRasterWindow & );
impl<'a> /*trait*/ QRasterWindow_NewQRasterWindow for (&'a  QRasterWindow) {
  fn NewQRasterWindow(self) -> QRasterWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QRasterWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QRasterWindowC1ERKS_(qthis, arg0)};
    let rsthis = QRasterWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}
