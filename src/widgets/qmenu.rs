// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qmenu.h
// dst-file: /src/widgets/qmenu.rs
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
use super::qwidget::*; // 773
use std::ops::Deref;
use super::qaction::*; // 773
use super::super::gui::qicon::*; // 771
use super::super::core::qstring::*; // 771
use super::super::core::qobjectdefs::*; // 771
// use super::qplatformmenu::*; // 775
use super::super::core::qpoint::*; // 771
use super::super::core::qobject::*; // 771
use super::super::gui::qkeysequence::*; // 771
use super::super::core::qrect::*; // 771
use super::super::core::qsize::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMenu_Class_Size() -> c_int;
  // proto:  bool QMenu::isTearOffEnabled();
  fn C_ZNK5QMenu16isTearOffEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QMenu::toolTipsVisible();
  fn C_ZNK5QMenu15toolTipsVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QAction * QMenu::menuAction();
  fn C_ZNK5QMenu10menuActionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAction * QMenu::addAction(const QIcon & icon, const QString & text);
  fn C_ZN5QMenu9addActionERK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::setTearOffEnabled(bool );
  fn C_ZN5QMenu17setTearOffEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QAction * QMenu::addSection(const QString & text);
  fn C_ZN5QMenu10addSectionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QMenu::metaObject();
  fn C_ZNK5QMenu10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMenu::clear();
  fn C_ZN5QMenu5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QAction * QMenu::insertMenu(QAction * before, QMenu * menu);
  fn C_ZN5QMenu10insertMenuEP7QActionPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QIcon QMenu::icon();
  fn C_ZNK5QMenu4iconEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAction * QMenu::insertSection(QAction * before, const QString & text);
  fn C_ZN5QMenu13insertSectionEP7QActionRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QPlatformMenu * QMenu::platformMenu();
  fn C_ZN5QMenu12platformMenuEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMenu::setNoReplayFor(QWidget * widget);
  fn C_ZN5QMenu14setNoReplayForEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMenu::setIcon(const QIcon & icon);
  fn C_ZN5QMenu7setIconERK5QIcon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAction * QMenu::exec(const QPoint & pos, QAction * at);
  fn C_ZN5QMenu4execERK6QPointP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QMenu::separatorsCollapsible();
  fn C_ZNK5QMenu21separatorsCollapsibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QMenu * QMenu::addMenu(const QString & title);
  fn C_ZN5QMenu7addMenuERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::addSeparator();
  fn C_ZN5QMenu12addSeparatorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMenu::hideTearOffMenu();
  fn C_ZN5QMenu15hideTearOffMenuEv(qthis: u64 /* *mut c_void*/);
  // proto:  QAction * QMenu::addAction(const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
  fn C_ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_char, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::QMenu(QWidget * parent);
  fn C_ZN5QMenuC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QMenu::setActiveAction(QAction * act);
  fn C_ZN5QMenu15setActiveActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMenu::setSeparatorsCollapsible(bool collapse);
  fn C_ZN5QMenu24setSeparatorsCollapsibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QAction * QMenu::addAction(const QString & text);
  fn C_ZN5QMenu9addActionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::activeAction();
  fn C_ZNK5QMenu12activeActionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QMenu::isEmpty();
  fn C_ZNK5QMenu7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QAction * QMenu::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
  fn C_ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_char, arg4: *mut c_void) -> *mut c_void;
  // proto:  QRect QMenu::actionGeometry(QAction * );
  fn C_ZNK5QMenu14actionGeometryEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::QMenu(const QString & title, QWidget * parent);
  fn C_ZN5QMenuC2ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  QAction * QMenu::insertSeparator(QAction * before);
  fn C_ZN5QMenu15insertSeparatorEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::addSection(const QIcon & icon, const QString & text);
  fn C_ZN5QMenu10addSectionERK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QMenu::isTearOffMenuVisible();
  fn C_ZNK5QMenu20isTearOffMenuVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QMenu::~QMenu();
  fn C_ZN5QMenuD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QMenu::title();
  fn C_ZNK5QMenu5titleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAction * QMenu::defaultAction();
  fn C_ZNK5QMenu13defaultActionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAction * QMenu::addMenu(QMenu * menu);
  fn C_ZN5QMenu7addMenuEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QSize QMenu::sizeHint();
  fn C_ZNK5QMenu8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMenu::setDefaultAction(QAction * );
  fn C_ZN5QMenu16setDefaultActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAction * QMenu::actionAt(const QPoint & );
  fn C_ZNK5QMenu8actionAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::insertSection(QAction * before, const QIcon & icon, const QString & text);
  fn C_ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::popup(const QPoint & pos, QAction * at);
  fn C_ZN5QMenu5popupERK6QPointP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QMenu::setToolTipsVisible(bool visible);
  fn C_ZN5QMenu18setToolTipsVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QMenu::setTitle(const QString & title);
  fn C_ZN5QMenu8setTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QMenu * QMenu::addMenu(const QIcon & icon, const QString & title);
  fn C_ZN5QMenu7addMenuERK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::exec();
  fn C_ZN5QMenu4execEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QMenu_SlotProxy_connect__ZN5QMenu11aboutToShowEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QMenu_SlotProxy_connect__ZN5QMenu7hoveredEP7QAction(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QMenu_SlotProxy_connect__ZN5QMenu11aboutToHideEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QMenu_SlotProxy_connect__ZN5QMenu9triggeredEP7QAction(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QMenu)=1
#[derive(Default)]
pub struct QMenu {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _aboutToShow: QMenu_aboutToShow_signal,
  pub _triggered: QMenu_triggered_signal,
  pub _hovered: QMenu_hovered_signal,
  pub _aboutToHide: QMenu_aboutToHide_signal,
}

impl /*struct*/ QMenu {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMenu {
    return QMenu{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QMenu {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QMenu {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  bool QMenu::isTearOffEnabled();
impl /*struct*/ QMenu {
  pub fn isTearOffEnabled<RetType, T: QMenu_isTearOffEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTearOffEnabled(self);
    // return 1;
  }
}

pub trait QMenu_isTearOffEnabled<RetType> {
  fn isTearOffEnabled(self , rsthis: & QMenu) -> RetType;
}

  // proto:  bool QMenu::isTearOffEnabled();
impl<'a> /*trait*/ QMenu_isTearOffEnabled<i8> for () {
  fn isTearOffEnabled(self , rsthis: & QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu16isTearOffEnabledEv()};
    let mut ret = unsafe {C_ZNK5QMenu16isTearOffEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QMenu::toolTipsVisible();
impl /*struct*/ QMenu {
  pub fn toolTipsVisible<RetType, T: QMenu_toolTipsVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTipsVisible(self);
    // return 1;
  }
}

pub trait QMenu_toolTipsVisible<RetType> {
  fn toolTipsVisible(self , rsthis: & QMenu) -> RetType;
}

  // proto:  bool QMenu::toolTipsVisible();
impl<'a> /*trait*/ QMenu_toolTipsVisible<i8> for () {
  fn toolTipsVisible(self , rsthis: & QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu15toolTipsVisibleEv()};
    let mut ret = unsafe {C_ZNK5QMenu15toolTipsVisibleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QAction * QMenu::menuAction();
impl /*struct*/ QMenu {
  pub fn menuAction<RetType, T: QMenu_menuAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.menuAction(self);
    // return 1;
  }
}

pub trait QMenu_menuAction<RetType> {
  fn menuAction(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::menuAction();
impl<'a> /*trait*/ QMenu_menuAction<QAction> for () {
  fn menuAction(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu10menuActionEv()};
    let mut ret = unsafe {C_ZNK5QMenu10menuActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QMenu::addAction(const QIcon & icon, const QString & text);
impl /*struct*/ QMenu {
  pub fn addAction<RetType, T: QMenu_addAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addAction(self);
    // return 1;
  }
}

pub trait QMenu_addAction<RetType> {
  fn addAction(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::addAction(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QMenu_addAction<QAction> for (&'a QIcon, &'a QString) {
  fn addAction(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu9addActionERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMenu::setTearOffEnabled(bool );
impl /*struct*/ QMenu {
  pub fn setTearOffEnabled<RetType, T: QMenu_setTearOffEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTearOffEnabled(self);
    // return 1;
  }
}

pub trait QMenu_setTearOffEnabled<RetType> {
  fn setTearOffEnabled(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::setTearOffEnabled(bool );
impl<'a> /*trait*/ QMenu_setTearOffEnabled<()> for (i8) {
  fn setTearOffEnabled(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu17setTearOffEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN5QMenu17setTearOffEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QMenu::addSection(const QString & text);
impl /*struct*/ QMenu {
  pub fn addSection<RetType, T: QMenu_addSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addSection(self);
    // return 1;
  }
}

pub trait QMenu_addSection<RetType> {
  fn addSection(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::addSection(const QString & text);
impl<'a> /*trait*/ QMenu_addSection<QAction> for (&'a QString) {
  fn addSection(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu10addSectionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu10addSectionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QMenu::metaObject();
impl /*struct*/ QMenu {
  pub fn metaObject<RetType, T: QMenu_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMenu_metaObject<RetType> {
  fn metaObject(self , rsthis: & QMenu) -> RetType;
}

  // proto:  const QMetaObject * QMenu::metaObject();
impl<'a> /*trait*/ QMenu_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QMenu) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu10metaObjectEv()};
    let mut ret = unsafe {C_ZNK5QMenu10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMenu::clear();
impl /*struct*/ QMenu {
  pub fn clear<RetType, T: QMenu_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QMenu_clear<RetType> {
  fn clear(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::clear();
impl<'a> /*trait*/ QMenu_clear<()> for () {
  fn clear(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu5clearEv()};
     unsafe {C_ZN5QMenu5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAction * QMenu::insertMenu(QAction * before, QMenu * menu);
impl /*struct*/ QMenu {
  pub fn insertMenu<RetType, T: QMenu_insertMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertMenu(self);
    // return 1;
  }
}

pub trait QMenu_insertMenu<RetType> {
  fn insertMenu(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::insertMenu(QAction * before, QMenu * menu);
impl<'a> /*trait*/ QMenu_insertMenu<QAction> for (&'a QAction, &'a QMenu) {
  fn insertMenu(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu10insertMenuEP7QActionPS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu10insertMenuEP7QActionPS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QIcon QMenu::icon();
impl /*struct*/ QMenu {
  pub fn icon<RetType, T: QMenu_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QMenu_icon<RetType> {
  fn icon(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QIcon QMenu::icon();
impl<'a> /*trait*/ QMenu_icon<QIcon> for () {
  fn icon(self , rsthis: & QMenu) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu4iconEv()};
    let mut ret = unsafe {C_ZNK5QMenu4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QMenu::insertSection(QAction * before, const QString & text);
impl /*struct*/ QMenu {
  pub fn insertSection<RetType, T: QMenu_insertSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertSection(self);
    // return 1;
  }
}

pub trait QMenu_insertSection<RetType> {
  fn insertSection(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::insertSection(QAction * before, const QString & text);
impl<'a> /*trait*/ QMenu_insertSection<QAction> for (&'a QAction, &'a QString) {
  fn insertSection(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu13insertSectionEP7QActionRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu13insertSectionEP7QActionRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPlatformMenu * QMenu::platformMenu();
impl /*struct*/ QMenu {
  pub fn platformMenu<RetType, T: QMenu_platformMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.platformMenu(self);
    // return 1;
  }
}

pub trait QMenu_platformMenu<RetType> {
  fn platformMenu(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QPlatformMenu * QMenu::platformMenu();
impl<'a> /*trait*/ QMenu_platformMenu<u64> for () {
  fn platformMenu(self , rsthis: & QMenu) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu12platformMenuEv()};
    let mut ret = unsafe {C_ZN5QMenu12platformMenuEv(rsthis.qclsinst)};
    return ret as u64; // 4
    // return 1;
  }
}

  // proto:  void QMenu::setNoReplayFor(QWidget * widget);
impl /*struct*/ QMenu {
  pub fn setNoReplayFor<RetType, T: QMenu_setNoReplayFor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNoReplayFor(self);
    // return 1;
  }
}

pub trait QMenu_setNoReplayFor<RetType> {
  fn setNoReplayFor(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::setNoReplayFor(QWidget * widget);
impl<'a> /*trait*/ QMenu_setNoReplayFor<()> for (&'a QWidget) {
  fn setNoReplayFor(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu14setNoReplayForEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN5QMenu14setNoReplayForEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMenu::setIcon(const QIcon & icon);
impl /*struct*/ QMenu {
  pub fn setIcon<RetType, T: QMenu_setIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QMenu_setIcon<RetType> {
  fn setIcon(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QMenu_setIcon<()> for (&'a QIcon) {
  fn setIcon(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN5QMenu7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QMenu::exec(const QPoint & pos, QAction * at);
impl /*struct*/ QMenu {
  pub fn exec<RetType, T: QMenu_exec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exec(self);
    // return 1;
  }
}

pub trait QMenu_exec<RetType> {
  fn exec(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::exec(const QPoint & pos, QAction * at);
impl<'a> /*trait*/ QMenu_exec<QAction> for (&'a QPoint, Option<&'a QAction>) {
  fn exec(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu4execERK6QPointP7QAction()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu4execERK6QPointP7QAction(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMenu::separatorsCollapsible();
impl /*struct*/ QMenu {
  pub fn separatorsCollapsible<RetType, T: QMenu_separatorsCollapsible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.separatorsCollapsible(self);
    // return 1;
  }
}

pub trait QMenu_separatorsCollapsible<RetType> {
  fn separatorsCollapsible(self , rsthis: & QMenu) -> RetType;
}

  // proto:  bool QMenu::separatorsCollapsible();
impl<'a> /*trait*/ QMenu_separatorsCollapsible<i8> for () {
  fn separatorsCollapsible(self , rsthis: & QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu21separatorsCollapsibleEv()};
    let mut ret = unsafe {C_ZNK5QMenu21separatorsCollapsibleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QMenu * QMenu::addMenu(const QString & title);
impl /*struct*/ QMenu {
  pub fn addMenu<RetType, T: QMenu_addMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addMenu(self);
    // return 1;
  }
}

pub trait QMenu_addMenu<RetType> {
  fn addMenu(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QMenu * QMenu::addMenu(const QString & title);
impl<'a> /*trait*/ QMenu_addMenu<QMenu> for (&'a QString) {
  fn addMenu(self , rsthis: & QMenu) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7addMenuERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu7addMenuERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QMenu::addSeparator();
impl /*struct*/ QMenu {
  pub fn addSeparator<RetType, T: QMenu_addSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addSeparator(self);
    // return 1;
  }
}

pub trait QMenu_addSeparator<RetType> {
  fn addSeparator(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::addSeparator();
impl<'a> /*trait*/ QMenu_addSeparator<QAction> for () {
  fn addSeparator(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu12addSeparatorEv()};
    let mut ret = unsafe {C_ZN5QMenu12addSeparatorEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMenu::hideTearOffMenu();
impl /*struct*/ QMenu {
  pub fn hideTearOffMenu<RetType, T: QMenu_hideTearOffMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hideTearOffMenu(self);
    // return 1;
  }
}

pub trait QMenu_hideTearOffMenu<RetType> {
  fn hideTearOffMenu(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::hideTearOffMenu();
impl<'a> /*trait*/ QMenu_hideTearOffMenu<()> for () {
  fn hideTearOffMenu(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu15hideTearOffMenuEv()};
     unsafe {C_ZN5QMenu15hideTearOffMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAction * QMenu::addAction(const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
impl<'a> /*trait*/ QMenu_addAction<QAction> for (&'a QString, &'a QObject, &'a  String, Option<&'a QKeySequence>) {
  fn addAction(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let arg3 = (if self.3.is_none() {0} else {self.3.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMenu::QMenu(QWidget * parent);
impl /*struct*/ QMenu {
  pub fn new<T: QMenu_new>(value: T) -> QMenu {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMenu_new {
  fn new(self) -> QMenu;
}

  // proto:  void QMenu::QMenu(QWidget * parent);
impl<'a> /*trait*/ QMenu_new for (Option<&'a QWidget>) {
  fn new(self) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuC2EP7QWidget()};
    let ctysz: c_int = unsafe{QMenu_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN5QMenuC2EP7QWidget(arg0)};
    let rsthis = QMenu{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMenu::setActiveAction(QAction * act);
impl /*struct*/ QMenu {
  pub fn setActiveAction<RetType, T: QMenu_setActiveAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActiveAction(self);
    // return 1;
  }
}

pub trait QMenu_setActiveAction<RetType> {
  fn setActiveAction(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::setActiveAction(QAction * act);
impl<'a> /*trait*/ QMenu_setActiveAction<()> for (&'a QAction) {
  fn setActiveAction(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu15setActiveActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN5QMenu15setActiveActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMenu::setSeparatorsCollapsible(bool collapse);
impl /*struct*/ QMenu {
  pub fn setSeparatorsCollapsible<RetType, T: QMenu_setSeparatorsCollapsible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSeparatorsCollapsible(self);
    // return 1;
  }
}

pub trait QMenu_setSeparatorsCollapsible<RetType> {
  fn setSeparatorsCollapsible(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::setSeparatorsCollapsible(bool collapse);
impl<'a> /*trait*/ QMenu_setSeparatorsCollapsible<()> for (i8) {
  fn setSeparatorsCollapsible(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu24setSeparatorsCollapsibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN5QMenu24setSeparatorsCollapsibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QMenu::addAction(const QString & text);
impl<'a> /*trait*/ QMenu_addAction<QAction> for (&'a QString) {
  fn addAction(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu9addActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QMenu::activeAction();
impl /*struct*/ QMenu {
  pub fn activeAction<RetType, T: QMenu_activeAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeAction(self);
    // return 1;
  }
}

pub trait QMenu_activeAction<RetType> {
  fn activeAction(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::activeAction();
impl<'a> /*trait*/ QMenu_activeAction<QAction> for () {
  fn activeAction(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu12activeActionEv()};
    let mut ret = unsafe {C_ZNK5QMenu12activeActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMenu::isEmpty();
impl /*struct*/ QMenu {
  pub fn isEmpty<RetType, T: QMenu_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QMenu_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QMenu) -> RetType;
}

  // proto:  bool QMenu::isEmpty();
impl<'a> /*trait*/ QMenu_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu7isEmptyEv()};
    let mut ret = unsafe {C_ZNK5QMenu7isEmptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QAction * QMenu::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
impl<'a> /*trait*/ QMenu_addAction<QAction> for (&'a QIcon, &'a QString, &'a QObject, &'a  String, Option<&'a QKeySequence>) {
  fn addAction(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.as_ptr()  as *mut c_char;
    let arg4 = (if self.4.is_none() {0} else {self.4.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QMenu::actionGeometry(QAction * );
impl /*struct*/ QMenu {
  pub fn actionGeometry<RetType, T: QMenu_actionGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actionGeometry(self);
    // return 1;
  }
}

pub trait QMenu_actionGeometry<RetType> {
  fn actionGeometry(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QRect QMenu::actionGeometry(QAction * );
impl<'a> /*trait*/ QMenu_actionGeometry<QRect> for (&'a QAction) {
  fn actionGeometry(self , rsthis: & QMenu) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu14actionGeometryEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK5QMenu14actionGeometryEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMenu::QMenu(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QMenu_new for (&'a QString, Option<&'a QWidget>) {
  fn new(self) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuC2ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QMenu_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN5QMenuC2ERK7QStringP7QWidget(arg0, arg1)};
    let rsthis = QMenu{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAction * QMenu::insertSeparator(QAction * before);
impl /*struct*/ QMenu {
  pub fn insertSeparator<RetType, T: QMenu_insertSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertSeparator(self);
    // return 1;
  }
}

pub trait QMenu_insertSeparator<RetType> {
  fn insertSeparator(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::insertSeparator(QAction * before);
impl<'a> /*trait*/ QMenu_insertSeparator<QAction> for (&'a QAction) {
  fn insertSeparator(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu15insertSeparatorEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu15insertSeparatorEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QMenu::addSection(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QMenu_addSection<QAction> for (&'a QIcon, &'a QString) {
  fn addSection(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu10addSectionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu10addSectionERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMenu::isTearOffMenuVisible();
impl /*struct*/ QMenu {
  pub fn isTearOffMenuVisible<RetType, T: QMenu_isTearOffMenuVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTearOffMenuVisible(self);
    // return 1;
  }
}

pub trait QMenu_isTearOffMenuVisible<RetType> {
  fn isTearOffMenuVisible(self , rsthis: & QMenu) -> RetType;
}

  // proto:  bool QMenu::isTearOffMenuVisible();
impl<'a> /*trait*/ QMenu_isTearOffMenuVisible<i8> for () {
  fn isTearOffMenuVisible(self , rsthis: & QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu20isTearOffMenuVisibleEv()};
    let mut ret = unsafe {C_ZNK5QMenu20isTearOffMenuVisibleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QMenu::~QMenu();
impl /*struct*/ QMenu {
  pub fn free<RetType, T: QMenu_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMenu_free<RetType> {
  fn free(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::~QMenu();
impl<'a> /*trait*/ QMenu_free<()> for () {
  fn free(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuD2Ev()};
     unsafe {C_ZN5QMenuD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QMenu::title();
impl /*struct*/ QMenu {
  pub fn title<RetType, T: QMenu_title<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.title(self);
    // return 1;
  }
}

pub trait QMenu_title<RetType> {
  fn title(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QString QMenu::title();
impl<'a> /*trait*/ QMenu_title<QString> for () {
  fn title(self , rsthis: & QMenu) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu5titleEv()};
    let mut ret = unsafe {C_ZNK5QMenu5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QMenu::defaultAction();
impl /*struct*/ QMenu {
  pub fn defaultAction<RetType, T: QMenu_defaultAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultAction(self);
    // return 1;
  }
}

pub trait QMenu_defaultAction<RetType> {
  fn defaultAction(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::defaultAction();
impl<'a> /*trait*/ QMenu_defaultAction<QAction> for () {
  fn defaultAction(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu13defaultActionEv()};
    let mut ret = unsafe {C_ZNK5QMenu13defaultActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QMenu::addMenu(QMenu * menu);
impl<'a> /*trait*/ QMenu_addMenu<QAction> for (&'a QMenu) {
  fn addMenu(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7addMenuEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu7addMenuEPS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QMenu::sizeHint();
impl /*struct*/ QMenu {
  pub fn sizeHint<RetType, T: QMenu_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QMenu_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QSize QMenu::sizeHint();
impl<'a> /*trait*/ QMenu_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QMenu) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu8sizeHintEv()};
    let mut ret = unsafe {C_ZNK5QMenu8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMenu::setDefaultAction(QAction * );
impl /*struct*/ QMenu {
  pub fn setDefaultAction<RetType, T: QMenu_setDefaultAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultAction(self);
    // return 1;
  }
}

pub trait QMenu_setDefaultAction<RetType> {
  fn setDefaultAction(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::setDefaultAction(QAction * );
impl<'a> /*trait*/ QMenu_setDefaultAction<()> for (&'a QAction) {
  fn setDefaultAction(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu16setDefaultActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN5QMenu16setDefaultActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QMenu::actionAt(const QPoint & );
impl /*struct*/ QMenu {
  pub fn actionAt<RetType, T: QMenu_actionAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actionAt(self);
    // return 1;
  }
}

pub trait QMenu_actionAt<RetType> {
  fn actionAt(self , rsthis: & QMenu) -> RetType;
}

  // proto:  QAction * QMenu::actionAt(const QPoint & );
impl<'a> /*trait*/ QMenu_actionAt<QAction> for (&'a QPoint) {
  fn actionAt(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu8actionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK5QMenu8actionAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QMenu::insertSection(QAction * before, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QMenu_insertSection<QAction> for (&'a QAction, &'a QIcon, &'a QString) {
  fn insertSection(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMenu::popup(const QPoint & pos, QAction * at);
impl /*struct*/ QMenu {
  pub fn popup<RetType, T: QMenu_popup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.popup(self);
    // return 1;
  }
}

pub trait QMenu_popup<RetType> {
  fn popup(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::popup(const QPoint & pos, QAction * at);
impl<'a> /*trait*/ QMenu_popup<()> for (&'a QPoint, Option<&'a QAction>) {
  fn popup(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu5popupERK6QPointP7QAction()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
     unsafe {C_ZN5QMenu5popupERK6QPointP7QAction(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMenu::setToolTipsVisible(bool visible);
impl /*struct*/ QMenu {
  pub fn setToolTipsVisible<RetType, T: QMenu_setToolTipsVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTipsVisible(self);
    // return 1;
  }
}

pub trait QMenu_setToolTipsVisible<RetType> {
  fn setToolTipsVisible(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::setToolTipsVisible(bool visible);
impl<'a> /*trait*/ QMenu_setToolTipsVisible<()> for (i8) {
  fn setToolTipsVisible(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu18setToolTipsVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN5QMenu18setToolTipsVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMenu::setTitle(const QString & title);
impl /*struct*/ QMenu {
  pub fn setTitle<RetType, T: QMenu_setTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTitle(self);
    // return 1;
  }
}

pub trait QMenu_setTitle<RetType> {
  fn setTitle(self , rsthis: & QMenu) -> RetType;
}

  // proto:  void QMenu::setTitle(const QString & title);
impl<'a> /*trait*/ QMenu_setTitle<()> for (&'a QString) {
  fn setTitle(self , rsthis: & QMenu) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN5QMenu8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMenu * QMenu::addMenu(const QIcon & icon, const QString & title);
impl<'a> /*trait*/ QMenu_addMenu<QMenu> for (&'a QIcon, &'a QString) {
  fn addMenu(self , rsthis: & QMenu) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7addMenuERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QMenu7addMenuERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QMenu::exec();
impl<'a> /*trait*/ QMenu_exec<QAction> for () {
  fn exec(self , rsthis: & QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu4execEv()};
    let mut ret = unsafe {C_ZN5QMenu4execEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QMenu_aboutToShow
pub struct QMenu_aboutToShow_signal{poi:u64}
impl /* struct */ QMenu {
  pub fn aboutToShow(&self) -> QMenu_aboutToShow_signal {
     return QMenu_aboutToShow_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QMenu_aboutToShow_signal {
  pub fn connect<T: QMenu_aboutToShow_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QMenu_aboutToShow_signal_connect {
  fn connect(self, sigthis: QMenu_aboutToShow_signal);
}

#[derive(Default)] // for QMenu_triggered
pub struct QMenu_triggered_signal{poi:u64}
impl /* struct */ QMenu {
  pub fn triggered(&self) -> QMenu_triggered_signal {
     return QMenu_triggered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QMenu_triggered_signal {
  pub fn connect<T: QMenu_triggered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QMenu_triggered_signal_connect {
  fn connect(self, sigthis: QMenu_triggered_signal);
}

#[derive(Default)] // for QMenu_hovered
pub struct QMenu_hovered_signal{poi:u64}
impl /* struct */ QMenu {
  pub fn hovered(&self) -> QMenu_hovered_signal {
     return QMenu_hovered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QMenu_hovered_signal {
  pub fn connect<T: QMenu_hovered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QMenu_hovered_signal_connect {
  fn connect(self, sigthis: QMenu_hovered_signal);
}

#[derive(Default)] // for QMenu_aboutToHide
pub struct QMenu_aboutToHide_signal{poi:u64}
impl /* struct */ QMenu {
  pub fn aboutToHide(&self) -> QMenu_aboutToHide_signal {
     return QMenu_aboutToHide_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QMenu_aboutToHide_signal {
  pub fn connect<T: QMenu_aboutToHide_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QMenu_aboutToHide_signal_connect {
  fn connect(self, sigthis: QMenu_aboutToHide_signal);
}

// aboutToShow()
extern fn QMenu_aboutToShow_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QMenu_aboutToShow_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QMenu_aboutToShow_signal_connect for fn() {
  fn connect(self, sigthis: QMenu_aboutToShow_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMenu_aboutToShow_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QMenu_SlotProxy_connect__ZN5QMenu11aboutToShowEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QMenu_aboutToShow_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QMenu_aboutToShow_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMenu_aboutToShow_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QMenu_SlotProxy_connect__ZN5QMenu11aboutToShowEv(arg0, arg1, arg2)};
  }
}
// hovered(class QAction *)
extern fn QMenu_hovered_signal_connect_cb_1(rsfptr:fn(QAction), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QMenu_hovered_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QAction)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QMenu_hovered_signal_connect for fn(QAction) {
  fn connect(self, sigthis: QMenu_hovered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMenu_hovered_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QMenu_SlotProxy_connect__ZN5QMenu7hoveredEP7QAction(arg0, arg1, arg2)};
  }
}
impl /* trait */ QMenu_hovered_signal_connect for Box<Fn(QAction)> {
  fn connect(self, sigthis: QMenu_hovered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMenu_hovered_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QMenu_SlotProxy_connect__ZN5QMenu7hoveredEP7QAction(arg0, arg1, arg2)};
  }
}
// aboutToHide()
extern fn QMenu_aboutToHide_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QMenu_aboutToHide_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QMenu_aboutToHide_signal_connect for fn() {
  fn connect(self, sigthis: QMenu_aboutToHide_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMenu_aboutToHide_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QMenu_SlotProxy_connect__ZN5QMenu11aboutToHideEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QMenu_aboutToHide_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QMenu_aboutToHide_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMenu_aboutToHide_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QMenu_SlotProxy_connect__ZN5QMenu11aboutToHideEv(arg0, arg1, arg2)};
  }
}
// triggered(class QAction *)
extern fn QMenu_triggered_signal_connect_cb_3(rsfptr:fn(QAction), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QMenu_triggered_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QAction)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QMenu_triggered_signal_connect for fn(QAction) {
  fn connect(self, sigthis: QMenu_triggered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMenu_triggered_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QMenu_SlotProxy_connect__ZN5QMenu9triggeredEP7QAction(arg0, arg1, arg2)};
  }
}
impl /* trait */ QMenu_triggered_signal_connect for Box<Fn(QAction)> {
  fn connect(self, sigthis: QMenu_triggered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMenu_triggered_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QMenu_SlotProxy_connect__ZN5QMenu9triggeredEP7QAction(arg0, arg1, arg2)};
  }
}
// <= body block end

