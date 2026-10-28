//! ⚖️ Catalog-owned declaration domains and bounded changed-subtree admission.
use super::{IfcCatalog, IfcTypeExpression};

pub struct TypedView<'a,V>{pub name:&'a str,pub items:&'a[V]}
pub enum DomainView<'a,V>{Unset,Derived,Integer,Real,Reference(u64),String(&'a str),Enum(&'a str),Binary(&'a str),Aggregate(&'a[V]),TypedValue(TypedView<'a,V>)}
pub enum NumberView<'a>{Integer(i64),Binary64(f64),Decimal{coefficient:&'a str,negative:bool,scale:u32,exponent:Option<i32>}}
pub trait DomainValue:Sized{fn number(&self)->Option<NumberView<'_>>;fn view(&self)->DomainView<'_,Self>;fn numeric_valid(&self,tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>;fn numbers_equal(&self,other:&Self,tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>;fn numbers_identical(&self,other:&Self,tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>;}
pub struct DomainControl<'a> {
    pub tick: &'a mut dyn FnMut() -> Result<(), String>,
    pub reference: &'a mut dyn FnMut(u64, &str, &mut dyn FnMut() -> Result<(), String>) -> Result<bool, String>,
}
#[derive(Clone, Copy, PartialEq)]
pub enum ValueChange { Set, Insert, Remove, Move }
fn refusal(code: &str) -> String { format!("ifc-domain-{code}") }
fn step(control: &mut DomainControl<'_>, depth: usize) -> Result<(), String> { (control.tick)()?; if depth > 128 { return Err(refusal("depth")); } Ok(()) }
fn width(expression: &IfcTypeExpression, length: usize) -> bool { expression.width.is_none_or(|limit| if expression.fixed { length as i128 == limit as i128 } else { length as i128 <= limit as i128 }) }
fn named(name: &str) -> IfcTypeExpression { IfcTypeExpression { kind: "named".into(), name: Some(name.to_ascii_uppercase()), ..Default::default() } }
fn resolved<'a>(catalog: &'a IfcCatalog, expression: &'a IfcTypeExpression) -> &'a IfcTypeExpression { let mut expression = expression; for _ in 0..128 { if expression.kind != "named" { break; } let Some(declaration) = expression.name.as_ref().and_then(|name| catalog.types.get(&name.to_ascii_uppercase())) else { break; }; if declaration.kind != "defined" { break; } let Some(inner) = declaration.type_expr.as_ref() else { break; }; expression = inner; } expression }
pub fn bounds(expression: &IfcTypeExpression, length: usize) -> (i128, i128) { let length = length as i128; if expression.aggregate.as_deref() == Some("ARRAY") { let (Some(lower), Some(upper)) = (expression.lower, expression.upper) else { return (0, 0); }; let size = upper as i128 - lower as i128 + 1; return ((size - length).max(0), (length - size).max(0)); } (((expression.lower.unwrap_or(0) as i128) - length).max(0), expression.upper.map_or(0, |upper| (length - upper as i128).max(0))) }
fn target(catalog: &IfcCatalog, name: &str, id: u64, control: &mut DomainControl<'_>, depth: usize) -> Result<bool, String> { step(control, depth)?; if id == 0 { return Ok(false); } if catalog.entities.contains_key(name) { return (control.reference)(id, name, control.tick); } if let Some(declaration) = catalog.types.get(name).filter(|declaration| declaration.kind == "select") { for member in &declaration.members { if target(catalog, member, id, control, depth + 1)? { return Ok(true); } } } Ok(false) }
pub fn accepts(catalog: &IfcCatalog, expected: &str, actual: &str, depth: usize) -> bool { if depth > 128 { return false; } if expected == actual { return catalog.entities.contains_key(actual) || catalog.types.contains_key(actual); } if catalog.is_subtype(actual, expected) { return true; } catalog.types.get(expected).filter(|declaration| declaration.kind == "select").is_some_and(|declaration| declaration.members.iter().any(|member| accepts(catalog, member, actual, depth + 1))) }
fn text_equal(left: &str, right: &str, insensitive: bool, control: &mut DomainControl<'_>) -> Result<bool, String> { if left.len() != right.len() { return Ok(false); } for (a, b) in left.bytes().zip(right.bytes()) { (control.tick)()?; if if insensitive { !a.eq_ignore_ascii_case(&b) } else { a != b } { return Ok(false); } } Ok(true) }
/// ⚖️ Compare values using exact integer/REAL semantics and declared aggregate order.
pub fn equal<V:DomainValue>(catalog: &IfcCatalog, expression: &IfcTypeExpression, left: &V, right: &V, control: &mut DomainControl<'_>) -> Result<bool, String> { equal_inner(catalog, expression, left, right, control, 0) }
fn equal_inner<V:DomainValue>(catalog: &IfcCatalog, expression: &IfcTypeExpression, left: &V, right: &V, control: &mut DomainControl<'_>, depth: usize) -> Result<bool, String> {
    step(control, depth)?;
    if let DomainView::TypedValue(typed) = left.view() { if typed.items.len() == 1 { return equal_inner(catalog, &named(&typed.name), &typed.items[0], right, control, depth + 1); } }
    if let DomainView::TypedValue(typed) = right.view() { if typed.items.len() == 1 { return equal_inner(catalog, expression, left, &typed.items[0], control, depth + 1); } }
    let expression = resolved(catalog, expression);
    if matches!(left.view(),DomainView::Integer|DomainView::Real)&&matches!(right.view(),DomainView::Integer|DomainView::Real){return left.numbers_equal(right,control.tick);}
    Ok(match (left.view(), right.view()) {
        (DomainView::Reference(a), DomainView::Reference(b)) => a == b,
        (DomainView::String(a), DomainView::String(b)) | (DomainView::Binary(a), DomainView::Binary(b)) => text_equal(a, b, false, control)?,
        (DomainView::Enum(a), DomainView::Enum(b)) => text_equal(a, b, true, control)?,
        (DomainView::Aggregate(a), DomainView::Aggregate(b)) => {
            if a.len() != b.len() { return Ok(false); }
            let fallback = named("NUMBER"); let item = expression.item.as_deref().unwrap_or(&fallback);
            if !matches!(expression.aggregate.as_deref(), Some("SET" | "BAG")) { for (a, b) in a.iter().zip(b) { if !equal_inner(catalog, item, a, b, control, depth + 1)? { return Ok(false); } } return Ok(true); }
            for (index, value) in a.iter().enumerate() { let mut seen = false; for previous in &a[..index] { if equal_inner(catalog, item, value, previous, control, depth + 1)? { seen = true; break; } } if seen { continue; } let mut left_count = 0; let mut right_count = 0; for other in a { if equal_inner(catalog, item, value, other, control, depth + 1)? { left_count += 1; } } for other in b { if equal_inner(catalog, item, value, other, control, depth + 1)? { right_count += 1; } } if left_count == 0 || left_count != right_count { return Ok(false); } } true
        }
        _ => false,
    })
}
fn unique<V:DomainValue>(catalog: &IfcCatalog, expression: &IfcTypeExpression, values: &[V], control: &mut DomainControl<'_>) -> Result<(), String> { if !expression.unique && expression.aggregate.as_deref() != Some("SET") { return Ok(()); } let item = expression.item.as_deref().ok_or_else(|| refusal("aggregate-item"))?; for (index, value) in values.iter().enumerate() { for previous in &values[..index] { if equal(catalog, item, value, previous, control)? { return Err(refusal("unique")); } } } Ok(()) }
/// 🛂️ Validate one declared domain with caller-owned reference lookup and progress.
pub fn check_value<V:DomainValue>(catalog: &IfcCatalog, expression: &IfcTypeExpression, value: &V, optional: bool, constraints: bool, control: &mut DomainControl<'_>) -> Result<(), String> { validate(catalog, expression, value, optional, constraints, control, 0) }
fn validate<V:DomainValue>(catalog: &IfcCatalog, expression: &IfcTypeExpression, value: &V, optional: bool, constraints: bool, control: &mut DomainControl<'_>, depth: usize) -> Result<(), String> {
    step(control, depth)?;
    if matches!(value.view(), DomainView::Unset) { return if optional || expression.kind != "aggregate" && expression.optional { Ok(()) } else { Err(refusal("required")) }; }
    if matches!(value.view(), DomainView::Derived) { return Err(refusal("derived")); }
    if expression.kind == "aggregate" { let DomainView::Aggregate(items) = value.view() else { return Err(refusal("aggregate")); }; if constraints && bounds(expression, items.len()) != (0, 0) { return Err(refusal("bounds")); } let inner = expression.item.as_deref().ok_or_else(|| refusal("aggregate-item"))?; for item in items { validate(catalog, inner, item, expression.optional, constraints, control, depth + 1)?; } if constraints { unique(catalog, expression, items, control)?; } return Ok(()); }
    let name = expression.name.as_deref().unwrap_or("").to_ascii_uppercase(); let declaration = catalog.types.get(&name);
    if catalog.entities.contains_key(&name) || declaration.is_some_and(|declaration| declaration.kind == "select") && matches!(value.view(), DomainView::Reference(_)) { let DomainView::Reference(id) = value.view() else { return Err(refusal("reference")); }; return if target(catalog, &name, id, control, depth + 1)? { Ok(()) } else { Err(refusal("reference")) }; }
    if let Some(declaration) = declaration {
        if declaration.kind == "enum" { if let DomainView::Enum(literal) = value.view() { for member in &declaration.members { if text_equal(literal, member, true, control)? { return Ok(()); } } } return Err(refusal("enum")); }
        if declaration.kind == "select" { let DomainView::TypedValue(typed) = value.view() else { return Err(refusal("select")); }; if typed.items.len() != 1 || !accepts(catalog, &name, &typed.name.to_ascii_uppercase(), 0) { return Err(refusal("select")); } return validate(catalog, &named(&typed.name), &typed.items[0], false, constraints, control, depth + 1); }
        if let Some(inner) = &declaration.type_expr { let value = if let DomainView::TypedValue(typed) = value.view() { if typed.items.len() != 1 || !typed.name.eq_ignore_ascii_case(&name) { return Err(refusal("constructor")); } &typed.items[0] } else { value }; return validate(catalog, inner, value, false, constraints, control, depth + 1); }
    }
    match (name.as_str(), value.view()) {
        ("STRING", DomainView::String(text)) => { let mut count = 0; for _ in text.chars() { (control.tick)()?; count += 1; } if width(expression, count) { Ok(()) } else { Err(refusal("width")) } }
        ("INTEGER" | "NUMBER", DomainView::Integer) => if value.numeric_valid(control.tick)?{Ok(())}else{Err(refusal("integer"))},
        ("REAL" | "NUMBER", DomainView::Real) if value.numeric_valid(control.tick)? => Ok(()),
        ("BINARY", DomainView::Binary(text)) => { for _ in text.bytes() { (control.tick)()?; } if !semio_s_artifact_stdio_contract::part21::is_part21_binary(text) { return Err(refusal("binary")); } let unused = text.as_bytes()[0] - b'0'; let length = (text.len() - 1) * 4 - unused as usize; if width(expression, length) { Ok(()) } else { Err(refusal("width")) } }
        ("BOOLEAN" | "LOGICAL", DomainView::Enum(literal)) if ["T", "F"].iter().any(|member| literal.eq_ignore_ascii_case(member)) || name == "LOGICAL" && literal.eq_ignore_ascii_case("U") => Ok(()),
        _ => Err(refusal("type")),
    }
}
fn same<V:DomainValue>(left:&V,right:&V,control:&mut DomainControl<'_>,depth:usize)->Result<bool,String>{step(control,depth)?;Ok(match(left.view(),right.view()){
(DomainView::Unset,DomainView::Unset)|(DomainView::Derived,DomainView::Derived)=>true,
(DomainView::Integer,DomainView::Integer)|(DomainView::Real,DomainView::Real)=>left.numbers_identical(right,control.tick)?,
(DomainView::Reference(a),DomainView::Reference(b))=>a==b,
(DomainView::String(a),DomainView::String(b))|(DomainView::Enum(a),DomainView::Enum(b))|(DomainView::Binary(a),DomainView::Binary(b))=>text_equal(a,b,false,control)?,
(DomainView::Aggregate(a),DomainView::Aggregate(b))=>{if a.len()!=b.len(){return Ok(false);}for(a,b)in a.iter().zip(b){if !same(a,b,control,depth+1)?{return Ok(false);}}true},
(DomainView::TypedValue(a),DomainView::TypedValue(b))=>{if !text_equal(a.name,b.name,false,control)?||a.items.len()!=b.items.len(){return Ok(false);}for(a,b)in a.items.iter().zip(b.items){if !same(a,b,control,depth+1)?{return Ok(false);}}true},_=>false})}
/// 🪡️ Validate changed values and non-worsening bounds/uniqueness at their ancestors.
pub fn check_change<V:DomainValue>(catalog: &IfcCatalog, expression: &IfcTypeExpression, before: &V, after: &V, route: &[usize], operation: ValueChange, optional: bool, control: &mut DomainControl<'_>) -> Result<(), String> { change(catalog, expression, before, after, route, operation, optional, control, 0) }
fn change<V:DomainValue>(catalog: &IfcCatalog, expression: &IfcTypeExpression, before: &V, after: &V, route: &[usize], operation: ValueChange, optional: bool, control: &mut DomainControl<'_>, depth: usize) -> Result<(), String> {
    step(control, depth)?; if route.is_empty() { if same(before, after, control, depth + 1)? { return Ok(()); } return validate(catalog, expression, after, optional, true, control, depth + 1); }
    if let DomainView::TypedValue(typed) = after.view() { if typed.items.len() != 1 { return Err(refusal("constructor")); } let domain = resolved(catalog, expression); let name = domain.name.as_deref().unwrap_or("").to_ascii_uppercase(); let valid = if catalog.types.get(&name).is_some_and(|declaration| declaration.kind == "select") { accepts(catalog, &name, &typed.name.to_ascii_uppercase(), 0) } else { typed.name.eq_ignore_ascii_case(expression.name.as_deref().unwrap_or("")) && catalog.types.contains_key(&typed.name.to_ascii_uppercase()) }; if !valid { return Err(refusal("constructor")); } let DomainView::TypedValue(prior) = before.view() else { return validate(catalog, expression, after, optional, true, control, depth + 1); }; if prior.name != typed.name || route[0] != 0 || prior.items.len() != 1 { return validate(catalog, expression, after, optional, true, control, depth + 1); } return change(catalog, &named(&typed.name), &prior.items[0], &typed.items[0], &route[1..], operation, false, control, depth + 1); }
    let expression = resolved(catalog, expression); let (DomainView::Aggregate(prior), DomainView::Aggregate(next)) = (before.view(), after.view()) else { return validate(catalog, expression, after, optional, true, control, depth + 1); }; if expression.kind != "aggregate" { return validate(catalog, expression, after, optional, true, control, depth + 1); }
    let old = bounds(expression, prior.len()); let new = bounds(expression, next.len()); if new.0 > old.0 || new.1 > old.1 { return Err(refusal("bounds")); } if expression.aggregate.as_deref() == Some("ARRAY") && prior.len() != next.len() { return Err(refusal("array-edit")); }
    let item = expression.item.as_deref().ok_or_else(|| refusal("aggregate-item"))?;
    if expression.unique || expression.aggregate.as_deref() == Some("SET") { for value in next { let mut old_count = 0; let mut next_count = 0; for other in prior { if equal(catalog, item, value, other, control)? { old_count += 1; } } for other in next { if equal(catalog, item, value, other, control)? { next_count += 1; } } if next_count > old_count.max(1) { return Err(refusal("unique")); } } }
    if route.len() == 1 && matches!(operation, ValueChange::Remove | ValueChange::Move) { return Ok(()); } let next = next.get(route[0]).ok_or_else(|| refusal("path"))?; if route.len() == 1 && operation == ValueChange::Insert { return validate(catalog, item, next, expression.optional, true, control, depth + 1); } let prior = prior.get(route[0]).ok_or_else(|| refusal("path"))?; change(catalog, item, prior, next, &route[1..], operation, expression.optional, control, depth + 1)
}

/// 🪞️ Compare exact stored values under the original traversal control.
pub fn identical<V:DomainValue>(left:&V,right:&V,tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>{let mut reference=|_:u64,_:&str,_:&mut dyn FnMut()->Result<(),String>|Ok(false);same(left,right,&mut DomainControl{tick,reference:&mut reference},0)}
