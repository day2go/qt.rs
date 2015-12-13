// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qregexp::QRegExp;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QRegExpValidator::FreeQRegExpValidator();
  fn _ZN16QRegExpValidatorD0Ev() -> i32;
  // proto: const QRegExp & QRegExpValidator::regExp();
  fn _ZNK16QRegExpValidator6regExpEv() -> i32;
  // proto: const QMetaObject * QRegExpValidator::metaObject();
  fn _ZNK16QRegExpValidator10metaObjectEv() -> i32;
  // proto: void QRegExpValidator::NewQRegExpValidator(const QRegExp & rx, QObject * parent);
  fn _ZN16QRegExpValidatorC1ERK7QRegExpP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QRegExpValidator::setRegExp(const QRegExp & rx);
  fn _ZN16QRegExpValidator9setRegExpERK7QRegExp(arg0: *const c_void) -> i32;
  // proto: void QRegExpValidator::NewQRegExpValidator(const QRegExpValidator & );
  fn _ZN16QRegExpValidatorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QRegExpValidator::NewQRegExpValidator(QObject * parent);
  fn _ZN16QRegExpValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QRegExpValidator::regExpChanged(const QRegExp & regExp);
  fn _ZN16QRegExpValidator13regExpChangedERK7QRegExp(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QRegExpValidator)=1
pub struct QRegExpValidator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegExpValidator {
  pub fn FreeQRegExpValidator<T: QRegExpValidator_FreeQRegExpValidator>(&mut self, value: T) -> i32 {
    value.FreeQRegExpValidator(self);
    return 1;
  }
}

pub trait QRegExpValidator_FreeQRegExpValidator {
  fn FreeQRegExpValidator(self, this: &mut QRegExpValidator) -> i32;
}

// proto: void QRegExpValidator::FreeQRegExpValidator();
impl<'a> /*trait*/ QRegExpValidator_FreeQRegExpValidator for () {
  fn FreeQRegExpValidator(self, this: &mut QRegExpValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorD0Ev()};
    unsafe {_ZN16QRegExpValidatorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QRegExpValidator {
  pub fn regExp<T: QRegExpValidator_regExp>(&mut self, value: T) -> i32 {
    value.regExp(self);
    return 1;
  }
}

pub trait QRegExpValidator_regExp {
  fn regExp(self, this: &mut QRegExpValidator) -> i32;
}

// proto: const QRegExp & QRegExpValidator::regExp();
impl<'a> /*trait*/ QRegExpValidator_regExp for () {
  fn regExp(self, this: &mut QRegExpValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QRegExpValidator6regExpEv()};
    unsafe {_ZNK16QRegExpValidator6regExpEv()};
    return 1;
  }
}

impl /*struct*/ QRegExpValidator {
  pub fn metaObject<T: QRegExpValidator_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QRegExpValidator_metaObject {
  fn metaObject(self, this: &mut QRegExpValidator) -> i32;
}

// proto: const QMetaObject * QRegExpValidator::metaObject();
impl<'a> /*trait*/ QRegExpValidator_metaObject for () {
  fn metaObject(self, this: &mut QRegExpValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QRegExpValidator10metaObjectEv()};
    unsafe {_ZNK16QRegExpValidator10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QRegExpValidator {
  pub fn NewQRegExpValidator<T: QRegExpValidator_NewQRegExpValidator>(value: T) -> QRegExpValidator {
    let rsthis = value.NewQRegExpValidator();
    return rsthis;
    // return 1;
  }
}

pub trait QRegExpValidator_NewQRegExpValidator {
  fn NewQRegExpValidator(self) -> QRegExpValidator;
}

// proto: void QRegExpValidator::NewQRegExpValidator(const QRegExp & rx, QObject * parent);
impl<'a> /*trait*/ QRegExpValidator_NewQRegExpValidator for (&'a  QRegExp, &'a mut QObject) {
  fn NewQRegExpValidator(self) -> QRegExpValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorC1ERK7QRegExpP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN16QRegExpValidatorC1ERK7QRegExpP7QObject(qthis, arg0, arg1)};
    let rsthis = QRegExpValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegExpValidator {
  pub fn setRegExp<T: QRegExpValidator_setRegExp>(&mut self, value: T) -> i32 {
    value.setRegExp(self);
    return 1;
  }
}

pub trait QRegExpValidator_setRegExp {
  fn setRegExp(self, this: &mut QRegExpValidator) -> i32;
}

// proto: void QRegExpValidator::setRegExp(const QRegExp & rx);
impl<'a> /*trait*/ QRegExpValidator_setRegExp for (&'a  QRegExp) {
  fn setRegExp(self, this: &mut QRegExpValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidator9setRegExpERK7QRegExp()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QRegExpValidator9setRegExpERK7QRegExp(arg0)};
    return 1;
  }
}

// proto: void QRegExpValidator::NewQRegExpValidator(const QRegExpValidator & );
impl<'a> /*trait*/ QRegExpValidator_NewQRegExpValidator for (&'a  QRegExpValidator) {
  fn NewQRegExpValidator(self) -> QRegExpValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QRegExpValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QRegExpValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QRegExpValidator::NewQRegExpValidator(QObject * parent);
impl<'a> /*trait*/ QRegExpValidator_NewQRegExpValidator for (&'a mut QObject) {
  fn NewQRegExpValidator(self) -> QRegExpValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QRegExpValidatorC1EP7QObject(qthis, arg0)};
    let rsthis = QRegExpValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegExpValidator {
  pub fn regExpChanged<T: QRegExpValidator_regExpChanged>(&mut self, value: T) -> i32 {
    value.regExpChanged(self);
    return 1;
  }
}

pub trait QRegExpValidator_regExpChanged {
  fn regExpChanged(self, this: &mut QRegExpValidator) -> i32;
}

// proto: void QRegExpValidator::regExpChanged(const QRegExp & regExp);
impl<'a> /*trait*/ QRegExpValidator_regExpChanged for (&'a  QRegExp) {
  fn regExpChanged(self, this: &mut QRegExpValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidator13regExpChangedERK7QRegExp()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QRegExpValidator13regExpChangedERK7QRegExp(arg0)};
    return 1;
  }
}
