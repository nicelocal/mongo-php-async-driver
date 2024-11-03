use std::str::FromStr;

use ext_php_rs::convert::FromZval;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::error::Result as PhpExtResult;


use ext_php_rs::flags::DataType;



use ext_php_rs::prelude::PhpException;
use ext_php_rs::prelude::PhpResult;


use ext_php_rs::types::ArrayKey;
use ext_php_rs::types::ZendHashTable;

use ext_php_rs::types::ZendStr;
use ext_php_rs::types::Zval;
use ext_php_rs::zend::ClassEntry;

use mongodb::bson::Document;
use mongodb::bson::Bson;
use mongodb::bson::RawBsonRef;
use mongodb::bson::RawDocumentBuf;
use mongodb::bson::oid::ObjectId;




pub fn zval_to_bson(zval: &Zval) -> PhpResult<Bson> {
    match zval.get_type() {
        DataType::Null => Ok(Bson::Null),
        DataType::False => Ok(Bson::Boolean(false)),
        DataType::True => Ok(Bson::Boolean(true)),
        DataType::Bool => Ok(Bson::Boolean(zval.bool().unwrap())),
        DataType::Long => Ok(Bson::Int64(zval.long().unwrap())),
        DataType::Double => Ok(Bson::Double(zval.double().unwrap())),
        DataType::String => match zval.str() {
            None => Err("Binary data was provided!".into()),
            Some(v) => Ok(Bson::String(v.to_owned()))
        },
        DataType::Array => {
            let arr = zval.array().unwrap();
            let mut expected_idx = 0;
            for (key, _) in arr.iter() {
                match key {
                    ArrayKey::Long(idx) => {
                        if idx == expected_idx {
                            expected_idx += 1;
                            continue;
                        }
                    },
                    _ => ()
                }
                return Ok(Bson::Document(zval_to_document(zval)?));
            }
            let mut barr = Vec::with_capacity(arr.len());
            for (_, val) in arr.iter() {
                barr.push(zval_to_bson(val)?)
            }
            Ok(Bson::Array(barr))
        },
        DataType::Reference => zval_to_bson(zval.reference().unwrap()),
        DataType::Ptr => zval_to_bson(unsafe { *zval.ptr().unwrap() }),
        DataType::Object(t) => {
            let object = zval.object().unwrap();
            if object.get_class_name().unwrap() == "\\MongoDB\\Bson\\ObjectId" {
                return Ok(Bson::ObjectId(
                    ObjectId::from_str(
                        object.try_call_method(
                            "__toString", 
                            vec![]
                        ).unwrap().str().unwrap()
                    ).unwrap()
                ))
            }
            Err(format!("Unexpected object of type {}, {}!", t.unwrap(), zval.object().unwrap().get_class_name().unwrap()).into())
        },
        t => Err(format!("Unexpected type {}", t).into())
    }
}
pub fn zval_to_document(zval: &Zval) -> PhpResult<Document> {
    let mut doc = Document::new();
    match zval.get_type() {
        DataType::Array => {
            let zval = zval.array().unwrap();
            for (key, val) in zval.iter() {
                match key {
                    ArrayKey::Long(idx) => return Err(format!("Unexpected integer key {}", idx).into()),
                    ArrayKey::String(k) => doc.insert(k, zval_to_bson(val)?)
                };
            }
            Ok(doc)
        },
        t => Err(format!("Unexpected type {}", t).into())
    }
}

pub fn bson_to_zval(bson: RawBsonRef) -> PhpExtResult<Zval> {
    Ok(match bson {
        RawBsonRef::Array(v) => {
            let mut zarr = ZendHashTable::new();
            for value in v {
                zarr.push(bson_to_zval(value.map_err(|e| PhpException::default(e.to_string()).throw().unwrap_err())?))?
            }
            zarr.into_zval(false)?
        },
        RawBsonRef::Double(v) => v.into_zval(false)?,
        RawBsonRef::Int32(v) => v.into_zval(false)?,
        RawBsonRef::Int64(v) => v.into_zval(false)?,
        RawBsonRef::Boolean(v) => v.into_zval(false)?,
        RawBsonRef::Document(v) => {
            let mut zval = ZendHashTable::new();
            for kv in v {
                let (key, value) = kv.map_err(|e| PhpException::default(e.to_string()).throw().unwrap_err())?;
                zval.insert(key, bson_to_zval(value))?;
            }
            zval.into_zval(false)?
        },
        RawBsonRef::String(v) => v.into_zval(false)?,
        RawBsonRef::Binary(v) => {
            let mut zval = Zval::new();
            zval.set_zend_string(ZendStr::new(v.bytes, false));
            zval
        },
        RawBsonRef::ObjectId(v) => {
            let res = ClassEntry::try_find("\\MongoDB\\Bson\\ObjectId").unwrap()
                .new();
            res.try_call_method("__construct", vec![&v.to_hex()])?;
            res.into_zval(false)?
        },
        _ => todo!()
    })
}

pub fn document_to_zval(document: &RawDocumentBuf, zval: &mut Zval) -> PhpExtResult<()> {
    let mut zhash = ZendHashTable::new();
    for el in document {
        let (key, value) = el.map_err(|e| PhpException::default(e.to_string()).throw().unwrap_err())?;
        zhash.insert(key, bson_to_zval(value)?)?;
    }
    zhash.set_zval(zval, false)
}

pub struct PhpDocument(pub Document);
pub struct PhpRawDocument(pub RawDocumentBuf);

impl<'a> FromZval<'a> for PhpDocument {
    const TYPE: DataType = DataType::Array;
    fn from_zval(zval: &'a Zval) -> Option<Self> {
        match zval_to_document(zval) {
            Ok(v) => Some(PhpDocument(v)),
            Err(_) => None
        }
    }
}

impl Into<Document> for PhpDocument {
    fn into(self) -> Document {
        return self.0
    }
}

impl IntoZval for PhpRawDocument {
    const TYPE: DataType = DataType::Array;
    #[inline]
    fn set_zval(self, zv: &mut Zval, _persistent: bool) -> PhpExtResult<()> {
        document_to_zval((&self.0).into(), zv)
    }
}

impl<'a> IntoZval for &'a PhpRawDocument {
    const TYPE: DataType = DataType::Array;
    #[inline]
    fn set_zval(self, zv: &mut Zval, _persistent: bool) -> PhpExtResult<()> {
        document_to_zval((&self.0).into(), zv)
    }
}

impl From<RawDocumentBuf> for PhpRawDocument {
    fn from(value: RawDocumentBuf) -> Self {
        Self(value)
    }
}
