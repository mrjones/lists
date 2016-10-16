// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct User {
    // message fields
    id: ::std::option::Option<i64>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for User {}

impl User {
    pub fn new() -> User {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static User {
        static mut instance: ::protobuf::lazy::Lazy<User> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const User,
        };
        unsafe {
            instance.get(|| {
                User {
                    id: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    // optional string name = 2;

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
        };
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
}

impl ::protobuf::Message for User {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<User>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for User {
    fn new() -> User {
        User::new()
    }

    fn descriptor_static(_: ::std::option::Option<User>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "id",
                    User::has_id,
                    User::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    User::has_name,
                    User::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<User>(
                    "User",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for User {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for User {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct List {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    items: ::protobuf::RepeatedField<ListItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for List {}

impl List {
    pub fn new() -> List {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static List {
        static mut instance: ::protobuf::lazy::Lazy<List> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const List,
        };
        unsafe {
            instance.get(|| {
                List {
                    name: ::protobuf::SingularField::none(),
                    items: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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

    // repeated .ListItem items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<ListItem>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<ListItem> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<ListItem> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[ListItem] {
        &self.items
    }
}

impl ::protobuf::Message for List {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.items {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<List>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for List {
    fn new() -> List {
        List::new()
    }

    fn descriptor_static(_: ::std::option::Option<List>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    List::has_name,
                    List::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "items",
                    List::get_items,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<List>(
                    "List",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for List {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        self.name == other.name &&
        self.items == other.items &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for List {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListItem {
    // message fields
    id: ::std::option::Option<i64>,
    name: ::protobuf::SingularField<::std::string::String>,
    description: ::protobuf::SingularField<::std::string::String>,
    link_annotations: ::protobuf::RepeatedField<LinkAnnotation>,
    text_annotations: ::protobuf::RepeatedField<TextAnnotation>,
    streeteasy_annotations: ::protobuf::RepeatedField<StreetEasyAnnotation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListItem {}

impl ListItem {
    pub fn new() -> ListItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListItem {
        static mut instance: ::protobuf::lazy::Lazy<ListItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListItem,
        };
        unsafe {
            instance.get(|| {
                ListItem {
                    id: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    description: ::protobuf::SingularField::none(),
                    link_annotations: ::protobuf::RepeatedField::new(),
                    text_annotations: ::protobuf::RepeatedField::new(),
                    streeteasy_annotations: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    // optional string name = 2;

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
        };
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

    // optional string description = 3;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .LinkAnnotation link_annotations = 4;

    pub fn clear_link_annotations(&mut self) {
        self.link_annotations.clear();
    }

    // Param is passed by value, moved
    pub fn set_link_annotations(&mut self, v: ::protobuf::RepeatedField<LinkAnnotation>) {
        self.link_annotations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_link_annotations(&mut self) -> &mut ::protobuf::RepeatedField<LinkAnnotation> {
        &mut self.link_annotations
    }

    // Take field
    pub fn take_link_annotations(&mut self) -> ::protobuf::RepeatedField<LinkAnnotation> {
        ::std::mem::replace(&mut self.link_annotations, ::protobuf::RepeatedField::new())
    }

    pub fn get_link_annotations(&self) -> &[LinkAnnotation] {
        &self.link_annotations
    }

    // repeated .TextAnnotation text_annotations = 5;

    pub fn clear_text_annotations(&mut self) {
        self.text_annotations.clear();
    }

    // Param is passed by value, moved
    pub fn set_text_annotations(&mut self, v: ::protobuf::RepeatedField<TextAnnotation>) {
        self.text_annotations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_text_annotations(&mut self) -> &mut ::protobuf::RepeatedField<TextAnnotation> {
        &mut self.text_annotations
    }

    // Take field
    pub fn take_text_annotations(&mut self) -> ::protobuf::RepeatedField<TextAnnotation> {
        ::std::mem::replace(&mut self.text_annotations, ::protobuf::RepeatedField::new())
    }

    pub fn get_text_annotations(&self) -> &[TextAnnotation] {
        &self.text_annotations
    }

    // repeated .StreetEasyAnnotation streeteasy_annotations = 6;

    pub fn clear_streeteasy_annotations(&mut self) {
        self.streeteasy_annotations.clear();
    }

    // Param is passed by value, moved
    pub fn set_streeteasy_annotations(&mut self, v: ::protobuf::RepeatedField<StreetEasyAnnotation>) {
        self.streeteasy_annotations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_streeteasy_annotations(&mut self) -> &mut ::protobuf::RepeatedField<StreetEasyAnnotation> {
        &mut self.streeteasy_annotations
    }

    // Take field
    pub fn take_streeteasy_annotations(&mut self) -> ::protobuf::RepeatedField<StreetEasyAnnotation> {
        ::std::mem::replace(&mut self.streeteasy_annotations, ::protobuf::RepeatedField::new())
    }

    pub fn get_streeteasy_annotations(&self) -> &[StreetEasyAnnotation] {
        &self.streeteasy_annotations
    }
}

impl ::protobuf::Message for ListItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.link_annotations));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.text_annotations));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.streeteasy_annotations));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.description {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.link_annotations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.text_annotations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.streeteasy_annotations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(3, &v));
        };
        for v in &self.link_annotations {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.text_annotations {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.streeteasy_annotations {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ListItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListItem {
    fn new() -> ListItem {
        ListItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "id",
                    ListItem::has_id,
                    ListItem::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    ListItem::has_name,
                    ListItem::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    ListItem::has_description,
                    ListItem::get_description,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "link_annotations",
                    ListItem::get_link_annotations,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "text_annotations",
                    ListItem::get_text_annotations,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "streeteasy_annotations",
                    ListItem::get_streeteasy_annotations,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListItem>(
                    "ListItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListItem {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_description();
        self.clear_link_annotations();
        self.clear_text_annotations();
        self.clear_streeteasy_annotations();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListItem {
    fn eq(&self, other: &ListItem) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.description == other.description &&
        self.link_annotations == other.link_annotations &&
        self.text_annotations == other.text_annotations &&
        self.streeteasy_annotations == other.streeteasy_annotations &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LinkAnnotation {
    // message fields
    id: ::std::option::Option<i64>,
    url: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LinkAnnotation {}

impl LinkAnnotation {
    pub fn new() -> LinkAnnotation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LinkAnnotation {
        static mut instance: ::protobuf::lazy::Lazy<LinkAnnotation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LinkAnnotation,
        };
        unsafe {
            instance.get(|| {
                LinkAnnotation {
                    id: ::std::option::Option::None,
                    url: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    // optional string url = 2;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for LinkAnnotation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.url {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.url.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LinkAnnotation>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LinkAnnotation {
    fn new() -> LinkAnnotation {
        LinkAnnotation::new()
    }

    fn descriptor_static(_: ::std::option::Option<LinkAnnotation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "id",
                    LinkAnnotation::has_id,
                    LinkAnnotation::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "url",
                    LinkAnnotation::has_url,
                    LinkAnnotation::get_url,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LinkAnnotation>(
                    "LinkAnnotation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LinkAnnotation {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_url();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LinkAnnotation {
    fn eq(&self, other: &LinkAnnotation) -> bool {
        self.id == other.id &&
        self.url == other.url &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LinkAnnotation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TextAnnotation {
    // message fields
    id: ::std::option::Option<i64>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TextAnnotation {}

impl TextAnnotation {
    pub fn new() -> TextAnnotation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TextAnnotation {
        static mut instance: ::protobuf::lazy::Lazy<TextAnnotation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TextAnnotation,
        };
        unsafe {
            instance.get(|| {
                TextAnnotation {
                    id: ::std::option::Option::None,
                    text: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    // optional string text = 2;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for TextAnnotation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.text.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TextAnnotation>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TextAnnotation {
    fn new() -> TextAnnotation {
        TextAnnotation::new()
    }

    fn descriptor_static(_: ::std::option::Option<TextAnnotation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "id",
                    TextAnnotation::has_id,
                    TextAnnotation::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    TextAnnotation::has_text,
                    TextAnnotation::get_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TextAnnotation>(
                    "TextAnnotation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TextAnnotation {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TextAnnotation {
    fn eq(&self, other: &TextAnnotation) -> bool {
        self.id == other.id &&
        self.text == other.text &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TextAnnotation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StreetEasyAnnotation {
    // message fields
    hash: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    price_usd: ::std::option::Option<i32>,
    open_houses: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StreetEasyAnnotation {}

impl StreetEasyAnnotation {
    pub fn new() -> StreetEasyAnnotation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StreetEasyAnnotation {
        static mut instance: ::protobuf::lazy::Lazy<StreetEasyAnnotation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StreetEasyAnnotation,
        };
        unsafe {
            instance.get(|| {
                StreetEasyAnnotation {
                    hash: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    price_usd: ::std::option::Option::None,
                    open_houses: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u64) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> u64 {
        self.hash.unwrap_or(0)
    }

    // optional string name = 2;

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
        };
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

    // optional int32 price_usd = 3;

    pub fn clear_price_usd(&mut self) {
        self.price_usd = ::std::option::Option::None;
    }

    pub fn has_price_usd(&self) -> bool {
        self.price_usd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_price_usd(&mut self, v: i32) {
        self.price_usd = ::std::option::Option::Some(v);
    }

    pub fn get_price_usd(&self) -> i32 {
        self.price_usd.unwrap_or(0)
    }

    // repeated string open_houses = 4;

    pub fn clear_open_houses(&mut self) {
        self.open_houses.clear();
    }

    // Param is passed by value, moved
    pub fn set_open_houses(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.open_houses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_open_houses(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.open_houses
    }

    // Take field
    pub fn take_open_houses(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.open_houses, ::protobuf::RepeatedField::new())
    }

    pub fn get_open_houses(&self) -> &[::std::string::String] {
        &self.open_houses
    }
}

impl ::protobuf::Message for StreetEasyAnnotation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.hash = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.price_usd = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.open_houses));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.hash {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.price_usd {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.open_houses {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.price_usd {
            try!(os.write_int32(3, v));
        };
        for v in &self.open_houses {
            try!(os.write_string(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StreetEasyAnnotation>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StreetEasyAnnotation {
    fn new() -> StreetEasyAnnotation {
        StreetEasyAnnotation::new()
    }

    fn descriptor_static(_: ::std::option::Option<StreetEasyAnnotation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "hash",
                    StreetEasyAnnotation::has_hash,
                    StreetEasyAnnotation::get_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    StreetEasyAnnotation::has_name,
                    StreetEasyAnnotation::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "price_usd",
                    StreetEasyAnnotation::has_price_usd,
                    StreetEasyAnnotation::get_price_usd,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "open_houses",
                    StreetEasyAnnotation::get_open_houses,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StreetEasyAnnotation>(
                    "StreetEasyAnnotation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StreetEasyAnnotation {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_name();
        self.clear_price_usd();
        self.clear_open_houses();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StreetEasyAnnotation {
    fn eq(&self, other: &StreetEasyAnnotation) -> bool {
        self.hash == other.hash &&
        self.name == other.name &&
        self.price_usd == other.price_usd &&
        self.open_houses == other.open_houses &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StreetEasyAnnotation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x61, 0x70, 0x69, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2a, 0x0a, 0x04, 0x55,
    0x73, 0x65, 0x72, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52,
    0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x3b, 0x0a, 0x04, 0x4c, 0x69, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x05, 0x69,
    0x74, 0x65, 0x6d, 0x73, 0x22, 0x96, 0x02, 0x0a, 0x08, 0x4c, 0x69, 0x73, 0x74, 0x49, 0x74, 0x65,
    0x6d, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x02, 0x69,
    0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x3a, 0x0a, 0x10, 0x6c, 0x69, 0x6e, 0x6b, 0x5f,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x0f, 0x2e, 0x4c, 0x69, 0x6e, 0x6b, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x0f, 0x6c, 0x69, 0x6e, 0x6b, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x12, 0x3a, 0x0a, 0x10, 0x74, 0x65, 0x78, 0x74, 0x5f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e,
    0x54, 0x65, 0x78, 0x74, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0f,
    0x74, 0x65, 0x78, 0x74, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12,
    0x4c, 0x0a, 0x16, 0x73, 0x74, 0x72, 0x65, 0x65, 0x74, 0x65, 0x61, 0x73, 0x79, 0x5f, 0x61, 0x6e,
    0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x15, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x65, 0x74, 0x45, 0x61, 0x73, 0x79, 0x41, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x15, 0x73, 0x74, 0x72, 0x65, 0x65, 0x74, 0x65, 0x61,
    0x73, 0x79, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0x32, 0x0a,
    0x0e, 0x4c, 0x69, 0x6e, 0x6b, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x02, 0x69, 0x64, 0x12,
    0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72,
    0x6c, 0x22, 0x34, 0x0a, 0x0e, 0x54, 0x65, 0x78, 0x74, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52,
    0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x22, 0x7c, 0x0a, 0x14, 0x53, 0x74, 0x72, 0x65, 0x65,
    0x74, 0x45, 0x61, 0x73, 0x79, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x68,
    0x61, 0x73, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x70, 0x72, 0x69, 0x63, 0x65,
    0x5f, 0x75, 0x73, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x70, 0x72, 0x69, 0x63,
    0x65, 0x55, 0x73, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x70, 0x65, 0x6e, 0x5f, 0x68, 0x6f, 0x75,
    0x73, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0a, 0x6f, 0x70, 0x65, 0x6e, 0x48,
    0x6f, 0x75, 0x73, 0x65, 0x73, 0x4a, 0xa1, 0x0b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x25, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02,
    0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x03, 0x02, 0x02, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x04, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x04, 0x02, 0x03, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x09, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x10, 0x11, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x07, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x07, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x08, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x02,
    0x07, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x09, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x09, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x09, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x09, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x1c,
    0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0c, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x0d, 0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x0d, 0x02, 0x0c, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x0d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x08,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x0d, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0e, 0x02, 0x0d, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0e, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0e, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f,
    0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0f, 0x02, 0x0e,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x09, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x11, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x11, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x11,
    0x1a, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x11, 0x2d, 0x2e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x12, 0x02, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x12, 0x1a, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x12, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x13,
    0x02, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x06, 0x12, 0x03, 0x13, 0x0b, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x13, 0x20, 0x36, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x13, 0x39, 0x3a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x16, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x16,
    0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x17, 0x02, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x17, 0x02, 0x16, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x08, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x18, 0x02, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x18, 0x02, 0x17, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x09, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x18, 0x0f, 0x10, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1b, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x1b, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03,
    0x1c, 0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1c, 0x02,
    0x1b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x07,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x1d, 0x02, 0x1c, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x1d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1d, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d,
    0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x20, 0x00, 0x25, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x20, 0x08, 0x1c, 0x0a, 0x23, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x12, 0x22, 0x16, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x3a, 0x20,
    0x4d, 0x61, 0x6b, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x22, 0x69, 0x64, 0x22, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x21, 0x02, 0x20, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01,
    0x12, 0x03, 0x22, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x22, 0x02, 0x21, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x22,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x09, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x10, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x23, 0x02, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0x23, 0x02, 0x22, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x23, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x23, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x23, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x24, 0x02,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x24, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x24, 0x20, 0x21, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];

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
