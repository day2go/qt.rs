// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK5QSize9boundedToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK5QSize7isValidEv() -> i32;
  fn _ZNK5QSize6isNullEv() -> i32;
  fn _ZN5QSizeC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK5QSize10expandedToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK5QSize6heightEv() -> i32;
  fn _ZN5QSize7rheightEv() -> i32;
  fn _ZN5QSizeC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK5QSize5widthEv() -> i32;
  fn _ZNK5QSize10transposedEv() -> i32;
  fn _ZN5QSize6rwidthEv() -> i32;
  fn _ZN5QSize9setHeightEi(arg0: c_int) -> i32;
  fn _ZNK5QSize7isEmptyEv() -> i32;
  fn _ZN5QSize8setWidthEi(arg0: c_int) -> i32;
  fn _ZN5QSize9transposeEv() -> i32;
}

// body block begin
// class sizeof(QSize)=8
pub struct QSize {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSize {
  pub fn boundedTo<T: QSize_boundedTo>(&mut self, value: T) -> i32 {
    value.boundedTo(self);
    return 1;
  }
}

pub trait QSize_boundedTo {
  fn boundedTo(self, this: &mut QSize) -> i32;
}

// proto: QSize QSize::boundedTo(const QSize & );
impl<'a> /*trait*/ QSize_boundedTo for (&'a  QSize) {
  fn boundedTo(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize9boundedToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QSize9boundedToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn isValid<T: QSize_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QSize_isValid {
  fn isValid(self, this: &mut QSize) -> i32;
}

// proto: bool QSize::isValid();
impl<'a> /*trait*/ QSize_isValid for () {
  fn isValid(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize7isValidEv()};
    unsafe {_ZNK5QSize7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn isNull<T: QSize_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QSize_isNull {
  fn isNull(self, this: &mut QSize) -> i32;
}

// proto: bool QSize::isNull();
impl<'a> /*trait*/ QSize_isNull for () {
  fn isNull(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize6isNullEv()};
    unsafe {_ZNK5QSize6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn NewQSize<T: QSize_NewQSize>(value: T) -> QSize {
    let rsthis = value.NewQSize();
    return rsthis;
    // return 1;
  }
}

pub trait QSize_NewQSize {
  fn NewQSize(self) -> QSize;
}

// proto: void QSize::NewQSize();
impl<'a> /*trait*/ QSize_NewQSize for () {
  fn NewQSize(self) -> QSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSizeC1Ev()};
    unsafe {_ZN5QSizeC1Ev(qthis)};
    let rsthis = QSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn expandedTo<T: QSize_expandedTo>(&mut self, value: T) -> i32 {
    value.expandedTo(self);
    return 1;
  }
}

pub trait QSize_expandedTo {
  fn expandedTo(self, this: &mut QSize) -> i32;
}

// proto: QSize QSize::expandedTo(const QSize & );
impl<'a> /*trait*/ QSize_expandedTo for (&'a  QSize) {
  fn expandedTo(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize10expandedToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QSize10expandedToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn height<T: QSize_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QSize_height {
  fn height(self, this: &mut QSize) -> i32;
}

// proto: int QSize::height();
impl<'a> /*trait*/ QSize_height for () {
  fn height(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize6heightEv()};
    unsafe {_ZNK5QSize6heightEv()};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn rheight<T: QSize_rheight>(&mut self, value: T) -> i32 {
    value.rheight(self);
    return 1;
  }
}

pub trait QSize_rheight {
  fn rheight(self, this: &mut QSize) -> i32;
}

// proto: int & QSize::rheight();
impl<'a> /*trait*/ QSize_rheight for () {
  fn rheight(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize7rheightEv()};
    unsafe {_ZN5QSize7rheightEv()};
    return 1;
  }
}

// proto: void QSize::NewQSize(int w, int h);
impl<'a> /*trait*/ QSize_NewQSize for (i32, i32) {
  fn NewQSize(self) -> QSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSizeC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN5QSizeC1Eii(qthis, arg0, arg1)};
    let rsthis = QSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn width<T: QSize_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QSize_width {
  fn width(self, this: &mut QSize) -> i32;
}

// proto: int QSize::width();
impl<'a> /*trait*/ QSize_width for () {
  fn width(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize5widthEv()};
    unsafe {_ZNK5QSize5widthEv()};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn transposed<T: QSize_transposed>(&mut self, value: T) -> i32 {
    value.transposed(self);
    return 1;
  }
}

pub trait QSize_transposed {
  fn transposed(self, this: &mut QSize) -> i32;
}

// proto: QSize QSize::transposed();
impl<'a> /*trait*/ QSize_transposed for () {
  fn transposed(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize10transposedEv()};
    unsafe {_ZNK5QSize10transposedEv()};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn rwidth<T: QSize_rwidth>(&mut self, value: T) -> i32 {
    value.rwidth(self);
    return 1;
  }
}

pub trait QSize_rwidth {
  fn rwidth(self, this: &mut QSize) -> i32;
}

// proto: int & QSize::rwidth();
impl<'a> /*trait*/ QSize_rwidth for () {
  fn rwidth(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize6rwidthEv()};
    unsafe {_ZN5QSize6rwidthEv()};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn setHeight<T: QSize_setHeight>(&mut self, value: T) -> i32 {
    value.setHeight(self);
    return 1;
  }
}

pub trait QSize_setHeight {
  fn setHeight(self, this: &mut QSize) -> i32;
}

// proto: void QSize::setHeight(int h);
impl<'a> /*trait*/ QSize_setHeight for (i32) {
  fn setHeight(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize9setHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QSize9setHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn isEmpty<T: QSize_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QSize_isEmpty {
  fn isEmpty(self, this: &mut QSize) -> i32;
}

// proto: bool QSize::isEmpty();
impl<'a> /*trait*/ QSize_isEmpty for () {
  fn isEmpty(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize7isEmptyEv()};
    unsafe {_ZNK5QSize7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn setWidth<T: QSize_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QSize_setWidth {
  fn setWidth(self, this: &mut QSize) -> i32;
}

// proto: void QSize::setWidth(int w);
impl<'a> /*trait*/ QSize_setWidth for (i32) {
  fn setWidth(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize8setWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QSize8setWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSize {
  pub fn transpose<T: QSize_transpose>(&mut self, value: T) -> i32 {
    value.transpose(self);
    return 1;
  }
}

pub trait QSize_transpose {
  fn transpose(self, this: &mut QSize) -> i32;
}

// proto: void QSize::transpose();
impl<'a> /*trait*/ QSize_transpose for () {
  fn transpose(self, this: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize9transposeEv()};
    unsafe {_ZN5QSize9transposeEv()};
    return 1;
  }
}
