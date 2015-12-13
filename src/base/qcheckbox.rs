// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QMetaObject * QCheckBox::metaObject();
  fn _ZNK9QCheckBox10metaObjectEv() -> i32;
  // proto: QSize QCheckBox::minimumSizeHint();
  fn _ZNK9QCheckBox15minimumSizeHintEv() -> i32;
  // proto: void QCheckBox::FreeQCheckBox();
  fn _ZN9QCheckBoxD0Ev() -> i32;
  // proto: QSize QCheckBox::sizeHint();
  fn _ZNK9QCheckBox8sizeHintEv() -> i32;
  // proto: void QCheckBox::stateChanged(int );
  fn _ZN9QCheckBox12stateChangedEi(arg0: c_int) -> i32;
  // proto: void QCheckBox::setTristate(bool y);
  fn _ZN9QCheckBox11setTristateEb(arg0: int8_t) -> i32;
  // proto: void QCheckBox::NewQCheckBox(const QCheckBox & );
  fn _ZN9QCheckBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QCheckBox::NewQCheckBox(QWidget * parent);
  fn _ZN9QCheckBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: bool QCheckBox::isTristate();
  fn _ZNK9QCheckBox10isTristateEv() -> i32;
  // proto: void QCheckBox::NewQCheckBox(const QString & text, QWidget * parent);
  fn _ZN9QCheckBoxC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QCheckBox)=1
pub struct QCheckBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCheckBox {
  pub fn metaObject<T: QCheckBox_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QCheckBox_metaObject {
  fn metaObject(self, this: &mut QCheckBox) -> i32;
}

// proto: const QMetaObject * QCheckBox::metaObject();
impl<'a> /*trait*/ QCheckBox_metaObject for () {
  fn metaObject(self, this: &mut QCheckBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox10metaObjectEv()};
    unsafe {_ZNK9QCheckBox10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QCheckBox {
  pub fn minimumSizeHint<T: QCheckBox_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QCheckBox_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QCheckBox) -> i32;
}

// proto: QSize QCheckBox::minimumSizeHint();
impl<'a> /*trait*/ QCheckBox_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QCheckBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox15minimumSizeHintEv()};
    unsafe {_ZNK9QCheckBox15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QCheckBox {
  pub fn FreeQCheckBox<T: QCheckBox_FreeQCheckBox>(&mut self, value: T) -> i32 {
    value.FreeQCheckBox(self);
    return 1;
  }
}

pub trait QCheckBox_FreeQCheckBox {
  fn FreeQCheckBox(self, this: &mut QCheckBox) -> i32;
}

// proto: void QCheckBox::FreeQCheckBox();
impl<'a> /*trait*/ QCheckBox_FreeQCheckBox for () {
  fn FreeQCheckBox(self, this: &mut QCheckBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxD0Ev()};
    unsafe {_ZN9QCheckBoxD0Ev()};
    return 1;
  }
}

impl /*struct*/ QCheckBox {
  pub fn sizeHint<T: QCheckBox_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QCheckBox_sizeHint {
  fn sizeHint(self, this: &mut QCheckBox) -> i32;
}

// proto: QSize QCheckBox::sizeHint();
impl<'a> /*trait*/ QCheckBox_sizeHint for () {
  fn sizeHint(self, this: &mut QCheckBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox8sizeHintEv()};
    unsafe {_ZNK9QCheckBox8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QCheckBox {
  pub fn stateChanged<T: QCheckBox_stateChanged>(&mut self, value: T) -> i32 {
    value.stateChanged(self);
    return 1;
  }
}

pub trait QCheckBox_stateChanged {
  fn stateChanged(self, this: &mut QCheckBox) -> i32;
}

// proto: void QCheckBox::stateChanged(int );
impl<'a> /*trait*/ QCheckBox_stateChanged for (i32) {
  fn stateChanged(self, this: &mut QCheckBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBox12stateChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QCheckBox12stateChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QCheckBox {
  pub fn setTristate<T: QCheckBox_setTristate>(&mut self, value: T) -> i32 {
    value.setTristate(self);
    return 1;
  }
}

pub trait QCheckBox_setTristate {
  fn setTristate(self, this: &mut QCheckBox) -> i32;
}

// proto: void QCheckBox::setTristate(bool y);
impl<'a> /*trait*/ QCheckBox_setTristate for (i8) {
  fn setTristate(self, this: &mut QCheckBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBox11setTristateEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QCheckBox11setTristateEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QCheckBox {
  pub fn NewQCheckBox<T: QCheckBox_NewQCheckBox>(value: T) -> QCheckBox {
    let rsthis = value.NewQCheckBox();
    return rsthis;
    // return 1;
  }
}

pub trait QCheckBox_NewQCheckBox {
  fn NewQCheckBox(self) -> QCheckBox;
}

// proto: void QCheckBox::NewQCheckBox(const QCheckBox & );
impl<'a> /*trait*/ QCheckBox_NewQCheckBox for (&'a  QCheckBox) {
  fn NewQCheckBox(self) -> QCheckBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QCheckBoxC1ERKS_(qthis, arg0)};
    let rsthis = QCheckBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QCheckBox::NewQCheckBox(QWidget * parent);
impl<'a> /*trait*/ QCheckBox_NewQCheckBox for (&'a mut QWidget) {
  fn NewQCheckBox(self) -> QCheckBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QCheckBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QCheckBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCheckBox {
  pub fn isTristate<T: QCheckBox_isTristate>(&mut self, value: T) -> i32 {
    value.isTristate(self);
    return 1;
  }
}

pub trait QCheckBox_isTristate {
  fn isTristate(self, this: &mut QCheckBox) -> i32;
}

// proto: bool QCheckBox::isTristate();
impl<'a> /*trait*/ QCheckBox_isTristate for () {
  fn isTristate(self, this: &mut QCheckBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox10isTristateEv()};
    unsafe {_ZNK9QCheckBox10isTristateEv()};
    return 1;
  }
}

// proto: void QCheckBox::NewQCheckBox(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QCheckBox_NewQCheckBox for (&'a  QString, &'a mut QWidget) {
  fn NewQCheckBox(self) -> QCheckBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QCheckBoxC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QCheckBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}
