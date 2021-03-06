// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qcursor.h
// dst-file: /src/gui/qcursor.rs
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
use super::qscreen::*; // 773
use super::qpixmap::*; // 773
use super::qbitmap::*; // 773
use super::super::core::qpoint::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCursor_Class_Size() -> c_int;
  // proto: static void QCursor::setPos(QScreen * screen, int x, int y);
  fn C_ZN7QCursor6setPosEP7QScreenii(arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  QPixmap QCursor::pixmap();
  fn C_ZNK7QCursor6pixmapEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCursor::QCursor(const QBitmap & bitmap, const QBitmap & mask, int hotX, int hotY);
  fn C_ZN7QCursorC2ERK7QBitmapS2_ii(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int) -> u64;
  // proto:  void QCursor::QCursor(const QPixmap & pixmap, int hotX, int hotY);
  fn C_ZN7QCursorC2ERK7QPixmapii(arg0: *mut c_void, arg1: c_int, arg2: c_int) -> u64;
  // proto:  void QCursor::~QCursor();
  fn C_ZN7QCursorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QBitmap * QCursor::mask();
  fn C_ZNK7QCursor4maskEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCursor::QCursor(const QCursor & cursor);
  fn C_ZN7QCursorC2ERKS_(arg0: *mut c_void) -> u64;
  // proto: static void QCursor::setPos(int x, int y);
  fn C_ZN7QCursor6setPosEii(arg0: c_int, arg1: c_int);
  // proto: static void QCursor::setPos(QScreen * screen, const QPoint & p);
  fn C_ZN7QCursor6setPosEP7QScreenRK6QPoint(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static void QCursor::setPos(const QPoint & p);
  fn C_ZN7QCursor6setPosERK6QPoint(arg0: *mut c_void);
  // proto:  const QBitmap * QCursor::bitmap();
  fn C_ZNK7QCursor6bitmapEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QPoint QCursor::pos(const QScreen * screen);
  fn C_ZN7QCursor3posEPK7QScreen(arg0: *mut c_void) -> *mut c_void;
  // proto: static QPoint QCursor::pos();
  fn C_ZN7QCursor3posEv() -> *mut c_void;
  // proto:  void QCursor::QCursor();
  fn C_ZN7QCursorC2Ev() -> u64;
  // proto:  QPoint QCursor::hotSpot();
  fn C_ZNK7QCursor7hotSpotEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QCursor)=8
#[derive(Default)]
pub struct QCursor {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QCursor {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QCursor {
    return QCursor{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static void QCursor::setPos(QScreen * screen, int x, int y);
impl /*struct*/ QCursor {
  pub fn setPos_s<RetType, T: QCursor_setPos_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPos_s();
    // return 1;
  }
}

pub trait QCursor_setPos_s<RetType> {
  fn setPos_s(self ) -> RetType;
}

  // proto: static void QCursor::setPos(QScreen * screen, int x, int y);
impl<'a> /*trait*/ QCursor_setPos_s<()> for (&'a QScreen, i32, i32) {
  fn setPos_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosEP7QScreenii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN7QCursor6setPosEP7QScreenii(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QPixmap QCursor::pixmap();
impl /*struct*/ QCursor {
  pub fn pixmap<RetType, T: QCursor_pixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixmap(self);
    // return 1;
  }
}

pub trait QCursor_pixmap<RetType> {
  fn pixmap(self , rsthis: & QCursor) -> RetType;
}

  // proto:  QPixmap QCursor::pixmap();
impl<'a> /*trait*/ QCursor_pixmap<QPixmap> for () {
  fn pixmap(self , rsthis: & QCursor) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor6pixmapEv()};
    let mut ret = unsafe {C_ZNK7QCursor6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCursor::QCursor(const QBitmap & bitmap, const QBitmap & mask, int hotX, int hotY);
impl /*struct*/ QCursor {
  pub fn new<T: QCursor_new>(value: T) -> QCursor {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QCursor_new {
  fn new(self) -> QCursor;
}

  // proto:  void QCursor::QCursor(const QBitmap & bitmap, const QBitmap & mask, int hotX, int hotY);
impl<'a> /*trait*/ QCursor_new for (&'a QBitmap, &'a QBitmap, Option<i32>, Option<i32>) {
  fn new(self) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorC2ERK7QBitmapS2_ii()};
    let ctysz: c_int = unsafe{QCursor_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = (if self.2.is_none() {-1} else {self.2.unwrap()})  as c_int;
    let arg3 = (if self.3.is_none() {-1} else {self.3.unwrap()})  as c_int;
    let qthis: u64 = unsafe {C_ZN7QCursorC2ERK7QBitmapS2_ii(arg0, arg1, arg2, arg3)};
    let rsthis = QCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCursor::QCursor(const QPixmap & pixmap, int hotX, int hotY);
impl<'a> /*trait*/ QCursor_new for (&'a QPixmap, Option<i32>, Option<i32>) {
  fn new(self) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorC2ERK7QPixmapii()};
    let ctysz: c_int = unsafe{QCursor_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {-1} else {self.1.unwrap()})  as c_int;
    let arg2 = (if self.2.is_none() {-1} else {self.2.unwrap()})  as c_int;
    let qthis: u64 = unsafe {C_ZN7QCursorC2ERK7QPixmapii(arg0, arg1, arg2)};
    let rsthis = QCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCursor::~QCursor();
impl /*struct*/ QCursor {
  pub fn free<RetType, T: QCursor_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QCursor_free<RetType> {
  fn free(self , rsthis: & QCursor) -> RetType;
}

  // proto:  void QCursor::~QCursor();
impl<'a> /*trait*/ QCursor_free<()> for () {
  fn free(self , rsthis: & QCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorD2Ev()};
     unsafe {C_ZN7QCursorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QBitmap * QCursor::mask();
impl /*struct*/ QCursor {
  pub fn mask<RetType, T: QCursor_mask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mask(self);
    // return 1;
  }
}

pub trait QCursor_mask<RetType> {
  fn mask(self , rsthis: & QCursor) -> RetType;
}

  // proto:  const QBitmap * QCursor::mask();
impl<'a> /*trait*/ QCursor_mask<QBitmap> for () {
  fn mask(self , rsthis: & QCursor) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor4maskEv()};
    let mut ret = unsafe {C_ZNK7QCursor4maskEv(rsthis.qclsinst)};
    let mut ret1 = QBitmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCursor::QCursor(const QCursor & cursor);
impl<'a> /*trait*/ QCursor_new for (&'a QCursor) {
  fn new(self) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorC2ERKS_()};
    let ctysz: c_int = unsafe{QCursor_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QCursorC2ERKS_(arg0)};
    let rsthis = QCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QCursor::setPos(int x, int y);
impl<'a> /*trait*/ QCursor_setPos_s<()> for (i32, i32) {
  fn setPos_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QCursor6setPosEii(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QCursor::setPos(QScreen * screen, const QPoint & p);
impl<'a> /*trait*/ QCursor_setPos_s<()> for (&'a QScreen, &'a QPoint) {
  fn setPos_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosEP7QScreenRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QCursor6setPosEP7QScreenRK6QPoint(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QCursor::setPos(const QPoint & p);
impl<'a> /*trait*/ QCursor_setPos_s<()> for (&'a QPoint) {
  fn setPos_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QCursor6setPosERK6QPoint(arg0)};
    // return 1;
  }
}

  // proto:  const QBitmap * QCursor::bitmap();
impl /*struct*/ QCursor {
  pub fn bitmap<RetType, T: QCursor_bitmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bitmap(self);
    // return 1;
  }
}

pub trait QCursor_bitmap<RetType> {
  fn bitmap(self , rsthis: & QCursor) -> RetType;
}

  // proto:  const QBitmap * QCursor::bitmap();
impl<'a> /*trait*/ QCursor_bitmap<QBitmap> for () {
  fn bitmap(self , rsthis: & QCursor) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor6bitmapEv()};
    let mut ret = unsafe {C_ZNK7QCursor6bitmapEv(rsthis.qclsinst)};
    let mut ret1 = QBitmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QPoint QCursor::pos(const QScreen * screen);
impl /*struct*/ QCursor {
  pub fn pos_s<RetType, T: QCursor_pos_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.pos_s();
    // return 1;
  }
}

pub trait QCursor_pos_s<RetType> {
  fn pos_s(self ) -> RetType;
}

  // proto: static QPoint QCursor::pos(const QScreen * screen);
impl<'a> /*trait*/ QCursor_pos_s<QPoint> for (&'a QScreen) {
  fn pos_s(self ) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor3posEPK7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QCursor3posEPK7QScreen(arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QPoint QCursor::pos();
impl<'a> /*trait*/ QCursor_pos_s<QPoint> for () {
  fn pos_s(self ) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor3posEv()};
    let mut ret = unsafe {C_ZN7QCursor3posEv()};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCursor::QCursor();
impl<'a> /*trait*/ QCursor_new for () {
  fn new(self) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorC2Ev()};
    let ctysz: c_int = unsafe{QCursor_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN7QCursorC2Ev()};
    let rsthis = QCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPoint QCursor::hotSpot();
impl /*struct*/ QCursor {
  pub fn hotSpot<RetType, T: QCursor_hotSpot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hotSpot(self);
    // return 1;
  }
}

pub trait QCursor_hotSpot<RetType> {
  fn hotSpot(self , rsthis: & QCursor) -> RetType;
}

  // proto:  QPoint QCursor::hotSpot();
impl<'a> /*trait*/ QCursor_hotSpot<QPoint> for () {
  fn hotSpot(self , rsthis: & QCursor) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor7hotSpotEv()};
    let mut ret = unsafe {C_ZNK7QCursor7hotSpotEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

