// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qaction::QAction;
use super::qicon::QIcon;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QActionGroup::QActionGroup(QObject * parent);
  fn _ZN12QActionGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QList<QAction *> QActionGroup::actions();
  fn _ZNK12QActionGroup7actionsEv(qthis: *mut c_void);
  // proto:  void QActionGroup::setDisabled(bool b);
  fn _ZN12QActionGroup11setDisabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QActionGroup::setEnabled(bool );
  fn _ZN12QActionGroup10setEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  const QMetaObject * QActionGroup::metaObject();
  fn _ZNK12QActionGroup10metaObjectEv(qthis: *mut c_void);
  // proto:  QAction * QActionGroup::addAction(QAction * a);
  fn _ZN12QActionGroup9addActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QActionGroup::~QActionGroup();
  fn _ZN12QActionGroupD0Ev(qthis: *mut c_void);
  // proto:  QAction * QActionGroup::checkedAction();
  fn _ZNK12QActionGroup13checkedActionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAction * QActionGroup::addAction(const QIcon & icon, const QString & text);
  fn _ZN12QActionGroup9addActionERK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QActionGroup::setVisible(bool );
  fn _ZN12QActionGroup10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QActionGroup::isVisible();
  fn _ZNK12QActionGroup9isVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  void QActionGroup::setExclusive(bool );
  fn _ZN12QActionGroup12setExclusiveEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QAction * QActionGroup::addAction(const QString & text);
  fn _ZN12QActionGroup9addActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QActionGroup::triggered(QAction * );
  fn _ZN12QActionGroup9triggeredEP7QAction(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QActionGroup::isEnabled();
  fn _ZNK12QActionGroup9isEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QActionGroup::isExclusive();
  fn _ZNK12QActionGroup11isExclusiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QActionGroup::removeAction(QAction * a);
  fn _ZN12QActionGroup12removeActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QActionGroup::QActionGroup(const QActionGroup & );
  fn _ZN12QActionGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QActionGroup::hovered(QAction * );
  fn _ZN12QActionGroup7hoveredEP7QAction(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QActionGroup)=1
pub struct QActionGroup {
  pub qclsinst: *mut c_void,
}

  // proto:  void QActionGroup::QActionGroup(QObject * parent);
impl /*struct*/ QActionGroup {
  pub fn NewQActionGroup<T: QActionGroup_NewQActionGroup>(value: T) -> QActionGroup {
    let rsthis = value.NewQActionGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QActionGroup_NewQActionGroup {
  fn NewQActionGroup(self) -> QActionGroup;
}

  // proto:  void QActionGroup::QActionGroup(QObject * parent);
impl<'a> /*trait*/ QActionGroup_NewQActionGroup for (QObject) {
  fn NewQActionGroup(self) -> QActionGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QActionGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QActionGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QAction *> QActionGroup::actions();
impl /*struct*/ QActionGroup {
  pub fn actions<RetType, T: QActionGroup_actions<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.actions(self);
    // return 1;
  }
}

pub trait QActionGroup_actions<RetType> {
  fn actions(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  QList<QAction *> QActionGroup::actions();
impl<'a> /*trait*/ QActionGroup_actions<()> for () {
  fn actions(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup7actionsEv()};
     unsafe {_ZNK12QActionGroup7actionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QActionGroup::setDisabled(bool b);
impl /*struct*/ QActionGroup {
  pub fn setDisabled<RetType, T: QActionGroup_setDisabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDisabled(self);
    // return 1;
  }
}

pub trait QActionGroup_setDisabled<RetType> {
  fn setDisabled(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::setDisabled(bool b);
impl<'a> /*trait*/ QActionGroup_setDisabled<()> for (i8) {
  fn setDisabled(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup11setDisabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QActionGroup11setDisabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QActionGroup::setEnabled(bool );
impl /*struct*/ QActionGroup {
  pub fn setEnabled<RetType, T: QActionGroup_setEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QActionGroup_setEnabled<RetType> {
  fn setEnabled(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::setEnabled(bool );
impl<'a> /*trait*/ QActionGroup_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QActionGroup10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QActionGroup::metaObject();
impl /*struct*/ QActionGroup {
  pub fn metaObject<RetType, T: QActionGroup_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QActionGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  const QMetaObject * QActionGroup::metaObject();
impl<'a> /*trait*/ QActionGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup10metaObjectEv()};
     unsafe {_ZNK12QActionGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAction * QActionGroup::addAction(QAction * a);
impl /*struct*/ QActionGroup {
  pub fn addAction<RetType, T: QActionGroup_addAction<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addAction(self);
    // return 1;
  }
}

pub trait QActionGroup_addAction<RetType> {
  fn addAction(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  QAction * QActionGroup::addAction(QAction * a);
impl<'a> /*trait*/ QActionGroup_addAction<QAction> for (QAction) {
  fn addAction(self , rsthis: &mut QActionGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9addActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QActionGroup9addActionEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QActionGroup::~QActionGroup();
impl /*struct*/ QActionGroup {
  pub fn FreeQActionGroup<RetType, T: QActionGroup_FreeQActionGroup<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQActionGroup(self);
    // return 1;
  }
}

pub trait QActionGroup_FreeQActionGroup<RetType> {
  fn FreeQActionGroup(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::~QActionGroup();
impl<'a> /*trait*/ QActionGroup_FreeQActionGroup<()> for () {
  fn FreeQActionGroup(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroupD0Ev()};
     unsafe {_ZN12QActionGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAction * QActionGroup::checkedAction();
impl /*struct*/ QActionGroup {
  pub fn checkedAction<RetType, T: QActionGroup_checkedAction<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.checkedAction(self);
    // return 1;
  }
}

pub trait QActionGroup_checkedAction<RetType> {
  fn checkedAction(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  QAction * QActionGroup::checkedAction();
impl<'a> /*trait*/ QActionGroup_checkedAction<QAction> for () {
  fn checkedAction(self , rsthis: &mut QActionGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup13checkedActionEv()};
    let mut ret = unsafe {_ZNK12QActionGroup13checkedActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QActionGroup::addAction(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QActionGroup_addAction<QAction> for (QIcon, QString) {
  fn addAction(self , rsthis: &mut QActionGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9addActionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QActionGroup9addActionERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QActionGroup::setVisible(bool );
impl /*struct*/ QActionGroup {
  pub fn setVisible<RetType, T: QActionGroup_setVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QActionGroup_setVisible<RetType> {
  fn setVisible(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::setVisible(bool );
impl<'a> /*trait*/ QActionGroup_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QActionGroup10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QActionGroup::isVisible();
impl /*struct*/ QActionGroup {
  pub fn isVisible<RetType, T: QActionGroup_isVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QActionGroup_isVisible<RetType> {
  fn isVisible(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  bool QActionGroup::isVisible();
impl<'a> /*trait*/ QActionGroup_isVisible<i8> for () {
  fn isVisible(self , rsthis: &mut QActionGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup9isVisibleEv()};
    let mut ret = unsafe {_ZNK12QActionGroup9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QActionGroup::setExclusive(bool );
impl /*struct*/ QActionGroup {
  pub fn setExclusive<RetType, T: QActionGroup_setExclusive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setExclusive(self);
    // return 1;
  }
}

pub trait QActionGroup_setExclusive<RetType> {
  fn setExclusive(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::setExclusive(bool );
impl<'a> /*trait*/ QActionGroup_setExclusive<()> for (i8) {
  fn setExclusive(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup12setExclusiveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QActionGroup12setExclusiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QActionGroup::addAction(const QString & text);
impl<'a> /*trait*/ QActionGroup_addAction<QAction> for (QString) {
  fn addAction(self , rsthis: &mut QActionGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9addActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QActionGroup9addActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QActionGroup::triggered(QAction * );
impl /*struct*/ QActionGroup {
  pub fn triggered<RetType, T: QActionGroup_triggered<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.triggered(self);
    // return 1;
  }
}

pub trait QActionGroup_triggered<RetType> {
  fn triggered(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::triggered(QAction * );
impl<'a> /*trait*/ QActionGroup_triggered<()> for (QAction) {
  fn triggered(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9triggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QActionGroup9triggeredEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QActionGroup::isEnabled();
impl /*struct*/ QActionGroup {
  pub fn isEnabled<RetType, T: QActionGroup_isEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QActionGroup_isEnabled<RetType> {
  fn isEnabled(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  bool QActionGroup::isEnabled();
impl<'a> /*trait*/ QActionGroup_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: &mut QActionGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup9isEnabledEv()};
    let mut ret = unsafe {_ZNK12QActionGroup9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QActionGroup::isExclusive();
impl /*struct*/ QActionGroup {
  pub fn isExclusive<RetType, T: QActionGroup_isExclusive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isExclusive(self);
    // return 1;
  }
}

pub trait QActionGroup_isExclusive<RetType> {
  fn isExclusive(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  bool QActionGroup::isExclusive();
impl<'a> /*trait*/ QActionGroup_isExclusive<i8> for () {
  fn isExclusive(self , rsthis: &mut QActionGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup11isExclusiveEv()};
    let mut ret = unsafe {_ZNK12QActionGroup11isExclusiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QActionGroup::removeAction(QAction * a);
impl /*struct*/ QActionGroup {
  pub fn removeAction<RetType, T: QActionGroup_removeAction<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeAction(self);
    // return 1;
  }
}

pub trait QActionGroup_removeAction<RetType> {
  fn removeAction(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::removeAction(QAction * a);
impl<'a> /*trait*/ QActionGroup_removeAction<()> for (QAction) {
  fn removeAction(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup12removeActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QActionGroup12removeActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QActionGroup::QActionGroup(const QActionGroup & );
impl<'a> /*trait*/ QActionGroup_NewQActionGroup for (QActionGroup) {
  fn NewQActionGroup(self) -> QActionGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QActionGroupC1ERKS_(qthis, arg0)};
    let rsthis = QActionGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QActionGroup::hovered(QAction * );
impl /*struct*/ QActionGroup {
  pub fn hovered<RetType, T: QActionGroup_hovered<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hovered(self);
    // return 1;
  }
}

pub trait QActionGroup_hovered<RetType> {
  fn hovered(self , rsthis: &mut QActionGroup) -> RetType;
}

  // proto:  void QActionGroup::hovered(QAction * );
impl<'a> /*trait*/ QActionGroup_hovered<()> for (QAction) {
  fn hovered(self , rsthis: &mut QActionGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup7hoveredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QActionGroup7hoveredEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}
