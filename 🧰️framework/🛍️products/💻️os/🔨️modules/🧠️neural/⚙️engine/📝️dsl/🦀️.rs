//! 📝️ The untagged neural vocabulary is the dynamic JSON-equivalent literal of the DSL: an object is a dictionary, any other scalar an atom.
use super::*;
use semio_framework_dsl_record::{BorrowedDslField,BorrowedShape,DslField,FieldValue,NativeSchemaControl,Shape,native_encoding::{FieldProjectionView,projection_path_error}};
use semio_framework_value::{NativeEncodeControl,ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

enum Node<'a>{Dictionary(&'a Dictionary),Atom(&'a Atom)}

fn descend<'a>(root:&'a Dictionary,path:&[usize])->Result<Node<'a>,ValueError>{
    let mut node=Node::Dictionary(root);
    for (position,rank) in path.iter().enumerate() {
        let Node::Dictionary(dictionary)=node else {return Err(projection_path_error())};
        node=match dictionary.entry_at_rank(*rank).ok_or_else(projection_path_error)?.1 {
            Value::Dictionary(next)=>Node::Dictionary(next),
            Value::Atom(atom) if position+1==path.len()=>Node::Atom(atom),
            Value::Atom(_)=>return Err(projection_path_error()),
        };
    }
    Ok(node)
}

fn value_node<'a>(value:&'a Value,path:&[usize])->Result<Node<'a>,ValueError>{
    match value {Value::Dictionary(dictionary)=>descend(dictionary,path),Value::Atom(atom) if path.is_empty()=>Ok(Node::Atom(atom)),Value::Atom(_)=>Err(projection_path_error())}
}

fn node_view(node:Node<'_>)->FieldProjectionView<'_>{
    match node {
        Node::Dictionary(dictionary)=>FieldProjectionView::IntrinsicObject(dictionary.len()),
        Node::Atom(Atom::Null)=>FieldProjectionView::IntrinsicNull,
        Node::Atom(Atom::Boolean(value))=>FieldProjectionView::IntrinsicBool(*value),
        Node::Atom(Atom::Integer(value))=>FieldProjectionView::IntrinsicNumber(Number::Int(*value)),
        Node::Atom(Atom::Decimal(value))=>FieldProjectionView::IntrinsicNumber(Number::Float(*value)),
        Node::Atom(Atom::String(value))=>FieldProjectionView::IntrinsicText(value),
    }
}

fn node_key<'a>(node:Node<'a>,index:usize)->Result<&'a str,ValueError>{
    match node {Node::Dictionary(dictionary)=>dictionary.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(projection_path_error),Node::Atom(_)=>Err(projection_path_error())}
}

fn literal(value:&FieldValue)->Result<&DslValue,ValueError>{
    match value {FieldValue::Value(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected a dynamic literal"))}
}

macro_rules! dynamic_literal {
    ($owner:ty,$label:literal,$identity:ident,$view:ident,$key:ident) => {
        impl BorrowedDslField for $owner {const SHAPE:BorrowedShape=BorrowedShape::Value;}
        impl DslField for $owner {
            fn projection_identity(&self)->usize{$identity(self)}
            fn projection_view(&self,path:&[usize])->Result<FieldProjectionView<'_>,ValueError>{$view(self,path)}
            fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{$key(self,path,index)}
            fn shape()->Shape{Shape::Value}
            fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Value)}
            fn to_value(&self)->FieldValue{FieldValue::Value(ToValue::to_value(self))}
            fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{ToValue::to_value_controlled(self,control).map(FieldValue::Value)}
            fn from_value(value:&FieldValue)->Result<Self,String>{
                match value {FieldValue::Value(value)=>FromValue::from_value(value.clone()).map_err(|error|format!("expected {}: {error:?}",$label)),other=>Err(format!("expected {}, found {other:?}",$label))}
            }
            fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{FromValue::from_value_controlled(literal(value)?,control)}
            fn retire_decoded(self){FromValue::retire_decoded(self)}
        }
    };
}

fn address<T>(owner:&T)->usize{owner as *const T as *const () as usize}
/// 🪪️ A moved dictionary keeps the identity of its shared original root; an empty one falls back to its own address.
fn dictionary_identity(dictionary:&Dictionary)->usize{dictionary.original_root_identity().unwrap_or_else(||address(dictionary))}
fn value_identity(value:&Value)->usize{match value {Value::Dictionary(dictionary)=>dictionary_identity(dictionary),Value::Atom(_)=>address(value)}}

fn atom_view<'a>(atom:&'a Atom,path:&[usize])->Result<FieldProjectionView<'a>,ValueError>{if path.is_empty(){Ok(node_view(Node::Atom(atom)))}else{Err(projection_path_error())}}
fn atom_key<'a>(_:&'a Atom,_:&[usize],_:usize)->Result<&'a str,ValueError>{Err(projection_path_error())}
fn dictionary_view<'a>(dictionary:&'a Dictionary,path:&[usize])->Result<FieldProjectionView<'a>,ValueError>{descend(dictionary,path).map(node_view)}
fn dictionary_key<'a>(dictionary:&'a Dictionary,path:&[usize],index:usize)->Result<&'a str,ValueError>{descend(dictionary,path).and_then(|node|node_key(node,index))}
fn value_view<'a>(value:&'a Value,path:&[usize])->Result<FieldProjectionView<'a>,ValueError>{value_node(value,path).map(node_view)}
fn value_key<'a>(value:&'a Value,path:&[usize],index:usize)->Result<&'a str,ValueError>{value_node(value,path).and_then(|node|node_key(node,index))}

dynamic_literal!(Atom,"an atom",address,atom_view,atom_key);
dynamic_literal!(Dictionary,"a dictionary",dictionary_identity,dictionary_view,dictionary_key);
dynamic_literal!(Value,"a value",value_identity,value_view,value_key);
