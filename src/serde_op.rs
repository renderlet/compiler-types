use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
};

use serde::de::MapAccess;

use crate::types::{
    Center, Color, Comp, Copy, CornerCut, Extrude, Hemisphere, InnerArch, InnerCircle,
    InnerSemiCircle, Insert, Offset, Op, Pyramid, RoofGable, RoofHip, Rotate, SetupProjection,
    ShapeL, ShapeU, Size, Split, Taper, Texture, Translate,
};

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
            __field8,
            __field9,
            __field10,
            __field11,
            __field12,
            __field13,
            __field14,
            __field15,
            __field16,
            __field17,
            __field18,
            __field19,
            __field20,
            __field21,
            __field22,
            __field23,
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
                    "center" => Ok(__Field::__field0),
                    "color" => Ok(__Field::__field1),
                    "comp" => Ok(__Field::__field2),
                    "copy" => Ok(__Field::__field3),
                    "cornerCut" => Ok(__Field::__field4),
                    "extrude" => Ok(__Field::__field5),
                    "hemisphere" => Ok(__Field::__field6),
                    "innerArch" => Ok(__Field::__field7),
                    "innerCircle" => Ok(__Field::__field8),
                    "innerSemiCircle" => Ok(__Field::__field9),
                    "insert" => Ok(__Field::__field10),
                    "offset" => Ok(__Field::__field11),
                    "pyramid" => Ok(__Field::__field12),
                    "roofGable" => Ok(__Field::__field13),
                    "roofHip" => Ok(__Field::__field14),
                    "rotate" => Ok(__Field::__field15),
                    "setupProjection" => Ok(__Field::__field16),
                    "shapeL" => Ok(__Field::__field17),
                    "shapeU" => Ok(__Field::__field18),
                    "size" => Ok(__Field::__field19),
                    "split" => Ok(__Field::__field20),
                    "taper" => Ok(__Field::__field21),
                    "texture" => Ok(__Field::__field22),
                    "translate" => Ok(__Field::__field23),
                    _ => Err(serde::de::Error::unknown_variant(__value, VARIANTS)),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"center" => Ok(__Field::__field0),
                    b"color" => Ok(__Field::__field1),
                    b"comp" => Ok(__Field::__field2),
                    b"copy" => Ok(__Field::__field3),
                    b"cornerCut" => Ok(__Field::__field4),
                    b"extrude" => Ok(__Field::__field5),
                    b"hemisphere" => Ok(__Field::__field6),
                    b"innerArch" => Ok(__Field::__field7),
                    b"innerCircle" => Ok(__Field::__field8),
                    b"innerSemiCircle" => Ok(__Field::__field9),
                    b"insert" => Ok(__Field::__field10),
                    b"offset" => Ok(__Field::__field11),
                    b"pyramid" => Ok(__Field::__field12),
                    b"roofGable" => Ok(__Field::__field13),
                    b"roofHip" => Ok(__Field::__field14),
                    b"rotate" => Ok(__Field::__field15),
                    b"setupProjection" => Ok(__Field::__field16),
                    b"shapeL" => Ok(__Field::__field17),
                    b"shapeU" => Ok(__Field::__field18),
                    b"size" => Ok(__Field::__field19),
                    b"split" => Ok(__Field::__field20),
                    b"taper" => Ok(__Field::__field21),
                    b"texture" => Ok(__Field::__field22),
                    b"translate" => Ok(__Field::__field23),
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
                let mut __field0: Option<Center> = None;
                let mut __field1: Option<Color> = None;
                let mut __field2: Option<Vec<Comp>> = None;
                let mut __field3: Option<Copy> = None;
                let mut __field4: Option<CornerCut> = None;
                let mut __field5: Option<Extrude> = None;
                let mut __field6: Option<Hemisphere> = None;
                let mut __field7: Option<InnerArch> = None;
                let mut __field8: Option<InnerCircle> = None;
                let mut __field9: Option<InnerSemiCircle> = None;
                let mut __field10: Option<Insert> = None;
                let mut __field11: Option<Offset> = None;
                let mut __field12: Option<Pyramid> = None;
                let mut __field13: Option<RoofGable> = None;
                let mut __field14: Option<RoofHip> = None;
                let mut __field15: Option<Rotate> = None;
                let mut __field16: Option<SetupProjection> = None;
                let mut __field17: Option<ShapeL> = None;
                let mut __field18: Option<ShapeU> = None;
                let mut __field19: Option<Size> = None;
                let mut __field20: Option<Split> = None;
                let mut __field21: Option<Taper> = None;
                let mut __field22: Option<Texture> = None;
                let mut __field23: Option<Translate> = None;
                while let Some(__key) = MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "center",
                                ));
                            }
                            __field0 = Some(MapAccess::next_value::<Center>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "color",
                                ));
                            }
                            __field1 = Some(MapAccess::next_value::<Color>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "comp",
                                ));
                            }
                            __field2 = Some(MapAccess::next_value::<Vec<Comp>>(&mut __map)?);
                        }
                        __Field::__field3 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "copy",
                                ));
                            }
                            __field3 = Some(MapAccess::next_value::<Copy>(&mut __map)?);
                        }
                        __Field::__field4 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "cornerCut",
                                ));
                            }
                            __field4 = Some(MapAccess::next_value::<CornerCut>(&mut __map)?);
                        }
                        __Field::__field5 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "extrude",
                                ));
                            }
                            __field5 = Some(MapAccess::next_value::<Extrude>(&mut __map)?);
                        }
                        __Field::__field6 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "hemisphere",
                                ));
                            }
                            __field6 = Some(MapAccess::next_value::<Hemisphere>(&mut __map)?);
                        }
                        __Field::__field7 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "innerArch",
                                ));
                            }
                            __field7 = Some(MapAccess::next_value::<InnerArch>(&mut __map)?);
                        }
                        __Field::__field8 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "innerCircle",
                                ));
                            }
                            __field8 = Some(MapAccess::next_value::<InnerCircle>(&mut __map)?);
                        }
                        __Field::__field9 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "innerSemiCircle",
                                ));
                            }
                            __field9 = Some(MapAccess::next_value::<InnerSemiCircle>(&mut __map)?);
                        }
                        __Field::__field10 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "insert",
                                ));
                            }
                            __field10 = Some(MapAccess::next_value::<Insert>(&mut __map)?);
                        }
                        __Field::__field11 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "offset",
                                ));
                            }
                            __field11 = Some(MapAccess::next_value::<Offset>(&mut __map)?);
                        }
                        __Field::__field12 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "pyramid",
                                ));
                            }
                            __field12 = Some(MapAccess::next_value::<Pyramid>(&mut __map)?);
                        }
                        __Field::__field13 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "roofGable",
                                ));
                            }
                            __field13 = Some(MapAccess::next_value::<RoofGable>(&mut __map)?);
                        }
                        __Field::__field14 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "roofHip",
                                ));
                            }
                            __field14 = Some(MapAccess::next_value::<RoofHip>(&mut __map)?);
                        }
                        __Field::__field15 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "rotate",
                                ));
                            }
                            __field15 = Some(MapAccess::next_value::<Rotate>(&mut __map)?);
                        }
                        __Field::__field16 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "setupProjection",
                                ));
                            }
                            __field16 = Some(MapAccess::next_value::<SetupProjection>(&mut __map)?);
                        }
                        __Field::__field17 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "shapeL",
                                ));
                            }
                            __field17 = Some(MapAccess::next_value::<ShapeL>(&mut __map)?);
                        }
                        __Field::__field18 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "shapeU",
                                ));
                            }
                            __field18 = Some(MapAccess::next_value::<ShapeU>(&mut __map)?);
                        }
                        __Field::__field19 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "size",
                                ));
                            }
                            __field19 = Some(MapAccess::next_value::<Size>(&mut __map)?);
                        }
                        __Field::__field20 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "split",
                                ));
                            }
                            __field20 = Some(MapAccess::next_value::<Split>(&mut __map)?);
                        }
                        __Field::__field21 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "taper",
                                ));
                            }
                            __field21 = Some(MapAccess::next_value::<Taper>(&mut __map)?);
                        }
                        __Field::__field22 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "texture",
                                ));
                            }
                            __field22 = Some(MapAccess::next_value::<Texture>(&mut __map)?);
                        }
                        __Field::__field23 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "translate",
                                ));
                            }
                            __field23 = Some(MapAccess::next_value::<Translate>(&mut __map)?);
                        }
                    }
                }

                if let Some(__field0) = __field0 {
                    return Ok(Op::Center(__field0));
                }
                if let Some(__field1) = __field1 {
                    return Ok(Op::Color(__field1));
                }
                if let Some(__field2) = __field2 {
                    return Ok(Op::Comp(__field2));
                }
                if let Some(__field3) = __field3 {
                    return Ok(Op::Copy(__field3));
                }
                if let Some(__field4) = __field4 {
                    return Ok(Op::CornerCut(__field4));
                }
                if let Some(__field5) = __field5 {
                    return Ok(Op::Extrude(__field5));
                }
                if let Some(__field6) = __field6 {
                    return Ok(Op::Hemisphere(__field6));
                }
                if let Some(__field7) = __field7 {
                    return Ok(Op::InnerArch(__field7));
                }
                if let Some(__field8) = __field8 {
                    return Ok(Op::InnerCircle(__field8));
                }
                if let Some(__field9) = __field9 {
                    return Ok(Op::InnerSemiCircle(__field9));
                }
                if let Some(__field10) = __field10 {
                    return Ok(Op::Insert(__field10));
                }
                if let Some(__field11) = __field11 {
                    return Ok(Op::Offset(__field11));
                }
                if let Some(__field12) = __field12 {
                    return Ok(Op::Pyramid(__field12));
                }
                if let Some(__field13) = __field13 {
                    return Ok(Op::RoofGable(__field13));
                }
                if let Some(__field14) = __field14 {
                    return Ok(Op::RoofHip(__field14));
                }
                if let Some(__field15) = __field15 {
                    return Ok(Op::Rotate(__field15));
                }
                if let Some(__field16) = __field16 {
                    return Ok(Op::SetupProjection(__field16));
                }
                if let Some(__field17) = __field17 {
                    return Ok(Op::ShapeL(__field17));
                }
                if let Some(__field18) = __field18 {
                    return Ok(Op::ShapeU(__field18));
                }
                if let Some(__field19) = __field19 {
                    return Ok(Op::Size(__field19));
                }
                if let Some(__field20) = __field20 {
                    return Ok(Op::Split(__field20));
                }
                if let Some(__field21) = __field21 {
                    return Ok(Op::Taper(__field21));
                }
                if let Some(__field22) = __field22 {
                    return Ok(Op::Texture(__field22));
                }
                if let Some(__field23) = __field23 {
                    return Ok(Op::Translate(__field23));
                }
                todo!("TODO: Return `Err(?)`")
            }
        }
        #[doc(hidden)]
        const VARIANTS: &'static [&'static str] = &[
            "center",
            "color",
            "comp",
            "copy",
            "cornerCut",
            "extrude",
            "hemisphere",
            "innerArch",
            "innerCircle",
            "innerSemiCircle",
            "insert",
            "offset",
            "pyramid",
            "roofGable",
            "roofHip",
            "rotate",
            "setupProjection",
            "shapeL",
            "shapeU",
            "size",
            "split",
            "taper",
            "texture",
            "translate",
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
            Op::Center(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "center", &__field0)
            }
            Op::Color(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "color", &__field0)
            }
            Op::Comp(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "comp", &__field0)
            }
            Op::Copy(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "copy", &__field0)
            }
            Op::CornerCut(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "cornerCut",
                &__field0,
            ),
            Op::Extrude(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "extrude",
                &__field0,
            ),
            Op::Hemisphere(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "hemisphere",
                &__field0,
            ),
            Op::InnerArch(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "innerArch",
                &__field0,
            ),
            Op::InnerCircle(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "innerCircle",
                &__field0,
            ),
            Op::InnerSemiCircle(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "innerSemiCircle",
                &__field0,
            ),
            Op::Insert(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "insert", &__field0)
            }
            Op::Offset(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "offset", &__field0)
            }
            Op::Pyramid(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "pyramid",
                &__field0,
            ),
            Op::RoofGable(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "roofGable",
                &__field0,
            ),
            Op::RoofHip(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "roofHip",
                &__field0,
            ),
            Op::Rotate(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "rotate", &__field0)
            }
            Op::SetupProjection(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "setupProjection",
                &__field0,
            ),
            Op::ShapeL(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "shapeL", &__field0)
            }
            Op::ShapeU(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "shapeU", &__field0)
            }
            Op::Size(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "size", &__field0)
            }
            Op::Split(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "split", &__field0)
            }
            Op::Taper(ref __field0) => {
                serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "taper", &__field0)
            }
            Op::Texture(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "texture",
                &__field0,
            ),
            Op::Translate(ref __field0) => serde::ser::SerializeStruct::serialize_field(
                &mut _serde_state,
                "translate",
                &__field0,
            ),
        }?;
        serde::ser::SerializeStruct::end(_serde_state)
    }
}
