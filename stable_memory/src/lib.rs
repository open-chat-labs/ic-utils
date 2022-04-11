use serde::{Serialize, de::DeserializeOwned};
use std::error::Error;
use std::io::{Read, Write};
use ic_cdk::api::stable::{BufferedStableReader, BufferedStableWriter};

pub fn serialize_to_stable_memory<S: Serialize>(state: S, buffer_size: usize) -> Result<(), impl Error> {
    let writer = BufferedStableWriter::new(buffer_size);
    serialize(state, writer)
}

pub fn deserialize_from_stable_memory<S: DeserializeOwned>(buffer_size: usize) -> Result<S, impl Error> {
    let reader = BufferedStableReader::new(buffer_size);
    deserialize(reader)
}

fn serialize<T, W>(value: T, writer: W) -> Result<(), impl Error>
    where
        T: Serialize,
        W: Write,
{
    let mut serializer = rmp_serde::Serializer::new(writer).with_struct_map();
    value.serialize(&mut serializer).map(|_| ())
}

fn deserialize<T, R>(reader: R) -> Result<T, impl Error>
    where
        T: DeserializeOwned,
        R: Read,
{
    let mut deserializer = rmp_serde::Deserializer::new(reader);
    T::deserialize(&mut deserializer)
}
