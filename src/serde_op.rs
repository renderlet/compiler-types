use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
};

use crate::types::{Color, Comp, Extrude, Offset, Op, SetupProjection, Size, Split, Texture};

impl<'de> serde::Deserialize<'de> for Op {
    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __field3,
            __field4,
            __field5,
            __field6,
            __field7,
        }
        #[doc(hidden)]
        struct __FieldVisitor;

        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut Formatter) -> fmt::Result {
                Formatter::write_str(__formatter, "variant identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => Ok(__Field::__field0),
                    1u64 => Ok(__Field::__field1),
                    2u64 => Ok(__Field::__field2),
                    3u64 => Ok(__Field::__field3),
                    4u64 => Ok(__Field::__field4),
                    5u64 => Ok(__Field::__field5),
                    _ => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(__value),
                        &"variant index 0 <= i < 6",
                    )),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "color" => Ok(__Field::__field0),
                    "comp" => Ok(__Field::__field1),
                    "extrude" => Ok(__Field::__field2),
                    "offset" => Ok(__Field::__field3),
                    "setupProjection" => Ok(__Field::__field4),
                    "size" => Ok(__Field::__field5),
                    "split" => Ok(__Field::__field6),
                    "texture" => Ok(__Field::__field7),
                    _ => Err(serde::de::Error::unknown_variant(__value, VARIANTS)),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"color" => Ok(__Field::__field0),
                    b"comp" => Ok(__Field::__field1),
                    b"extrude" => Ok(__Field::__field2),
                    b"offset" => Ok(__Field::__field3),
                    b"setupProjection" => Ok(__Field::__field4),
                    b"size" => Ok(__Field::__field5),
                    b"split" => Ok(__Field::__field6),
                    b"texture" => Ok(__Field::__field7),
                    _ => {
                        let __value = &String::from_utf8_lossy(__value);
                        Err(serde::de::Error::unknown_variant(__value, VARIANTS))
                    }
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: PhantomData<Op>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = Op;
            fn expecting(&self, __formatter: &mut Formatter) -> fmt::Result {
                Formatter::write_str(__formatter, "enum Op")
            }

            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: Option<Color> = None;
                let mut __field1: Option<Vec<Comp>> = None;
                let mut __field2: Option<Extrude> = None;
                let mut __field3: Option<Offset> = None;
                let mut __field4: Option<SetupProjection> = None;
                let mut __field5: Option<Size> = None;
                let mut __field6: Option<Split> = None;
                let mut __field7: Option<Texture> = None;
                while let Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "color",
                                ));
                            }
                            __field0 = Some(serde::de::MapAccess::next_value::<Color>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if Option::is_some(&__field1) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "comp",
                                ));
                            }
                            __field1 =
                                Some(serde::de::MapAccess::next_value::<Vec<Comp>>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if Option::is_some(&__field2) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "extrude",
                                ));
                            }
                            __field2 =
                                Some(serde::de::MapAccess::next_value::<Extrude>(&mut __map)?);
                        }
                        __Field::__field3 => {
                            if Option::is_some(&__field3) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "offset",
                                ));
                            }
                            __field3 =
                                Some(serde::de::MapAccess::next_value::<Offset>(&mut __map)?);
                        }
                        __Field::__field4 => {
                            if Option::is_some(&__field4) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "setupProjection",
                                ));
                            }
                            __field4 = Some(serde::de::MapAccess::next_value::<SetupProjection>(
                                &mut __map,
                            )?);
                        }
                        __Field::__field5 => {
                            if Option::is_some(&__field5) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "size",
                                ));
                            }
                            __field5 = Some(serde::de::MapAccess::next_value::<Size>(&mut __map)?);
                        }
                        __Field::__field6 => {
                            if Option::is_some(&__field6) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "split",
                                ));
                            }
                            __field6 = Some(serde::de::MapAccess::next_value::<Split>(&mut __map)?);
                        }
                        __Field::__field7 => {
                            if Option::is_some(&__field7) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "texture",
                                ));
                            }
                            __field7 =
                                Some(serde::de::MapAccess::next_value::<Texture>(&mut __map)?);
                        }
                    }
                }

                if let Some(__field0) = __field0 {
                    return Ok(Op::Color(__field0));
                }
                if let Some(__field1) = __field1 {
                    return Ok(Op::Comp(__field1));
                }
                if let Some(__field2) = __field2 {
                    return Ok(Op::Extrude(__field2));
                }
                if let Some(__field3) = __field3 {
                    return Ok(Op::Offset(__field3));
                }
                if let Some(__field4) = __field4 {
                    return Ok(Op::SetupProjection(__field4));
                }
                if let Some(__field5) = __field5 {
                    return Ok(Op::Size(__field5));
                }
                if let Some(__field6) = __field6 {
                    return Ok(Op::Split(__field6));
                }
                if let Some(__field7) = __field7 {
                    return Ok(Op::Texture(__field7));
                }
                todo!("TODO: Return `Err(?)`")
            }
        }
        #[doc(hidden)]
        const VARIANTS: &'static [&'static str] = &[
            "color",
            "comp",
            "extrude",
            "offset",
            "setupProjection",
            "size",
            "split",
            "texture",
        ];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "op",
            VARIANTS,
            __Visitor {
                marker: PhantomData::<Op>,
                lifetime: PhantomData,
            },
        )
    }
}

impl serde::Serialize for Op {
    fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut _serde_state = serde::Serializer::serialize_struct(__serializer, "Op", 1)?;

        match *self {
            Op::Color(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "color", &__field0)
            }
            Op::Comp(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "comp", &__field0)
            }
            Op::Extrude(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "extrude",
                &__field0,
            ),
            Op::Offset(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "offset", &__field0)
            }
            Op::SetupProjection(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "setupProjection",
                &__field0,
            ),
            Op::Size(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "size", &__field0)
            }
            Op::Split(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "split", &__field0)
            }
            Op::Texture(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "texture",
                &__field0,
            ),
        }?;
        serde::ser::SerializeStruct::end(_serde_state)
    }
}
