// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qstring::QString;
use super::qwidget::QWidget;
use super::qobject::QObject;
use super::qaction::QAction;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static void QWhatsThis::hideText();
  fn _ZN10QWhatsThis8hideTextEv() ;
  // proto: static void QWhatsThis::enterWhatsThisMode();
  fn _ZN10QWhatsThis18enterWhatsThisModeEv() ;
  // proto: static bool QWhatsThis::inWhatsThisMode();
  fn _ZN10QWhatsThis15inWhatsThisModeEv() -> int8_t;
  // proto: static void QWhatsThis::leaveWhatsThisMode();
  fn _ZN10QWhatsThis18leaveWhatsThisModeEv() ;
  // proto:  void QWhatsThis::NewQWhatsThis();
  fn _ZN10QWhatsThisC1Ev(qthis: *mut c_void) ;
  // proto: static void QWhatsThis::showText(const QPoint & pos, const QString & text, QWidget * w);
  fn _ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto: static QAction * QWhatsThis::createAction(QObject * parent);
  fn _ZN10QWhatsThis12createActionEP7QObject(arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QWhatsThis)=1
pub struct QWhatsThis {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWhatsThis {
  pub fn hideText<RetType, T: QWhatsThis_hideText<RetType>>(&mut self, value: T) -> RetType {
    return value.hideText(self);
    // return 1;
  }
}

pub trait QWhatsThis_hideText<RetType> {
  fn hideText(self, rsthis: &mut QWhatsThis) -> RetType;
}

// proto: static void QWhatsThis::hideText();
impl<'a> /*trait*/ QWhatsThis_hideText<()> for () {
  fn hideText(self, rsthis: &mut QWhatsThis) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis8hideTextEv()};
     unsafe {_ZN10QWhatsThis8hideTextEv()};
    // return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn enterWhatsThisMode<RetType, T: QWhatsThis_enterWhatsThisMode<RetType>>(&mut self, value: T) -> RetType {
    return value.enterWhatsThisMode(self);
    // return 1;
  }
}

pub trait QWhatsThis_enterWhatsThisMode<RetType> {
  fn enterWhatsThisMode(self, rsthis: &mut QWhatsThis) -> RetType;
}

// proto: static void QWhatsThis::enterWhatsThisMode();
impl<'a> /*trait*/ QWhatsThis_enterWhatsThisMode<()> for () {
  fn enterWhatsThisMode(self, rsthis: &mut QWhatsThis) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis18enterWhatsThisModeEv()};
     unsafe {_ZN10QWhatsThis18enterWhatsThisModeEv()};
    // return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn inWhatsThisMode<RetType, T: QWhatsThis_inWhatsThisMode<RetType>>(&mut self, value: T) -> RetType {
    return value.inWhatsThisMode(self);
    // return 1;
  }
}

pub trait QWhatsThis_inWhatsThisMode<RetType> {
  fn inWhatsThisMode(self, rsthis: &mut QWhatsThis) -> RetType;
}

// proto: static bool QWhatsThis::inWhatsThisMode();
impl<'a> /*trait*/ QWhatsThis_inWhatsThisMode<i8> for () {
  fn inWhatsThisMode(self, rsthis: &mut QWhatsThis) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis15inWhatsThisModeEv()};
    let mut ret = unsafe {_ZN10QWhatsThis15inWhatsThisModeEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn leaveWhatsThisMode<RetType, T: QWhatsThis_leaveWhatsThisMode<RetType>>(&mut self, value: T) -> RetType {
    return value.leaveWhatsThisMode(self);
    // return 1;
  }
}

pub trait QWhatsThis_leaveWhatsThisMode<RetType> {
  fn leaveWhatsThisMode(self, rsthis: &mut QWhatsThis) -> RetType;
}

// proto: static void QWhatsThis::leaveWhatsThisMode();
impl<'a> /*trait*/ QWhatsThis_leaveWhatsThisMode<()> for () {
  fn leaveWhatsThisMode(self, rsthis: &mut QWhatsThis) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis18leaveWhatsThisModeEv()};
     unsafe {_ZN10QWhatsThis18leaveWhatsThisModeEv()};
    // return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn NewQWhatsThis<T: QWhatsThis_NewQWhatsThis>(value: T) -> QWhatsThis {
    let rsthis = value.NewQWhatsThis();
    return rsthis;
    // return 1;
  }
}

pub trait QWhatsThis_NewQWhatsThis {
  fn NewQWhatsThis(self) -> QWhatsThis;
}

// proto: void QWhatsThis::NewQWhatsThis();
impl<'a> /*trait*/ QWhatsThis_NewQWhatsThis for () {
  fn NewQWhatsThis(self) -> QWhatsThis {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThisC1Ev()};
    unsafe {_ZN10QWhatsThisC1Ev(qthis)};
    let rsthis = QWhatsThis{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn showText<RetType, T: QWhatsThis_showText<RetType>>(&mut self, value: T) -> RetType {
    return value.showText(self);
    // return 1;
  }
}

pub trait QWhatsThis_showText<RetType> {
  fn showText(self, rsthis: &mut QWhatsThis) -> RetType;
}

// proto: static void QWhatsThis::showText(const QPoint & pos, const QString & text, QWidget * w);
impl<'a> /*trait*/ QWhatsThis_showText<()> for (&'a  QPoint, &'a  QString, &'a mut QWidget) {
  fn showText(self, rsthis: &mut QWhatsThis) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget(arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn createAction<RetType, T: QWhatsThis_createAction<RetType>>(&mut self, value: T) -> RetType {
    return value.createAction(self);
    // return 1;
  }
}

pub trait QWhatsThis_createAction<RetType> {
  fn createAction(self, rsthis: &mut QWhatsThis) -> RetType;
}

// proto: static QAction * QWhatsThis::createAction(QObject * parent);
impl<'a> /*trait*/ QWhatsThis_createAction<QAction> for (&'a mut QObject) {
  fn createAction(self, rsthis: &mut QWhatsThis) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis12createActionEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QWhatsThis12createActionEP7QObject(arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

