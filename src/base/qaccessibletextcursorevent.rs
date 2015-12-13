// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibleinterface::QAccessibleInterface;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QAccessibleTextCursorEvent::NewQAccessibleTextCursorEvent(QAccessibleInterface * iface, int cursorPos);
  fn _ZN26QAccessibleTextCursorEventC1EP20QAccessibleInterfacei(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: void QAccessibleTextCursorEvent::setCursorPosition(int position);
  fn _ZN26QAccessibleTextCursorEvent17setCursorPositionEi(arg0: c_int) -> i32;
  // proto: int QAccessibleTextCursorEvent::cursorPosition();
  fn _ZNK26QAccessibleTextCursorEvent14cursorPositionEv() -> i32;
  // proto: void QAccessibleTextCursorEvent::NewQAccessibleTextCursorEvent(QObject * obj, int cursorPos);
  fn _ZN26QAccessibleTextCursorEventC1EP7QObjecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QAccessibleTextCursorEvent)=32
pub struct QAccessibleTextCursorEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn NewQAccessibleTextCursorEvent<T: QAccessibleTextCursorEvent_NewQAccessibleTextCursorEvent>(value: T) -> QAccessibleTextCursorEvent {
    let rsthis = value.NewQAccessibleTextCursorEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextCursorEvent_NewQAccessibleTextCursorEvent {
  fn NewQAccessibleTextCursorEvent(self) -> QAccessibleTextCursorEvent;
}

// proto: void QAccessibleTextCursorEvent::NewQAccessibleTextCursorEvent(QAccessibleInterface * iface, int cursorPos);
impl<'a> /*trait*/ QAccessibleTextCursorEvent_NewQAccessibleTextCursorEvent for (&'a mut QAccessibleInterface, i32) {
  fn NewQAccessibleTextCursorEvent(self) -> QAccessibleTextCursorEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleTextCursorEventC1EP20QAccessibleInterfacei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN26QAccessibleTextCursorEventC1EP20QAccessibleInterfacei(qthis, arg0, arg1)};
    let rsthis = QAccessibleTextCursorEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn setCursorPosition<T: QAccessibleTextCursorEvent_setCursorPosition>(&mut self, value: T) -> i32 {
    value.setCursorPosition(self);
    return 1;
  }
}

pub trait QAccessibleTextCursorEvent_setCursorPosition {
  fn setCursorPosition(self, this: &mut QAccessibleTextCursorEvent) -> i32;
}

// proto: void QAccessibleTextCursorEvent::setCursorPosition(int position);
impl<'a> /*trait*/ QAccessibleTextCursorEvent_setCursorPosition for (i32) {
  fn setCursorPosition(self, this: &mut QAccessibleTextCursorEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleTextCursorEvent17setCursorPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN26QAccessibleTextCursorEvent17setCursorPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn cursorPosition<T: QAccessibleTextCursorEvent_cursorPosition>(&mut self, value: T) -> i32 {
    value.cursorPosition(self);
    return 1;
  }
}

pub trait QAccessibleTextCursorEvent_cursorPosition {
  fn cursorPosition(self, this: &mut QAccessibleTextCursorEvent) -> i32;
}

// proto: int QAccessibleTextCursorEvent::cursorPosition();
impl<'a> /*trait*/ QAccessibleTextCursorEvent_cursorPosition for () {
  fn cursorPosition(self, this: &mut QAccessibleTextCursorEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleTextCursorEvent14cursorPositionEv()};
    unsafe {_ZNK26QAccessibleTextCursorEvent14cursorPositionEv()};
    return 1;
  }
}

// proto: void QAccessibleTextCursorEvent::NewQAccessibleTextCursorEvent(QObject * obj, int cursorPos);
impl<'a> /*trait*/ QAccessibleTextCursorEvent_NewQAccessibleTextCursorEvent for (&'a mut QObject, i32) {
  fn NewQAccessibleTextCursorEvent(self) -> QAccessibleTextCursorEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleTextCursorEventC1EP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN26QAccessibleTextCursorEventC1EP7QObjecti(qthis, arg0, arg1)};
    let rsthis = QAccessibleTextCursorEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}
