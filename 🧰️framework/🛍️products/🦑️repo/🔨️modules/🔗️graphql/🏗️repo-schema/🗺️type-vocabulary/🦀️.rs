//! 🗺️ Type vocabulary of the repo GraphQL schema: the type reference, argument, field, object, interface, union, enum, input and named types, the schema and its scalars. See [`super`].

use super::*;

//#region 📜️Schema
/// 🗺️ A type reference in a field, argument or input-field position. Mirrors the reference
/// implementation's `NonNull`/`List` wrappers around a named type, except that the name is
/// resolved against [`Schema::types`] instead of a pointer, so the graph may be cyclic without a
/// thunk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeRef {
    /// 🏷️ A named type, resolved against the schema's type table.
    Named(String),
    /// ❗️ A wrapper asserting the value is never null.
    NonNull(Box<TypeRef>),
    /// 📃️ A wrapper for a homogeneous sequence.
    List(Box<TypeRef>),
}

impl TypeRef {
    /// 🏷️ The innermost named type, ignoring every wrapper.
    pub fn type_name(&self) -> &str {
        match self {
            TypeRef::Named(name) => name,
            TypeRef::NonNull(inner) | TypeRef::List(inner) => inner.type_name(),
        }
    }

    /// 📜️ The SDL rendering of this reference.
    pub fn render(&self) -> String {
        match self {
            TypeRef::Named(name) => name.clone(),
            TypeRef::NonNull(inner) => format!("{}!", inner.render()),
            TypeRef::List(inner) => format!("[{}]", inner.render()),
        }
    }
}

/// 🏷️ A named type reference.
pub(crate) fn ty(name: &str) -> TypeRef {
    TypeRef::Named(name.to_string())
}

/// ❗️ Wraps a reference as non-null.
pub(super) fn nn(inner: TypeRef) -> TypeRef {
    TypeRef::NonNull(Box::new(inner))
}

/// 📃️ Wraps a reference as a list.
pub(super) fn lst(inner: TypeRef) -> TypeRef {
    TypeRef::List(Box::new(inner))
}

/// 📃️ The `[T!]!` shape every collection field of this schema uses.
pub(super) fn nn_list(inner: TypeRef) -> TypeRef {
    nn(lst(nn(inner)))
}

/// 🎚️ One declared field argument and its default.
#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    /// 🏷️ The argument name.
    pub name: String,
    /// 🗺️ The declared argument type.
    pub ty: TypeRef,
    /// 🎁️ The declared default, `Json::Null` when there is none.
    pub default: Json,
}

/// 🧩️ One selectable field of an object or interface type.
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    /// 🏷️ The field name.
    pub name: String,
    /// 🗺️ The declared field type.
    pub ty: TypeRef,
    /// 🎚️ The declared arguments, in declaration order.
    pub args: Vec<Argument>,
    /// 🔑️ The resolver key, or `None` when the field is read off the source.
    pub resolver: Option<String>,
}

impl Field {
    /// 🧩️ A field resolved by reading the source object.
    pub(crate) fn new(name: &str, ty: TypeRef) -> Field {
        Field { name: name.to_string(), ty, args: Vec::new(), resolver: None }
    }

    /// 🔑️ A field resolved by the named resolver.
    pub(super) fn resolved(name: &str, ty: TypeRef, resolver: &str) -> Field {
        Field { name: name.to_string(), ty, args: Vec::new(), resolver: Some(resolver.to_string()) }
    }

    /// 🎚️ Declares one argument without a default.
    pub(super) fn arg(mut self, name: &str, ty: TypeRef) -> Field {
        self.args.push(Argument { name: name.to_string(), ty, default: Json::Null });
        self
    }
}

/// 🏛️ A composite type with a field table.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🎭️ The interfaces this object declares.
    pub interfaces: Vec<String>,
    /// 🧩️ The field table.
    pub fields: Vec<Field>,
}

impl ObjectType {
    /// 🧩️ Looks one field up by name.
    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name == name)
    }
}

/// 🎭️ An abstract type resolved to a concrete object at execution time.
#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🧩️ The field table every implementor carries.
    pub fields: Vec<Field>,
}

/// 🎭️ A closed set of object types resolved at execution time.
#[derive(Clone, Debug, PartialEq)]
pub struct UnionType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🏛️ The member object type names.
    pub types: Vec<String>,
}

/// 🔢️ A closed set of named constants and the wire value each stands for.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🔢️ Member name to wire value, in declaration order.
    pub values: Vec<(String, String)>,
}

/// 📥️ A composite type accepted as an argument.
#[derive(Clone, Debug, PartialEq)]
pub struct InputObjectType {
    /// 🏷️ The type name.
    pub name: String,
    /// 📥️ The input field table, in declaration order.
    pub fields: Vec<(String, TypeRef)>,
}

/// 📜️ Anything nameable in the owned schema vocabulary.
#[derive(Clone, Debug, PartialEq)]
pub enum NamedType {
    /// 🔤️ A leaf type serialised straight into the result.
    Scalar(String),
    /// 🏛️ A composite type with a field table.
    Object(ObjectType),
    /// 🎭️ An abstract type with a field table.
    Interface(InterfaceType),
    /// 🎭️ A closed set of object types.
    Union(UnionType),
    /// 🔢️ A closed set of named constants.
    Enum(EnumType),
    /// 📥️ A composite argument type.
    InputObject(InputObjectType),
}

impl NamedType {
    /// 🏷️ The name this type is registered under.
    pub fn name(&self) -> &str {
        match self {
            NamedType::Scalar(name) => name,
            NamedType::Object(object) => &object.name,
            NamedType::Interface(interface) => &interface.name,
            NamedType::Union(union) => &union.name,
            NamedType::Enum(enumeration) => &enumeration.name,
            NamedType::InputObject(input) => &input.name,
        }
    }
}

/// 🗺️ The executable schema: a type table plus the operation roots.
#[derive(Clone, Debug, PartialEq)]
pub struct Schema {
    /// 📚️ Every declared type, by name.
    pub types: BTreeMap<String, NamedType>,
    /// 🔍️ The query root type name.
    pub query: String,
    /// ✍️ The mutation root type name, when one is configured.
    pub mutation: Option<String>,
}

impl Schema {
    /// 🏛️ Looks one object type up by name.
    pub fn object(&self, name: &str) -> Option<&ObjectType> {
        match self.types.get(name) {
            Some(NamedType::Object(object)) => Some(object),
            _ => None,
        }
    }
}

/// 🔤️ The five scalar names the repo schema is built from.
pub(super) const SCALARS: &[&str] = &["String", "Int", "Boolean", "ID", "DateTime"];
//#endregion 📜️Schema
