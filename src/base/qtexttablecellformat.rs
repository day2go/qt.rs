// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextformat::QTextFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextTableCellFormat::NewQTextTableCellFormat();
  fn _ZN20QTextTableCellFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QTextTableCellFormat::setLeftPadding(qreal padding);
  fn _ZN20QTextTableCellFormat14setLeftPaddingEd(arg0: c_double) -> i32;
  // proto: bool QTextTableCellFormat::isValid();
  fn _ZNK20QTextTableCellFormat7isValidEv() -> i32;
  // proto: void QTextTableCellFormat::setTopPadding(qreal padding);
  fn _ZN20QTextTableCellFormat13setTopPaddingEd(arg0: c_double) -> i32;
  // proto: double QTextTableCellFormat::leftPadding();
  fn _ZNK20QTextTableCellFormat11leftPaddingEv() -> i32;
  // proto: void QTextTableCellFormat::setPadding(qreal padding);
  fn _ZN20QTextTableCellFormat10setPaddingEd(arg0: c_double) -> i32;
  // proto: double QTextTableCellFormat::topPadding();
  fn _ZNK20QTextTableCellFormat10topPaddingEv() -> i32;
  // proto: double QTextTableCellFormat::rightPadding();
  fn _ZNK20QTextTableCellFormat12rightPaddingEv() -> i32;
  // proto: void QTextTableCellFormat::NewQTextTableCellFormat(const QTextFormat & fmt);
  fn _ZN20QTextTableCellFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QTextTableCellFormat::bottomPadding();
  fn _ZNK20QTextTableCellFormat13bottomPaddingEv() -> i32;
  // proto: void QTextTableCellFormat::setRightPadding(qreal padding);
  fn _ZN20QTextTableCellFormat15setRightPaddingEd(arg0: c_double) -> i32;
  // proto: void QTextTableCellFormat::setBottomPadding(qreal padding);
  fn _ZN20QTextTableCellFormat16setBottomPaddingEd(arg0: c_double) -> i32;
}

// body block begin
// class sizeof(QTextTableCellFormat)=1
pub struct QTextTableCellFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextTableCellFormat {
  pub fn NewQTextTableCellFormat<T: QTextTableCellFormat_NewQTextTableCellFormat>(value: T) -> QTextTableCellFormat {
    let rsthis = value.NewQTextTableCellFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableCellFormat_NewQTextTableCellFormat {
  fn NewQTextTableCellFormat(self) -> QTextTableCellFormat;
}

// proto: void QTextTableCellFormat::NewQTextTableCellFormat();
impl<'a> /*trait*/ QTextTableCellFormat_NewQTextTableCellFormat for () {
  fn NewQTextTableCellFormat(self) -> QTextTableCellFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormatC1Ev()};
    unsafe {_ZN20QTextTableCellFormatC1Ev(qthis)};
    let rsthis = QTextTableCellFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setLeftPadding<T: QTextTableCellFormat_setLeftPadding>(&mut self, value: T) -> i32 {
    value.setLeftPadding(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_setLeftPadding {
  fn setLeftPadding(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: void QTextTableCellFormat::setLeftPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setLeftPadding for (f64) {
  fn setLeftPadding(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat14setLeftPaddingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN20QTextTableCellFormat14setLeftPaddingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn isValid<T: QTextTableCellFormat_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_isValid {
  fn isValid(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: bool QTextTableCellFormat::isValid();
impl<'a> /*trait*/ QTextTableCellFormat_isValid for () {
  fn isValid(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat7isValidEv()};
    unsafe {_ZNK20QTextTableCellFormat7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setTopPadding<T: QTextTableCellFormat_setTopPadding>(&mut self, value: T) -> i32 {
    value.setTopPadding(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_setTopPadding {
  fn setTopPadding(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: void QTextTableCellFormat::setTopPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setTopPadding for (f64) {
  fn setTopPadding(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat13setTopPaddingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN20QTextTableCellFormat13setTopPaddingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn leftPadding<T: QTextTableCellFormat_leftPadding>(&mut self, value: T) -> i32 {
    value.leftPadding(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_leftPadding {
  fn leftPadding(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: double QTextTableCellFormat::leftPadding();
impl<'a> /*trait*/ QTextTableCellFormat_leftPadding for () {
  fn leftPadding(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat11leftPaddingEv()};
    unsafe {_ZNK20QTextTableCellFormat11leftPaddingEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setPadding<T: QTextTableCellFormat_setPadding>(&mut self, value: T) -> i32 {
    value.setPadding(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_setPadding {
  fn setPadding(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: void QTextTableCellFormat::setPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setPadding for (f64) {
  fn setPadding(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat10setPaddingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN20QTextTableCellFormat10setPaddingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn topPadding<T: QTextTableCellFormat_topPadding>(&mut self, value: T) -> i32 {
    value.topPadding(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_topPadding {
  fn topPadding(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: double QTextTableCellFormat::topPadding();
impl<'a> /*trait*/ QTextTableCellFormat_topPadding for () {
  fn topPadding(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat10topPaddingEv()};
    unsafe {_ZNK20QTextTableCellFormat10topPaddingEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn rightPadding<T: QTextTableCellFormat_rightPadding>(&mut self, value: T) -> i32 {
    value.rightPadding(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_rightPadding {
  fn rightPadding(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: double QTextTableCellFormat::rightPadding();
impl<'a> /*trait*/ QTextTableCellFormat_rightPadding for () {
  fn rightPadding(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat12rightPaddingEv()};
    unsafe {_ZNK20QTextTableCellFormat12rightPaddingEv()};
    return 1;
  }
}

// proto: void QTextTableCellFormat::NewQTextTableCellFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextTableCellFormat_NewQTextTableCellFormat for (&'a  QTextFormat) {
  fn NewQTextTableCellFormat(self) -> QTextTableCellFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN20QTextTableCellFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextTableCellFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn bottomPadding<T: QTextTableCellFormat_bottomPadding>(&mut self, value: T) -> i32 {
    value.bottomPadding(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_bottomPadding {
  fn bottomPadding(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: double QTextTableCellFormat::bottomPadding();
impl<'a> /*trait*/ QTextTableCellFormat_bottomPadding for () {
  fn bottomPadding(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat13bottomPaddingEv()};
    unsafe {_ZNK20QTextTableCellFormat13bottomPaddingEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setRightPadding<T: QTextTableCellFormat_setRightPadding>(&mut self, value: T) -> i32 {
    value.setRightPadding(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_setRightPadding {
  fn setRightPadding(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: void QTextTableCellFormat::setRightPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setRightPadding for (f64) {
  fn setRightPadding(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat15setRightPaddingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN20QTextTableCellFormat15setRightPaddingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setBottomPadding<T: QTextTableCellFormat_setBottomPadding>(&mut self, value: T) -> i32 {
    value.setBottomPadding(self);
    return 1;
  }
}

pub trait QTextTableCellFormat_setBottomPadding {
  fn setBottomPadding(self, this: &mut QTextTableCellFormat) -> i32;
}

// proto: void QTextTableCellFormat::setBottomPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setBottomPadding for (f64) {
  fn setBottomPadding(self, this: &mut QTextTableCellFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat16setBottomPaddingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN20QTextTableCellFormat16setBottomPaddingEd(arg0)};
    return 1;
  }
}
