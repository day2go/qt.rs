// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qgraphicslayout.h
// dst-file: /src/widgets/qgraphicslayout.rs
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
use super::qgraphicslayoutitem::QGraphicsLayoutItem; // 773
use std::ops::Deref;
use super::super::core::qcoreevent::QEvent; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QGraphicsLayout::updateGeometry();
  fn _ZN15QGraphicsLayout14updateGeometryEv(qthis: *mut c_void);
  // proto:  bool QGraphicsLayout::isActivated();
  fn _ZNK15QGraphicsLayout11isActivatedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGraphicsLayout::invalidate();
  fn _ZN15QGraphicsLayout10invalidateEv(qthis: *mut c_void);
  // proto:  void QGraphicsLayout::removeAt(int index);
  fn _ZN15QGraphicsLayout8removeAtEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
  fn _ZNK15QGraphicsLayout6itemAtEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QGraphicsLayout::QGraphicsLayout(const QGraphicsLayout & );
  fn _ZN15QGraphicsLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN15QGraphicsLayout18setContentsMarginsEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QGraphicsLayout::widgetEvent(QEvent * e);
  fn _ZN15QGraphicsLayout11widgetEventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static bool QGraphicsLayout::instantInvalidatePropagation();
  fn _ZN15QGraphicsLayout28instantInvalidatePropagationEv() -> c_char;
  // proto: static void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
  fn _ZN15QGraphicsLayout31setInstantInvalidatePropagationEb(arg0: c_char);
  // proto:  void QGraphicsLayout::~QGraphicsLayout();
  fn _ZN15QGraphicsLayoutD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsLayout::activate();
  fn _ZN15QGraphicsLayout8activateEv(qthis: *mut c_void);
  // proto:  int QGraphicsLayout::count();
  fn _ZNK15QGraphicsLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsLayout::QGraphicsLayout(QGraphicsLayoutItem * parent);
  fn _ZN15QGraphicsLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsLayout)=1
pub struct QGraphicsLayout {
  qbase: QGraphicsLayoutItem,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLayout {
  pub fn inheritFrom(qthis: *mut c_void) -> QGraphicsLayout {
    return QGraphicsLayout{qbase: QGraphicsLayoutItem::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QGraphicsLayout {
  type Target = QGraphicsLayoutItem;

  fn deref(&self) -> &QGraphicsLayoutItem {
    return &self.qbase;
  }
}
impl AsRef<QGraphicsLayoutItem> for QGraphicsLayout {
  fn as_ref(&self) -> &QGraphicsLayoutItem {
    return &self.qbase;
  }
}
  // proto:  void QGraphicsLayout::updateGeometry();
impl /*struct*/ QGraphicsLayout {
  pub fn updateGeometry<RetType, T: QGraphicsLayout_updateGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.updateGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_updateGeometry<RetType> {
  fn updateGeometry(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::updateGeometry();
impl<'a> /*trait*/ QGraphicsLayout_updateGeometry<()> for () {
  fn updateGeometry(self , rsthis: &mut QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout14updateGeometryEv()};
     unsafe {_ZN15QGraphicsLayout14updateGeometryEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsLayout::isActivated();
impl /*struct*/ QGraphicsLayout {
  pub fn isActivated<RetType, T: QGraphicsLayout_isActivated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isActivated(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_isActivated<RetType> {
  fn isActivated(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  bool QGraphicsLayout::isActivated();
impl<'a> /*trait*/ QGraphicsLayout_isActivated<i8> for () {
  fn isActivated(self , rsthis: &mut QGraphicsLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout11isActivatedEv()};
    let mut ret = unsafe {_ZNK15QGraphicsLayout11isActivatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::invalidate();
impl /*struct*/ QGraphicsLayout {
  pub fn invalidate<RetType, T: QGraphicsLayout_invalidate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::invalidate();
impl<'a> /*trait*/ QGraphicsLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: &mut QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout10invalidateEv()};
     unsafe {_ZN15QGraphicsLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::removeAt(int index);
impl /*struct*/ QGraphicsLayout {
  pub fn removeAt<RetType, T: QGraphicsLayout_removeAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeAt(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_removeAt<RetType> {
  fn removeAt(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsLayout_removeAt<()> for (i32) {
  fn removeAt(self , rsthis: &mut QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QGraphicsLayout8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
impl /*struct*/ QGraphicsLayout {
  pub fn itemAt<RetType, T: QGraphicsLayout_itemAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
impl<'a> /*trait*/ QGraphicsLayout_itemAt<()> for (i32) {
  fn itemAt(self , rsthis: &mut QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout6itemAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK15QGraphicsLayout6itemAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::QGraphicsLayout(const QGraphicsLayout & );
impl /*struct*/ QGraphicsLayout {
  pub fn NewQGraphicsLayout<T: QGraphicsLayout_NewQGraphicsLayout>(value: T) -> QGraphicsLayout {
    let rsthis = value.NewQGraphicsLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLayout_NewQGraphicsLayout {
  fn NewQGraphicsLayout(self) -> QGraphicsLayout;
}

  // proto:  void QGraphicsLayout::QGraphicsLayout(const QGraphicsLayout & );
impl<'a> /*trait*/ QGraphicsLayout_NewQGraphicsLayout for (QGraphicsLayout) {
  fn NewQGraphicsLayout(self) -> QGraphicsLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsLayout{/**/qbase: QGraphicsLayoutItem::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl /*struct*/ QGraphicsLayout {
  pub fn getContentsMargins<RetType, T: QGraphicsLayout_getContentsMargins<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_getContentsMargins<RetType> {
  fn getContentsMargins(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsLayout_getContentsMargins<()> for (&'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn getContentsMargins(self , rsthis: &mut QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_double;
    let arg1 = self.1.as_ptr()  as *mut c_double;
    let arg2 = self.2.as_ptr()  as *mut c_double;
    let arg3 = self.3.as_ptr()  as *mut c_double;
     unsafe {_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl /*struct*/ QGraphicsLayout {
  pub fn setContentsMargins<RetType, T: QGraphicsLayout_setContentsMargins<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_setContentsMargins<RetType> {
  fn setContentsMargins(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsLayout_setContentsMargins<()> for (f64, f64, f64, f64) {
  fn setContentsMargins(self , rsthis: &mut QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout18setContentsMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN15QGraphicsLayout18setContentsMarginsEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::widgetEvent(QEvent * e);
impl /*struct*/ QGraphicsLayout {
  pub fn widgetEvent<RetType, T: QGraphicsLayout_widgetEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widgetEvent(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_widgetEvent<RetType> {
  fn widgetEvent(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::widgetEvent(QEvent * e);
impl<'a> /*trait*/ QGraphicsLayout_widgetEvent<()> for (QEvent) {
  fn widgetEvent(self , rsthis: &mut QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout11widgetEventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsLayout11widgetEventEP6QEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static bool QGraphicsLayout::instantInvalidatePropagation();
impl /*struct*/ QGraphicsLayout {
  pub fn instantInvalidatePropagation_s<RetType, T: QGraphicsLayout_instantInvalidatePropagation_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.instantInvalidatePropagation_s();
    // return 1;
  }
}

pub trait QGraphicsLayout_instantInvalidatePropagation_s<RetType> {
  fn instantInvalidatePropagation_s(self ) -> RetType;
}

  // proto: static bool QGraphicsLayout::instantInvalidatePropagation();
impl<'a> /*trait*/ QGraphicsLayout_instantInvalidatePropagation_s<i8> for () {
  fn instantInvalidatePropagation_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout28instantInvalidatePropagationEv()};
    let mut ret = unsafe {_ZN15QGraphicsLayout28instantInvalidatePropagationEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
impl /*struct*/ QGraphicsLayout {
  pub fn setInstantInvalidatePropagation_s<RetType, T: QGraphicsLayout_setInstantInvalidatePropagation_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setInstantInvalidatePropagation_s();
    // return 1;
  }
}

pub trait QGraphicsLayout_setInstantInvalidatePropagation_s<RetType> {
  fn setInstantInvalidatePropagation_s(self ) -> RetType;
}

  // proto: static void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
impl<'a> /*trait*/ QGraphicsLayout_setInstantInvalidatePropagation_s<()> for (i8) {
  fn setInstantInvalidatePropagation_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb(arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::~QGraphicsLayout();
impl /*struct*/ QGraphicsLayout {
  pub fn FreeQGraphicsLayout<RetType, T: QGraphicsLayout_FreeQGraphicsLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsLayout(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_FreeQGraphicsLayout<RetType> {
  fn FreeQGraphicsLayout(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::~QGraphicsLayout();
impl<'a> /*trait*/ QGraphicsLayout_FreeQGraphicsLayout<()> for () {
  fn FreeQGraphicsLayout(self , rsthis: &mut QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayoutD0Ev()};
     unsafe {_ZN15QGraphicsLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::activate();
impl /*struct*/ QGraphicsLayout {
  pub fn activate<RetType, T: QGraphicsLayout_activate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.activate(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_activate<RetType> {
  fn activate(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::activate();
impl<'a> /*trait*/ QGraphicsLayout_activate<()> for () {
  fn activate(self , rsthis: &mut QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout8activateEv()};
     unsafe {_ZN15QGraphicsLayout8activateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QGraphicsLayout::count();
impl /*struct*/ QGraphicsLayout {
  pub fn count<RetType, T: QGraphicsLayout_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_count<RetType> {
  fn count(self , rsthis: &mut QGraphicsLayout) -> RetType;
}

  // proto:  int QGraphicsLayout::count();
impl<'a> /*trait*/ QGraphicsLayout_count<i32> for () {
  fn count(self , rsthis: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout5countEv()};
    let mut ret = unsafe {_ZNK15QGraphicsLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::QGraphicsLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLayout_NewQGraphicsLayout for (QGraphicsLayoutItem) {
  fn NewQGraphicsLayout(self) -> QGraphicsLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayoutC1EP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsLayoutC1EP19QGraphicsLayoutItem(qthis, arg0)};
    let rsthis = QGraphicsLayout{/**/qbase: QGraphicsLayoutItem::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

