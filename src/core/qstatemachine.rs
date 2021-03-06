// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qstatemachine.h
// dst-file: /src/core/qstatemachine.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qstate::*; // 773
use std::ops::Deref;
use super::qobject::*; // 773
// use super::qlist::*; // 775
use super::qcoreevent::*; // 773
// use super::qset::*; // 775
use super::qabstractanimation::*; // 773
use super::qstring::*; // 773
use super::qobjectdefs::*; // 773
use super::qabstractstate::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStateMachine_Class_Size() -> c_int;
  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
  fn C_ZNK13QStateMachine17defaultAnimationsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
  fn C_ZN13QStateMachine16postDelayedEventEP6QEventi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QSet<QAbstractState *> QStateMachine::configuration();
  fn C_ZNK13QStateMachine13configurationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStateMachine::setRunning(bool running);
  fn C_ZN13QStateMachine10setRunningEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStateMachine::addDefaultAnimation(QAbstractAnimation * animation);
  fn C_ZN13QStateMachine19addDefaultAnimationEP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStateMachine::removeDefaultAnimation(QAbstractAnimation * animation);
  fn C_ZN13QStateMachine22removeDefaultAnimationEP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStateMachine::setAnimated(bool enabled);
  fn C_ZN13QStateMachine11setAnimatedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStateMachine::QStateMachine(QObject * parent);
  fn C_ZN13QStateMachineC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QString QStateMachine::errorString();
  fn C_ZNK13QStateMachine11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QStateMachine::isRunning();
  fn C_ZNK13QStateMachine9isRunningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
  fn C_ZN13QStateMachine18cancelDelayedEventEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QStateMachine::~QStateMachine();
  fn C_ZN13QStateMachineD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QStateMachine::metaObject();
  fn C_ZNK13QStateMachine10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStateMachine::addState(QAbstractState * state);
  fn C_ZN13QStateMachine8addStateEP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStateMachine::clearError();
  fn C_ZN13QStateMachine10clearErrorEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QStateMachine::removeState(QAbstractState * state);
  fn C_ZN13QStateMachine11removeStateEP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStateMachine::stop();
  fn C_ZN13QStateMachine4stopEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QStateMachine::isAnimated();
  fn C_ZNK13QStateMachine10isAnimatedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QStateMachine::start();
  fn C_ZN13QStateMachine5startEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
  fn C_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  fn QStateMachine_SlotProxy_connect__ZN13QStateMachine14runningChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStateMachine)=1
#[derive(Default)]
pub struct QStateMachine {
  qbase: QState,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _started: QStateMachine_started_signal,
  pub _runningChanged: QStateMachine_runningChanged_signal,
  pub _stopped: QStateMachine_stopped_signal,
}

impl /*struct*/ QStateMachine {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStateMachine {
    return QStateMachine{qbase: QState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStateMachine {
  type Target = QState;

  fn deref(&self) -> &QState {
    return & self.qbase;
  }
}
impl AsRef<QState> for QStateMachine {
  fn as_ref(& self) -> & QState {
    return & self.qbase;
  }
}
  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
impl /*struct*/ QStateMachine {
  pub fn defaultAnimations<RetType, T: QStateMachine_defaultAnimations<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultAnimations(self);
    // return 1;
  }
}

pub trait QStateMachine_defaultAnimations<RetType> {
  fn defaultAnimations(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
impl<'a> /*trait*/ QStateMachine_defaultAnimations<u64> for () {
  fn defaultAnimations(self , rsthis: & QStateMachine) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine17defaultAnimationsEv()};
    let mut ret = unsafe {C_ZNK13QStateMachine17defaultAnimationsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
impl /*struct*/ QStateMachine {
  pub fn postDelayedEvent<RetType, T: QStateMachine_postDelayedEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.postDelayedEvent(self);
    // return 1;
  }
}

pub trait QStateMachine_postDelayedEvent<RetType> {
  fn postDelayedEvent(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
impl<'a> /*trait*/ QStateMachine_postDelayedEvent<i32> for (&'a QEvent, i32) {
  fn postDelayedEvent(self , rsthis: & QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine16postDelayedEventEP6QEventi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN13QStateMachine16postDelayedEventEP6QEventi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QSet<QAbstractState *> QStateMachine::configuration();
impl /*struct*/ QStateMachine {
  pub fn configuration<RetType, T: QStateMachine_configuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.configuration(self);
    // return 1;
  }
}

pub trait QStateMachine_configuration<RetType> {
  fn configuration(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  QSet<QAbstractState *> QStateMachine::configuration();
impl<'a> /*trait*/ QStateMachine_configuration<u64> for () {
  fn configuration(self , rsthis: & QStateMachine) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine13configurationEv()};
    let mut ret = unsafe {C_ZNK13QStateMachine13configurationEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QStateMachine::setRunning(bool running);
impl /*struct*/ QStateMachine {
  pub fn setRunning<RetType, T: QStateMachine_setRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRunning(self);
    // return 1;
  }
}

pub trait QStateMachine_setRunning<RetType> {
  fn setRunning(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::setRunning(bool running);
impl<'a> /*trait*/ QStateMachine_setRunning<()> for (i8) {
  fn setRunning(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10setRunningEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QStateMachine10setRunningEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::addDefaultAnimation(QAbstractAnimation * animation);
impl /*struct*/ QStateMachine {
  pub fn addDefaultAnimation<RetType, T: QStateMachine_addDefaultAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addDefaultAnimation(self);
    // return 1;
  }
}

pub trait QStateMachine_addDefaultAnimation<RetType> {
  fn addDefaultAnimation(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::addDefaultAnimation(QAbstractAnimation * animation);
impl<'a> /*trait*/ QStateMachine_addDefaultAnimation<()> for (&'a QAbstractAnimation) {
  fn addDefaultAnimation(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine19addDefaultAnimationEP18QAbstractAnimation()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStateMachine19addDefaultAnimationEP18QAbstractAnimation(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::removeDefaultAnimation(QAbstractAnimation * animation);
impl /*struct*/ QStateMachine {
  pub fn removeDefaultAnimation<RetType, T: QStateMachine_removeDefaultAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeDefaultAnimation(self);
    // return 1;
  }
}

pub trait QStateMachine_removeDefaultAnimation<RetType> {
  fn removeDefaultAnimation(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::removeDefaultAnimation(QAbstractAnimation * animation);
impl<'a> /*trait*/ QStateMachine_removeDefaultAnimation<()> for (&'a QAbstractAnimation) {
  fn removeDefaultAnimation(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine22removeDefaultAnimationEP18QAbstractAnimation()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStateMachine22removeDefaultAnimationEP18QAbstractAnimation(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::setAnimated(bool enabled);
impl /*struct*/ QStateMachine {
  pub fn setAnimated<RetType, T: QStateMachine_setAnimated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAnimated(self);
    // return 1;
  }
}

pub trait QStateMachine_setAnimated<RetType> {
  fn setAnimated(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::setAnimated(bool enabled);
impl<'a> /*trait*/ QStateMachine_setAnimated<()> for (i8) {
  fn setAnimated(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11setAnimatedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QStateMachine11setAnimatedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::QStateMachine(QObject * parent);
impl /*struct*/ QStateMachine {
  pub fn new<T: QStateMachine_new>(value: T) -> QStateMachine {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStateMachine_new {
  fn new(self) -> QStateMachine;
}

  // proto:  void QStateMachine::QStateMachine(QObject * parent);
impl<'a> /*trait*/ QStateMachine_new for (Option<&'a QObject>) {
  fn new(self) -> QStateMachine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineC2EP7QObject()};
    let ctysz: c_int = unsafe{QStateMachine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QStateMachineC2EP7QObject(arg0)};
    let rsthis = QStateMachine{qbase: QState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QStateMachine::errorString();
impl /*struct*/ QStateMachine {
  pub fn errorString<RetType, T: QStateMachine_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QStateMachine_errorString<RetType> {
  fn errorString(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  QString QStateMachine::errorString();
impl<'a> /*trait*/ QStateMachine_errorString<QString> for () {
  fn errorString(self , rsthis: & QStateMachine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine11errorStringEv()};
    let mut ret = unsafe {C_ZNK13QStateMachine11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStateMachine::isRunning();
impl /*struct*/ QStateMachine {
  pub fn isRunning<RetType, T: QStateMachine_isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QStateMachine_isRunning<RetType> {
  fn isRunning(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::isRunning();
impl<'a> /*trait*/ QStateMachine_isRunning<i8> for () {
  fn isRunning(self , rsthis: & QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine9isRunningEv()};
    let mut ret = unsafe {C_ZNK13QStateMachine9isRunningEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
impl /*struct*/ QStateMachine {
  pub fn cancelDelayedEvent<RetType, T: QStateMachine_cancelDelayedEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancelDelayedEvent(self);
    // return 1;
  }
}

pub trait QStateMachine_cancelDelayedEvent<RetType> {
  fn cancelDelayedEvent(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
impl<'a> /*trait*/ QStateMachine_cancelDelayedEvent<i8> for (i32) {
  fn cancelDelayedEvent(self , rsthis: & QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine18cancelDelayedEventEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN13QStateMachine18cancelDelayedEventEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStateMachine::~QStateMachine();
impl /*struct*/ QStateMachine {
  pub fn free<RetType, T: QStateMachine_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStateMachine_free<RetType> {
  fn free(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::~QStateMachine();
impl<'a> /*trait*/ QStateMachine_free<()> for () {
  fn free(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineD2Ev()};
     unsafe {C_ZN13QStateMachineD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QStateMachine::metaObject();
impl /*struct*/ QStateMachine {
  pub fn metaObject<RetType, T: QStateMachine_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStateMachine_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  const QMetaObject * QStateMachine::metaObject();
impl<'a> /*trait*/ QStateMachine_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QStateMachine) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QStateMachine10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStateMachine::addState(QAbstractState * state);
impl /*struct*/ QStateMachine {
  pub fn addState<RetType, T: QStateMachine_addState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addState(self);
    // return 1;
  }
}

pub trait QStateMachine_addState<RetType> {
  fn addState(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::addState(QAbstractState * state);
impl<'a> /*trait*/ QStateMachine_addState<()> for (&'a QAbstractState) {
  fn addState(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine8addStateEP14QAbstractState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStateMachine8addStateEP14QAbstractState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::clearError();
impl /*struct*/ QStateMachine {
  pub fn clearError<RetType, T: QStateMachine_clearError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearError(self);
    // return 1;
  }
}

pub trait QStateMachine_clearError<RetType> {
  fn clearError(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::clearError();
impl<'a> /*trait*/ QStateMachine_clearError<()> for () {
  fn clearError(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10clearErrorEv()};
     unsafe {C_ZN13QStateMachine10clearErrorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStateMachine::removeState(QAbstractState * state);
impl /*struct*/ QStateMachine {
  pub fn removeState<RetType, T: QStateMachine_removeState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeState(self);
    // return 1;
  }
}

pub trait QStateMachine_removeState<RetType> {
  fn removeState(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::removeState(QAbstractState * state);
impl<'a> /*trait*/ QStateMachine_removeState<()> for (&'a QAbstractState) {
  fn removeState(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11removeStateEP14QAbstractState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStateMachine11removeStateEP14QAbstractState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::stop();
impl /*struct*/ QStateMachine {
  pub fn stop<RetType, T: QStateMachine_stop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QStateMachine_stop<RetType> {
  fn stop(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::stop();
impl<'a> /*trait*/ QStateMachine_stop<()> for () {
  fn stop(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine4stopEv()};
     unsafe {C_ZN13QStateMachine4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QStateMachine::isAnimated();
impl /*struct*/ QStateMachine {
  pub fn isAnimated<RetType, T: QStateMachine_isAnimated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAnimated(self);
    // return 1;
  }
}

pub trait QStateMachine_isAnimated<RetType> {
  fn isAnimated(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::isAnimated();
impl<'a> /*trait*/ QStateMachine_isAnimated<i8> for () {
  fn isAnimated(self , rsthis: & QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10isAnimatedEv()};
    let mut ret = unsafe {C_ZNK13QStateMachine10isAnimatedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStateMachine::start();
impl /*struct*/ QStateMachine {
  pub fn start<RetType, T: QStateMachine_start<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QStateMachine_start<RetType> {
  fn start(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::start();
impl<'a> /*trait*/ QStateMachine_start<()> for () {
  fn start(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine5startEv()};
     unsafe {C_ZN13QStateMachine5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
impl /*struct*/ QStateMachine {
  pub fn eventFilter<RetType, T: QStateMachine_eventFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.eventFilter(self);
    // return 1;
  }
}

pub trait QStateMachine_eventFilter<RetType> {
  fn eventFilter(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
impl<'a> /*trait*/ QStateMachine_eventFilter<i8> for (&'a QObject, &'a QEvent) {
  fn eventFilter(self , rsthis: & QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

#[derive(Default)] // for QStateMachine_started
pub struct QStateMachine_started_signal{poi:u64}
impl /* struct */ QStateMachine {
  pub fn started(&self) -> QStateMachine_started_signal {
     return QStateMachine_started_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStateMachine_started_signal {
  pub fn connect<T: QStateMachine_started_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStateMachine_started_signal_connect {
  fn connect(self, sigthis: QStateMachine_started_signal);
}

#[derive(Default)] // for QStateMachine_runningChanged
pub struct QStateMachine_runningChanged_signal{poi:u64}
impl /* struct */ QStateMachine {
  pub fn runningChanged(&self) -> QStateMachine_runningChanged_signal {
     return QStateMachine_runningChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStateMachine_runningChanged_signal {
  pub fn connect<T: QStateMachine_runningChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStateMachine_runningChanged_signal_connect {
  fn connect(self, sigthis: QStateMachine_runningChanged_signal);
}

#[derive(Default)] // for QStateMachine_stopped
pub struct QStateMachine_stopped_signal{poi:u64}
impl /* struct */ QStateMachine {
  pub fn stopped(&self) -> QStateMachine_stopped_signal {
     return QStateMachine_stopped_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStateMachine_stopped_signal {
  pub fn connect<T: QStateMachine_stopped_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStateMachine_stopped_signal_connect {
  fn connect(self, sigthis: QStateMachine_stopped_signal);
}

// runningChanged(_Bool)
extern fn QStateMachine_runningChanged_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QStateMachine_runningChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QStateMachine_runningChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QStateMachine_runningChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStateMachine_runningChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStateMachine_SlotProxy_connect__ZN13QStateMachine14runningChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStateMachine_runningChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QStateMachine_runningChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStateMachine_runningChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QStateMachine_SlotProxy_connect__ZN13QStateMachine14runningChangedEb(arg0, arg1, arg2)};
  }
}
// <= body block end

