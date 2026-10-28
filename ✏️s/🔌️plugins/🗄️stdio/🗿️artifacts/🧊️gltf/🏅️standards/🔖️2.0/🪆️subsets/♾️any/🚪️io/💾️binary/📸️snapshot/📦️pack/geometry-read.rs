//! 📥️ Selected native fields bind actual geometry rows while the original generic source retains omitted semantics.
use super::*;
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress};
use store::NativeSnapshotBodyWallet;
struct Port<'a, 'b, 'c> {
    native: &'a mut NativeDecodeControl<'b>,
    body: &'c mut NativeSnapshotBodyWallet,
}
impl Port<'_, '_, '_> {
    fn effect<T: Copy>(&mut self, n: usize, f: impl FnOnce() -> T) -> Result<T, ValueError> {
        self.native.checkpoint()?;
        self.body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: n, maximum_depth: 2, ..Default::default() })?;
        let value = f();
        self.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: n, ..Default::default() })?;
        self.native.step()?;
        Ok(value)
    }
    fn vector<T>(&mut self, n: usize, out: &mut Vec<T>) -> Result<(), ValueError> {
        self.body.allocate_vec_into(self.native, n, out, 2)?;
        self.native.step()
    }
    fn text(&mut self, s: &str, out: &mut String) -> Result<(), ValueError> {
        self.body.copy_text_into(self.native, s, out, 2)?;
        self.native.step()
    }
}
fn field(r: &RecordValue, id: u16) -> Result<&FieldValue, ValueError> {
    r.get(id).ok_or_else(|| invalid("selected glTF native field is missing"))
}
fn record(v: &FieldValue) -> Result<&RecordValue, ValueError> {
    if let FieldValue::Record(r) = v { Ok(r) } else { Err(invalid("selected glTF native row is not a record")) }
}
fn list(v: &FieldValue) -> Result<&[FieldValue], ValueError> {
    match v {
        FieldValue::List(v) | FieldValue::Tuple(v) => Ok(v),
        _ => Err(invalid("selected glTF native field is not a list")),
    }
}
fn array(r: &RecordValue, id: u16) -> Result<&[FieldValue], ValueError> {
    list(field(r, id)?)
}
fn uint(v: &FieldValue, p: &mut Port<'_, '_, '_>) -> Result<usize, ValueError> {
    let n = match v {
        FieldValue::UInt(n) => p.effect(size_of::<u64>(), || *n)?,
        _ => return Err(invalid("selected glTF native field is not unsigned")),
    };
    usize::try_from(n).map_err(|_| invalid("selected glTF native integer exceeds native domain"))
}
fn optional_uint(r: &RecordValue, id: u16, p: &mut Port<'_, '_, '_>) -> Result<Option<usize>, ValueError> {
    match r.get(id) {
        None | Some(FieldValue::Absent) => Ok(None),
        Some(v) => uint(v, p).map(Some),
    }
}
fn number(v: &FieldValue, p: &mut Port<'_, '_, '_>) -> Result<f64, ValueError> {
    let FieldValue::Float(n) = v else {
        return Err(invalid("selected glTF native measure is not Binary64"));
    };
    let n = p.effect(8, || *n)?;
    if n.is_finite() { Ok(n) } else { Err(invalid("selected glTF native measure is nonfinite")) }
}
fn text(v: &FieldValue) -> Result<&str, ValueError> {
    match v {
        FieldValue::Text(v) => Ok(v),
        _ => Err(invalid("selected glTF native field is not text")),
    }
}
fn optional_text(r: &RecordValue, id: u16, out: &mut Option<String>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if let Some(v) = r.get(id).filter(|v| !matches!(v, FieldValue::Absent)) {
        p.effect(size_of::<String>(), || *out = Some(String::new()))?;
        p.text(text(v)?, out.as_mut().unwrap())?;
    }
    Ok(())
}
fn optional_fixed<const N: usize>(r: &RecordValue, id: u16, p: &mut Port<'_, '_, '_>) -> Result<Option<[f64; N]>, ValueError> {
    let Some(v) = r.get(id).filter(|v| !matches!(v, FieldValue::Absent)) else { return Ok(None) };
    let v = list(v)?;
    if v.len() != N {
        return Err(invalid("selected glTF native transform has wrong dimension"));
    }
    let mut out = [0.; N];
    for (slot, v) in out.iter_mut().zip(v) {
        *slot = number(v, p)?;
    }
    Ok(Some(out))
}
fn empty_list(r: &RecordValue, id: u16, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    p.effect(size_of::<usize>(), || ())?;
    if !array(r, id)?.is_empty() {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "native static geometry refuses animation, skinning, morph targets and required extensions"));
    }
    Ok(())
}
fn absent(r: &RecordValue, id: u16, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    p.effect(size_of::<usize>(), || ())?;
    if r.get(id).is_some_and(|v| !matches!(v, FieldValue::Absent)) {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "native static geometry refuses selected extensions, skinning and sparse accessors"));
    }
    Ok(())
}
fn indices(v: &[FieldValue], out: &mut Vec<usize>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    p.vector(v.len(), out)?;
    for v in v {
        let n = uint(v, p)?;
        p.effect(size_of::<usize>(), || out.push(n))?;
    }
    Ok(())
}
fn bounds(r: &RecordValue, id: u16, out: &mut Option<Vec<f64>>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if let Some(v) = r.get(id).filter(|v| !matches!(v, FieldValue::Absent)) {
        let v = list(v)?;
        if v.len() > 16 {
            return Err(invalid("selected glTF native accessor bounds exceed matrix dimensions"));
        }
        p.effect(size_of::<Vec<f64>>(), || *out = Some(Vec::new()))?;
        let out = out.as_mut().unwrap();
        p.vector(v.len(), out)?;
        for v in v {
            let n = number(v, p)?;
            p.effect(8, || out.push(n))?;
        }
    }
    Ok(())
}
fn empty() -> GltfSnapshot {
    GltfSnapshot {
        schema: String::new(),
        document: GltfDocument {
            asset: GltfAsset { version: String::new(), generator: None, copyright: None, min_version: None, extensions: None, extras: None },
            scene: None,
            scenes: Vec::new(),
            nodes: Vec::new(),
            meshes: Vec::new(),
            accessors: Vec::new(),
            buffer_views: Vec::new(),
            buffers: Vec::new(),
            materials: Vec::new(),
            textures: Vec::new(),
            images: Vec::new(),
            samplers: Vec::new(),
            skins: Vec::new(),
            animations: Vec::new(),
            cameras: Vec::new(),
            extensions_used: Vec::new(),
            extensions_required: Vec::new(),
            extensions: None,
            extras: None,
        },
        buffers: Vec::new(),
        source_form: GltfSourceForm::Json,
    }
}
pub(super) fn receive(state: &mut Receiving, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<(), ValueError> {
    owner.receive::<Option<GltfSnapshot>, ()>(|target, native, body| {
        native.scoped_stage(|native| {
            native.begin_stage(0)?;
            let mut p = Port { native, body };
            p.effect(size_of::<Option<GltfSnapshot>>(), || *target = Some(None))?;
            let source = state.source.as_mut().unwrap();
            if !p.effect(10, || source.schema == "stdio.gltf")? {
                return Err(invalid("native glTF source schema differs"));
            }
            p.effect(size_of::<GltfSnapshot>(), || *target.as_mut().unwrap() = Some(empty()))?;
            let out = target.as_mut().unwrap().as_mut().unwrap();
            p.text("stdio.gltf", &mut out.schema)?;
            out.source_form = match source.source_form {
                Form::Json => GltfSourceForm::Json,
                Form::Glb => GltfSourceForm::Glb,
            };
            let r = &source.document.0;
            let d = &mut out.document;
            let asset = record(field(r, 0)?)?;
            let version = text(field(asset, 0)?)?;
            if !p.effect(version.len().min(4), || version == "2.0")? {
                return Err(invalid("native glTF asset version must be 2.0"));
            }
            p.text(version, &mut d.asset.version)?;
            optional_text(asset, 1, &mut d.asset.generator, &mut p)?;
            optional_text(asset, 2, &mut d.asset.copyright, &mut p)?;
            optional_text(asset, 3, &mut d.asset.min_version, &mut p)?;
            if d.asset.min_version.as_deref().is_some_and(|v| v != "2.0") {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "native glTF minimum version exceeds supported 2.0"));
            }
            absent(asset, 4, &mut p)?;
            for id in [12, 13, 15, 16] {
                empty_list(r, id, &mut p)?;
            }
            absent(r, 17, &mut p)?;
            d.scene = optional_uint(r, 1, &mut p)?;
            let values = array(r, 2)?;
            p.vector(values.len(), &mut d.scenes)?;
            for v in values {
                let row = record(v)?;
                p.effect(size_of::<GltfScene>(), || d.scenes.push(GltfScene::default()))?;
                let out = d.scenes.last_mut().unwrap();
                indices(array(row, 0)?, &mut out.nodes, &mut p)?;
                optional_text(row, 1, &mut out.name, &mut p)?;
                absent(row, 2, &mut p)?;
            }
            let values = array(r, 3)?;
            p.vector(values.len(), &mut d.nodes)?;
            for v in values {
                let row = record(v)?;
                absent(row, 3, &mut p)?;
                empty_list(row, 8, &mut p)?;
                absent(row, 10, &mut p)?;
                p.effect(size_of::<GltfNode>(), || d.nodes.push(GltfNode::default()))?;
                let out = d.nodes.last_mut().unwrap();
                indices(array(row, 0)?, &mut out.children, &mut p)?;
                out.mesh = optional_uint(row, 1, &mut p)?;
                out.matrix = optional_fixed(row, 4, &mut p)?;
                out.translation = optional_fixed(row, 5, &mut p)?;
                out.rotation = optional_fixed(row, 6, &mut p)?;
                out.scale = optional_fixed(row, 7, &mut p)?;
                optional_text(row, 9, &mut out.name, &mut p)?;
            }
            let values = array(r, 4)?;
            p.vector(values.len(), &mut d.meshes)?;
            for v in values {
                let row = record(v)?;
                empty_list(row, 1, &mut p)?;
                absent(row, 3, &mut p)?;
                p.effect(size_of::<GltfMesh>(), || d.meshes.push(GltfMesh { primitives: Vec::new(), weights: Vec::new(), name: None, extensions: None, extras: None }))?;
                let out = d.meshes.last_mut().unwrap();
                optional_text(row, 2, &mut out.name, &mut p)?;
                let primitives = array(row, 0)?;
                p.vector(primitives.len(), &mut out.primitives)?;
                for primitive in primitives {
                    let row = record(primitive)?;
                    empty_list(row, 4, &mut p)?;
                    absent(row, 5, &mut p)?;
                    p.effect(size_of::<GltfPrimitive>(), || out.primitives.push(GltfPrimitive::default()))?;
                    let out = out.primitives.last_mut().unwrap();
                    out.indices = optional_uint(row, 1, &mut p)?;
                    out.mode = optional_uint(row, 3, &mut p)?.map(|n| n as u64);
                    p.vector(1, &mut out.attributes)?;
                    for attribute in array(row, 0)? {
                        let a = record(attribute)?;
                        let key = text(field(a, 0)?)?;
                        let position = p.effect(key.len().min(9), || key == "POSITION")?;
                        if position {
                            if !out.attributes.is_empty() {
                                return Err(invalid("native glTF primitive has repeated POSITION semantics"));
                            }
                            p.effect(size_of::<(String, usize)>(), || out.attributes.push((String::new(), 0)))?;
                            p.text("POSITION", &mut out.attributes[0].0)?;
                            out.attributes[0].1 = uint(field(a, 1)?, &mut p)?;
                        }
                    }
                    if out.attributes.is_empty() {
                        return Err(invalid("native glTF primitive lacks POSITION semantics"));
                    }
                }
            }
            let values = array(r, 5)?;
            p.vector(values.len(), &mut d.accessors)?;
            for v in values {
                let row = record(v)?;
                absent(row, 8, &mut p)?;
                absent(row, 10, &mut p)?;
                let component = match field(row, 2)? {
                    FieldValue::Enum(0) => GltfComponentType::Byte,
                    FieldValue::Enum(1) => GltfComponentType::UnsignedByte,
                    FieldValue::Enum(2) => GltfComponentType::Short,
                    FieldValue::Enum(3) => GltfComponentType::UnsignedShort,
                    FieldValue::Enum(4) => GltfComponentType::UnsignedInt,
                    FieldValue::Enum(5) => GltfComponentType::Float,
                    _ => return Err(invalid("native glTF accessor component type is invalid")),
                };
                let kind = match field(row, 5)? {
                    FieldValue::Enum(0) => GltfAccessorType::Scalar,
                    FieldValue::Enum(1) => GltfAccessorType::Vec2,
                    FieldValue::Enum(2) => GltfAccessorType::Vec3,
                    FieldValue::Enum(3) => GltfAccessorType::Vec4,
                    FieldValue::Enum(4) => GltfAccessorType::Mat2,
                    FieldValue::Enum(5) => GltfAccessorType::Mat3,
                    FieldValue::Enum(6) => GltfAccessorType::Mat4,
                    _ => return Err(invalid("native glTF accessor kind is invalid")),
                };
                let count = uint(field(row, 4)?, &mut p)?;
                p.effect(size_of::<GltfAccessor>(), || {
                    d.accessors.push(GltfAccessor { buffer_view: None, byte_offset: 0, component_type: component, normalized: false, count, kind, max: None, min: None, sparse: None, name: None, extensions: None, extras: None })
                })?;
                let out = d.accessors.last_mut().unwrap();
                out.buffer_view = optional_uint(row, 0, &mut p)?;
                out.byte_offset = uint(field(row, 1)?, &mut p)?;
                out.normalized = match field(row, 3)? {
                    FieldValue::Bool(v) => *v,
                    _ => return Err(invalid("native glTF accessor normalized flag is invalid")),
                };
                bounds(row, 6, &mut out.max, &mut p)?;
                bounds(row, 7, &mut out.min, &mut p)?;
                optional_text(row, 9, &mut out.name, &mut p)?;
            }
            let values = array(r, 6)?;
            p.vector(values.len(), &mut d.buffer_views)?;
            for v in values {
                let row = record(v)?;
                absent(row, 6, &mut p)?;
                let buffer = uint(field(row, 0)?, &mut p)?;
                let byte_length = uint(field(row, 2)?, &mut p)?;
                p.effect(size_of::<GltfBufferView>(), || d.buffer_views.push(GltfBufferView { buffer, byte_offset: 0, byte_length, byte_stride: None, target: None, name: None, extensions: None, extras: None }))?;
                let out = d.buffer_views.last_mut().unwrap();
                out.byte_offset = uint(field(row, 1)?, &mut p)?;
                out.byte_stride = optional_uint(row, 3, &mut p)?;
                out.target = optional_uint(row, 4, &mut p)?.map(|n| n as u64);
                optional_text(row, 5, &mut out.name, &mut p)?;
            }
            let values = array(r, 7)?;
            if values.len() != source.buffers.len() {
                return Err(invalid("native glTF resolved buffers and declared buffers differ"));
            }
            p.vector(values.len(), &mut d.buffers)?;
            p.vector(values.len(), &mut out.buffers)?;
            for (value, bytes) in values.iter().zip(&mut source.buffers) {
                let row = record(value)?;
                absent(row, 3, &mut p)?;
                let byte_length = uint(field(row, 0)?, &mut p)?;
                if byte_length != bytes.bytes.len() {
                    return Err(invalid("native glTF resolved buffer extent differs from its declaration"));
                }
                p.effect(size_of::<GltfBuffer>(), || d.buffers.push(GltfBuffer { byte_length, uri: None, name: None, extensions: None, extras: None }))?;
                optional_text(row, 2, &mut d.buffers.last_mut().unwrap().name, &mut p)?;
                p.effect(size_of::<Vec<u8>>(), || out.buffers.push(std::mem::take(&mut bytes.bytes)))?;
            }
            p.effect(size_of::<GltfSnapshot>(), || state.target = target.as_mut().unwrap().take())?;
            Ok(())
        })
    })
}
