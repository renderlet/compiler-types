use crate::types::{Color, Comp, Extrude, Offset, Op, SetupProjection, Size, Split, Texture};

impl<'de> serde::Deserialize<'de> for Op {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
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
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "variant identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    3u64 => serde::__private::Ok(__Field::__field3),
                    4u64 => serde::__private::Ok(__Field::__field4),
                    5u64 => serde::__private::Ok(__Field::__field5),
                    _ => serde::__private::Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(__value),
                        &"variant index 0 <= i < 6",
                    )),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "color" => serde::__private::Ok(__Field::__field0),
                    "comp" => serde::__private::Ok(__Field::__field1),
                    "extrude" => serde::__private::Ok(__Field::__field2),
                    "offset" => serde::__private::Ok(__Field::__field3),
                    "setupProjection" => serde::__private::Ok(__Field::__field4),
                    "size" => serde::__private::Ok(__Field::__field5),
                    "split" => serde::__private::Ok(__Field::__field6),
                    "texture" => serde::__private::Ok(__Field::__field7),
                    _ => {
                        serde::__private::Err(serde::de::Error::unknown_variant(__value, VARIANTS))
                    }
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"color" => serde::__private::Ok(__Field::__field0),
                    b"comp" => serde::__private::Ok(__Field::__field1),
                    b"extrude" => serde::__private::Ok(__Field::__field2),
                    b"offset" => serde::__private::Ok(__Field::__field3),
                    b"setupProjection" => serde::__private::Ok(__Field::__field4),
                    b"size" => serde::__private::Ok(__Field::__field5),
                    b"split" => serde::__private::Ok(__Field::__field6),
                    b"texture" => serde::__private::Ok(__Field::__field7),
                    _ => {
                        let __value = &serde::__private::from_utf8_lossy(__value);
                        serde::__private::Err(serde::de::Error::unknown_variant(__value, VARIANTS))
                    }
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<Op>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = Op;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "enum Op")
            }

            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<Option<Color>> = serde::__private::None;
                let mut __field1: serde::__private::Option<Option<Vec<Comp>>> =
                    serde::__private::None;
                let mut __field2: serde::__private::Option<Option<Extrude>> =
                    serde::__private::None;
                let mut __field3: serde::__private::Option<Option<Offset>> = serde::__private::None;
                let mut __field4: serde::__private::Option<Option<SetupProjection>> =
                    serde::__private::None;
                let mut __field5: serde::__private::Option<Option<Size>> = serde::__private::None;
                let mut __field6: serde::__private::Option<Option<Split>> = serde::__private::None;
                let mut __field7: serde::__private::Option<Option<Texture>> =
                    serde::__private::None;
                while let serde::__private::Some(__key) =
                    serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("color"),
                                );
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                Option<Color>,
                            >(
                                &mut __map
                            )?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("comp"),
                                );
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                Option<Vec<Comp>>,
                            >(
                                &mut __map
                            )?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("extrude"),
                                );
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                Option<Extrude>,
                            >(
                                &mut __map
                            )?);
                        }
                        __Field::__field3 => {
                            if serde::__private::Option::is_some(&__field3) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("offset"),
                                );
                            }
                            __field3 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                Option<Offset>,
                            >(
                                &mut __map
                            )?);
                        }
                        __Field::__field4 => {
                            if serde::__private::Option::is_some(&__field4) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "setupProjection",
                                    ),
                                );
                            }
                            __field4 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                Option<SetupProjection>,
                            >(
                                &mut __map
                            )?);
                        }
                        __Field::__field5 => {
                            if serde::__private::Option::is_some(&__field5) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("size"),
                                );
                            }
                            __field5 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                Option<Size>,
                            >(
                                &mut __map
                            )?);
                        }
                        __Field::__field6 => {
                            if serde::__private::Option::is_some(&__field6) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("split"),
                                );
                            }
                            __field6 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                Option<Split>,
                            >(
                                &mut __map
                            )?);
                        }
                        __Field::__field7 => {
                            if serde::__private::Option::is_some(&__field7) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("texture"),
                                );
                            }
                            __field7 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                Option<Texture>,
                            >(
                                &mut __map
                            )?);
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("color")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("comp")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("extrude")?,
                };
                let __field3 = match __field3 {
                    serde::__private::Some(__field3) => __field3,
                    serde::__private::None => serde::__private::de::missing_field("offset")?,
                };
                let __field4 = match __field4 {
                    serde::__private::Some(__field4) => __field4,
                    serde::__private::None => {
                        serde::__private::de::missing_field("setupProjection")?
                    }
                };
                let __field5 = match __field5 {
                    serde::__private::Some(__field5) => __field5,
                    serde::__private::None => serde::__private::de::missing_field("size")?,
                };
                let __field6 = match __field6 {
                    serde::__private::Some(__field6) => __field6,
                    serde::__private::None => serde::__private::de::missing_field("split")?,
                };
                let __field7 = match __field7 {
                    serde::__private::Some(__field7) => __field7,
                    serde::__private::None => serde::__private::de::missing_field("texture")?,
                };

                if let Some(__field0) = __field0 {
                    return serde::__private::Ok(Op::Color(__field0));
                }
                if let Some(__field1) = __field1 {
                    return serde::__private::Ok(Op::Comp(__field1));
                }
                if let Some(__field2) = __field2 {
                    return serde::__private::Ok(Op::Extrude(__field2));
                }
                if let Some(__field3) = __field3 {
                    return serde::__private::Ok(Op::Offset(__field3));
                }
                if let Some(__field4) = __field4 {
                    return serde::__private::Ok(Op::SetupProjection(__field4));
                }
                if let Some(__field5) = __field5 {
                    return serde::__private::Ok(Op::Size(__field5));
                }
                if let Some(__field6) = __field6 {
                    return serde::__private::Ok(Op::Split(__field6));
                }
                if let Some(__field7) = __field7 {
                    return serde::__private::Ok(Op::Texture(__field7));
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
                marker: serde::__private::PhantomData::<Op>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

impl serde::Serialize for Op {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
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
