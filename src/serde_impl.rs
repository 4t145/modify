use crate::Annotated;

impl<T, A> serde::Serialize for Annotated<T, A>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        T::serialize(&self.value, serializer)
    }
}

impl<'de, T, A> serde::Deserialize<'de> for Annotated<T, A>
where
    T: serde::Deserialize<'de>,
    A: Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Annotated {
            value,
            annotation: Default::default(),
        })
    }
}
