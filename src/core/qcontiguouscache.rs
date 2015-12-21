// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qcontiguouscache.h
// dst-file: /src/core/qcontiguouscache.rs
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
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto: static QContiguousCacheData * QContiguousCacheData::allocateData(int size, int alignment);
  fn _ZN20QContiguousCacheData12allocateDataEii(arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto: static void QContiguousCacheData::freeData(QContiguousCacheData * data);
  fn _ZN20QContiguousCacheData8freeDataEPS_(arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QContiguousCacheData)=1
pub struct QContiguousCacheData {
  pub qclsinst: *mut c_void,
}

  // proto: static QContiguousCacheData * QContiguousCacheData::allocateData(int size, int alignment);
impl /*struct*/ QContiguousCacheData {
  pub fn allocateData_s<RetType, T: QContiguousCacheData_allocateData_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.allocateData_s();
    // return 1;
  }
}

pub trait QContiguousCacheData_allocateData_s<RetType> {
  fn allocateData_s(self ) -> RetType;
}

  // proto: static QContiguousCacheData * QContiguousCacheData::allocateData(int size, int alignment);
impl<'a> /*trait*/ QContiguousCacheData_allocateData_s<QContiguousCacheData> for (i32, i32) {
  fn allocateData_s(self ) -> QContiguousCacheData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QContiguousCacheData12allocateDataEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN20QContiguousCacheData12allocateDataEii(arg0, arg1)};
    let mut ret1 = QContiguousCacheData{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static void QContiguousCacheData::freeData(QContiguousCacheData * data);
impl /*struct*/ QContiguousCacheData {
  pub fn freeData_s<RetType, T: QContiguousCacheData_freeData_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.freeData_s();
    // return 1;
  }
}

pub trait QContiguousCacheData_freeData_s<RetType> {
  fn freeData_s(self ) -> RetType;
}

  // proto: static void QContiguousCacheData::freeData(QContiguousCacheData * data);
impl<'a> /*trait*/ QContiguousCacheData_freeData_s<()> for (QContiguousCacheData) {
  fn freeData_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QContiguousCacheData8freeDataEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QContiguousCacheData8freeDataEPS_(arg0)};
    // return 1;
  }
}

// <= body block end
