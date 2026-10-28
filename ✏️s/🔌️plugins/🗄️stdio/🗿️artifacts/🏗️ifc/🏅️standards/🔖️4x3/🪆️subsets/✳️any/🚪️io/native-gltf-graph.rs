//! 🏗️ Static geometry authors the shared IFC graph directly into original retained entity and value slots.
use super::*;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcHeader};
pub(super) enum V<'a> {
    U,
    D,
    I(i64),
    R(f64),
    Ref(u64),
    S(&'a str),
    E(&'a str),
    A(&'a [V<'a>]),
}
#[derive(Clone, Copy)]
pub(super) struct Context {
    pub(super) representation: u64,
    pub(super) placement: u64,
}
pub(super) fn entity_capacity(primitives: usize) -> Result<usize, ValueError> {
    primitives.checked_mul(5).and_then(|count| count.checked_add(7)).ok_or_else(|| limit("native geometry entity census overflow"))
}
pub(super) fn empty() -> IfcSnapshot {
    IfcSnapshot { schema: String::new(), header: IfcHeader { file_description: Vec::new(), file_name: Vec::new(), file_schema: Vec::new() }, entities: Vec::new() }
}
pub(super) fn fill(source: &V<'_>, out: &mut IfcValue, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    port.effect(size_of::<IfcValue>(), || {
        *out = match source {
            V::U => IfcValue::Unset,
            V::D => IfcValue::Derived,
            V::I(n) => IfcValue::Integer(*n),
            V::R(n) => IfcValue::Real(*n),
            V::Ref(n) => IfcValue::Reference(*n),
            V::S(_) => IfcValue::String(String::new()),
            V::E(_) => IfcValue::Enum(String::new()),
            V::A(_) => IfcValue::Aggregate(Vec::new()),
        }
    })?;
    match (source, out) {
        (V::R(n), _) if !n.is_finite() => return Err(invalid("native geometry authored a nonfinite IFC real")),
        (V::S(s), IfcValue::String(out)) | (V::E(s), IfcValue::Enum(out)) => {
            port.text(s.len(), out)?;
            let mut at = 0;
            while at < s.len() {
                let mut end = (at + 256).min(s.len());
                while !s.is_char_boundary(end) {
                    end -= 1;
                }
                let value = &s[at..end];
                port.effect(value.len(), || out.push_str(value))?;
                at = end;
            }
        }
        (V::A(values), IfcValue::Aggregate(out)) => values_into(values, out, port)?,
        _ => {}
    }
    Ok(())
}
pub(super) fn values_into(values: &[V<'_>], out: &mut Vec<IfcValue>, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    port.vector(values.len(), out)?;
    for value in values {
        port.effect(size_of::<IfcValue>(), || out.push(IfcValue::Unset))?;
        fill(value, out.last_mut().unwrap(), port)?;
    }
    Ok(())
}
pub(super) fn add(graph: &mut IfcSnapshot, name: &str, args: &[V<'_>], port: &mut Port<'_, '_, '_>) -> Result<u64, ValueError> {
    if graph.entities.len() == graph.entities.capacity() {
        return Err(limit("native geometry exceeded its admitted IFC entity backing"));
    }
    let id = graph.entities.len() as u64 + 1;
    port.effect(size_of::<IfcEntity>(), || graph.entities.push(IfcEntity { id, name: String::new(), args: Vec::new(), complex: Vec::new() }))?;
    let row = graph.entities.last_mut().unwrap();
    port.text(name.len(), &mut row.name)?;
    port.effect(name.len(), || row.name.push_str(name))?;
    values_into(args, &mut row.args, port)?;
    Ok(id)
}
pub(super) fn begin(graph: &mut IfcSnapshot, primitives: usize, port: &mut Port<'_, '_, '_>) -> Result<Context, ValueError> {
    let entities = entity_capacity(port.effect(size_of::<usize>(), || primitives)?)?;
    port.text(9, &mut graph.schema)?;
    port.effect(9, || graph.schema.push_str("stdio.ifc"))?;
    values_into(&[V::A(&[V::S("ViewDefinition [notYetAssigned]")]), V::S("2;1")], &mut graph.header.file_description, port)?;
    values_into(&[V::S("geometry.ifc"), V::S(""), V::A(&[V::S("")]), V::A(&[V::S("")]), V::S("semio"), V::S("semio"), V::S("")], &mut graph.header.file_name, port)?;
    values_into(&[V::A(&[V::S("IFC4X3_ADD2")])], &mut graph.header.file_schema, port)?;
    port.vector(entities, &mut graph.entities)?;
    let point = add(graph, "IFCCARTESIANPOINT", &[V::A(&[V::R(0.), V::R(0.), V::R(0.)])], port)?;
    let axis = add(graph, "IFCAXIS2PLACEMENT3D", &[V::Ref(point), V::U, V::U], port)?;
    let placement = add(graph, "IFCLOCALPLACEMENT", &[V::U, V::Ref(axis)], port)?;
    let context = add(graph, "IFCGEOMETRICREPRESENTATIONCONTEXT", &[V::U, V::S("Model"), V::I(3), V::R(1e-6), V::Ref(axis), V::U], port)?;
    let unit = add(graph, "IFCSIUNIT", &[V::D, V::E("LENGTHUNIT"), V::U, V::E("METRE")], port)?;
    let units = add(graph, "IFCUNITASSIGNMENT", &[V::A(&[V::Ref(unit)])], port)?;
    add(graph, "IFCPROJECT", &[V::S("0000000000000000000001"), V::U, V::S("glTF geometry"), V::U, V::U, V::U, V::U, V::A(&[V::Ref(context)]), V::Ref(units)], port)?;
    port.effect(size_of::<Context>(), || Context { representation: context, placement })
}
pub(super) fn proxy(graph: &mut IfcSnapshot, context: Context, item: u64, name: &str, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let context = port.effect(size_of::<Context>(), || context)?;
    let rep = add(graph, "IFCSHAPEREPRESENTATION", &[V::Ref(context.representation), V::S("Body"), V::S("Tessellation"), V::A(&[V::Ref(item)])], port)?;
    let shape = add(graph, "IFCPRODUCTDEFINITIONSHAPE", &[V::U, V::U, V::A(&[V::Ref(rep)])], port)?;
    let mut guid = [b'0'; 22];
    let mut id = graph.entities.len() as u64 + 1;
    let mut at = 22;
    while id != 0 {
        at -= 1;
        guid[at] = b'0' + (id % 10) as u8;
        id /= 10;
    }
    let guid = std::str::from_utf8(&guid).unwrap();
    add(graph, "IFCBUILDINGELEMENTPROXY", &[V::S(guid), V::U, V::S(name), V::U, V::U, V::Ref(context.placement), V::Ref(shape), V::U, V::E("NOTDEFINED")], port)?;
    Ok(())
}
