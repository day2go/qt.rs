// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qlayoutitem.h
// dst-file: /src/widgets/qlayoutitem.rs
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
use std::ops::Deref;
// use super::qlayoutitem::QSpacerItem; // 773
use super::super::core::qsize::QSize; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qrect::QRect; // 771
use super::qlayout::QLayout; // 773
// use super::qlayoutitem::QLayoutItem; // 773
use super::qsizepolicy::QSizePolicy; // 773
// use super::qlayoutitem::QWidgetItem; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QSpacerItem * QLayoutItem::spacerItem();
  fn _ZN11QLayoutItem10spacerItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QLayoutItem::minimumSize();
  fn _ZNK11QLayoutItem11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QLayoutItem::widget();
  fn _ZN11QLayoutItem6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayoutItem::invalidate();
  fn _ZN11QLayoutItem10invalidateEv(qthis: *mut c_void);
  // proto:  void QLayoutItem::setGeometry(const QRect & );
  fn _ZN11QLayoutItem11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QLayout * QLayoutItem::layout();
  fn _ZN11QLayoutItem6layoutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QLayoutItem::isEmpty();
  fn _ZNK11QLayoutItem7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QSize QLayoutItem::sizeHint();
  fn _ZNK11QLayoutItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayoutItem::~QLayoutItem();
  fn _ZN11QLayoutItemD0Ev(qthis: *mut c_void);
  // proto:  bool QLayoutItem::hasHeightForWidth();
  fn _ZNK11QLayoutItem17hasHeightForWidthEv(qthis: *mut c_void) -> c_char;
  // proto:  int QLayoutItem::heightForWidth(int );
  fn _ZNK11QLayoutItem14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QRect QLayoutItem::geometry();
  fn _ZNK11QLayoutItem8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QLayoutItem::maximumSize();
  fn _ZNK11QLayoutItem11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QLayoutItem::minimumHeightForWidth(int );
  fn _ZNK11QLayoutItem21minimumHeightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QSize QSpacerItem::minimumSize();
  fn _ZNK11QSpacerItem11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizePolicy QSpacerItem::sizePolicy();
  fn _ZNK11QSpacerItem10sizePolicyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSpacerItem::~QSpacerItem();
  fn _ZN11QSpacerItemD0Ev(qthis: *mut c_void);
  // proto:  QSize QSpacerItem::sizeHint();
  fn _ZNK11QSpacerItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QSpacerItem::maximumSize();
  fn _ZNK11QSpacerItem11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSpacerItem::isEmpty();
  fn _ZNK11QSpacerItem7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QRect QSpacerItem::geometry();
  fn _ZNK11QSpacerItem8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSpacerItem::setGeometry(const QRect & );
  fn _ZN11QSpacerItem11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSpacerItem * QSpacerItem::spacerItem();
  fn _ZN11QSpacerItem10spacerItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QWidgetItem::sizeHint();
  fn _ZNK11QWidgetItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QWidgetItem::minimumSize();
  fn _ZNK11QWidgetItem11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidgetItem::hasHeightForWidth();
  fn _ZNK11QWidgetItem17hasHeightForWidthEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWidgetItem::~QWidgetItem();
  fn _ZN11QWidgetItemD0Ev(qthis: *mut c_void);
  // proto:  void QWidgetItem::QWidgetItem(QWidget * w);
  fn _ZN11QWidgetItemC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QWidgetItem::widget();
  fn _ZN11QWidgetItem6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidgetItem::setGeometry(const QRect & );
  fn _ZN11QWidgetItem11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QWidgetItem::heightForWidth(int );
  fn _ZNK11QWidgetItem14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QWidgetItem::QWidgetItem(const QWidgetItem & );
  fn _ZN11QWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QWidgetItem::maximumSize();
  fn _ZNK11QWidgetItem11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidgetItem::isEmpty();
  fn _ZNK11QWidgetItem7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QRect QWidgetItem::geometry();
  fn _ZNK11QWidgetItem8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QWidgetItemV2::sizeHint();
  fn _ZNK13QWidgetItemV28sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QWidgetItemV2::minimumSize();
  fn _ZNK13QWidgetItemV211minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWidgetItemV2::heightForWidth(int width);
  fn _ZNK13QWidgetItemV214heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QWidgetItemV2::~QWidgetItemV2();
  fn _ZN13QWidgetItemV2D0Ev(qthis: *mut c_void);
  // proto:  void QWidgetItemV2::QWidgetItemV2(QWidget * widget);
  fn _ZN13QWidgetItemV2C1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QWidgetItemV2::maximumSize();
  fn _ZNK13QWidgetItemV211maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidgetItemV2::QWidgetItemV2(const QWidgetItemV2 & );
  fn _ZN13QWidgetItemV2C1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLayoutItem)=1
pub struct QLayoutItem {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QSpacerItem)=1
pub struct QSpacerItem {
  qbase: QLayoutItem,
  pub qclsinst: *mut c_void,
}

// class sizeof(QWidgetItem)=1
pub struct QWidgetItem {
  qbase: QLayoutItem,
  pub qclsinst: *mut c_void,
}

// class sizeof(QWidgetItemV2)=1
pub struct QWidgetItemV2 {
  qbase: QWidgetItem,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLayoutItem {
  pub fn inheritFrom(qthis: *mut c_void) -> QLayoutItem {
    return QLayoutItem{qclsinst: qthis};
  }
}
  // proto:  QSpacerItem * QLayoutItem::spacerItem();
impl /*struct*/ QLayoutItem {
  pub fn spacerItem<RetType, T: QLayoutItem_spacerItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.spacerItem(self);
    // return 1;
  }
}

pub trait QLayoutItem_spacerItem<RetType> {
  fn spacerItem(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QSpacerItem * QLayoutItem::spacerItem();
impl<'a> /*trait*/ QLayoutItem_spacerItem<QSpacerItem> for () {
  fn spacerItem(self , rsthis: &mut QLayoutItem) -> QSpacerItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10spacerItemEv()};
    let mut ret = unsafe {_ZN11QLayoutItem10spacerItemEv(rsthis.qclsinst)};
    let mut ret1 = QSpacerItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QLayoutItem::minimumSize();
impl /*struct*/ QLayoutItem {
  pub fn minimumSize<RetType, T: QLayoutItem_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QLayoutItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QSize QLayoutItem::minimumSize();
impl<'a> /*trait*/ QLayoutItem_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QLayoutItem::widget();
impl /*struct*/ QLayoutItem {
  pub fn widget<RetType, T: QLayoutItem_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QLayoutItem_widget<RetType> {
  fn widget(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QWidget * QLayoutItem::widget();
impl<'a> /*trait*/ QLayoutItem_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QLayoutItem) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6widgetEv()};
    let mut ret = unsafe {_ZN11QLayoutItem6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLayoutItem::invalidate();
impl /*struct*/ QLayoutItem {
  pub fn invalidate<RetType, T: QLayoutItem_invalidate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QLayoutItem_invalidate<RetType> {
  fn invalidate(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  void QLayoutItem::invalidate();
impl<'a> /*trait*/ QLayoutItem_invalidate<()> for () {
  fn invalidate(self , rsthis: &mut QLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10invalidateEv()};
     unsafe {_ZN11QLayoutItem10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLayoutItem::setGeometry(const QRect & );
impl /*struct*/ QLayoutItem {
  pub fn setGeometry<RetType, T: QLayoutItem_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QLayoutItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  void QLayoutItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QLayoutItem_setGeometry<()> for (QRect) {
  fn setGeometry(self , rsthis: &mut QLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QLayoutItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QLayout * QLayoutItem::layout();
impl /*struct*/ QLayoutItem {
  pub fn layout<RetType, T: QLayoutItem_layout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.layout(self);
    // return 1;
  }
}

pub trait QLayoutItem_layout<RetType> {
  fn layout(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QLayout * QLayoutItem::layout();
impl<'a> /*trait*/ QLayoutItem_layout<QLayout> for () {
  fn layout(self , rsthis: &mut QLayoutItem) -> QLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6layoutEv()};
    let mut ret = unsafe {_ZN11QLayoutItem6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QLayout::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QLayoutItem::isEmpty();
impl /*struct*/ QLayoutItem {
  pub fn isEmpty<RetType, T: QLayoutItem_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QLayoutItem_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  bool QLayoutItem::isEmpty();
impl<'a> /*trait*/ QLayoutItem_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QLayoutItem::sizeHint();
impl /*struct*/ QLayoutItem {
  pub fn sizeHint<RetType, T: QLayoutItem_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QLayoutItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QSize QLayoutItem::sizeHint();
impl<'a> /*trait*/ QLayoutItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLayoutItem::~QLayoutItem();
impl /*struct*/ QLayoutItem {
  pub fn FreeQLayoutItem<RetType, T: QLayoutItem_FreeQLayoutItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQLayoutItem(self);
    // return 1;
  }
}

pub trait QLayoutItem_FreeQLayoutItem<RetType> {
  fn FreeQLayoutItem(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  void QLayoutItem::~QLayoutItem();
impl<'a> /*trait*/ QLayoutItem_FreeQLayoutItem<()> for () {
  fn FreeQLayoutItem(self , rsthis: &mut QLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItemD0Ev()};
     unsafe {_ZN11QLayoutItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLayoutItem::hasHeightForWidth();
impl /*struct*/ QLayoutItem {
  pub fn hasHeightForWidth<RetType, T: QLayoutItem_hasHeightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  bool QLayoutItem::hasHeightForWidth();
impl<'a> /*trait*/ QLayoutItem_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: &mut QLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QLayoutItem::heightForWidth(int );
impl /*struct*/ QLayoutItem {
  pub fn heightForWidth<RetType, T: QLayoutItem_heightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  int QLayoutItem::heightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QLayoutItem14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QLayoutItem::geometry();
impl /*struct*/ QLayoutItem {
  pub fn geometry<RetType, T: QLayoutItem_geometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QLayoutItem_geometry<RetType> {
  fn geometry(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QRect QLayoutItem::geometry();
impl<'a> /*trait*/ QLayoutItem_geometry<QRect> for () {
  fn geometry(self , rsthis: &mut QLayoutItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QLayoutItem::maximumSize();
impl /*struct*/ QLayoutItem {
  pub fn maximumSize<RetType, T: QLayoutItem_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QLayoutItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QSize QLayoutItem::maximumSize();
impl<'a> /*trait*/ QLayoutItem_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QLayoutItem::minimumHeightForWidth(int );
impl /*struct*/ QLayoutItem {
  pub fn minimumHeightForWidth<RetType, T: QLayoutItem_minimumHeightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_minimumHeightForWidth<RetType> {
  fn minimumHeightForWidth(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  int QLayoutItem::minimumHeightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_minimumHeightForWidth<i32> for (i32) {
  fn minimumHeightForWidth(self , rsthis: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QLayoutItem21minimumHeightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn inheritFrom(qthis: *mut c_void) -> QSpacerItem {
    return QSpacerItem{qbase: QLayoutItem::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QSpacerItem {
  type Target = QLayoutItem;

  fn deref(&self) -> &QLayoutItem {
    return &self.qbase;
  }
}
impl AsRef<QLayoutItem> for QSpacerItem {
  fn as_ref(&self) -> &QLayoutItem {
    return &self.qbase;
  }
}
  // proto:  QSize QSpacerItem::minimumSize();
impl /*struct*/ QSpacerItem {
  pub fn minimumSize<RetType, T: QSpacerItem_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QSpacerItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSize QSpacerItem::minimumSize();
impl<'a> /*trait*/ QSpacerItem_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizePolicy QSpacerItem::sizePolicy();
impl /*struct*/ QSpacerItem {
  pub fn sizePolicy<RetType, T: QSpacerItem_sizePolicy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy(self);
    // return 1;
  }
}

pub trait QSpacerItem_sizePolicy<RetType> {
  fn sizePolicy(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSizePolicy QSpacerItem::sizePolicy();
impl<'a> /*trait*/ QSpacerItem_sizePolicy<QSizePolicy> for () {
  fn sizePolicy(self , rsthis: &mut QSpacerItem) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem10sizePolicyEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem10sizePolicyEv(rsthis.qclsinst)};
    let mut ret1 = QSizePolicy::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSpacerItem::~QSpacerItem();
impl /*struct*/ QSpacerItem {
  pub fn FreeQSpacerItem<RetType, T: QSpacerItem_FreeQSpacerItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSpacerItem(self);
    // return 1;
  }
}

pub trait QSpacerItem_FreeQSpacerItem<RetType> {
  fn FreeQSpacerItem(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  void QSpacerItem::~QSpacerItem();
impl<'a> /*trait*/ QSpacerItem_FreeQSpacerItem<()> for () {
  fn FreeQSpacerItem(self , rsthis: &mut QSpacerItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItemD0Ev()};
     unsafe {_ZN11QSpacerItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QSpacerItem::sizeHint();
impl /*struct*/ QSpacerItem {
  pub fn sizeHint<RetType, T: QSpacerItem_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSpacerItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSize QSpacerItem::sizeHint();
impl<'a> /*trait*/ QSpacerItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QSpacerItem::maximumSize();
impl /*struct*/ QSpacerItem {
  pub fn maximumSize<RetType, T: QSpacerItem_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QSpacerItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSize QSpacerItem::maximumSize();
impl<'a> /*trait*/ QSpacerItem_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSpacerItem::isEmpty();
impl /*struct*/ QSpacerItem {
  pub fn isEmpty<RetType, T: QSpacerItem_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QSpacerItem_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  bool QSpacerItem::isEmpty();
impl<'a> /*trait*/ QSpacerItem_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QSpacerItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QSpacerItem::geometry();
impl /*struct*/ QSpacerItem {
  pub fn geometry<RetType, T: QSpacerItem_geometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QSpacerItem_geometry<RetType> {
  fn geometry(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QRect QSpacerItem::geometry();
impl<'a> /*trait*/ QSpacerItem_geometry<QRect> for () {
  fn geometry(self , rsthis: &mut QSpacerItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSpacerItem::setGeometry(const QRect & );
impl /*struct*/ QSpacerItem {
  pub fn setGeometry<RetType, T: QSpacerItem_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QSpacerItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  void QSpacerItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QSpacerItem_setGeometry<()> for (QRect) {
  fn setGeometry(self , rsthis: &mut QSpacerItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QSpacerItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSpacerItem * QSpacerItem::spacerItem();
impl /*struct*/ QSpacerItem {
  pub fn spacerItem<RetType, T: QSpacerItem_spacerItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.spacerItem(self);
    // return 1;
  }
}

pub trait QSpacerItem_spacerItem<RetType> {
  fn spacerItem(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSpacerItem * QSpacerItem::spacerItem();
impl<'a> /*trait*/ QSpacerItem_spacerItem<QSpacerItem> for () {
  fn spacerItem(self , rsthis: &mut QSpacerItem) -> QSpacerItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem10spacerItemEv()};
    let mut ret = unsafe {_ZN11QSpacerItem10spacerItemEv(rsthis.qclsinst)};
    let mut ret1 = QSpacerItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn inheritFrom(qthis: *mut c_void) -> QWidgetItem {
    return QWidgetItem{qbase: QLayoutItem::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QWidgetItem {
  type Target = QLayoutItem;

  fn deref(&self) -> &QLayoutItem {
    return &self.qbase;
  }
}
impl AsRef<QLayoutItem> for QWidgetItem {
  fn as_ref(&self) -> &QLayoutItem {
    return &self.qbase;
  }
}
  // proto:  QSize QWidgetItem::sizeHint();
impl /*struct*/ QWidgetItem {
  pub fn sizeHint<RetType, T: QWidgetItem_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QWidgetItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  QSize QWidgetItem::sizeHint();
impl<'a> /*trait*/ QWidgetItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QWidgetItem::minimumSize();
impl /*struct*/ QWidgetItem {
  pub fn minimumSize<RetType, T: QWidgetItem_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QWidgetItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  QSize QWidgetItem::minimumSize();
impl<'a> /*trait*/ QWidgetItem_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidgetItem::hasHeightForWidth();
impl /*struct*/ QWidgetItem {
  pub fn hasHeightForWidth<RetType, T: QWidgetItem_hasHeightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItem_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  bool QWidgetItem::hasHeightForWidth();
impl<'a> /*trait*/ QWidgetItem_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: &mut QWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWidgetItem::~QWidgetItem();
impl /*struct*/ QWidgetItem {
  pub fn FreeQWidgetItem<RetType, T: QWidgetItem_FreeQWidgetItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQWidgetItem(self);
    // return 1;
  }
}

pub trait QWidgetItem_FreeQWidgetItem<RetType> {
  fn FreeQWidgetItem(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  void QWidgetItem::~QWidgetItem();
impl<'a> /*trait*/ QWidgetItem_FreeQWidgetItem<()> for () {
  fn FreeQWidgetItem(self , rsthis: &mut QWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemD0Ev()};
     unsafe {_ZN11QWidgetItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidgetItem::QWidgetItem(QWidget * w);
impl /*struct*/ QWidgetItem {
  pub fn NewQWidgetItem<T: QWidgetItem_NewQWidgetItem>(value: T) -> QWidgetItem {
    let rsthis = value.NewQWidgetItem();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetItem_NewQWidgetItem {
  fn NewQWidgetItem(self) -> QWidgetItem;
}

  // proto:  void QWidgetItem::QWidgetItem(QWidget * w);
impl<'a> /*trait*/ QWidgetItem_NewQWidgetItem for (QWidget) {
  fn NewQWidgetItem(self) -> QWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWidgetItemC1EP7QWidget(qthis, arg0)};
    let rsthis = QWidgetItem{/**/qbase: QLayoutItem::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QWidget * QWidgetItem::widget();
impl /*struct*/ QWidgetItem {
  pub fn widget<RetType, T: QWidgetItem_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QWidgetItem_widget<RetType> {
  fn widget(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  QWidget * QWidgetItem::widget();
impl<'a> /*trait*/ QWidgetItem_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QWidgetItem) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem6widgetEv()};
    let mut ret = unsafe {_ZN11QWidgetItem6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidgetItem::setGeometry(const QRect & );
impl /*struct*/ QWidgetItem {
  pub fn setGeometry<RetType, T: QWidgetItem_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QWidgetItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  void QWidgetItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QWidgetItem_setGeometry<()> for (QRect) {
  fn setGeometry(self , rsthis: &mut QWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QWidgetItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWidgetItem::heightForWidth(int );
impl /*struct*/ QWidgetItem {
  pub fn heightForWidth<RetType, T: QWidgetItem_heightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItem_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  int QWidgetItem::heightForWidth(int );
impl<'a> /*trait*/ QWidgetItem_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QWidgetItem14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWidgetItem::QWidgetItem(const QWidgetItem & );
impl<'a> /*trait*/ QWidgetItem_NewQWidgetItem for (QWidgetItem) {
  fn NewQWidgetItem(self) -> QWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QWidgetItem{/**/qbase: QLayoutItem::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QWidgetItem::maximumSize();
impl /*struct*/ QWidgetItem {
  pub fn maximumSize<RetType, T: QWidgetItem_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QWidgetItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  QSize QWidgetItem::maximumSize();
impl<'a> /*trait*/ QWidgetItem_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidgetItem::isEmpty();
impl /*struct*/ QWidgetItem {
  pub fn isEmpty<RetType, T: QWidgetItem_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QWidgetItem_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  bool QWidgetItem::isEmpty();
impl<'a> /*trait*/ QWidgetItem_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QWidgetItem::geometry();
impl /*struct*/ QWidgetItem {
  pub fn geometry<RetType, T: QWidgetItem_geometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QWidgetItem_geometry<RetType> {
  fn geometry(self , rsthis: &mut QWidgetItem) -> RetType;
}

  // proto:  QRect QWidgetItem::geometry();
impl<'a> /*trait*/ QWidgetItem_geometry<QRect> for () {
  fn geometry(self , rsthis: &mut QWidgetItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidgetItemV2 {
  pub fn inheritFrom(qthis: *mut c_void) -> QWidgetItemV2 {
    return QWidgetItemV2{qbase: QWidgetItem::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QWidgetItemV2 {
  type Target = QWidgetItem;

  fn deref(&self) -> &QWidgetItem {
    return &self.qbase;
  }
}
impl AsRef<QWidgetItem> for QWidgetItemV2 {
  fn as_ref(&self) -> &QWidgetItem {
    return &self.qbase;
  }
}
  // proto:  QSize QWidgetItemV2::sizeHint();
impl /*struct*/ QWidgetItemV2 {
  pub fn sizeHint<RetType, T: QWidgetItemV2_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  QSize QWidgetItemV2::sizeHint();
impl<'a> /*trait*/ QWidgetItemV2_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QWidgetItemV2) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV28sizeHintEv()};
    let mut ret = unsafe {_ZNK13QWidgetItemV28sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QWidgetItemV2::minimumSize();
impl /*struct*/ QWidgetItemV2 {
  pub fn minimumSize<RetType, T: QWidgetItemV2_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  QSize QWidgetItemV2::minimumSize();
impl<'a> /*trait*/ QWidgetItemV2_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QWidgetItemV2) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV211minimumSizeEv()};
    let mut ret = unsafe {_ZNK13QWidgetItemV211minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWidgetItemV2::heightForWidth(int width);
impl /*struct*/ QWidgetItemV2 {
  pub fn heightForWidth<RetType, T: QWidgetItemV2_heightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  int QWidgetItemV2::heightForWidth(int width);
impl<'a> /*trait*/ QWidgetItemV2_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QWidgetItemV2) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV214heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QWidgetItemV214heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWidgetItemV2::~QWidgetItemV2();
impl /*struct*/ QWidgetItemV2 {
  pub fn FreeQWidgetItemV2<RetType, T: QWidgetItemV2_FreeQWidgetItemV2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQWidgetItemV2(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_FreeQWidgetItemV2<RetType> {
  fn FreeQWidgetItemV2(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  void QWidgetItemV2::~QWidgetItemV2();
impl<'a> /*trait*/ QWidgetItemV2_FreeQWidgetItemV2<()> for () {
  fn FreeQWidgetItemV2(self , rsthis: &mut QWidgetItemV2) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2D0Ev()};
     unsafe {_ZN13QWidgetItemV2D0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidgetItemV2::QWidgetItemV2(QWidget * widget);
impl /*struct*/ QWidgetItemV2 {
  pub fn NewQWidgetItemV2<T: QWidgetItemV2_NewQWidgetItemV2>(value: T) -> QWidgetItemV2 {
    let rsthis = value.NewQWidgetItemV2();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetItemV2_NewQWidgetItemV2 {
  fn NewQWidgetItemV2(self) -> QWidgetItemV2;
}

  // proto:  void QWidgetItemV2::QWidgetItemV2(QWidget * widget);
impl<'a> /*trait*/ QWidgetItemV2_NewQWidgetItemV2 for (QWidget) {
  fn NewQWidgetItemV2(self) -> QWidgetItemV2 {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2C1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetItemV2C1EP7QWidget(qthis, arg0)};
    let rsthis = QWidgetItemV2{/**/qbase: QWidgetItem::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QWidgetItemV2::maximumSize();
impl /*struct*/ QWidgetItemV2 {
  pub fn maximumSize<RetType, T: QWidgetItemV2_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  QSize QWidgetItemV2::maximumSize();
impl<'a> /*trait*/ QWidgetItemV2_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QWidgetItemV2) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV211maximumSizeEv()};
    let mut ret = unsafe {_ZNK13QWidgetItemV211maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidgetItemV2::QWidgetItemV2(const QWidgetItemV2 & );
impl<'a> /*trait*/ QWidgetItemV2_NewQWidgetItemV2 for (QWidgetItemV2) {
  fn NewQWidgetItemV2(self) -> QWidgetItemV2 {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2C1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetItemV2C1ERKS_(qthis, arg0)};
    let rsthis = QWidgetItemV2{/**/qbase: QWidgetItem::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

