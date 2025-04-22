pub mod option_range {
    use core::ops::Range;
    use serde::{
        de::{DeserializeOwned, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use std::{fmt, marker::PhantomData};

    pub struct RangeVisitor<Idx> {
        pub expecting: &'static str,
        pub phantom: PhantomData<Idx>,
    }

    impl<'de, Idx> Visitor<'de> for RangeVisitor<Idx>
    where
        Idx: std::str::FromStr + Deserialize<'de>,
        Idx::Err: fmt::Display,
    {
        type Value = (Idx, Idx);

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let items = value.split(',').collect::<Vec<_>>();
            if items.len() != 2 {
                return Err(serde::de::Error::invalid_length(items.len(), &self));
            }
            let start = items[0].parse::<Idx>().map_err(serde::de::Error::custom)?;
            let end = items[1].parse::<Idx>().map_err(serde::de::Error::custom)?;
            Ok((start, end))
        }
    }

    pub fn serialize<Idx, S>(range: &Option<Range<Idx>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        Idx: Copy + fmt::Display + Serialize + DeserializeOwned,
        S: Serializer,
    {
        let range = match range {
            None => return Option::serialize(&None::<Range<Idx>>, serializer),
            Some(range) => range,
        };
        serializer.collect_str(&format!("{:.6},{:.6}", range.start, range.end))
    }

    pub fn deserialize<'de, Idx, D>(deserializer: D) -> Result<Option<Range<Idx>>, D::Error>
    where
        Idx: std::str::FromStr + DeserializeOwned,
        Idx::Err: fmt::Display,
        D: Deserializer<'de>,
    {
        let (start, end) = deserializer.deserialize_str(RangeVisitor {
            expecting: "range (e.g. `x,y`)",
            phantom: PhantomData,
        })?;
        Ok(Some(start..end))
    }
}
