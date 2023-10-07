use super::*;
use serde::{ser::SerializeSeq, Serialize, Serializer};

impl<T> Serialize for NyarTuple<T>
where
    T: Serialize + Clone,
{
    /// The binding name is part of the type and will be erased during serialization
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.raw.len()))?;
        for element in self.raw.iter() {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
