package graphql

// #region 📜️Schema

/** 🗺️ How a type reference wraps the type it names. */
type TypeRefKind int

const (
	// 🏷️ A named type, resolved against the schema's type table.
	TypeRefNamed TypeRefKind = iota
	// ❗️ A wrapper asserting the value is never null.
	TypeRefNonNull
	// 📃️ A wrapper for a homogeneous sequence.
	TypeRefList
)

/** 🗺️ A type reference in a field, argument or input-field position. The name is resolved against
 * [Schema.Types] instead of a pointer, so the graph may be cyclic without a thunk. */
type TypeRef struct {
	Kind TypeRefKind
	Name string
	Of   *TypeRef
}

/** 🏷️ A named type reference. */
func Ty(name string) TypeRef { return TypeRef{Kind: TypeRefNamed, Name: name} }

/** ❗️ Wraps a reference as non-null. */
func NN(inner TypeRef) TypeRef { return TypeRef{Kind: TypeRefNonNull, Of: &inner} }

/** 📃️ Wraps a reference as a list. */
func Lst(inner TypeRef) TypeRef { return TypeRef{Kind: TypeRefList, Of: &inner} }

/** 📃️ The `[T!]!` shape every collection field of this schema uses. */
func NNList(inner TypeRef) TypeRef { return NN(Lst(NN(inner))) }

/** 🏷️ The innermost named type, ignoring every wrapper. */
func (reference TypeRef) TypeName() string {
	if reference.Kind == TypeRefNamed {
		return reference.Name
	}
	return reference.Of.TypeName()
}

/** 📜️ The SDL rendering of this reference. */
func (reference TypeRef) Render() string {
	switch reference.Kind {
	case TypeRefNonNull:
		return reference.Of.Render() + "!"
	case TypeRefList:
		return "[" + reference.Of.Render() + "]"
	default:
		return reference.Name
	}
}

/** 🎚️ One declared field argument and its default. */
type Argument struct {
	Name    string
	Type    TypeRef
	Default interface{}
}

/** 🧩️ One selectable field of an object or interface type. */
type Field struct {
	Name     string
	Type     TypeRef
	Args     []Argument
	Resolver string
}

/** 🧩️ A field resolved by reading the source object. */
func NewField(name string, kind TypeRef) Field { return Field{Name: name, Type: kind} }

/** 🔑️ A field resolved by the named resolver. */
func ResolvedField(name string, kind TypeRef, resolver string) Field {
	return Field{Name: name, Type: kind, Resolver: resolver}
}

/** 🎚️ Declares one argument without a default. */
func (field Field) Arg(name string, kind TypeRef) Field {
	field.Args = append(append([]Argument{}, field.Args...), Argument{Name: name, Type: kind})
	return field
}

/** 📜️ Anything nameable in the owned schema vocabulary. */
type NamedType interface{ TypeName() string }

/** 🔤️ A leaf type serialised straight into the result. */
type ScalarType struct{ Name string }

func (declared *ScalarType) TypeName() string { return declared.Name }

/** 🏛️ A composite type with a field table. */
type ObjectType struct {
	Name       string
	Interfaces []string
	Fields     []Field
}

func (declared *ObjectType) TypeName() string { return declared.Name }

/** 🧩️ Looks one field up by name. */
func (declared *ObjectType) Field(name string) *Field {
	for index := range declared.Fields {
		if declared.Fields[index].Name == name {
			return &declared.Fields[index]
		}
	}
	return nil
}

/** 🎭️ An abstract type resolved to a concrete object at execution time. */
type InterfaceType struct {
	Name   string
	Fields []Field
}

func (declared *InterfaceType) TypeName() string { return declared.Name }

/** 🎭️ A closed set of object types resolved at execution time. */
type UnionType struct {
	Name  string
	Types []string
}

func (declared *UnionType) TypeName() string { return declared.Name }

/** 🔢️ One member of an enum type and the wire value it stands for. */
type EnumValue struct {
	Name string
	Wire string
}

/** 🔢️ A closed set of named constants. */
type EnumType struct {
	Name   string
	Values []EnumValue
}

func (declared *EnumType) TypeName() string { return declared.Name }

/** 📥️ One field of an input object type. */
type InputField struct {
	Name string
	Type TypeRef
}

/** 📥️ A composite type accepted as an argument. */
type InputObjectType struct {
	Name   string
	Fields []InputField
}

func (declared *InputObjectType) TypeName() string { return declared.Name }

/** 🗺️ The executable schema: a type table plus the operation roots. */
type Schema struct {
	Types    map[string]NamedType
	Query    string
	Mutation string
}

/** 🏛️ Looks one object type up by name. */
func (schema Schema) Object(name string) *ObjectType {
	if object, ok := schema.Types[name].(*ObjectType); ok {
		return object
	}
	return nil
}

/** 🔤️ The five scalar names the repo schema is built from. */
var Scalars = []string{"String", "Int", "Boolean", "ID", "DateTime"}

// #endregion 📜️Schema
