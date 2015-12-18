// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qitemeditorcreatorbase::QItemEditorCreatorBase;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QItemEditorFactory::NewQItemEditorFactory();
  fn _ZN18QItemEditorFactoryC1Ev(qthis: *mut c_void) ;
  // proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
  fn _ZNK18QItemEditorFactory17valuePropertyNameEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
  fn _ZN18QItemEditorFactory14defaultFactoryEv() -> *mut c_void;
  // proto:  void QItemEditorFactory::FreeQItemEditorFactory();
  fn _ZN18QItemEditorFactoryD0Ev(qthis: *mut c_void) ;
  // proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
  fn _ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
  fn _ZN18QItemEditorFactory17setDefaultFactoryEPS_(arg0: *mut c_void) ;
  // proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
  fn _ZNK18QItemEditorFactory12createEditorEiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QItemEditorFactory)=1
pub struct QItemEditorFactory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemEditorFactory {
  pub fn NewQItemEditorFactory<T: QItemEditorFactory_NewQItemEditorFactory>(value: T) -> QItemEditorFactory {
    let rsthis = value.NewQItemEditorFactory();
    return rsthis;
    // return 1;
  }
}

pub trait QItemEditorFactory_NewQItemEditorFactory {
  fn NewQItemEditorFactory(self) -> QItemEditorFactory;
}

// proto: void QItemEditorFactory::NewQItemEditorFactory();
impl<'a> /*trait*/ QItemEditorFactory_NewQItemEditorFactory for () {
  fn NewQItemEditorFactory(self) -> QItemEditorFactory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactoryC1Ev()};
    unsafe {_ZN18QItemEditorFactoryC1Ev(qthis)};
    let rsthis = QItemEditorFactory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn valuePropertyName<RetType, T: QItemEditorFactory_valuePropertyName<RetType>>(&mut self, value: T) -> RetType {
    return value.valuePropertyName(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_valuePropertyName<RetType> {
  fn valuePropertyName(self, rsthis: &mut QItemEditorFactory) -> RetType;
}

// proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
impl<'a> /*trait*/ QItemEditorFactory_valuePropertyName<QByteArray> for (i32) {
  fn valuePropertyName(self, rsthis: &mut QItemEditorFactory) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QItemEditorFactory17valuePropertyNameEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK18QItemEditorFactory17valuePropertyNameEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn defaultFactory<RetType, T: QItemEditorFactory_defaultFactory<RetType>>(&mut self, value: T) -> RetType {
    return value.defaultFactory(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_defaultFactory<RetType> {
  fn defaultFactory(self, rsthis: &mut QItemEditorFactory) -> RetType;
}

// proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
impl<'a> /*trait*/ QItemEditorFactory_defaultFactory<QItemEditorFactory> for () {
  fn defaultFactory(self, rsthis: &mut QItemEditorFactory) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory14defaultFactoryEv()};
    let mut ret = unsafe {_ZN18QItemEditorFactory14defaultFactoryEv()};
    let mut ret1 = QItemEditorFactory{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn FreeQItemEditorFactory<RetType, T: QItemEditorFactory_FreeQItemEditorFactory<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQItemEditorFactory(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_FreeQItemEditorFactory<RetType> {
  fn FreeQItemEditorFactory(self, rsthis: &mut QItemEditorFactory) -> RetType;
}

// proto:  void QItemEditorFactory::FreeQItemEditorFactory();
impl<'a> /*trait*/ QItemEditorFactory_FreeQItemEditorFactory<()> for () {
  fn FreeQItemEditorFactory(self, rsthis: &mut QItemEditorFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactoryD0Ev()};
     unsafe {_ZN18QItemEditorFactoryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn registerEditor<RetType, T: QItemEditorFactory_registerEditor<RetType>>(&mut self, value: T) -> RetType {
    return value.registerEditor(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_registerEditor<RetType> {
  fn registerEditor(self, rsthis: &mut QItemEditorFactory) -> RetType;
}

// proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
impl<'a> /*trait*/ QItemEditorFactory_registerEditor<()> for (i32, &'a mut QItemEditorCreatorBase) {
  fn registerEditor(self, rsthis: &mut QItemEditorFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn setDefaultFactory<RetType, T: QItemEditorFactory_setDefaultFactory<RetType>>(&mut self, value: T) -> RetType {
    return value.setDefaultFactory(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_setDefaultFactory<RetType> {
  fn setDefaultFactory(self, rsthis: &mut QItemEditorFactory) -> RetType;
}

// proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QItemEditorFactory_setDefaultFactory<()> for (&'a mut QItemEditorFactory) {
  fn setDefaultFactory(self, rsthis: &mut QItemEditorFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory17setDefaultFactoryEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QItemEditorFactory17setDefaultFactoryEPS_(arg0)};
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn createEditor<RetType, T: QItemEditorFactory_createEditor<RetType>>(&mut self, value: T) -> RetType {
    return value.createEditor(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_createEditor<RetType> {
  fn createEditor(self, rsthis: &mut QItemEditorFactory) -> RetType;
}

// proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
impl<'a> /*trait*/ QItemEditorFactory_createEditor<QWidget> for (i32, &'a mut QWidget) {
  fn createEditor(self, rsthis: &mut QItemEditorFactory) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QItemEditorFactory12createEditorEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QItemEditorFactory12createEditorEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

