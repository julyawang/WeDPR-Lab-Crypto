// Copyright 2020 WeDPR Lab Project Authors. Licensed under Apache-2.0.

// This file is generated by rust-protobuf 2.20.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `crypto/zkp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct BalanceProof {
    // message fields
    pub c: ::std::vec::Vec<u8>,
    pub m1: ::std::vec::Vec<u8>,
    pub m2: ::std::vec::Vec<u8>,
    pub m3: ::std::vec::Vec<u8>,
    pub m4: ::std::vec::Vec<u8>,
    pub m5: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BalanceProof {
    fn default() -> &'a BalanceProof {
        <BalanceProof as ::protobuf::Message>::default_instance()
    }
}

impl BalanceProof {
    pub fn new() -> BalanceProof {
        ::std::default::Default::default()
    }

    // bytes c = 1;


    pub fn get_c(&self) -> &[u8] {
        &self.c
    }
    pub fn clear_c(&mut self) {
        self.c.clear();
    }

    // Param is passed by value, moved
    pub fn set_c(&mut self, v: ::std::vec::Vec<u8>) {
        self.c = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_c(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.c
    }

    // Take field
    pub fn take_c(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.c, ::std::vec::Vec::new())
    }

    // bytes m1 = 2;


    pub fn get_m1(&self) -> &[u8] {
        &self.m1
    }
    pub fn clear_m1(&mut self) {
        self.m1.clear();
    }

    // Param is passed by value, moved
    pub fn set_m1(&mut self, v: ::std::vec::Vec<u8>) {
        self.m1 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_m1(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.m1
    }

    // Take field
    pub fn take_m1(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.m1, ::std::vec::Vec::new())
    }

    // bytes m2 = 3;


    pub fn get_m2(&self) -> &[u8] {
        &self.m2
    }
    pub fn clear_m2(&mut self) {
        self.m2.clear();
    }

    // Param is passed by value, moved
    pub fn set_m2(&mut self, v: ::std::vec::Vec<u8>) {
        self.m2 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_m2(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.m2
    }

    // Take field
    pub fn take_m2(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.m2, ::std::vec::Vec::new())
    }

    // bytes m3 = 4;


    pub fn get_m3(&self) -> &[u8] {
        &self.m3
    }
    pub fn clear_m3(&mut self) {
        self.m3.clear();
    }

    // Param is passed by value, moved
    pub fn set_m3(&mut self, v: ::std::vec::Vec<u8>) {
        self.m3 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_m3(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.m3
    }

    // Take field
    pub fn take_m3(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.m3, ::std::vec::Vec::new())
    }

    // bytes m4 = 5;


    pub fn get_m4(&self) -> &[u8] {
        &self.m4
    }
    pub fn clear_m4(&mut self) {
        self.m4.clear();
    }

    // Param is passed by value, moved
    pub fn set_m4(&mut self, v: ::std::vec::Vec<u8>) {
        self.m4 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_m4(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.m4
    }

    // Take field
    pub fn take_m4(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.m4, ::std::vec::Vec::new())
    }

    // bytes m5 = 6;


    pub fn get_m5(&self) -> &[u8] {
        &self.m5
    }
    pub fn clear_m5(&mut self) {
        self.m5.clear();
    }

    // Param is passed by value, moved
    pub fn set_m5(&mut self, v: ::std::vec::Vec<u8>) {
        self.m5 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_m5(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.m5
    }

    // Take field
    pub fn take_m5(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.m5, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for BalanceProof {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.c)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.m1)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.m2)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.m3)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.m4)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.m5)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.c.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.c);
        }
        if !self.m1.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.m1);
        }
        if !self.m2.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.m2);
        }
        if !self.m3.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.m3);
        }
        if !self.m4.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.m4);
        }
        if !self.m5.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.m5);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.c.is_empty() {
            os.write_bytes(1, &self.c)?;
        }
        if !self.m1.is_empty() {
            os.write_bytes(2, &self.m1)?;
        }
        if !self.m2.is_empty() {
            os.write_bytes(3, &self.m2)?;
        }
        if !self.m3.is_empty() {
            os.write_bytes(4, &self.m3)?;
        }
        if !self.m4.is_empty() {
            os.write_bytes(5, &self.m4)?;
        }
        if !self.m5.is_empty() {
            os.write_bytes(6, &self.m5)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> BalanceProof {
        BalanceProof::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "c",
                |m: &BalanceProof| { &m.c },
                |m: &mut BalanceProof| { &mut m.c },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "m1",
                |m: &BalanceProof| { &m.m1 },
                |m: &mut BalanceProof| { &mut m.m1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "m2",
                |m: &BalanceProof| { &m.m2 },
                |m: &mut BalanceProof| { &mut m.m2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "m3",
                |m: &BalanceProof| { &m.m3 },
                |m: &mut BalanceProof| { &mut m.m3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "m4",
                |m: &BalanceProof| { &m.m4 },
                |m: &mut BalanceProof| { &mut m.m4 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "m5",
                |m: &BalanceProof| { &m.m5 },
                |m: &mut BalanceProof| { &mut m.m5 },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<BalanceProof>(
                "BalanceProof",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static BalanceProof {
        static instance: ::protobuf::rt::LazyV2<BalanceProof> = ::protobuf::rt::LazyV2::INIT;
        instance.get(BalanceProof::new)
    }
}

impl ::protobuf::Clear for BalanceProof {
    fn clear(&mut self) {
        self.c.clear();
        self.m1.clear();
        self.m2.clear();
        self.m3.clear();
        self.m4.clear();
        self.m5.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BalanceProof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BalanceProof {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10crypto/zkp.proto\x12\"com.webank.blockchain.crypto.proto\"l\n\x0cB\
    alanceProof\x12\x0c\n\x01c\x18\x01\x20\x01(\x0cR\x01c\x12\x0e\n\x02m1\
    \x18\x02\x20\x01(\x0cR\x02m1\x12\x0e\n\x02m2\x18\x03\x20\x01(\x0cR\x02m2\
    \x12\x0e\n\x02m3\x18\x04\x20\x01(\x0cR\x02m3\x12\x0e\n\x02m4\x18\x05\x20\
    \x01(\x0cR\x02m4\x12\x0e\n\x02m5\x18\x06\x20\x01(\x0cR\x02m5B&\n\"com.we\
    bank.blockchain.crypto.protoP\x01b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
