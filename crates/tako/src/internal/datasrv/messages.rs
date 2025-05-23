/*
   This file contains message for uploading/download data objects

   To avoid unnecessary data copying, some messages has variants Up and Down,
   Up is used for uploading side (serialization end)
   Down is used by downloading side (deserialization end)
*/

use crate::datasrv::DataObjectId;
use crate::internal::datasrv::DataObject;
use crate::internal::datasrv::dataobj::{DataInputId, OutputId};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct DataObjectSlice {
    pub data_object: Rc<DataObject>,
    pub start: usize,
    pub end: usize,
}

impl Serialize for DataObjectSlice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.data_object.data()[self.start..self.end])
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct PutDataUp<'a> {
    #[serde(with = "serde_bytes")]
    pub data: &'a [u8],
}

#[derive(Debug, Serialize)]
pub(crate) enum FromLocalDataClientMessageUp<'a> {
    PutDataObject {
        data_id: OutputId,
        mime_type: Option<&'a str>,
        size: u64,
        data: PutDataUp<'a>,
    },
    PutDataObjectPart(PutDataUp<'a>),
    GetInput {
        input_id: DataInputId,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DataDown {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum FromLocalDataClientMessageDown {
    PutDataObject {
        data_id: OutputId,
        mime_type: Option<String>,
        size: u64,
        data: DataDown,
    },
    PutDataObjectPart(DataDown),
    GetInput {
        input_id: DataInputId,
    },
}

#[derive(Debug, Serialize)]
pub(crate) enum ToLocalDataClientMessageUp {
    Uploaded(OutputId),
    DataObject {
        mime_type: Option<String>,
        size: u64,
        data: DataObjectSlice,
    },
    DataObjectPart(DataObjectSlice),
    Error(String),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Remove once the mime type is used
pub(crate) enum ToLocalDataClientMessageDown {
    Uploaded(OutputId),
    DataObject {
        mime_type: Option<String>,
        size: u64,
        data: DataDown,
    },
    DataObjectPart(DataDown),
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum FromDataClientMessage {
    GetObject { data_id: DataObjectId },
}

#[derive(Debug, Serialize)]
pub(crate) enum ToDataClientMessageUp {
    DataObject {
        mime_type: Option<String>,
        size: u64,
        data: DataObjectSlice,
    },
    DataObjectPart(DataObjectSlice),
    NotFound,
}

#[derive(Debug, Deserialize)]
pub(crate) enum ToDataClientMessageDown {
    DataObject {
        mime_type: Option<String>,
        size: u64,
        data: DataDown,
    },
    DataObjectPart(DataDown),
    NotFound,
}
