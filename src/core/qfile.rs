// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qobject::QObject;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QFile::symLinkTarget();
  fn _ZNK5QFile13symLinkTargetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFile::QFile();
  fn _ZN5QFileC1Ev(qthis: *mut c_void);
  // proto:  void QFile::QFile(QObject * parent);
  fn _ZN5QFileC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static bool QFile::link(const QString & oldname, const QString & newName);
  fn _ZN5QFile4linkERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static bool QFile::rename(const QString & oldName, const QString & newName);
  fn _ZN5QFile6renameERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  bool QFile::link(const QString & newName);
  fn _ZN5QFile4linkERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto: static bool QFile::resize(const QString & filename, qint64 sz);
  fn _ZN5QFile6resizeERK7QStringx(arg0: *mut c_void, arg1: c_longlong) -> c_char;
  // proto: static bool QFile::exists(const QString & fileName);
  fn _ZN5QFile6existsERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  void QFile::~QFile();
  fn _ZN5QFileD0Ev(qthis: *mut c_void);
  // proto: static bool QFile::copy(const QString & fileName, const QString & newName);
  fn _ZN5QFile4copyERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static QString QFile::readLink(const QString & fileName);
  fn _ZN5QFile8readLinkERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFile::exists();
  fn _ZNK5QFile6existsEv(qthis: *mut c_void) -> c_char;
  // proto:  qint64 QFile::size();
  fn _ZNK5QFile4sizeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QFile::resize(qint64 sz);
  fn _ZN5QFile6resizeEx(qthis: *mut c_void, arg0: c_longlong) -> c_char;
  // proto:  void QFile::QFile(const QFile & );
  fn _ZN5QFileC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFile::setFileName(const QString & name);
  fn _ZN5QFile11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QFile::remove();
  fn _ZN5QFile6removeEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QFile::copy(const QString & newName);
  fn _ZN5QFile4copyERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto: static QByteArray QFile::encodeName(const QString & fileName);
  fn _ZN5QFile10encodeNameERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QFile::decodeName(const QByteArray & localFileName);
  fn _ZN5QFile10decodeNameERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFile::rename(const QString & newName);
  fn _ZN5QFile6renameERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QString QFile::fileName();
  fn _ZNK5QFile8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QString QFile::decodeName(const char * localFileName);
  fn _ZN5QFile10decodeNameEPKc(arg0: *mut c_char) -> *mut c_void;
  // proto:  const QMetaObject * QFile::metaObject();
  fn _ZNK5QFile10metaObjectEv(qthis: *mut c_void);
  // proto:  void QFile::QFile(const QString & name, QObject * parent);
  fn _ZN5QFileC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto: static QString QFile::symLinkTarget(const QString & fileName);
  fn _ZN5QFile13symLinkTargetERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static bool QFile::remove(const QString & fileName);
  fn _ZN5QFile6removeERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  void QFile::QFile(const QString & name);
  fn _ZN5QFileC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QFile::readLink();
  fn _ZNK5QFile8readLinkEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QFile)=1
pub struct QFile {
  pub qclsinst: *mut c_void,
}

  // proto:  QString QFile::symLinkTarget();
impl /*struct*/ QFile {
  pub fn symLinkTarget<RetType, T: QFile_symLinkTarget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.symLinkTarget(self);
    // return 1;
  }
}

pub trait QFile_symLinkTarget<RetType> {
  fn symLinkTarget(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  QString QFile::symLinkTarget();
impl<'a> /*trait*/ QFile_symLinkTarget<QString> for () {
  fn symLinkTarget(self , rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile13symLinkTargetEv()};
    let mut ret = unsafe {_ZNK5QFile13symLinkTargetEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QFile::QFile();
impl /*struct*/ QFile {
  pub fn NewQFile<T: QFile_NewQFile>(value: T) -> QFile {
    let rsthis = value.NewQFile();
    return rsthis;
    // return 1;
  }
}

pub trait QFile_NewQFile {
  fn NewQFile(self) -> QFile;
}

  // proto:  void QFile::QFile();
impl<'a> /*trait*/ QFile_NewQFile for () {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1Ev()};
    unsafe {_ZN5QFileC1Ev(qthis)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFile::QFile(QObject * parent);
impl<'a> /*trait*/ QFile_NewQFile for (QObject) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1EP7QObject(qthis, arg0)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static bool QFile::link(const QString & oldname, const QString & newName);
impl /*struct*/ QFile {
  pub fn link_s<RetType, T: QFile_link_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.link_s();
    // return 1;
  }
}

pub trait QFile_link_s<RetType> {
  fn link_s(self ) -> RetType;
}

  // proto: static bool QFile::link(const QString & oldname, const QString & newName);
impl<'a> /*trait*/ QFile_link_s<i8> for (QString, QString) {
  fn link_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4linkERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile4linkERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QFile::rename(const QString & oldName, const QString & newName);
impl /*struct*/ QFile {
  pub fn rename_s<RetType, T: QFile_rename_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.rename_s();
    // return 1;
  }
}

pub trait QFile_rename_s<RetType> {
  fn rename_s(self ) -> RetType;
}

  // proto: static bool QFile::rename(const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QFile_rename_s<i8> for (QString, QString) {
  fn rename_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6renameERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile6renameERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFile::link(const QString & newName);
impl /*struct*/ QFile {
  pub fn link<RetType, T: QFile_link<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.link(self);
    // return 1;
  }
}

pub trait QFile_link<RetType> {
  fn link(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  bool QFile::link(const QString & newName);
impl<'a> /*trait*/ QFile_link<i8> for (QString) {
  fn link(self , rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4linkERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile4linkERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QFile::resize(const QString & filename, qint64 sz);
impl /*struct*/ QFile {
  pub fn resize_s<RetType, T: QFile_resize_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.resize_s();
    // return 1;
  }
}

pub trait QFile_resize_s<RetType> {
  fn resize_s(self ) -> RetType;
}

  // proto: static bool QFile::resize(const QString & filename, qint64 sz);
impl<'a> /*trait*/ QFile_resize_s<i8> for (QString, i64) {
  fn resize_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6resizeERK7QStringx()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_longlong;
    let mut ret = unsafe {_ZN5QFile6resizeERK7QStringx(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QFile::exists(const QString & fileName);
impl /*struct*/ QFile {
  pub fn exists_s<RetType, T: QFile_exists_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.exists_s();
    // return 1;
  }
}

pub trait QFile_exists_s<RetType> {
  fn exists_s(self ) -> RetType;
}

  // proto: static bool QFile::exists(const QString & fileName);
impl<'a> /*trait*/ QFile_exists_s<i8> for (QString) {
  fn exists_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6existsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile6existsERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFile::~QFile();
impl /*struct*/ QFile {
  pub fn FreeQFile<RetType, T: QFile_FreeQFile<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQFile(self);
    // return 1;
  }
}

pub trait QFile_FreeQFile<RetType> {
  fn FreeQFile(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  void QFile::~QFile();
impl<'a> /*trait*/ QFile_FreeQFile<()> for () {
  fn FreeQFile(self , rsthis: &mut QFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileD0Ev()};
     unsafe {_ZN5QFileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static bool QFile::copy(const QString & fileName, const QString & newName);
impl /*struct*/ QFile {
  pub fn copy_s<RetType, T: QFile_copy_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.copy_s();
    // return 1;
  }
}

pub trait QFile_copy_s<RetType> {
  fn copy_s(self ) -> RetType;
}

  // proto: static bool QFile::copy(const QString & fileName, const QString & newName);
impl<'a> /*trait*/ QFile_copy_s<i8> for (QString, QString) {
  fn copy_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4copyERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile4copyERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QString QFile::readLink(const QString & fileName);
impl /*struct*/ QFile {
  pub fn readLink_s<RetType, T: QFile_readLink_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.readLink_s();
    // return 1;
  }
}

pub trait QFile_readLink_s<RetType> {
  fn readLink_s(self ) -> RetType;
}

  // proto: static QString QFile::readLink(const QString & fileName);
impl<'a> /*trait*/ QFile_readLink_s<QString> for (QString) {
  fn readLink_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile8readLinkERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile8readLinkERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFile::exists();
impl /*struct*/ QFile {
  pub fn exists<RetType, T: QFile_exists<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.exists(self);
    // return 1;
  }
}

pub trait QFile_exists<RetType> {
  fn exists(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  bool QFile::exists();
impl<'a> /*trait*/ QFile_exists<i8> for () {
  fn exists(self , rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile6existsEv()};
    let mut ret = unsafe {_ZNK5QFile6existsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QFile::size();
impl /*struct*/ QFile {
  pub fn size<RetType, T: QFile_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QFile_size<RetType> {
  fn size(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  qint64 QFile::size();
impl<'a> /*trait*/ QFile_size<i64> for () {
  fn size(self , rsthis: &mut QFile) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile4sizeEv()};
    let mut ret = unsafe {_ZNK5QFile4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QFile::resize(qint64 sz);
impl /*struct*/ QFile {
  pub fn resize<RetType, T: QFile_resize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QFile_resize<RetType> {
  fn resize(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  bool QFile::resize(qint64 sz);
impl<'a> /*trait*/ QFile_resize<i8> for (i64) {
  fn resize(self , rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6resizeEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN5QFile6resizeEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFile::QFile(const QFile & );
impl<'a> /*trait*/ QFile_NewQFile for (QFile) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1ERKS_(qthis, arg0)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFile::setFileName(const QString & name);
impl /*struct*/ QFile {
  pub fn setFileName<RetType, T: QFile_setFileName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QFile_setFileName<RetType> {
  fn setFileName(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  void QFile::setFileName(const QString & name);
impl<'a> /*trait*/ QFile_setFileName<()> for (QString) {
  fn setFileName(self , rsthis: &mut QFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFile11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QFile::remove();
impl /*struct*/ QFile {
  pub fn remove<RetType, T: QFile_remove<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QFile_remove<RetType> {
  fn remove(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  bool QFile::remove();
impl<'a> /*trait*/ QFile_remove<i8> for () {
  fn remove(self , rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6removeEv()};
    let mut ret = unsafe {_ZN5QFile6removeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFile::copy(const QString & newName);
impl /*struct*/ QFile {
  pub fn copy<RetType, T: QFile_copy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.copy(self);
    // return 1;
  }
}

pub trait QFile_copy<RetType> {
  fn copy(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  bool QFile::copy(const QString & newName);
impl<'a> /*trait*/ QFile_copy<i8> for (QString) {
  fn copy(self , rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4copyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile4copyERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QByteArray QFile::encodeName(const QString & fileName);
impl /*struct*/ QFile {
  pub fn encodeName_s<RetType, T: QFile_encodeName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.encodeName_s();
    // return 1;
  }
}

pub trait QFile_encodeName_s<RetType> {
  fn encodeName_s(self ) -> RetType;
}

  // proto: static QByteArray QFile::encodeName(const QString & fileName);
impl<'a> /*trait*/ QFile_encodeName_s<QByteArray> for (QString) {
  fn encodeName_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile10encodeNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile10encodeNameERK7QString(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QString QFile::decodeName(const QByteArray & localFileName);
impl /*struct*/ QFile {
  pub fn decodeName_s<RetType, T: QFile_decodeName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.decodeName_s();
    // return 1;
  }
}

pub trait QFile_decodeName_s<RetType> {
  fn decodeName_s(self ) -> RetType;
}

  // proto: static QString QFile::decodeName(const QByteArray & localFileName);
impl<'a> /*trait*/ QFile_decodeName_s<QString> for (QByteArray) {
  fn decodeName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile10decodeNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile10decodeNameERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFile::rename(const QString & newName);
impl /*struct*/ QFile {
  pub fn rename<RetType, T: QFile_rename<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rename(self);
    // return 1;
  }
}

pub trait QFile_rename<RetType> {
  fn rename(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  bool QFile::rename(const QString & newName);
impl<'a> /*trait*/ QFile_rename<i8> for (QString) {
  fn rename(self , rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6renameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile6renameERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QFile::fileName();
impl /*struct*/ QFile {
  pub fn fileName<RetType, T: QFile_fileName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QFile_fileName<RetType> {
  fn fileName(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  QString QFile::fileName();
impl<'a> /*trait*/ QFile_fileName<QString> for () {
  fn fileName(self , rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile8fileNameEv()};
    let mut ret = unsafe {_ZNK5QFile8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QString QFile::decodeName(const char * localFileName);
impl<'a> /*trait*/ QFile_decodeName_s<QString> for (&'a  String) {
  fn decodeName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile10decodeNameEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN5QFile10decodeNameEPKc(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QFile::metaObject();
impl /*struct*/ QFile {
  pub fn metaObject<RetType, T: QFile_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFile_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  const QMetaObject * QFile::metaObject();
impl<'a> /*trait*/ QFile_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile10metaObjectEv()};
     unsafe {_ZNK5QFile10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFile::QFile(const QString & name, QObject * parent);
impl<'a> /*trait*/ QFile_NewQFile for (QString, QObject) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QString QFile::symLinkTarget(const QString & fileName);
impl /*struct*/ QFile {
  pub fn symLinkTarget_s<RetType, T: QFile_symLinkTarget_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.symLinkTarget_s();
    // return 1;
  }
}

pub trait QFile_symLinkTarget_s<RetType> {
  fn symLinkTarget_s(self ) -> RetType;
}

  // proto: static QString QFile::symLinkTarget(const QString & fileName);
impl<'a> /*trait*/ QFile_symLinkTarget_s<QString> for (QString) {
  fn symLinkTarget_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile13symLinkTargetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile13symLinkTargetERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static bool QFile::remove(const QString & fileName);
impl /*struct*/ QFile {
  pub fn remove_s<RetType, T: QFile_remove_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.remove_s();
    // return 1;
  }
}

pub trait QFile_remove_s<RetType> {
  fn remove_s(self ) -> RetType;
}

  // proto: static bool QFile::remove(const QString & fileName);
impl<'a> /*trait*/ QFile_remove_s<i8> for (QString) {
  fn remove_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile6removeERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFile::QFile(const QString & name);
impl<'a> /*trait*/ QFile_NewQFile for (QString) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1ERK7QString(qthis, arg0)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QFile::readLink();
impl /*struct*/ QFile {
  pub fn readLink<RetType, T: QFile_readLink<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.readLink(self);
    // return 1;
  }
}

pub trait QFile_readLink<RetType> {
  fn readLink(self , rsthis: &mut QFile) -> RetType;
}

  // proto:  QString QFile::readLink();
impl<'a> /*trait*/ QFile_readLink<QString> for () {
  fn readLink(self , rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile8readLinkEv()};
    let mut ret = unsafe {_ZNK5QFile8readLinkEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}
