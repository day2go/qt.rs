// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QTemporaryDir::remove();
  fn _ZN13QTemporaryDir6removeEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTemporaryDir::autoRemove();
  fn _ZNK13QTemporaryDir10autoRemoveEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTemporaryDir::isValid();
  fn _ZNK13QTemporaryDir7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTemporaryDir::setAutoRemove(bool b);
  fn _ZN13QTemporaryDir13setAutoRemoveEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTemporaryDir::FreeQTemporaryDir();
  fn _ZN13QTemporaryDirD0Ev(qthis: *mut c_void) ;
  // proto:  void QTemporaryDir::NewQTemporaryDir();
  fn _ZN13QTemporaryDirC1Ev(qthis: *mut c_void) ;
  // proto:  void QTemporaryDir::NewQTemporaryDir(const QString & templateName);
  fn _ZN13QTemporaryDirC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTemporaryDir::path();
  fn _ZNK13QTemporaryDir4pathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTemporaryDir::NewQTemporaryDir(const QTemporaryDir & );
  fn _ZN13QTemporaryDirC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTemporaryDir)=1
pub struct QTemporaryDir {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTemporaryDir {
  pub fn remove<T: QTemporaryDir_remove>(&mut self, value: T) -> i8 {
    return value.remove(self);
    // return 1;
  }
}

pub trait QTemporaryDir_remove {
  fn remove(self, rsthis: &mut QTemporaryDir) -> i8;
}

// proto:  bool QTemporaryDir::remove();
impl<'a> /*trait*/ QTemporaryDir_remove for () {
  fn remove(self, rsthis: &mut QTemporaryDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDir6removeEv()};
    let mut ret = unsafe {_ZN13QTemporaryDir6removeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn autoRemove<T: QTemporaryDir_autoRemove>(&mut self, value: T) -> i8 {
    return value.autoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryDir_autoRemove {
  fn autoRemove(self, rsthis: &mut QTemporaryDir) -> i8;
}

// proto:  bool QTemporaryDir::autoRemove();
impl<'a> /*trait*/ QTemporaryDir_autoRemove for () {
  fn autoRemove(self, rsthis: &mut QTemporaryDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir10autoRemoveEv()};
    let mut ret = unsafe {_ZNK13QTemporaryDir10autoRemoveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn isValid<T: QTemporaryDir_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTemporaryDir_isValid {
  fn isValid(self, rsthis: &mut QTemporaryDir) -> i8;
}

// proto:  bool QTemporaryDir::isValid();
impl<'a> /*trait*/ QTemporaryDir_isValid for () {
  fn isValid(self, rsthis: &mut QTemporaryDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir7isValidEv()};
    let mut ret = unsafe {_ZNK13QTemporaryDir7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn setAutoRemove<T: QTemporaryDir_setAutoRemove>(&mut self, value: T)  {
     value.setAutoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryDir_setAutoRemove {
  fn setAutoRemove(self, rsthis: &mut QTemporaryDir) ;
}

// proto:  void QTemporaryDir::setAutoRemove(bool b);
impl<'a> /*trait*/ QTemporaryDir_setAutoRemove for (i8) {
  fn setAutoRemove(self, rsthis: &mut QTemporaryDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDir13setAutoRemoveEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QTemporaryDir13setAutoRemoveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn FreeQTemporaryDir<T: QTemporaryDir_FreeQTemporaryDir>(&mut self, value: T)  {
     value.FreeQTemporaryDir(self);
    // return 1;
  }
}

pub trait QTemporaryDir_FreeQTemporaryDir {
  fn FreeQTemporaryDir(self, rsthis: &mut QTemporaryDir) ;
}

// proto:  void QTemporaryDir::FreeQTemporaryDir();
impl<'a> /*trait*/ QTemporaryDir_FreeQTemporaryDir for () {
  fn FreeQTemporaryDir(self, rsthis: &mut QTemporaryDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirD0Ev()};
     unsafe {_ZN13QTemporaryDirD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn NewQTemporaryDir<T: QTemporaryDir_NewQTemporaryDir>(value: T) -> QTemporaryDir {
    let rsthis = value.NewQTemporaryDir();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryDir_NewQTemporaryDir {
  fn NewQTemporaryDir(self) -> QTemporaryDir;
}

// proto: void QTemporaryDir::NewQTemporaryDir();
impl<'a> /*trait*/ QTemporaryDir_NewQTemporaryDir for () {
  fn NewQTemporaryDir(self) -> QTemporaryDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1Ev()};
    unsafe {_ZN13QTemporaryDirC1Ev(qthis)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTemporaryDir::NewQTemporaryDir(const QString & templateName);
impl<'a> /*trait*/ QTemporaryDir_NewQTemporaryDir for (&'a  QString) {
  fn NewQTemporaryDir(self) -> QTemporaryDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QTemporaryDirC1ERK7QString(qthis, arg0)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn path<T: QTemporaryDir_path>(&mut self, value: T) -> QString {
    return value.path(self);
    // return 1;
  }
}

pub trait QTemporaryDir_path {
  fn path(self, rsthis: &mut QTemporaryDir) -> QString;
}

// proto:  QString QTemporaryDir::path();
impl<'a> /*trait*/ QTemporaryDir_path for () {
  fn path(self, rsthis: &mut QTemporaryDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir4pathEv()};
    let mut ret = unsafe {_ZNK13QTemporaryDir4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTemporaryDir::NewQTemporaryDir(const QTemporaryDir & );
impl<'a> /*trait*/ QTemporaryDir_NewQTemporaryDir for (&'a  QTemporaryDir) {
  fn NewQTemporaryDir(self) -> QTemporaryDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QTemporaryDirC1ERKS_(qthis, arg0)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

