// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Account {
    // message fields
    id: ::std::option::Option<u64>,
    email: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Account {}

impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Account {
        static mut instance: ::protobuf::lazy::Lazy<Account> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Account,
        };
        unsafe {
            instance.get(Account::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        }
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for Account {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.email.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Account {
    fn new() -> Account {
        Account::new()
    }

    fn descriptor_static(_: ::std::option::Option<Account>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Account::get_id_for_reflect,
                    Account::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    Account::get_email_for_reflect,
                    Account::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Account::get_name_for_reflect,
                    Account::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Account>(
                    "Account",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Account {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_email();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Account {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Account {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountFindOrCreate {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    email: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountFindOrCreate {}

impl AccountFindOrCreate {
    pub fn new() -> AccountFindOrCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountFindOrCreate {
        static mut instance: ::protobuf::lazy::Lazy<AccountFindOrCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountFindOrCreate,
        };
        unsafe {
            instance.get(AccountFindOrCreate::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        }
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }
}

impl ::protobuf::Message for AccountFindOrCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.email.as_ref() {
            os.write_string(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountFindOrCreate {
    fn new() -> AccountFindOrCreate {
        AccountFindOrCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountFindOrCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AccountFindOrCreate::get_name_for_reflect,
                    AccountFindOrCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    AccountFindOrCreate::get_email_for_reflect,
                    AccountFindOrCreate::mut_email_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountFindOrCreate>(
                    "AccountFindOrCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountFindOrCreate {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_email();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountFindOrCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountFindOrCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountGet {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountGet {}

impl AccountGet {
    pub fn new() -> AccountGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountGet {
        static mut instance: ::protobuf::lazy::Lazy<AccountGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountGet,
        };
        unsafe {
            instance.get(AccountGet::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for AccountGet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountGet {
    fn new() -> AccountGet {
        AccountGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AccountGet::get_name_for_reflect,
                    AccountGet::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountGet>(
                    "AccountGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountGet {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountGetId {
    // message fields
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountGetId {}

impl AccountGetId {
    pub fn new() -> AccountGetId {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountGetId {
        static mut instance: ::protobuf::lazy::Lazy<AccountGetId> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountGetId,
        };
        unsafe {
            instance.get(AccountGetId::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }
}

impl ::protobuf::Message for AccountGetId {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountGetId {
    fn new() -> AccountGetId {
        AccountGetId::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountGetId>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    AccountGetId::get_id_for_reflect,
                    AccountGetId::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountGetId>(
                    "AccountGetId",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountGetId {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountGetId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountGetId {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountCreate {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    email: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountCreate {}

impl AccountCreate {
    pub fn new() -> AccountCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountCreate {
        static mut instance: ::protobuf::lazy::Lazy<AccountCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountCreate,
        };
        unsafe {
            instance.get(AccountCreate::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        }
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }
}

impl ::protobuf::Message for AccountCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.email.as_ref() {
            os.write_string(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountCreate {
    fn new() -> AccountCreate {
        AccountCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AccountCreate::get_name_for_reflect,
                    AccountCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    AccountCreate::get_email_for_reflect,
                    AccountCreate::mut_email_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountCreate>(
                    "AccountCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountCreate {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_email();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountUpdate {
    // message fields
    id: ::std::option::Option<u64>,
    email: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountUpdate {}

impl AccountUpdate {
    pub fn new() -> AccountUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountUpdate {
        static mut instance: ::protobuf::lazy::Lazy<AccountUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountUpdate,
        };
        unsafe {
            instance.get(AccountUpdate::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        }
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }
}

impl ::protobuf::Message for AccountUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.email.as_ref() {
            os.write_string(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountUpdate {
    fn new() -> AccountUpdate {
        AccountUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    AccountUpdate::get_id_for_reflect,
                    AccountUpdate::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    AccountUpdate::get_email_for_reflect,
                    AccountUpdate::mut_email_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountUpdate>(
                    "AccountUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountUpdate {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_email();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccessToken {
    // message fields
    account_id: ::std::option::Option<u64>,
    flags: ::std::option::Option<u32>,
    expires: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccessToken {}

impl AccessToken {
    pub fn new() -> AccessToken {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccessToken {
        static mut instance: ::protobuf::lazy::Lazy<AccessToken> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccessToken,
        };
        unsafe {
            instance.get(AccessToken::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional uint32 flags = 2;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flags
    }

    // optional int64 expires = 3;

    pub fn clear_expires(&mut self) {
        self.expires = ::std::option::Option::None;
    }

    pub fn has_expires(&self) -> bool {
        self.expires.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expires(&mut self, v: i64) {
        self.expires = ::std::option::Option::Some(v);
    }

    pub fn get_expires(&self) -> i64 {
        self.expires.unwrap_or(0)
    }

    fn get_expires_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.expires
    }

    fn mut_expires_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.expires
    }
}

impl ::protobuf::Message for AccessToken {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.expires = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.expires {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.flags {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.expires {
            os.write_int64(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccessToken {
    fn new() -> AccessToken {
        AccessToken::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccessToken>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccessToken::get_account_id_for_reflect,
                    AccessToken::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    AccessToken::get_flags_for_reflect,
                    AccessToken::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "expires",
                    AccessToken::get_expires_for_reflect,
                    AccessToken::mut_expires_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccessToken>(
                    "AccessToken",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccessToken {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_flags();
        self.clear_expires();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccessToken {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountToken {
    // message fields
    id: ::std::option::Option<u64>,
    account_id: ::std::option::Option<u64>,
    token: ::protobuf::SingularField<::std::string::String>,
    created_at: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountToken {}

impl AccountToken {
    pub fn new() -> AccountToken {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountToken {
        static mut instance: ::protobuf::lazy::Lazy<AccountToken> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountToken,
        };
        unsafe {
            instance.get(AccountToken::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional uint64 account_id = 2;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string token = 3;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token
    }

    // optional string created_at = 4;

    pub fn clear_created_at(&mut self) {
        self.created_at.clear();
    }

    pub fn has_created_at(&self) -> bool {
        self.created_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_created_at(&mut self) -> &mut ::std::string::String {
        if self.created_at.is_none() {
            self.created_at.set_default();
        }
        self.created_at.as_mut().unwrap()
    }

    // Take field
    pub fn take_created_at(&mut self) -> ::std::string::String {
        self.created_at.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_created_at(&self) -> &str {
        match self.created_at.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_created_at_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.created_at
    }

    fn mut_created_at_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.created_at
    }
}

impl ::protobuf::Message for AccountToken {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.created_at)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.token.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.created_at.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.account_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.token.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.created_at.as_ref() {
            os.write_string(4, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountToken {
    fn new() -> AccountToken {
        AccountToken::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountToken>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    AccountToken::get_id_for_reflect,
                    AccountToken::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountToken::get_account_id_for_reflect,
                    AccountToken::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    AccountToken::get_token_for_reflect,
                    AccountToken::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "created_at",
                    AccountToken::get_created_at_for_reflect,
                    AccountToken::mut_created_at_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountToken>(
                    "AccountToken",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountToken {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_account_id();
        self.clear_token();
        self.clear_created_at();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountToken {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountTokensGet {
    // message fields
    account_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountTokensGet {}

impl AccountTokensGet {
    pub fn new() -> AccountTokensGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountTokensGet {
        static mut instance: ::protobuf::lazy::Lazy<AccountTokensGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountTokensGet,
        };
        unsafe {
            instance.get(AccountTokensGet::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }
}

impl ::protobuf::Message for AccountTokensGet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountTokensGet {
    fn new() -> AccountTokensGet {
        AccountTokensGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountTokensGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountTokensGet::get_account_id_for_reflect,
                    AccountTokensGet::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountTokensGet>(
                    "AccountTokensGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountTokensGet {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountTokensGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountTokensGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountTokens {
    // message fields
    tokens: ::protobuf::RepeatedField<AccountToken>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountTokens {}

impl AccountTokens {
    pub fn new() -> AccountTokens {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountTokens {
        static mut instance: ::protobuf::lazy::Lazy<AccountTokens> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountTokens,
        };
        unsafe {
            instance.get(AccountTokens::new)
        }
    }

    // repeated .sessionsrv.AccountToken tokens = 1;

    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: ::protobuf::RepeatedField<AccountToken>) {
        self.tokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tokens(&mut self) -> &mut ::protobuf::RepeatedField<AccountToken> {
        &mut self.tokens
    }

    // Take field
    pub fn take_tokens(&mut self) -> ::protobuf::RepeatedField<AccountToken> {
        ::std::mem::replace(&mut self.tokens, ::protobuf::RepeatedField::new())
    }

    pub fn get_tokens(&self) -> &[AccountToken] {
        &self.tokens
    }

    fn get_tokens_for_reflect(&self) -> &::protobuf::RepeatedField<AccountToken> {
        &self.tokens
    }

    fn mut_tokens_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AccountToken> {
        &mut self.tokens
    }
}

impl ::protobuf::Message for AccountTokens {
    fn is_initialized(&self) -> bool {
        for v in &self.tokens {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tokens)?;
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
        for value in &self.tokens {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tokens {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountTokens {
    fn new() -> AccountTokens {
        AccountTokens::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountTokens>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AccountToken>>(
                    "tokens",
                    AccountTokens::get_tokens_for_reflect,
                    AccountTokens::mut_tokens_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountTokens>(
                    "AccountTokens",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountTokens {
    fn clear(&mut self) {
        self.clear_tokens();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountTokens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountTokens {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountTokenCreate {
    // message fields
    account_id: ::std::option::Option<u64>,
    token: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountTokenCreate {}

impl AccountTokenCreate {
    pub fn new() -> AccountTokenCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountTokenCreate {
        static mut instance: ::protobuf::lazy::Lazy<AccountTokenCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountTokenCreate,
        };
        unsafe {
            instance.get(AccountTokenCreate::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string token = 2;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token
    }
}

impl ::protobuf::Message for AccountTokenCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.token.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.token.as_ref() {
            os.write_string(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountTokenCreate {
    fn new() -> AccountTokenCreate {
        AccountTokenCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountTokenCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountTokenCreate::get_account_id_for_reflect,
                    AccountTokenCreate::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    AccountTokenCreate::get_token_for_reflect,
                    AccountTokenCreate::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountTokenCreate>(
                    "AccountTokenCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountTokenCreate {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountTokenCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountTokenCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountTokenRevoke {
    // message fields
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountTokenRevoke {}

impl AccountTokenRevoke {
    pub fn new() -> AccountTokenRevoke {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountTokenRevoke {
        static mut instance: ::protobuf::lazy::Lazy<AccountTokenRevoke> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountTokenRevoke,
        };
        unsafe {
            instance.get(AccountTokenRevoke::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }
}

impl ::protobuf::Message for AccountTokenRevoke {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountTokenRevoke {
    fn new() -> AccountTokenRevoke {
        AccountTokenRevoke::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountTokenRevoke>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    AccountTokenRevoke::get_id_for_reflect,
                    AccountTokenRevoke::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountTokenRevoke>(
                    "AccountTokenRevoke",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountTokenRevoke {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountTokenRevoke {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountTokenRevoke {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitation {
    // message fields
    id: ::std::option::Option<u64>,
    origin_invitation_id: ::std::option::Option<u64>,
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitation {}

impl AccountOriginInvitation {
    pub fn new() -> AccountOriginInvitation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitation {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitation,
        };
        unsafe {
            instance.get(AccountOriginInvitation::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional uint64 origin_invitation_id = 2;

    pub fn clear_origin_invitation_id(&mut self) {
        self.origin_invitation_id = ::std::option::Option::None;
    }

    pub fn has_origin_invitation_id(&self) -> bool {
        self.origin_invitation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_invitation_id(&mut self, v: u64) {
        self.origin_invitation_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_invitation_id(&self) -> u64 {
        self.origin_invitation_id.unwrap_or(0)
    }

    fn get_origin_invitation_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_invitation_id
    }

    fn mut_origin_invitation_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_invitation_id
    }

    // optional uint64 account_id = 3;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string account_name = 4;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        }
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 5;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 6;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional uint64 owner_id = 7;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for AccountOriginInvitation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_invitation_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin_invitation_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.origin_invitation_id {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.account_id {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.account_name.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(5, v)?;
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(7, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitation {
    fn new() -> AccountOriginInvitation {
        AccountOriginInvitation::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    AccountOriginInvitation::get_id_for_reflect,
                    AccountOriginInvitation::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_invitation_id",
                    AccountOriginInvitation::get_origin_invitation_id_for_reflect,
                    AccountOriginInvitation::mut_origin_invitation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitation::get_account_id_for_reflect,
                    AccountOriginInvitation::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    AccountOriginInvitation::get_account_name_for_reflect,
                    AccountOriginInvitation::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    AccountOriginInvitation::get_origin_id_for_reflect,
                    AccountOriginInvitation::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    AccountOriginInvitation::get_origin_name_for_reflect,
                    AccountOriginInvitation::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    AccountOriginInvitation::get_owner_id_for_reflect,
                    AccountOriginInvitation::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitation>(
                    "AccountOriginInvitation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitation {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_origin_invitation_id();
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitationCreate {
    // message fields
    origin_invitation_id: ::std::option::Option<u64>,
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitationCreate {}

impl AccountOriginInvitationCreate {
    pub fn new() -> AccountOriginInvitationCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitationCreate {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitationCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitationCreate,
        };
        unsafe {
            instance.get(AccountOriginInvitationCreate::new)
        }
    }

    // optional uint64 origin_invitation_id = 1;

    pub fn clear_origin_invitation_id(&mut self) {
        self.origin_invitation_id = ::std::option::Option::None;
    }

    pub fn has_origin_invitation_id(&self) -> bool {
        self.origin_invitation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_invitation_id(&mut self, v: u64) {
        self.origin_invitation_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_invitation_id(&self) -> u64 {
        self.origin_invitation_id.unwrap_or(0)
    }

    fn get_origin_invitation_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_invitation_id
    }

    fn mut_origin_invitation_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_invitation_id
    }

    // optional uint64 account_id = 2;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string account_name = 3;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        }
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 4;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 5;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional uint64 owner_id = 6;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for AccountOriginInvitationCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_invitation_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_invitation_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_invitation_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.account_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.account_name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(4, v)?;
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(6, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitationCreate {
    fn new() -> AccountOriginInvitationCreate {
        AccountOriginInvitationCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitationCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_invitation_id",
                    AccountOriginInvitationCreate::get_origin_invitation_id_for_reflect,
                    AccountOriginInvitationCreate::mut_origin_invitation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitationCreate::get_account_id_for_reflect,
                    AccountOriginInvitationCreate::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    AccountOriginInvitationCreate::get_account_name_for_reflect,
                    AccountOriginInvitationCreate::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    AccountOriginInvitationCreate::get_origin_id_for_reflect,
                    AccountOriginInvitationCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    AccountOriginInvitationCreate::get_origin_name_for_reflect,
                    AccountOriginInvitationCreate::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    AccountOriginInvitationCreate::get_owner_id_for_reflect,
                    AccountOriginInvitationCreate::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitationCreate>(
                    "AccountOriginInvitationCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitationCreate {
    fn clear(&mut self) {
        self.clear_origin_invitation_id();
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitationCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitationCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitationAcceptRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    invite_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    ignore: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitationAcceptRequest {}

impl AccountOriginInvitationAcceptRequest {
    pub fn new() -> AccountOriginInvitationAcceptRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitationAcceptRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitationAcceptRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitationAcceptRequest,
        };
        unsafe {
            instance.get(AccountOriginInvitationAcceptRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional uint64 invite_id = 2;

    pub fn clear_invite_id(&mut self) {
        self.invite_id = ::std::option::Option::None;
    }

    pub fn has_invite_id(&self) -> bool {
        self.invite_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invite_id(&mut self, v: u64) {
        self.invite_id = ::std::option::Option::Some(v);
    }

    pub fn get_invite_id(&self) -> u64 {
        self.invite_id.unwrap_or(0)
    }

    fn get_invite_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invite_id
    }

    fn mut_invite_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invite_id
    }

    // optional string origin_name = 3;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional bool ignore = 4;

    pub fn clear_ignore(&mut self) {
        self.ignore = ::std::option::Option::None;
    }

    pub fn has_ignore(&self) -> bool {
        self.ignore.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ignore(&mut self, v: bool) {
        self.ignore = ::std::option::Option::Some(v);
    }

    pub fn get_ignore(&self) -> bool {
        self.ignore.unwrap_or(false)
    }

    fn get_ignore_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ignore
    }

    fn mut_ignore_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ignore
    }
}

impl ::protobuf::Message for AccountOriginInvitationAcceptRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.invite_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ignore = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.invite_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.ignore {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.invite_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.ignore {
            os.write_bool(4, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitationAcceptRequest {
    fn new() -> AccountOriginInvitationAcceptRequest {
        AccountOriginInvitationAcceptRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitationAcceptRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitationAcceptRequest::get_account_id_for_reflect,
                    AccountOriginInvitationAcceptRequest::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "invite_id",
                    AccountOriginInvitationAcceptRequest::get_invite_id_for_reflect,
                    AccountOriginInvitationAcceptRequest::mut_invite_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    AccountOriginInvitationAcceptRequest::get_origin_name_for_reflect,
                    AccountOriginInvitationAcceptRequest::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore",
                    AccountOriginInvitationAcceptRequest::get_ignore_for_reflect,
                    AccountOriginInvitationAcceptRequest::mut_ignore_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitationAcceptRequest>(
                    "AccountOriginInvitationAcceptRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitationAcceptRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invite_id();
        self.clear_origin_name();
        self.clear_ignore();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitationAcceptRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitationAcceptRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitationIgnoreRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    invitation_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitationIgnoreRequest {}

impl AccountOriginInvitationIgnoreRequest {
    pub fn new() -> AccountOriginInvitationIgnoreRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitationIgnoreRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitationIgnoreRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitationIgnoreRequest,
        };
        unsafe {
            instance.get(AccountOriginInvitationIgnoreRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional uint64 invitation_id = 2;

    pub fn clear_invitation_id(&mut self) {
        self.invitation_id = ::std::option::Option::None;
    }

    pub fn has_invitation_id(&self) -> bool {
        self.invitation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invitation_id(&mut self, v: u64) {
        self.invitation_id = ::std::option::Option::Some(v);
    }

    pub fn get_invitation_id(&self) -> u64 {
        self.invitation_id.unwrap_or(0)
    }

    fn get_invitation_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invitation_id
    }

    fn mut_invitation_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invitation_id
    }
}

impl ::protobuf::Message for AccountOriginInvitationIgnoreRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.invitation_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.invitation_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.invitation_id {
            os.write_uint64(2, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitationIgnoreRequest {
    fn new() -> AccountOriginInvitationIgnoreRequest {
        AccountOriginInvitationIgnoreRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitationIgnoreRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitationIgnoreRequest::get_account_id_for_reflect,
                    AccountOriginInvitationIgnoreRequest::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "invitation_id",
                    AccountOriginInvitationIgnoreRequest::get_invitation_id_for_reflect,
                    AccountOriginInvitationIgnoreRequest::mut_invitation_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitationIgnoreRequest>(
                    "AccountOriginInvitationIgnoreRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitationIgnoreRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invitation_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitationIgnoreRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitationIgnoreRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginInvitationRescindRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    invitation_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginInvitationRescindRequest {}

impl AccountOriginInvitationRescindRequest {
    pub fn new() -> AccountOriginInvitationRescindRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginInvitationRescindRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginInvitationRescindRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginInvitationRescindRequest,
        };
        unsafe {
            instance.get(AccountOriginInvitationRescindRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional uint64 invitation_id = 2;

    pub fn clear_invitation_id(&mut self) {
        self.invitation_id = ::std::option::Option::None;
    }

    pub fn has_invitation_id(&self) -> bool {
        self.invitation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invitation_id(&mut self, v: u64) {
        self.invitation_id = ::std::option::Option::Some(v);
    }

    pub fn get_invitation_id(&self) -> u64 {
        self.invitation_id.unwrap_or(0)
    }

    fn get_invitation_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invitation_id
    }

    fn mut_invitation_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invitation_id
    }
}

impl ::protobuf::Message for AccountOriginInvitationRescindRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.invitation_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.invitation_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.invitation_id {
            os.write_uint64(2, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginInvitationRescindRequest {
    fn new() -> AccountOriginInvitationRescindRequest {
        AccountOriginInvitationRescindRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginInvitationRescindRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginInvitationRescindRequest::get_account_id_for_reflect,
                    AccountOriginInvitationRescindRequest::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "invitation_id",
                    AccountOriginInvitationRescindRequest::get_invitation_id_for_reflect,
                    AccountOriginInvitationRescindRequest::mut_invitation_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginInvitationRescindRequest>(
                    "AccountOriginInvitationRescindRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginInvitationRescindRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invitation_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginInvitationRescindRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginInvitationRescindRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInvitationListRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInvitationListRequest {}

impl AccountInvitationListRequest {
    pub fn new() -> AccountInvitationListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInvitationListRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountInvitationListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInvitationListRequest,
        };
        unsafe {
            instance.get(AccountInvitationListRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }
}

impl ::protobuf::Message for AccountInvitationListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountInvitationListRequest {
    fn new() -> AccountInvitationListRequest {
        AccountInvitationListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInvitationListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountInvitationListRequest::get_account_id_for_reflect,
                    AccountInvitationListRequest::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInvitationListRequest>(
                    "AccountInvitationListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInvitationListRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInvitationListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInvitationListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInvitationListResponse {
    // message fields
    account_id: ::std::option::Option<u64>,
    invitations: ::protobuf::RepeatedField<AccountOriginInvitation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInvitationListResponse {}

impl AccountInvitationListResponse {
    pub fn new() -> AccountInvitationListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInvitationListResponse {
        static mut instance: ::protobuf::lazy::Lazy<AccountInvitationListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInvitationListResponse,
        };
        unsafe {
            instance.get(AccountInvitationListResponse::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // repeated .sessionsrv.AccountOriginInvitation invitations = 2;

    pub fn clear_invitations(&mut self) {
        self.invitations.clear();
    }

    // Param is passed by value, moved
    pub fn set_invitations(&mut self, v: ::protobuf::RepeatedField<AccountOriginInvitation>) {
        self.invitations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invitations(&mut self) -> &mut ::protobuf::RepeatedField<AccountOriginInvitation> {
        &mut self.invitations
    }

    // Take field
    pub fn take_invitations(&mut self) -> ::protobuf::RepeatedField<AccountOriginInvitation> {
        ::std::mem::replace(&mut self.invitations, ::protobuf::RepeatedField::new())
    }

    pub fn get_invitations(&self) -> &[AccountOriginInvitation] {
        &self.invitations
    }

    fn get_invitations_for_reflect(&self) -> &::protobuf::RepeatedField<AccountOriginInvitation> {
        &self.invitations
    }

    fn mut_invitations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AccountOriginInvitation> {
        &mut self.invitations
    }
}

impl ::protobuf::Message for AccountInvitationListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.invitations {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.invitations)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.invitations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.invitations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountInvitationListResponse {
    fn new() -> AccountInvitationListResponse {
        AccountInvitationListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInvitationListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountInvitationListResponse::get_account_id_for_reflect,
                    AccountInvitationListResponse::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AccountOriginInvitation>>(
                    "invitations",
                    AccountInvitationListResponse::get_invitations_for_reflect,
                    AccountInvitationListResponse::mut_invitations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInvitationListResponse>(
                    "AccountInvitationListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInvitationListResponse {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invitations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInvitationListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInvitationListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginCreate {
    // message fields
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginCreate {}

impl AccountOriginCreate {
    pub fn new() -> AccountOriginCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginCreate {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginCreate,
        };
        unsafe {
            instance.get(AccountOriginCreate::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string account_name = 2;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        }
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 3;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 4;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }
}

impl ::protobuf::Message for AccountOriginCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.account_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(4, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginCreate {
    fn new() -> AccountOriginCreate {
        AccountOriginCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginCreate::get_account_id_for_reflect,
                    AccountOriginCreate::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    AccountOriginCreate::get_account_name_for_reflect,
                    AccountOriginCreate::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    AccountOriginCreate::get_origin_id_for_reflect,
                    AccountOriginCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    AccountOriginCreate::get_origin_name_for_reflect,
                    AccountOriginCreate::mut_origin_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginCreate>(
                    "AccountOriginCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginCreate {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginListRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginListRequest {}

impl AccountOriginListRequest {
    pub fn new() -> AccountOriginListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginListRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginListRequest,
        };
        unsafe {
            instance.get(AccountOriginListRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }
}

impl ::protobuf::Message for AccountOriginListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginListRequest {
    fn new() -> AccountOriginListRequest {
        AccountOriginListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginListRequest::get_account_id_for_reflect,
                    AccountOriginListRequest::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginListRequest>(
                    "AccountOriginListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginListRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginListResponse {
    // message fields
    account_id: ::std::option::Option<u64>,
    origins: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginListResponse {}

impl AccountOriginListResponse {
    pub fn new() -> AccountOriginListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginListResponse {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginListResponse,
        };
        unsafe {
            instance.get(AccountOriginListResponse::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // repeated string origins = 2;

    pub fn clear_origins(&mut self) {
        self.origins.clear();
    }

    // Param is passed by value, moved
    pub fn set_origins(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.origins = v;
    }

    // Mutable pointer to the field.
    pub fn mut_origins(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.origins
    }

    // Take field
    pub fn take_origins(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.origins, ::protobuf::RepeatedField::new())
    }

    pub fn get_origins(&self) -> &[::std::string::String] {
        &self.origins
    }

    fn get_origins_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.origins
    }

    fn mut_origins_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.origins
    }
}

impl ::protobuf::Message for AccountOriginListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.origins)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.origins {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.origins {
            os.write_string(2, &v)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginListResponse {
    fn new() -> AccountOriginListResponse {
        AccountOriginListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginListResponse::get_account_id_for_reflect,
                    AccountOriginListResponse::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origins",
                    AccountOriginListResponse::get_origins_for_reflect,
                    AccountOriginListResponse::mut_origins_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginListResponse>(
                    "AccountOriginListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginListResponse {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_origins();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginRemove {
    // message fields
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginRemove {}

impl AccountOriginRemove {
    pub fn new() -> AccountOriginRemove {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginRemove {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginRemove> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginRemove,
        };
        unsafe {
            instance.get(AccountOriginRemove::new)
        }
    }

    // optional string account_name = 1;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        }
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }
}

impl ::protobuf::Message for AccountOriginRemove {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.account_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(2, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginRemove {
    fn new() -> AccountOriginRemove {
        AccountOriginRemove::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginRemove>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    AccountOriginRemove::get_account_name_for_reflect,
                    AccountOriginRemove::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    AccountOriginRemove::get_origin_id_for_reflect,
                    AccountOriginRemove::mut_origin_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginRemove>(
                    "AccountOriginRemove",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginRemove {
    fn clear(&mut self) {
        self.clear_account_name();
        self.clear_origin_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginRemove {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginRemove {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Session {
    // message fields
    id: ::std::option::Option<u64>,
    email: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    token: ::protobuf::SingularField<::std::string::String>,
    flags: ::std::option::Option<u32>,
    oauth_token: ::protobuf::SingularField<::std::string::String>,
    session_type: ::std::option::Option<SessionType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Session {}

impl Session {
    pub fn new() -> Session {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Session {
        static mut instance: ::protobuf::lazy::Lazy<Session> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Session,
        };
        unsafe {
            instance.get(Session::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        }
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string token = 4;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token
    }

    // optional uint32 flags = 5;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flags
    }

    // optional string oauth_token = 6;

    pub fn clear_oauth_token(&mut self) {
        self.oauth_token.clear();
    }

    pub fn has_oauth_token(&self) -> bool {
        self.oauth_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oauth_token(&mut self, v: ::std::string::String) {
        self.oauth_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_oauth_token(&mut self) -> &mut ::std::string::String {
        if self.oauth_token.is_none() {
            self.oauth_token.set_default();
        }
        self.oauth_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_oauth_token(&mut self) -> ::std::string::String {
        self.oauth_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_oauth_token(&self) -> &str {
        match self.oauth_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_oauth_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.oauth_token
    }

    fn mut_oauth_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.oauth_token
    }

    // optional .sessionsrv.SessionType session_type = 7;

    pub fn clear_session_type(&mut self) {
        self.session_type = ::std::option::Option::None;
    }

    pub fn has_session_type(&self) -> bool {
        self.session_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_session_type(&mut self, v: SessionType) {
        self.session_type = ::std::option::Option::Some(v);
    }

    pub fn get_session_type(&self) -> SessionType {
        self.session_type.unwrap_or(SessionType::User)
    }

    fn get_session_type_for_reflect(&self) -> &::std::option::Option<SessionType> {
        &self.session_type
    }

    fn mut_session_type_for_reflect(&mut self) -> &mut ::std::option::Option<SessionType> {
        &mut self.session_type
    }
}

impl ::protobuf::Message for Session {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.oauth_token)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.session_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.token.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.oauth_token.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.session_type {
            my_size += ::protobuf::rt::enum_size(7, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.email.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.token.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.flags {
            os.write_uint32(5, v)?;
        }
        if let Some(ref v) = self.oauth_token.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.session_type {
            os.write_enum(7, v.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Session {
    fn new() -> Session {
        Session::new()
    }

    fn descriptor_static(_: ::std::option::Option<Session>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Session::get_id_for_reflect,
                    Session::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    Session::get_email_for_reflect,
                    Session::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Session::get_name_for_reflect,
                    Session::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    Session::get_token_for_reflect,
                    Session::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    Session::get_flags_for_reflect,
                    Session::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "oauth_token",
                    Session::get_oauth_token_for_reflect,
                    Session::mut_oauth_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SessionType>>(
                    "session_type",
                    Session::get_session_type_for_reflect,
                    Session::mut_session_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Session>(
                    "Session",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Session {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_email();
        self.clear_name();
        self.clear_token();
        self.clear_flags();
        self.clear_oauth_token();
        self.clear_session_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Session {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Session {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionCreate {
    // message fields
    token: ::protobuf::SingularField<::std::string::String>,
    extern_id: ::protobuf::SingularField<::std::string::String>,
    email: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    provider: ::std::option::Option<OAuthProvider>,
    app_token: ::protobuf::SingularField<::std::string::String>,
    session_type: ::std::option::Option<SessionType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionCreate {}

impl SessionCreate {
    pub fn new() -> SessionCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionCreate {
        static mut instance: ::protobuf::lazy::Lazy<SessionCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionCreate,
        };
        unsafe {
            instance.get(SessionCreate::new)
        }
    }

    // optional string token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token
    }

    // optional string extern_id = 2;

    pub fn clear_extern_id(&mut self) {
        self.extern_id.clear();
    }

    pub fn has_extern_id(&self) -> bool {
        self.extern_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extern_id(&mut self, v: ::std::string::String) {
        self.extern_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_extern_id(&mut self) -> &mut ::std::string::String {
        if self.extern_id.is_none() {
            self.extern_id.set_default();
        }
        self.extern_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_extern_id(&mut self) -> ::std::string::String {
        self.extern_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_extern_id(&self) -> &str {
        match self.extern_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_extern_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.extern_id
    }

    fn mut_extern_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.extern_id
    }

    // optional string email = 3;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        }
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_email_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.email
    }

    // optional string name = 4;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional .sessionsrv.OAuthProvider provider = 5;

    pub fn clear_provider(&mut self) {
        self.provider = ::std::option::Option::None;
    }

    pub fn has_provider(&self) -> bool {
        self.provider.is_some()
    }

    // Param is passed by value, moved
    pub fn set_provider(&mut self, v: OAuthProvider) {
        self.provider = ::std::option::Option::Some(v);
    }

    pub fn get_provider(&self) -> OAuthProvider {
        self.provider.unwrap_or(OAuthProvider::GitHub)
    }

    fn get_provider_for_reflect(&self) -> &::std::option::Option<OAuthProvider> {
        &self.provider
    }

    fn mut_provider_for_reflect(&mut self) -> &mut ::std::option::Option<OAuthProvider> {
        &mut self.provider
    }

    // optional string app_token = 6;

    pub fn clear_app_token(&mut self) {
        self.app_token.clear();
    }

    pub fn has_app_token(&self) -> bool {
        self.app_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_token(&mut self, v: ::std::string::String) {
        self.app_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_token(&mut self) -> &mut ::std::string::String {
        if self.app_token.is_none() {
            self.app_token.set_default();
        }
        self.app_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_app_token(&mut self) -> ::std::string::String {
        self.app_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_app_token(&self) -> &str {
        match self.app_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_app_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.app_token
    }

    fn mut_app_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.app_token
    }

    // optional .sessionsrv.SessionType session_type = 7;

    pub fn clear_session_type(&mut self) {
        self.session_type = ::std::option::Option::None;
    }

    pub fn has_session_type(&self) -> bool {
        self.session_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_session_type(&mut self, v: SessionType) {
        self.session_type = ::std::option::Option::Some(v);
    }

    pub fn get_session_type(&self) -> SessionType {
        self.session_type.unwrap_or(SessionType::User)
    }

    fn get_session_type_for_reflect(&self) -> &::std::option::Option<SessionType> {
        &self.session_type
    }

    fn mut_session_type_for_reflect(&mut self) -> &mut ::std::option::Option<SessionType> {
        &mut self.session_type
    }
}

impl ::protobuf::Message for SessionCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.extern_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.provider = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.app_token)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.session_type = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.token.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.extern_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.provider {
            my_size += ::protobuf::rt::enum_size(5, v);
        }
        if let Some(ref v) = self.app_token.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.session_type {
            my_size += ::protobuf::rt::enum_size(7, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.token.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.extern_id.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.email.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.provider {
            os.write_enum(5, v.value())?;
        }
        if let Some(ref v) = self.app_token.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.session_type {
            os.write_enum(7, v.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SessionCreate {
    fn new() -> SessionCreate {
        SessionCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    SessionCreate::get_token_for_reflect,
                    SessionCreate::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "extern_id",
                    SessionCreate::get_extern_id_for_reflect,
                    SessionCreate::mut_extern_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    SessionCreate::get_email_for_reflect,
                    SessionCreate::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    SessionCreate::get_name_for_reflect,
                    SessionCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OAuthProvider>>(
                    "provider",
                    SessionCreate::get_provider_for_reflect,
                    SessionCreate::mut_provider_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "app_token",
                    SessionCreate::get_app_token_for_reflect,
                    SessionCreate::mut_app_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SessionType>>(
                    "session_type",
                    SessionCreate::get_session_type_for_reflect,
                    SessionCreate::mut_session_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionCreate>(
                    "SessionCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionCreate {
    fn clear(&mut self) {
        self.clear_token();
        self.clear_extern_id();
        self.clear_email();
        self.clear_name();
        self.clear_provider();
        self.clear_app_token();
        self.clear_session_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionGet {
    // message fields
    token: ::protobuf::SingularPtrField<SessionToken>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionGet {}

impl SessionGet {
    pub fn new() -> SessionGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionGet {
        static mut instance: ::protobuf::lazy::Lazy<SessionGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionGet,
        };
        unsafe {
            instance.get(SessionGet::new)
        }
    }

    // optional .sessionsrv.SessionToken token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: SessionToken) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut SessionToken {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> SessionToken {
        self.token.take().unwrap_or_else(|| SessionToken::new())
    }

    pub fn get_token(&self) -> &SessionToken {
        self.token.as_ref().unwrap_or_else(|| SessionToken::default_instance())
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularPtrField<SessionToken> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SessionToken> {
        &mut self.token
    }
}

impl ::protobuf::Message for SessionGet {
    fn is_initialized(&self) -> bool {
        for v in &self.token {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.token)?;
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
        if let Some(ref v) = self.token.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.token.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SessionGet {
    fn new() -> SessionGet {
        SessionGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionToken>>(
                    "token",
                    SessionGet::get_token_for_reflect,
                    SessionGet::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionGet>(
                    "SessionGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionGet {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionToken {
    // message fields
    account_id: ::std::option::Option<u64>,
    extern_id: ::protobuf::SingularField<::std::string::String>,
    provider: ::std::option::Option<OAuthProvider>,
    token: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionToken {}

impl SessionToken {
    pub fn new() -> SessionToken {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionToken {
        static mut instance: ::protobuf::lazy::Lazy<SessionToken> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionToken,
        };
        unsafe {
            instance.get(SessionToken::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string extern_id = 2;

    pub fn clear_extern_id(&mut self) {
        self.extern_id.clear();
    }

    pub fn has_extern_id(&self) -> bool {
        self.extern_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extern_id(&mut self, v: ::std::string::String) {
        self.extern_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_extern_id(&mut self) -> &mut ::std::string::String {
        if self.extern_id.is_none() {
            self.extern_id.set_default();
        }
        self.extern_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_extern_id(&mut self) -> ::std::string::String {
        self.extern_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_extern_id(&self) -> &str {
        match self.extern_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_extern_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.extern_id
    }

    fn mut_extern_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.extern_id
    }

    // optional .sessionsrv.OAuthProvider provider = 3;

    pub fn clear_provider(&mut self) {
        self.provider = ::std::option::Option::None;
    }

    pub fn has_provider(&self) -> bool {
        self.provider.is_some()
    }

    // Param is passed by value, moved
    pub fn set_provider(&mut self, v: OAuthProvider) {
        self.provider = ::std::option::Option::Some(v);
    }

    pub fn get_provider(&self) -> OAuthProvider {
        self.provider.unwrap_or(OAuthProvider::GitHub)
    }

    fn get_provider_for_reflect(&self) -> &::std::option::Option<OAuthProvider> {
        &self.provider
    }

    fn mut_provider_for_reflect(&mut self) -> &mut ::std::option::Option<OAuthProvider> {
        &mut self.provider
    }

    // optional bytes token = 4;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::vec::Vec<u8>) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::vec::Vec<u8> {
        self.token.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_token(&self) -> &[u8] {
        match self.token.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.token
    }
}

impl ::protobuf::Message for SessionToken {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.extern_id)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.provider = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.token)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.extern_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.provider {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(ref v) = self.token.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.extern_id.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.provider {
            os.write_enum(3, v.value())?;
        }
        if let Some(ref v) = self.token.as_ref() {
            os.write_bytes(4, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SessionToken {
    fn new() -> SessionToken {
        SessionToken::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionToken>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    SessionToken::get_account_id_for_reflect,
                    SessionToken::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "extern_id",
                    SessionToken::get_extern_id_for_reflect,
                    SessionToken::mut_extern_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OAuthProvider>>(
                    "provider",
                    SessionToken::get_provider_for_reflect,
                    SessionToken::mut_provider_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "token",
                    SessionToken::get_token_for_reflect,
                    SessionToken::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionToken>(
                    "SessionToken",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionToken {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_extern_id();
        self.clear_provider();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionToken {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OAuthProvider {
    GitHub = 0,
    None = 1,
    Bitbucket = 2,
}

impl ::protobuf::ProtobufEnum for OAuthProvider {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OAuthProvider> {
        match value {
            0 => ::std::option::Option::Some(OAuthProvider::GitHub),
            1 => ::std::option::Option::Some(OAuthProvider::None),
            2 => ::std::option::Option::Some(OAuthProvider::Bitbucket),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OAuthProvider] = &[
            OAuthProvider::GitHub,
            OAuthProvider::None,
            OAuthProvider::Bitbucket,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OAuthProvider>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OAuthProvider", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OAuthProvider {
}

impl ::protobuf::reflect::ProtobufValue for OAuthProvider {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SessionType {
    User = 0,
    Builder = 1,
}

impl ::protobuf::ProtobufEnum for SessionType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SessionType> {
        match value {
            0 => ::std::option::Option::Some(SessionType::User),
            1 => ::std::option::Option::Some(SessionType::Builder),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SessionType] = &[
            SessionType::User,
            SessionType::Builder,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<SessionType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SessionType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SessionType {
}

impl ::protobuf::reflect::ProtobufValue for SessionType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aprotocols/sessionsrv.proto\x12\nsessionsrv\"C\n\x07Account\x12\x0e\
    \n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\x14\n\x05email\x18\x02\x20\x01(\
    \tR\x05email\x12\x12\n\x04name\x18\x03\x20\x01(\tR\x04name\"?\n\x13Accou\
    ntFindOrCreate\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\
    \x05email\x18\x02\x20\x01(\tR\x05email\"\x20\n\nAccountGet\x12\x12\n\x04\
    name\x18\x01\x20\x01(\tR\x04name\"\x1e\n\x0cAccountGetId\x12\x0e\n\x02id\
    \x18\x01\x20\x01(\x04R\x02id\"9\n\rAccountCreate\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12\x14\n\x05email\x18\x02\x20\x01(\tR\x05email\
    \"5\n\rAccountUpdate\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\x14\
    \n\x05email\x18\x02\x20\x01(\tR\x05email\"\\\n\x0bAccessToken\x12\x1d\n\
    \naccount_id\x18\x01\x20\x01(\x04R\taccountId\x12\x14\n\x05flags\x18\x02\
    \x20\x01(\rR\x05flags\x12\x18\n\x07expires\x18\x03\x20\x01(\x03R\x07expi\
    res\"r\n\x0cAccountToken\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\
    \x1d\n\naccount_id\x18\x02\x20\x01(\x04R\taccountId\x12\x14\n\x05token\
    \x18\x03\x20\x01(\tR\x05token\x12\x1d\n\ncreated_at\x18\x04\x20\x01(\tR\
    \tcreatedAt\"1\n\x10AccountTokensGet\x12\x1d\n\naccount_id\x18\x01\x20\
    \x01(\x04R\taccountId\"A\n\rAccountTokens\x120\n\x06tokens\x18\x01\x20\
    \x03(\x0b2\x18.sessionsrv.AccountTokenR\x06tokens\"I\n\x12AccountTokenCr\
    eate\x12\x1d\n\naccount_id\x18\x01\x20\x01(\x04R\taccountId\x12\x14\n\
    \x05token\x18\x02\x20\x01(\tR\x05token\"$\n\x12AccountTokenRevoke\x12\
    \x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\"\xf6\x01\n\x17AccountOriginInv\
    itation\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x120\n\x14origin_inv\
    itation_id\x18\x02\x20\x01(\x04R\x12originInvitationId\x12\x1d\n\naccoun\
    t_id\x18\x03\x20\x01(\x04R\taccountId\x12!\n\x0caccount_name\x18\x04\x20\
    \x01(\tR\x0baccountName\x12\x1b\n\torigin_id\x18\x05\x20\x01(\x04R\x08or\
    iginId\x12\x1f\n\x0borigin_name\x18\x06\x20\x01(\tR\noriginName\x12\x19\
    \n\x08owner_id\x18\x07\x20\x01(\x04R\x07ownerId\"\xec\x01\n\x1dAccountOr\
    iginInvitationCreate\x120\n\x14origin_invitation_id\x18\x01\x20\x01(\x04\
    R\x12originInvitationId\x12\x1d\n\naccount_id\x18\x02\x20\x01(\x04R\tacc\
    ountId\x12!\n\x0caccount_name\x18\x03\x20\x01(\tR\x0baccountName\x12\x1b\
    \n\torigin_id\x18\x04\x20\x01(\x04R\x08originId\x12\x1f\n\x0borigin_name\
    \x18\x05\x20\x01(\tR\noriginName\x12\x19\n\x08owner_id\x18\x06\x20\x01(\
    \x04R\x07ownerId\"\x9b\x01\n$AccountOriginInvitationAcceptRequest\x12\
    \x1d\n\naccount_id\x18\x01\x20\x01(\x04R\taccountId\x12\x1b\n\tinvite_id\
    \x18\x02\x20\x01(\x04R\x08inviteId\x12\x1f\n\x0borigin_name\x18\x03\x20\
    \x01(\tR\noriginName\x12\x16\n\x06ignore\x18\x04\x20\x01(\x08R\x06ignore\
    \"j\n$AccountOriginInvitationIgnoreRequest\x12\x1d\n\naccount_id\x18\x01\
    \x20\x01(\x04R\taccountId\x12#\n\rinvitation_id\x18\x02\x20\x01(\x04R\
    \x0cinvitationId\"k\n%AccountOriginInvitationRescindRequest\x12\x1d\n\na\
    ccount_id\x18\x01\x20\x01(\x04R\taccountId\x12#\n\rinvitation_id\x18\x02\
    \x20\x01(\x04R\x0cinvitationId\"=\n\x1cAccountInvitationListRequest\x12\
    \x1d\n\naccount_id\x18\x01\x20\x01(\x04R\taccountId\"\x85\x01\n\x1dAccou\
    ntInvitationListResponse\x12\x1d\n\naccount_id\x18\x01\x20\x01(\x04R\tac\
    countId\x12E\n\x0binvitations\x18\x02\x20\x03(\x0b2#.sessionsrv.AccountO\
    riginInvitationR\x0binvitations\"\x95\x01\n\x13AccountOriginCreate\x12\
    \x1d\n\naccount_id\x18\x01\x20\x01(\x04R\taccountId\x12!\n\x0caccount_na\
    me\x18\x02\x20\x01(\tR\x0baccountName\x12\x1b\n\torigin_id\x18\x03\x20\
    \x01(\x04R\x08originId\x12\x1f\n\x0borigin_name\x18\x04\x20\x01(\tR\nori\
    ginName\"9\n\x18AccountOriginListRequest\x12\x1d\n\naccount_id\x18\x01\
    \x20\x01(\x04R\taccountId\"T\n\x19AccountOriginListResponse\x12\x1d\n\na\
    ccount_id\x18\x01\x20\x01(\x04R\taccountId\x12\x18\n\x07origins\x18\x02\
    \x20\x03(\tR\x07origins\"U\n\x13AccountOriginRemove\x12!\n\x0caccount_na\
    me\x18\x01\x20\x01(\tR\x0baccountName\x12\x1b\n\torigin_id\x18\x02\x20\
    \x01(\x04R\x08originId\"\xcc\x01\n\x07Session\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\x04R\x02id\x12\x14\n\x05email\x18\x02\x20\x01(\tR\x05email\x12\
    \x12\n\x04name\x18\x03\x20\x01(\tR\x04name\x12\x14\n\x05token\x18\x04\
    \x20\x01(\tR\x05token\x12\x14\n\x05flags\x18\x05\x20\x01(\rR\x05flags\
    \x12\x1f\n\x0boauth_token\x18\x06\x20\x01(\tR\noauthToken\x12:\n\x0csess\
    ion_type\x18\x07\x20\x01(\x0e2\x17.sessionsrv.SessionTypeR\x0bsessionTyp\
    e\"\xfc\x01\n\rSessionCreate\x12\x14\n\x05token\x18\x01\x20\x01(\tR\x05t\
    oken\x12\x1b\n\textern_id\x18\x02\x20\x01(\tR\x08externId\x12\x14\n\x05e\
    mail\x18\x03\x20\x01(\tR\x05email\x12\x12\n\x04name\x18\x04\x20\x01(\tR\
    \x04name\x125\n\x08provider\x18\x05\x20\x01(\x0e2\x19.sessionsrv.OAuthPr\
    oviderR\x08provider\x12\x1b\n\tapp_token\x18\x06\x20\x01(\tR\x08appToken\
    \x12:\n\x0csession_type\x18\x07\x20\x01(\x0e2\x17.sessionsrv.SessionType\
    R\x0bsessionType\"<\n\nSessionGet\x12.\n\x05token\x18\x01\x20\x01(\x0b2\
    \x18.sessionsrv.SessionTokenR\x05token\"\x97\x01\n\x0cSessionToken\x12\
    \x1d\n\naccount_id\x18\x01\x20\x01(\x04R\taccountId\x12\x1b\n\textern_id\
    \x18\x02\x20\x01(\tR\x08externId\x125\n\x08provider\x18\x03\x20\x01(\x0e\
    2\x19.sessionsrv.OAuthProviderR\x08provider\x12\x14\n\x05token\x18\x04\
    \x20\x01(\x0cR\x05token*4\n\rOAuthProvider\x12\n\n\x06GitHub\x10\0\x12\
    \x08\n\x04None\x10\x01\x12\r\n\tBitbucket\x10\x02*$\n\x0bSessionType\x12\
    \x08\n\x04User\x10\0\x12\x0b\n\x07Builder\x10\x01J\xe31\n\x07\x12\x05\0\
    \0\xaa\x01\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x01\x08\x12\n\n\n\x02\x05\0\x12\x04\x03\0\x07\x01\n\n\n\x03\x05\0\x01\
    \x12\x03\x03\x05\x12\n\x0b\n\x04\x05\0\x02\0\x12\x03\x04\x02\r\n\x0c\n\
    \x05\x05\0\x02\0\x01\x12\x03\x04\x02\x08\n\x0c\n\x05\x05\0\x02\0\x02\x12\
    \x03\x04\x0b\x0c\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x05\x02\x0b\n\x0c\n\
    \x05\x05\0\x02\x01\x01\x12\x03\x05\x02\x06\n\x0c\n\x05\x05\0\x02\x01\x02\
    \x12\x03\x05\t\n\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x06\x02\x10\n\x0c\n\
    \x05\x05\0\x02\x02\x01\x12\x03\x06\x02\x0b\n\x0c\n\x05\x05\0\x02\x02\x02\
    \x12\x03\x06\x0e\x0f\n\n\n\x02\x04\0\x12\x04\t\0\r\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\t\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x02\x19\n\x0c\n\
    \x05\x04\0\x02\0\x04\x12\x03\n\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \n\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\x12\x14\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03\n\x17\x18\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0b\x02\
    \x1c\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x0b\x02\n\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x0b\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x0b\x12\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b\x1a\x1b\n\x0b\n\
    \x04\x04\0\x02\x02\x12\x03\x0c\x02\x1b\n\x0c\n\x05\x04\0\x02\x02\x04\x12\
    \x03\x0c\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0c\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03\x0c\x12\x16\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x0c\x19\x1a\n\n\n\x02\x04\x01\x12\x04\x0f\0\x12\x01\n\n\n\x03\
    \x04\x01\x01\x12\x03\x0f\x08\x1b\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x10\
    \x02\x1b\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03\x10\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\
    \x10\x12\x16\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x10\x19\x1a\n\x0b\n\
    \x04\x04\x01\x02\x01\x12\x03\x11\x02\x1c\n\x0c\n\x05\x04\x01\x02\x01\x04\
    \x12\x03\x11\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x11\x0b\x11\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x11\x12\x17\n\x0c\n\x05\x04\x01\
    \x02\x01\x03\x12\x03\x11\x1a\x1b\n+\n\x02\x04\x02\x12\x04\x15\0\x17\x01\
    \x1a\x1f\x20get\x20an\x20account\x20by\x20GH\x20username\n\n\n\n\x03\x04\
    \x02\x01\x12\x03\x15\x08\x12\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x16\x02\
    \x1b\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x16\x02\n\n\x0c\n\x05\x04\x02\
    \x02\0\x05\x12\x03\x16\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x16\
    \x12\x16\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x16\x19\x1a\n\n\n\x02\x04\
    \x03\x12\x04\x19\0\x1b\x01\n\n\n\x03\x04\x03\x01\x12\x03\x19\x08\x14\n\
    \x0b\n\x04\x04\x03\x02\0\x12\x03\x1a\x02\x19\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x1a\x0b\x11\
    \n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1a\x12\x14\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x03\x1a\x17\x18\n\n\n\x02\x04\x04\x12\x04\x1d\0\x20\x01\n\
    \n\n\x03\x04\x04\x01\x12\x03\x1d\x08\x15\n\x0b\n\x04\x04\x04\x02\0\x12\
    \x03\x1e\x02\x1b\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03\x1e\x02\n\n\x0c\n\
    \x05\x04\x04\x02\0\x05\x12\x03\x1e\x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\
    \x12\x03\x1e\x12\x16\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x1e\x19\x1a\n\
    \x0b\n\x04\x04\x04\x02\x01\x12\x03\x1f\x02\x1c\n\x0c\n\x05\x04\x04\x02\
    \x01\x04\x12\x03\x1f\x02\n\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03\x1f\
    \x0b\x11\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x1f\x12\x17\n\x0c\n\x05\
    \x04\x04\x02\x01\x03\x12\x03\x1f\x1a\x1b\n\n\n\x02\x04\x05\x12\x04\"\0%\
    \x01\n\n\n\x03\x04\x05\x01\x12\x03\"\x08\x15\n\x0b\n\x04\x04\x05\x02\0\
    \x12\x03#\x02\x19\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03#\x02\n\n\x0c\n\
    \x05\x04\x05\x02\0\x05\x12\x03#\x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\
    \x12\x03#\x12\x14\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03#\x17\x18\n\x0b\n\
    \x04\x04\x05\x02\x01\x12\x03$\x02\x1c\n\x0c\n\x05\x04\x05\x02\x01\x04\
    \x12\x03$\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03$\x0b\x11\n\x0c\n\
    \x05\x04\x05\x02\x01\x01\x12\x03$\x12\x17\n\x0c\n\x05\x04\x05\x02\x01\
    \x03\x12\x03$\x1a\x1b\n\n\n\x02\x04\x06\x12\x04'\0+\x01\n\n\n\x03\x04\
    \x06\x01\x12\x03'\x08\x13\n\x0b\n\x04\x04\x06\x02\0\x12\x03(\x04#\n\x0c\
    \n\x05\x04\x06\x02\0\x04\x12\x03(\x04\x0c\n\x0c\n\x05\x04\x06\x02\0\x05\
    \x12\x03(\r\x13\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03(\x14\x1e\n\x0c\n\
    \x05\x04\x06\x02\0\x03\x12\x03(!\"\n\x0b\n\x04\x04\x06\x02\x01\x12\x03)\
    \x04\x1e\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03)\x04\x0c\n\x0c\n\x05\
    \x04\x06\x02\x01\x05\x12\x03)\r\x13\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\
    \x03)\x14\x19\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03)\x1c\x1d\n\x0b\n\
    \x04\x04\x06\x02\x02\x12\x03*\x04\x1f\n\x0c\n\x05\x04\x06\x02\x02\x04\
    \x12\x03*\x04\x0c\n\x0c\n\x05\x04\x06\x02\x02\x05\x12\x03*\r\x12\n\x0c\n\
    \x05\x04\x06\x02\x02\x01\x12\x03*\x13\x1a\n\x0c\n\x05\x04\x06\x02\x02\
    \x03\x12\x03*\x1d\x1e\n\n\n\x02\x04\x07\x12\x04-\02\x01\n\n\n\x03\x04\
    \x07\x01\x12\x03-\x08\x14\n\x0b\n\x04\x04\x07\x02\0\x12\x03.\x02\x19\n\
    \x0c\n\x05\x04\x07\x02\0\x04\x12\x03.\x02\n\n\x0c\n\x05\x04\x07\x02\0\
    \x05\x12\x03.\x0b\x11\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03.\x12\x14\n\
    \x0c\n\x05\x04\x07\x02\0\x03\x12\x03.\x17\x18\n\x0b\n\x04\x04\x07\x02\
    \x01\x12\x03/\x02!\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\x03/\x02\n\n\x0c\
    \n\x05\x04\x07\x02\x01\x05\x12\x03/\x0b\x11\n\x0c\n\x05\x04\x07\x02\x01\
    \x01\x12\x03/\x12\x1c\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03/\x1f\x20\n\
    \x0b\n\x04\x04\x07\x02\x02\x12\x030\x02\x1c\n\x0c\n\x05\x04\x07\x02\x02\
    \x04\x12\x030\x02\n\n\x0c\n\x05\x04\x07\x02\x02\x05\x12\x030\x0b\x11\n\
    \x0c\n\x05\x04\x07\x02\x02\x01\x12\x030\x12\x17\n\x0c\n\x05\x04\x07\x02\
    \x02\x03\x12\x030\x1a\x1b\n%\n\x04\x04\x07\x02\x03\x12\x031\x02!\"\x18\
    \x20RFC3339-formatted\x20time\n\n\x0c\n\x05\x04\x07\x02\x03\x04\x12\x031\
    \x02\n\n\x0c\n\x05\x04\x07\x02\x03\x05\x12\x031\x0b\x11\n\x0c\n\x05\x04\
    \x07\x02\x03\x01\x12\x031\x12\x1c\n\x0c\n\x05\x04\x07\x02\x03\x03\x12\
    \x031\x1f\x20\n\n\n\x02\x04\x08\x12\x044\06\x01\n\n\n\x03\x04\x08\x01\
    \x12\x034\x08\x18\n\x0b\n\x04\x04\x08\x02\0\x12\x035\x02!\n\x0c\n\x05\
    \x04\x08\x02\0\x04\x12\x035\x02\n\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x035\
    \x0b\x11\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x035\x12\x1c\n\x0c\n\x05\x04\
    \x08\x02\0\x03\x12\x035\x1f\x20\n\n\n\x02\x04\t\x12\x048\0:\x01\n\n\n\
    \x03\x04\t\x01\x12\x038\x08\x15\n\x0b\n\x04\x04\t\x02\0\x12\x039\x02#\n\
    \x0c\n\x05\x04\t\x02\0\x04\x12\x039\x02\n\n\x0c\n\x05\x04\t\x02\0\x06\
    \x12\x039\x0b\x17\n\x0c\n\x05\x04\t\x02\0\x01\x12\x039\x18\x1e\n\x0c\n\
    \x05\x04\t\x02\0\x03\x12\x039!\"\n\n\n\x02\x04\n\x12\x04<\0?\x01\n\n\n\
    \x03\x04\n\x01\x12\x03<\x08\x1a\n\x0b\n\x04\x04\n\x02\0\x12\x03=\x02!\n\
    \x0c\n\x05\x04\n\x02\0\x04\x12\x03=\x02\n\n\x0c\n\x05\x04\n\x02\0\x05\
    \x12\x03=\x0b\x11\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03=\x12\x1c\n\x0c\n\
    \x05\x04\n\x02\0\x03\x12\x03=\x1f\x20\n\x0b\n\x04\x04\n\x02\x01\x12\x03>\
    \x02\x1c\n\x0c\n\x05\x04\n\x02\x01\x04\x12\x03>\x02\n\n\x0c\n\x05\x04\n\
    \x02\x01\x05\x12\x03>\x0b\x11\n\x0c\n\x05\x04\n\x02\x01\x01\x12\x03>\x12\
    \x17\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03>\x1a\x1b\n\n\n\x02\x04\x0b\
    \x12\x04A\0C\x01\n\n\n\x03\x04\x0b\x01\x12\x03A\x08\x1a\n\x0b\n\x04\x04\
    \x0b\x02\0\x12\x03B\x02\x19\n\x0c\n\x05\x04\x0b\x02\0\x04\x12\x03B\x02\n\
    \n\x0c\n\x05\x04\x0b\x02\0\x05\x12\x03B\x0b\x11\n\x0c\n\x05\x04\x0b\x02\
    \0\x01\x12\x03B\x12\x14\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03B\x17\x18\n\
    \n\n\x02\x04\x0c\x12\x04E\0M\x01\n\n\n\x03\x04\x0c\x01\x12\x03E\x08\x1f\
    \n\x0b\n\x04\x04\x0c\x02\0\x12\x03F\x02\x19\n\x0c\n\x05\x04\x0c\x02\0\
    \x04\x12\x03F\x02\n\n\x0c\n\x05\x04\x0c\x02\0\x05\x12\x03F\x0b\x11\n\x0c\
    \n\x05\x04\x0c\x02\0\x01\x12\x03F\x12\x14\n\x0c\n\x05\x04\x0c\x02\0\x03\
    \x12\x03F\x17\x18\n\x0b\n\x04\x04\x0c\x02\x01\x12\x03G\x02+\n\x0c\n\x05\
    \x04\x0c\x02\x01\x04\x12\x03G\x02\n\n\x0c\n\x05\x04\x0c\x02\x01\x05\x12\
    \x03G\x0b\x11\n\x0c\n\x05\x04\x0c\x02\x01\x01\x12\x03G\x12&\n\x0c\n\x05\
    \x04\x0c\x02\x01\x03\x12\x03G)*\n\x0b\n\x04\x04\x0c\x02\x02\x12\x03H\x02\
    !\n\x0c\n\x05\x04\x0c\x02\x02\x04\x12\x03H\x02\n\n\x0c\n\x05\x04\x0c\x02\
    \x02\x05\x12\x03H\x0b\x11\n\x0c\n\x05\x04\x0c\x02\x02\x01\x12\x03H\x12\
    \x1c\n\x0c\n\x05\x04\x0c\x02\x02\x03\x12\x03H\x1f\x20\n\x0b\n\x04\x04\
    \x0c\x02\x03\x12\x03I\x02#\n\x0c\n\x05\x04\x0c\x02\x03\x04\x12\x03I\x02\
    \n\n\x0c\n\x05\x04\x0c\x02\x03\x05\x12\x03I\x0b\x11\n\x0c\n\x05\x04\x0c\
    \x02\x03\x01\x12\x03I\x12\x1e\n\x0c\n\x05\x04\x0c\x02\x03\x03\x12\x03I!\
    \"\n\x0b\n\x04\x04\x0c\x02\x04\x12\x03J\x02\x20\n\x0c\n\x05\x04\x0c\x02\
    \x04\x04\x12\x03J\x02\n\n\x0c\n\x05\x04\x0c\x02\x04\x05\x12\x03J\x0b\x11\
    \n\x0c\n\x05\x04\x0c\x02\x04\x01\x12\x03J\x12\x1b\n\x0c\n\x05\x04\x0c\
    \x02\x04\x03\x12\x03J\x1e\x1f\n\x0b\n\x04\x04\x0c\x02\x05\x12\x03K\x02\"\
    \n\x0c\n\x05\x04\x0c\x02\x05\x04\x12\x03K\x02\n\n\x0c\n\x05\x04\x0c\x02\
    \x05\x05\x12\x03K\x0b\x11\n\x0c\n\x05\x04\x0c\x02\x05\x01\x12\x03K\x12\
    \x1d\n\x0c\n\x05\x04\x0c\x02\x05\x03\x12\x03K\x20!\n\x0b\n\x04\x04\x0c\
    \x02\x06\x12\x03L\x02\x1f\n\x0c\n\x05\x04\x0c\x02\x06\x04\x12\x03L\x02\n\
    \n\x0c\n\x05\x04\x0c\x02\x06\x05\x12\x03L\x0b\x11\n\x0c\n\x05\x04\x0c\
    \x02\x06\x01\x12\x03L\x12\x1a\n\x0c\n\x05\x04\x0c\x02\x06\x03\x12\x03L\
    \x1d\x1e\n\n\n\x02\x04\r\x12\x04O\0V\x01\n\n\n\x03\x04\r\x01\x12\x03O\
    \x08%\n\x0b\n\x04\x04\r\x02\0\x12\x03P\x02+\n\x0c\n\x05\x04\r\x02\0\x04\
    \x12\x03P\x02\n\n\x0c\n\x05\x04\r\x02\0\x05\x12\x03P\x0b\x11\n\x0c\n\x05\
    \x04\r\x02\0\x01\x12\x03P\x12&\n\x0c\n\x05\x04\r\x02\0\x03\x12\x03P)*\n\
    \x0b\n\x04\x04\r\x02\x01\x12\x03Q\x02!\n\x0c\n\x05\x04\r\x02\x01\x04\x12\
    \x03Q\x02\n\n\x0c\n\x05\x04\r\x02\x01\x05\x12\x03Q\x0b\x11\n\x0c\n\x05\
    \x04\r\x02\x01\x01\x12\x03Q\x12\x1c\n\x0c\n\x05\x04\r\x02\x01\x03\x12\
    \x03Q\x1f\x20\n\x0b\n\x04\x04\r\x02\x02\x12\x03R\x02#\n\x0c\n\x05\x04\r\
    \x02\x02\x04\x12\x03R\x02\n\n\x0c\n\x05\x04\r\x02\x02\x05\x12\x03R\x0b\
    \x11\n\x0c\n\x05\x04\r\x02\x02\x01\x12\x03R\x12\x1e\n\x0c\n\x05\x04\r\
    \x02\x02\x03\x12\x03R!\"\n\x0b\n\x04\x04\r\x02\x03\x12\x03S\x02\x20\n\
    \x0c\n\x05\x04\r\x02\x03\x04\x12\x03S\x02\n\n\x0c\n\x05\x04\r\x02\x03\
    \x05\x12\x03S\x0b\x11\n\x0c\n\x05\x04\r\x02\x03\x01\x12\x03S\x12\x1b\n\
    \x0c\n\x05\x04\r\x02\x03\x03\x12\x03S\x1e\x1f\n\x0b\n\x04\x04\r\x02\x04\
    \x12\x03T\x02\"\n\x0c\n\x05\x04\r\x02\x04\x04\x12\x03T\x02\n\n\x0c\n\x05\
    \x04\r\x02\x04\x05\x12\x03T\x0b\x11\n\x0c\n\x05\x04\r\x02\x04\x01\x12\
    \x03T\x12\x1d\n\x0c\n\x05\x04\r\x02\x04\x03\x12\x03T\x20!\n\x0b\n\x04\
    \x04\r\x02\x05\x12\x03U\x02\x1f\n\x0c\n\x05\x04\r\x02\x05\x04\x12\x03U\
    \x02\n\n\x0c\n\x05\x04\r\x02\x05\x05\x12\x03U\x0b\x11\n\x0c\n\x05\x04\r\
    \x02\x05\x01\x12\x03U\x12\x1a\n\x0c\n\x05\x04\r\x02\x05\x03\x12\x03U\x1d\
    \x1e\n\n\n\x02\x04\x0e\x12\x04X\0]\x01\n\n\n\x03\x04\x0e\x01\x12\x03X\
    \x08,\n\x0b\n\x04\x04\x0e\x02\0\x12\x03Y\x02!\n\x0c\n\x05\x04\x0e\x02\0\
    \x04\x12\x03Y\x02\n\n\x0c\n\x05\x04\x0e\x02\0\x05\x12\x03Y\x0b\x11\n\x0c\
    \n\x05\x04\x0e\x02\0\x01\x12\x03Y\x12\x1c\n\x0c\n\x05\x04\x0e\x02\0\x03\
    \x12\x03Y\x1f\x20\n\x0b\n\x04\x04\x0e\x02\x01\x12\x03Z\x02\x20\n\x0c\n\
    \x05\x04\x0e\x02\x01\x04\x12\x03Z\x02\n\n\x0c\n\x05\x04\x0e\x02\x01\x05\
    \x12\x03Z\x0b\x11\n\x0c\n\x05\x04\x0e\x02\x01\x01\x12\x03Z\x12\x1b\n\x0c\
    \n\x05\x04\x0e\x02\x01\x03\x12\x03Z\x1e\x1f\n\x0b\n\x04\x04\x0e\x02\x02\
    \x12\x03[\x02\"\n\x0c\n\x05\x04\x0e\x02\x02\x04\x12\x03[\x02\n\n\x0c\n\
    \x05\x04\x0e\x02\x02\x05\x12\x03[\x0b\x11\n\x0c\n\x05\x04\x0e\x02\x02\
    \x01\x12\x03[\x12\x1d\n\x0c\n\x05\x04\x0e\x02\x02\x03\x12\x03[\x20!\n\
    \x0b\n\x04\x04\x0e\x02\x03\x12\x03\\\x02\x1b\n\x0c\n\x05\x04\x0e\x02\x03\
    \x04\x12\x03\\\x02\n\n\x0c\n\x05\x04\x0e\x02\x03\x05\x12\x03\\\x0b\x0f\n\
    \x0c\n\x05\x04\x0e\x02\x03\x01\x12\x03\\\x10\x16\n\x0c\n\x05\x04\x0e\x02\
    \x03\x03\x12\x03\\\x19\x1a\n\n\n\x02\x04\x0f\x12\x04_\0b\x01\n\n\n\x03\
    \x04\x0f\x01\x12\x03_\x08,\n\x0b\n\x04\x04\x0f\x02\0\x12\x03`\x02!\n\x0c\
    \n\x05\x04\x0f\x02\0\x04\x12\x03`\x02\n\n\x0c\n\x05\x04\x0f\x02\0\x05\
    \x12\x03`\x0b\x11\n\x0c\n\x05\x04\x0f\x02\0\x01\x12\x03`\x12\x1c\n\x0c\n\
    \x05\x04\x0f\x02\0\x03\x12\x03`\x1f\x20\n\x0b\n\x04\x04\x0f\x02\x01\x12\
    \x03a\x02$\n\x0c\n\x05\x04\x0f\x02\x01\x04\x12\x03a\x02\n\n\x0c\n\x05\
    \x04\x0f\x02\x01\x05\x12\x03a\x0b\x11\n\x0c\n\x05\x04\x0f\x02\x01\x01\
    \x12\x03a\x12\x1f\n\x0c\n\x05\x04\x0f\x02\x01\x03\x12\x03a\"#\n\n\n\x02\
    \x04\x10\x12\x04d\0g\x01\n\n\n\x03\x04\x10\x01\x12\x03d\x08-\n\x0b\n\x04\
    \x04\x10\x02\0\x12\x03e\x02!\n\x0c\n\x05\x04\x10\x02\0\x04\x12\x03e\x02\
    \n\n\x0c\n\x05\x04\x10\x02\0\x05\x12\x03e\x0b\x11\n\x0c\n\x05\x04\x10\
    \x02\0\x01\x12\x03e\x12\x1c\n\x0c\n\x05\x04\x10\x02\0\x03\x12\x03e\x1f\
    \x20\n\x0b\n\x04\x04\x10\x02\x01\x12\x03f\x02$\n\x0c\n\x05\x04\x10\x02\
    \x01\x04\x12\x03f\x02\n\n\x0c\n\x05\x04\x10\x02\x01\x05\x12\x03f\x0b\x11\
    \n\x0c\n\x05\x04\x10\x02\x01\x01\x12\x03f\x12\x1f\n\x0c\n\x05\x04\x10\
    \x02\x01\x03\x12\x03f\"#\n\n\n\x02\x04\x11\x12\x04i\0k\x01\n\n\n\x03\x04\
    \x11\x01\x12\x03i\x08$\n\x0b\n\x04\x04\x11\x02\0\x12\x03j\x02!\n\x0c\n\
    \x05\x04\x11\x02\0\x04\x12\x03j\x02\n\n\x0c\n\x05\x04\x11\x02\0\x05\x12\
    \x03j\x0b\x11\n\x0c\n\x05\x04\x11\x02\0\x01\x12\x03j\x12\x1c\n\x0c\n\x05\
    \x04\x11\x02\0\x03\x12\x03j\x1f\x20\n\n\n\x02\x04\x12\x12\x04m\0p\x01\n\
    \n\n\x03\x04\x12\x01\x12\x03m\x08%\n\x0b\n\x04\x04\x12\x02\0\x12\x03n\
    \x02!\n\x0c\n\x05\x04\x12\x02\0\x04\x12\x03n\x02\n\n\x0c\n\x05\x04\x12\
    \x02\0\x05\x12\x03n\x0b\x11\n\x0c\n\x05\x04\x12\x02\0\x01\x12\x03n\x12\
    \x1c\n\x0c\n\x05\x04\x12\x02\0\x03\x12\x03n\x1f\x20\n\x0b\n\x04\x04\x12\
    \x02\x01\x12\x03o\x023\n\x0c\n\x05\x04\x12\x02\x01\x04\x12\x03o\x02\n\n\
    \x0c\n\x05\x04\x12\x02\x01\x06\x12\x03o\x0b\"\n\x0c\n\x05\x04\x12\x02\
    \x01\x01\x12\x03o#.\n\x0c\n\x05\x04\x12\x02\x01\x03\x12\x03o12\n\n\n\x02\
    \x04\x13\x12\x04r\0w\x01\n\n\n\x03\x04\x13\x01\x12\x03r\x08\x1b\n\x0b\n\
    \x04\x04\x13\x02\0\x12\x03s\x02!\n\x0c\n\x05\x04\x13\x02\0\x04\x12\x03s\
    \x02\n\n\x0c\n\x05\x04\x13\x02\0\x05\x12\x03s\x0b\x11\n\x0c\n\x05\x04\
    \x13\x02\0\x01\x12\x03s\x12\x1c\n\x0c\n\x05\x04\x13\x02\0\x03\x12\x03s\
    \x1f\x20\n\x0b\n\x04\x04\x13\x02\x01\x12\x03t\x02#\n\x0c\n\x05\x04\x13\
    \x02\x01\x04\x12\x03t\x02\n\n\x0c\n\x05\x04\x13\x02\x01\x05\x12\x03t\x0b\
    \x11\n\x0c\n\x05\x04\x13\x02\x01\x01\x12\x03t\x12\x1e\n\x0c\n\x05\x04\
    \x13\x02\x01\x03\x12\x03t!\"\n\x0b\n\x04\x04\x13\x02\x02\x12\x03u\x02\
    \x20\n\x0c\n\x05\x04\x13\x02\x02\x04\x12\x03u\x02\n\n\x0c\n\x05\x04\x13\
    \x02\x02\x05\x12\x03u\x0b\x11\n\x0c\n\x05\x04\x13\x02\x02\x01\x12\x03u\
    \x12\x1b\n\x0c\n\x05\x04\x13\x02\x02\x03\x12\x03u\x1e\x1f\n\x0b\n\x04\
    \x04\x13\x02\x03\x12\x03v\x02\"\n\x0c\n\x05\x04\x13\x02\x03\x04\x12\x03v\
    \x02\n\n\x0c\n\x05\x04\x13\x02\x03\x05\x12\x03v\x0b\x11\n\x0c\n\x05\x04\
    \x13\x02\x03\x01\x12\x03v\x12\x1d\n\x0c\n\x05\x04\x13\x02\x03\x03\x12\
    \x03v\x20!\n\n\n\x02\x04\x14\x12\x04y\0{\x01\n\n\n\x03\x04\x14\x01\x12\
    \x03y\x08\x20\n\x0b\n\x04\x04\x14\x02\0\x12\x03z\x02!\n\x0c\n\x05\x04\
    \x14\x02\0\x04\x12\x03z\x02\n\n\x0c\n\x05\x04\x14\x02\0\x05\x12\x03z\x0b\
    \x11\n\x0c\n\x05\x04\x14\x02\0\x01\x12\x03z\x12\x1c\n\x0c\n\x05\x04\x14\
    \x02\0\x03\x12\x03z\x1f\x20\n\x0b\n\x02\x04\x15\x12\x05}\0\x80\x01\x01\n\
    \n\n\x03\x04\x15\x01\x12\x03}\x08!\n\x0b\n\x04\x04\x15\x02\0\x12\x03~\
    \x02!\n\x0c\n\x05\x04\x15\x02\0\x04\x12\x03~\x02\n\n\x0c\n\x05\x04\x15\
    \x02\0\x05\x12\x03~\x0b\x11\n\x0c\n\x05\x04\x15\x02\0\x01\x12\x03~\x12\
    \x1c\n\x0c\n\x05\x04\x15\x02\0\x03\x12\x03~\x1f\x20\n\x0b\n\x04\x04\x15\
    \x02\x01\x12\x03\x7f\x02\x1e\n\x0c\n\x05\x04\x15\x02\x01\x04\x12\x03\x7f\
    \x02\n\n\x0c\n\x05\x04\x15\x02\x01\x05\x12\x03\x7f\x0b\x11\n\x0c\n\x05\
    \x04\x15\x02\x01\x01\x12\x03\x7f\x12\x19\n\x0c\n\x05\x04\x15\x02\x01\x03\
    \x12\x03\x7f\x1c\x1d\n\x0c\n\x02\x04\x16\x12\x06\x82\x01\0\x85\x01\x01\n\
    \x0b\n\x03\x04\x16\x01\x12\x04\x82\x01\x08\x1b\n\x0c\n\x04\x04\x16\x02\0\
    \x12\x04\x83\x01\x02#\n\r\n\x05\x04\x16\x02\0\x04\x12\x04\x83\x01\x02\n\
    \n\r\n\x05\x04\x16\x02\0\x05\x12\x04\x83\x01\x0b\x11\n\r\n\x05\x04\x16\
    \x02\0\x01\x12\x04\x83\x01\x12\x1e\n\r\n\x05\x04\x16\x02\0\x03\x12\x04\
    \x83\x01!\"\n\x0c\n\x04\x04\x16\x02\x01\x12\x04\x84\x01\x02\x20\n\r\n\
    \x05\x04\x16\x02\x01\x04\x12\x04\x84\x01\x02\n\n\r\n\x05\x04\x16\x02\x01\
    \x05\x12\x04\x84\x01\x0b\x11\n\r\n\x05\x04\x16\x02\x01\x01\x12\x04\x84\
    \x01\x12\x1b\n\r\n\x05\x04\x16\x02\x01\x03\x12\x04\x84\x01\x1e\x1f\n\x0c\
    \n\x02\x05\x01\x12\x06\x87\x01\0\x8a\x01\x01\n\x0b\n\x03\x05\x01\x01\x12\
    \x04\x87\x01\x05\x10\n\x0c\n\x04\x05\x01\x02\0\x12\x04\x88\x01\x02\x0b\n\
    \r\n\x05\x05\x01\x02\0\x01\x12\x04\x88\x01\x02\x06\n\r\n\x05\x05\x01\x02\
    \0\x02\x12\x04\x88\x01\t\n\n\x0c\n\x04\x05\x01\x02\x01\x12\x04\x89\x01\
    \x02\x0e\n\r\n\x05\x05\x01\x02\x01\x01\x12\x04\x89\x01\x02\t\n\r\n\x05\
    \x05\x01\x02\x01\x02\x12\x04\x89\x01\x0c\r\n\x0c\n\x02\x04\x17\x12\x06\
    \x8c\x01\0\x95\x01\x01\n\x0b\n\x03\x04\x17\x01\x12\x04\x8c\x01\x08\x0f\n\
    \x0c\n\x04\x04\x17\x02\0\x12\x04\x8d\x01\x02\x19\n\r\n\x05\x04\x17\x02\0\
    \x04\x12\x04\x8d\x01\x02\n\n\r\n\x05\x04\x17\x02\0\x05\x12\x04\x8d\x01\
    \x0b\x11\n\r\n\x05\x04\x17\x02\0\x01\x12\x04\x8d\x01\x12\x14\n\r\n\x05\
    \x04\x17\x02\0\x03\x12\x04\x8d\x01\x17\x18\n\x0c\n\x04\x04\x17\x02\x01\
    \x12\x04\x8e\x01\x02\x1c\n\r\n\x05\x04\x17\x02\x01\x04\x12\x04\x8e\x01\
    \x02\n\n\r\n\x05\x04\x17\x02\x01\x05\x12\x04\x8e\x01\x0b\x11\n\r\n\x05\
    \x04\x17\x02\x01\x01\x12\x04\x8e\x01\x12\x17\n\r\n\x05\x04\x17\x02\x01\
    \x03\x12\x04\x8e\x01\x1a\x1b\n\x0c\n\x04\x04\x17\x02\x02\x12\x04\x8f\x01\
    \x02\x1b\n\r\n\x05\x04\x17\x02\x02\x04\x12\x04\x8f\x01\x02\n\n\r\n\x05\
    \x04\x17\x02\x02\x05\x12\x04\x8f\x01\x0b\x11\n\r\n\x05\x04\x17\x02\x02\
    \x01\x12\x04\x8f\x01\x12\x16\n\r\n\x05\x04\x17\x02\x02\x03\x12\x04\x8f\
    \x01\x19\x1a\nD\n\x04\x04\x17\x02\x03\x12\x04\x91\x01\x02\x1c\x1a6\x20ba\
    se64\x20encoded\x20binary\x20representation\x20of\x20SessionToken\n\n\r\
    \n\x05\x04\x17\x02\x03\x04\x12\x04\x91\x01\x02\n\n\r\n\x05\x04\x17\x02\
    \x03\x05\x12\x04\x91\x01\x0b\x11\n\r\n\x05\x04\x17\x02\x03\x01\x12\x04\
    \x91\x01\x12\x17\n\r\n\x05\x04\x17\x02\x03\x03\x12\x04\x91\x01\x1a\x1b\n\
    \x0c\n\x04\x04\x17\x02\x04\x12\x04\x92\x01\x02\x1c\n\r\n\x05\x04\x17\x02\
    \x04\x04\x12\x04\x92\x01\x02\n\n\r\n\x05\x04\x17\x02\x04\x05\x12\x04\x92\
    \x01\x0b\x11\n\r\n\x05\x04\x17\x02\x04\x01\x12\x04\x92\x01\x12\x17\n\r\n\
    \x05\x04\x17\x02\x04\x03\x12\x04\x92\x01\x1a\x1b\n\x0c\n\x04\x04\x17\x02\
    \x05\x12\x04\x93\x01\x02\"\n\r\n\x05\x04\x17\x02\x05\x04\x12\x04\x93\x01\
    \x02\n\n\r\n\x05\x04\x17\x02\x05\x05\x12\x04\x93\x01\x0b\x11\n\r\n\x05\
    \x04\x17\x02\x05\x01\x12\x04\x93\x01\x12\x1d\n\r\n\x05\x04\x17\x02\x05\
    \x03\x12\x04\x93\x01\x20!\n!\n\x04\x04\x17\x02\x06\x12\x04\x94\x01\x02(\
    \"\x13\x20TBD\x20-\x20Remove\x20this\n\n\r\n\x05\x04\x17\x02\x06\x04\x12\
    \x04\x94\x01\x02\n\n\r\n\x05\x04\x17\x02\x06\x06\x12\x04\x94\x01\x0b\x16\
    \n\r\n\x05\x04\x17\x02\x06\x01\x12\x04\x94\x01\x17#\n\r\n\x05\x04\x17\
    \x02\x06\x03\x12\x04\x94\x01&'\n\x0c\n\x02\x04\x18\x12\x06\x97\x01\0\x9f\
    \x01\x01\n\x0b\n\x03\x04\x18\x01\x12\x04\x97\x01\x08\x15\n\x0c\n\x04\x04\
    \x18\x02\0\x12\x04\x98\x01\x02\x1c\n\r\n\x05\x04\x18\x02\0\x04\x12\x04\
    \x98\x01\x02\n\n\r\n\x05\x04\x18\x02\0\x05\x12\x04\x98\x01\x0b\x11\n\r\n\
    \x05\x04\x18\x02\0\x01\x12\x04\x98\x01\x12\x17\n\r\n\x05\x04\x18\x02\0\
    \x03\x12\x04\x98\x01\x1a\x1b\n\x0c\n\x04\x04\x18\x02\x01\x12\x04\x99\x01\
    \x02\x20\n\r\n\x05\x04\x18\x02\x01\x04\x12\x04\x99\x01\x02\n\n\r\n\x05\
    \x04\x18\x02\x01\x05\x12\x04\x99\x01\x0b\x11\n\r\n\x05\x04\x18\x02\x01\
    \x01\x12\x04\x99\x01\x12\x1b\n\r\n\x05\x04\x18\x02\x01\x03\x12\x04\x99\
    \x01\x1e\x1f\n\x0c\n\x04\x04\x18\x02\x02\x12\x04\x9a\x01\x02\x1c\n\r\n\
    \x05\x04\x18\x02\x02\x04\x12\x04\x9a\x01\x02\n\n\r\n\x05\x04\x18\x02\x02\
    \x05\x12\x04\x9a\x01\x0b\x11\n\r\n\x05\x04\x18\x02\x02\x01\x12\x04\x9a\
    \x01\x12\x17\n\r\n\x05\x04\x18\x02\x02\x03\x12\x04\x9a\x01\x1a\x1b\n\x0c\
    \n\x04\x04\x18\x02\x03\x12\x04\x9b\x01\x02\x1b\n\r\n\x05\x04\x18\x02\x03\
    \x04\x12\x04\x9b\x01\x02\n\n\r\n\x05\x04\x18\x02\x03\x05\x12\x04\x9b\x01\
    \x0b\x11\n\r\n\x05\x04\x18\x02\x03\x01\x12\x04\x9b\x01\x12\x16\n\r\n\x05\
    \x04\x18\x02\x03\x03\x12\x04\x9b\x01\x19\x1a\n\x0c\n\x04\x04\x18\x02\x04\
    \x12\x04\x9c\x01\x02&\n\r\n\x05\x04\x18\x02\x04\x04\x12\x04\x9c\x01\x02\
    \n\n\r\n\x05\x04\x18\x02\x04\x06\x12\x04\x9c\x01\x0b\x18\n\r\n\x05\x04\
    \x18\x02\x04\x01\x12\x04\x9c\x01\x19!\n\r\n\x05\x04\x18\x02\x04\x03\x12\
    \x04\x9c\x01$%\n\x0c\n\x04\x04\x18\x02\x05\x12\x04\x9d\x01\x02\x20\n\r\n\
    \x05\x04\x18\x02\x05\x04\x12\x04\x9d\x01\x02\n\n\r\n\x05\x04\x18\x02\x05\
    \x05\x12\x04\x9d\x01\x0b\x11\n\r\n\x05\x04\x18\x02\x05\x01\x12\x04\x9d\
    \x01\x12\x1b\n\r\n\x05\x04\x18\x02\x05\x03\x12\x04\x9d\x01\x1e\x1f\n\x0c\
    \n\x04\x04\x18\x02\x06\x12\x04\x9e\x01\x02(\n\r\n\x05\x04\x18\x02\x06\
    \x04\x12\x04\x9e\x01\x02\n\n\r\n\x05\x04\x18\x02\x06\x06\x12\x04\x9e\x01\
    \x0b\x16\n\r\n\x05\x04\x18\x02\x06\x01\x12\x04\x9e\x01\x17#\n\r\n\x05\
    \x04\x18\x02\x06\x03\x12\x04\x9e\x01&'\n\x0c\n\x02\x04\x19\x12\x06\xa1\
    \x01\0\xa3\x01\x01\n\x0b\n\x03\x04\x19\x01\x12\x04\xa1\x01\x08\x12\n\x0c\
    \n\x04\x04\x19\x02\0\x12\x04\xa2\x01\x02\"\n\r\n\x05\x04\x19\x02\0\x04\
    \x12\x04\xa2\x01\x02\n\n\r\n\x05\x04\x19\x02\0\x06\x12\x04\xa2\x01\x0b\
    \x17\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\xa2\x01\x18\x1d\n\r\n\x05\x04\
    \x19\x02\0\x03\x12\x04\xa2\x01\x20!\n\x0c\n\x02\x04\x1a\x12\x06\xa5\x01\
    \0\xaa\x01\x01\n\x0b\n\x03\x04\x1a\x01\x12\x04\xa5\x01\x08\x14\n\x0c\n\
    \x04\x04\x1a\x02\0\x12\x04\xa6\x01\x02!\n\r\n\x05\x04\x1a\x02\0\x04\x12\
    \x04\xa6\x01\x02\n\n\r\n\x05\x04\x1a\x02\0\x05\x12\x04\xa6\x01\x0b\x11\n\
    \r\n\x05\x04\x1a\x02\0\x01\x12\x04\xa6\x01\x12\x1c\n\r\n\x05\x04\x1a\x02\
    \0\x03\x12\x04\xa6\x01\x1f\x20\n\x0c\n\x04\x04\x1a\x02\x01\x12\x04\xa7\
    \x01\x02\x20\n\r\n\x05\x04\x1a\x02\x01\x04\x12\x04\xa7\x01\x02\n\n\r\n\
    \x05\x04\x1a\x02\x01\x05\x12\x04\xa7\x01\x0b\x11\n\r\n\x05\x04\x1a\x02\
    \x01\x01\x12\x04\xa7\x01\x12\x1b\n\r\n\x05\x04\x1a\x02\x01\x03\x12\x04\
    \xa7\x01\x1e\x1f\n\x0c\n\x04\x04\x1a\x02\x02\x12\x04\xa8\x01\x02&\n\r\n\
    \x05\x04\x1a\x02\x02\x04\x12\x04\xa8\x01\x02\n\n\r\n\x05\x04\x1a\x02\x02\
    \x06\x12\x04\xa8\x01\x0b\x18\n\r\n\x05\x04\x1a\x02\x02\x01\x12\x04\xa8\
    \x01\x19!\n\r\n\x05\x04\x1a\x02\x02\x03\x12\x04\xa8\x01$%\n\x0c\n\x04\
    \x04\x1a\x02\x03\x12\x04\xa9\x01\x02\x1b\n\r\n\x05\x04\x1a\x02\x03\x04\
    \x12\x04\xa9\x01\x02\n\n\r\n\x05\x04\x1a\x02\x03\x05\x12\x04\xa9\x01\x0b\
    \x10\n\r\n\x05\x04\x1a\x02\x03\x01\x12\x04\xa9\x01\x11\x16\n\r\n\x05\x04\
    \x1a\x02\x03\x03\x12\x04\xa9\x01\x19\x1a\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
