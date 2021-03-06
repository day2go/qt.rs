// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qtextdocumentwriter.h
// dst-file: /src/gui/qtextdocumentwriter.rs
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
use super::super::core::qtextcodec::*; // 771
use super::super::core::qiodevice::*; // 771
use super::super::core::qbytearray::*; // 771
use super::super::core::qstring::*; // 771
use super::qtextdocument::*; // 773
use super::qtextdocumentfragment::*; // 773
// use super::qlist::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextDocumentWriter_Class_Size() -> c_int;
  // proto:  void QTextDocumentWriter::setCodec(QTextCodec * codec);
  fn C_ZN19QTextDocumentWriter8setCodecEP10QTextCodec(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextDocumentWriter::QTextDocumentWriter(QIODevice * device, const QByteArray & format);
  fn C_ZN19QTextDocumentWriterC2EP9QIODeviceRK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QTextDocumentWriter::setFileName(const QString & fileName);
  fn C_ZN19QTextDocumentWriter11setFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QByteArray QTextDocumentWriter::format();
  fn C_ZNK19QTextDocumentWriter6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextDocumentWriter::setDevice(QIODevice * device);
  fn C_ZN19QTextDocumentWriter9setDeviceEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextDocumentWriter::QTextDocumentWriter(const QString & fileName, const QByteArray & format);
  fn C_ZN19QTextDocumentWriterC2ERK7QStringRK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QTextDocumentWriter::setFormat(const QByteArray & format);
  fn C_ZN19QTextDocumentWriter9setFormatERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTextDocumentWriter::write(const QTextDocument * document);
  fn C_ZN19QTextDocumentWriter5writeEPK13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QTextDocumentWriter::write(const QTextDocumentFragment & fragment);
  fn C_ZN19QTextDocumentWriter5writeERK21QTextDocumentFragment(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QTextDocumentWriter::QTextDocumentWriter();
  fn C_ZN19QTextDocumentWriterC2Ev() -> u64;
  // proto:  QTextCodec * QTextDocumentWriter::codec();
  fn C_ZNK19QTextDocumentWriter5codecEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QTextDocumentWriter::fileName();
  fn C_ZNK19QTextDocumentWriter8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QList<QByteArray> QTextDocumentWriter::supportedDocumentFormats();
  fn C_ZN19QTextDocumentWriter24supportedDocumentFormatsEv() -> *mut c_void;
  // proto:  QIODevice * QTextDocumentWriter::device();
  fn C_ZNK19QTextDocumentWriter6deviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextDocumentWriter::~QTextDocumentWriter();
  fn C_ZN19QTextDocumentWriterD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QTextDocumentWriter)=8
#[derive(Default)]
pub struct QTextDocumentWriter {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextDocumentWriter {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextDocumentWriter {
    return QTextDocumentWriter{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextDocumentWriter::setCodec(QTextCodec * codec);
impl /*struct*/ QTextDocumentWriter {
  pub fn setCodec<RetType, T: QTextDocumentWriter_setCodec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCodec(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_setCodec<RetType> {
  fn setCodec(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  void QTextDocumentWriter::setCodec(QTextCodec * codec);
impl<'a> /*trait*/ QTextDocumentWriter_setCodec<()> for (&'a QTextCodec) {
  fn setCodec(self , rsthis: & QTextDocumentWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter8setCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QTextDocumentWriter8setCodecEP10QTextCodec(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocumentWriter::QTextDocumentWriter(QIODevice * device, const QByteArray & format);
impl /*struct*/ QTextDocumentWriter {
  pub fn new<T: QTextDocumentWriter_new>(value: T) -> QTextDocumentWriter {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentWriter_new {
  fn new(self) -> QTextDocumentWriter;
}

  // proto:  void QTextDocumentWriter::QTextDocumentWriter(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QTextDocumentWriter_new for (&'a QIODevice, &'a QByteArray) {
  fn new(self) -> QTextDocumentWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriterC2EP9QIODeviceRK10QByteArray()};
    let ctysz: c_int = unsafe{QTextDocumentWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QTextDocumentWriterC2EP9QIODeviceRK10QByteArray(arg0, arg1)};
    let rsthis = QTextDocumentWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextDocumentWriter::setFileName(const QString & fileName);
impl /*struct*/ QTextDocumentWriter {
  pub fn setFileName<RetType, T: QTextDocumentWriter_setFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_setFileName<RetType> {
  fn setFileName(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  void QTextDocumentWriter::setFileName(const QString & fileName);
impl<'a> /*trait*/ QTextDocumentWriter_setFileName<()> for (&'a QString) {
  fn setFileName(self , rsthis: & QTextDocumentWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QTextDocumentWriter11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QTextDocumentWriter::format();
impl /*struct*/ QTextDocumentWriter {
  pub fn format<RetType, T: QTextDocumentWriter_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_format<RetType> {
  fn format(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  QByteArray QTextDocumentWriter::format();
impl<'a> /*trait*/ QTextDocumentWriter_format<QByteArray> for () {
  fn format(self , rsthis: & QTextDocumentWriter) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QTextDocumentWriter6formatEv()};
    let mut ret = unsafe {C_ZNK19QTextDocumentWriter6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocumentWriter::setDevice(QIODevice * device);
impl /*struct*/ QTextDocumentWriter {
  pub fn setDevice<RetType, T: QTextDocumentWriter_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_setDevice<RetType> {
  fn setDevice(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  void QTextDocumentWriter::setDevice(QIODevice * device);
impl<'a> /*trait*/ QTextDocumentWriter_setDevice<()> for (&'a QIODevice) {
  fn setDevice(self , rsthis: & QTextDocumentWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QTextDocumentWriter9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocumentWriter::QTextDocumentWriter(const QString & fileName, const QByteArray & format);
impl<'a> /*trait*/ QTextDocumentWriter_new for (&'a QString, Option<&'a QByteArray>) {
  fn new(self) -> QTextDocumentWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriterC2ERK7QStringRK10QByteArray()};
    let ctysz: c_int = unsafe{QTextDocumentWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {QByteArray::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QTextDocumentWriterC2ERK7QStringRK10QByteArray(arg0, arg1)};
    let rsthis = QTextDocumentWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextDocumentWriter::setFormat(const QByteArray & format);
impl /*struct*/ QTextDocumentWriter {
  pub fn setFormat<RetType, T: QTextDocumentWriter_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_setFormat<RetType> {
  fn setFormat(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  void QTextDocumentWriter::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QTextDocumentWriter_setFormat<()> for (&'a QByteArray) {
  fn setFormat(self , rsthis: & QTextDocumentWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QTextDocumentWriter9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextDocumentWriter::write(const QTextDocument * document);
impl /*struct*/ QTextDocumentWriter {
  pub fn write<RetType, T: QTextDocumentWriter_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_write<RetType> {
  fn write(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  bool QTextDocumentWriter::write(const QTextDocument * document);
impl<'a> /*trait*/ QTextDocumentWriter_write<i8> for (&'a QTextDocument) {
  fn write(self , rsthis: & QTextDocumentWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter5writeEPK13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN19QTextDocumentWriter5writeEPK13QTextDocument(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QTextDocumentWriter::write(const QTextDocumentFragment & fragment);
impl<'a> /*trait*/ QTextDocumentWriter_write<i8> for (&'a QTextDocumentFragment) {
  fn write(self , rsthis: & QTextDocumentWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter5writeERK21QTextDocumentFragment()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN19QTextDocumentWriter5writeERK21QTextDocumentFragment(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTextDocumentWriter::QTextDocumentWriter();
impl<'a> /*trait*/ QTextDocumentWriter_new for () {
  fn new(self) -> QTextDocumentWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriterC2Ev()};
    let ctysz: c_int = unsafe{QTextDocumentWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN19QTextDocumentWriterC2Ev()};
    let rsthis = QTextDocumentWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextCodec * QTextDocumentWriter::codec();
impl /*struct*/ QTextDocumentWriter {
  pub fn codec<RetType, T: QTextDocumentWriter_codec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.codec(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_codec<RetType> {
  fn codec(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  QTextCodec * QTextDocumentWriter::codec();
impl<'a> /*trait*/ QTextDocumentWriter_codec<QTextCodec> for () {
  fn codec(self , rsthis: & QTextDocumentWriter) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QTextDocumentWriter5codecEv()};
    let mut ret = unsafe {C_ZNK19QTextDocumentWriter5codecEv(rsthis.qclsinst)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextDocumentWriter::fileName();
impl /*struct*/ QTextDocumentWriter {
  pub fn fileName<RetType, T: QTextDocumentWriter_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_fileName<RetType> {
  fn fileName(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  QString QTextDocumentWriter::fileName();
impl<'a> /*trait*/ QTextDocumentWriter_fileName<QString> for () {
  fn fileName(self , rsthis: & QTextDocumentWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QTextDocumentWriter8fileNameEv()};
    let mut ret = unsafe {C_ZNK19QTextDocumentWriter8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QTextDocumentWriter::supportedDocumentFormats();
impl /*struct*/ QTextDocumentWriter {
  pub fn supportedDocumentFormats_s<RetType, T: QTextDocumentWriter_supportedDocumentFormats_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportedDocumentFormats_s();
    // return 1;
  }
}

pub trait QTextDocumentWriter_supportedDocumentFormats_s<RetType> {
  fn supportedDocumentFormats_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QTextDocumentWriter::supportedDocumentFormats();
impl<'a> /*trait*/ QTextDocumentWriter_supportedDocumentFormats_s<u64> for () {
  fn supportedDocumentFormats_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter24supportedDocumentFormatsEv()};
    let mut ret = unsafe {C_ZN19QTextDocumentWriter24supportedDocumentFormatsEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QIODevice * QTextDocumentWriter::device();
impl /*struct*/ QTextDocumentWriter {
  pub fn device<RetType, T: QTextDocumentWriter_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_device<RetType> {
  fn device(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  QIODevice * QTextDocumentWriter::device();
impl<'a> /*trait*/ QTextDocumentWriter_device<QIODevice> for () {
  fn device(self , rsthis: & QTextDocumentWriter) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QTextDocumentWriter6deviceEv()};
    let mut ret = unsafe {C_ZNK19QTextDocumentWriter6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocumentWriter::~QTextDocumentWriter();
impl /*struct*/ QTextDocumentWriter {
  pub fn free<RetType, T: QTextDocumentWriter_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextDocumentWriter_free<RetType> {
  fn free(self , rsthis: & QTextDocumentWriter) -> RetType;
}

  // proto:  void QTextDocumentWriter::~QTextDocumentWriter();
impl<'a> /*trait*/ QTextDocumentWriter_free<()> for () {
  fn free(self , rsthis: & QTextDocumentWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriterD2Ev()};
     unsafe {C_ZN19QTextDocumentWriterD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

