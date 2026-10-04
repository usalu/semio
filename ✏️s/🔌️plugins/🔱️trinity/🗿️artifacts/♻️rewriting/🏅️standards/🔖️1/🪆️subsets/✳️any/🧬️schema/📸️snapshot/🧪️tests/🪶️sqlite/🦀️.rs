//! ♻️ Actual Rewriting parent fields and semantic SQL, before Native provider opt-in.
use super::PropertyValue;
use crate::*;
use store::{ArtifactDsl, ArtifactPack};
fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()
}
fn words() -> Vec<u64> {
    laws()["binary64Bits"].as_array().unwrap().iter().map(|v| u64::from_str_radix(v.as_str().unwrap(), 16).unwrap()).collect()
}
fn property(v: &serde_json::Value, word: u64) -> PropertyValue {
    match v["variant"].as_str().unwrap() {
        "null" => PropertyValue::Null,
        "bool" => PropertyValue::Bool(v["value"].as_bool().unwrap()),
        "number" => PropertyValue::Number(f64::from_bits(word)),
        "string" => PropertyValue::String(v["value"].as_str().unwrap().into()),
        "array" => PropertyValue::Array(v["elements"].as_array().unwrap().iter().map(|v| property(v, word)).collect()),
        "object" => PropertyValue::Object(v["members"].as_array().unwrap().iter().map(|v| (v["key"].as_str().unwrap().into(), property(&v["value"], word))).collect()),
        _ => panic!("neutral property variant"),
    }
}
fn read_intrinsic(value:&serde_json::Value)->semio_framework_value::DslValue{
 use semio_framework_value::{DslValue,Number};
 match value{
  serde_json::Value::Object(fields)if fields.len()==1&&fields.contains_key("bits")=>DslValue::Number(Number::Float(f64::from_bits(u64::from_str_radix(fields["bits"].as_str().unwrap(),16).unwrap()))),
  serde_json::Value::Object(fields)=>DslValue::object(fields.iter().map(|(key,value)|(key.clone(),read_intrinsic(value)))),
  serde_json::Value::Array(values)=>DslValue::Array(values.iter().map(read_intrinsic).collect()),
  value=>DslValue::from(value),
 }
}
fn view_intrinsic(value:&semio_framework_value::DslValue)->serde_json::Value{
 use semio_framework_value::{DslValue,Number};
 match value{
  DslValue::Number(Number::Float(value))=>serde_json::json!({"bits":format!("{:016x}",value.to_bits())}),
  DslValue::Number(Number::UInt(value))=>serde_json::json!(value),DslValue::Number(Number::Int(value))=>serde_json::json!(value),
  DslValue::Null=>serde_json::Value::Null,DslValue::Bool(value)=>serde_json::json!(value),DslValue::String(value)=>serde_json::json!(value),DslValue::Bytes(value)=>serde_json::json!(value),
  DslValue::Array(values)=>serde_json::Value::Array(values.iter().map(view_intrinsic).collect()),
  DslValue::Object(fields)=>serde_json::Value::Object(fields.iter().map(|(key,value)|(key.clone(),view_intrinsic(value))).collect()),
 }
}

fn tagged_property(value:&serde_json::Value,word:u64)->PropertyValue{
 match value["kind"].as_str().unwrap(){
  "null"=>PropertyValue::Null,"bool"=>PropertyValue::Bool(value["value"].as_bool().unwrap()),"number"=>PropertyValue::Number(f64::from_bits(word)),"string"=>PropertyValue::String(value["value"].as_str().unwrap().into()),
  "array"=>PropertyValue::Array(value["values"].as_array().unwrap().iter().map(|value|tagged_property(value,word)).collect()),
  "object"=>PropertyValue::Object(value["values"].as_object().unwrap().iter().map(|(key,value)|(key.clone(),tagged_property(value,word))).collect()),
  _=>panic!("neutral tagged property variant"),
 }
}
fn rhs_fixture(value:&serde_json::Value,word:u64)->crate::standards::v1::subsets::any::schema::Rhs{
 use crate::standards::v1::subsets::any::schema::{Assignment,ParameterKind,ParameterSpec,Pattern,Rhs};
 use semio_framework_value::FromValue;
 let pattern=|value:&serde_json::Value|Pattern::from_value(read_intrinsic(value)).unwrap();
 Rhs{
  create:value["create"].as_array().unwrap().iter().map(pattern).collect(),
  delete:value["delete"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().into()).collect(),
  set:value["set"].as_array().unwrap().iter().map(|value|Assignment{var:value["var"].as_str().unwrap().into(),prop:value["prop"].as_str().unwrap().into(),value:tagged_property(&value["value"],word)}).collect(),
  merge:value["merge"].as_array().unwrap().iter().map(pattern).collect(),
  parameters:value["parameters"].as_array().unwrap().iter().map(|value|ParameterSpec{name:value["name"].as_str().unwrap().into(),kind:match value["kind"].as_str().unwrap(){"string"=>ParameterKind::String,"number"=>ParameterKind::Number,"boolean"=>ParameterKind::Boolean,_=>panic!("neutral parameter family")},default:tagged_property(&value["default"],word)}).collect(),
 }
}

fn full(word:u64)->RewritingSnapshot{
 use semio_framework_value::FromValue;
 let l=laws();let c=&l["nativeCase"];
 RewritingSnapshot{
  working_graph:semio_s_artifact_trinity_jack::JackSnapshot::from_value(read_intrinsic(&c["workingGraph"])).unwrap(),
  lhs:crate::standards::v1::subsets::any::schema::Lhs::from_value(read_intrinsic(&c["lhs"])).unwrap(),
  rhs:rhs_fixture(&c["rhs"],word),
  parameter_bindings:c["parameterBindings"].as_array().unwrap().iter().map(|v|(v["key"].as_str().unwrap().into(),property(&v["value"],word))).collect(),
  rule_layout:c["ruleLayout"].as_array().unwrap().iter().map(|v|(v["key"].as_str().unwrap().into(),LayoutPoint{x:f64::from_bits(word),y:f64::from_bits(word)})).collect(),
 }
}

struct Owned(Option<RewritingSnapshot>);
impl Owned {
    fn new(v: RewritingSnapshot) -> Self {
        Self(Some(v))
    }
}
impl std::ops::Deref for Owned {
    type Target = RewritingSnapshot;
    fn deref(&self) -> &RewritingSnapshot {
        self.0.as_ref().unwrap()
    }
}
impl Drop for Owned {
    fn drop(&mut self) {
        if let Some(v) = self.0.take() {
            let factory = semio_framework_value::retirement::OwnedValueRetirementFactory::<RewritingSnapshot>::default();
            let mut cursor = store::ArtifactOwnedValueRetirementFactory::retire_owned(&factory, v);
            loop {
                match cursor.close_step(1, 65536).unwrap() {
                    store::SnapshotRetirementStep::Complete => {
                        assert!(cursor.terminal_is_empty());
                        break;
                    }
                    store::SnapshotRetirementStep::Blocked => panic!("rewriting retirement blocked"),
                    store::SnapshotRetirementStep::Pending { .. } => {}
                }
            }
        }
    }
}
fn assert_property(a: &PropertyValue, b: &PropertyValue) {
    match (a, b) {
        (PropertyValue::Null, PropertyValue::Null) => {}
        (PropertyValue::Bool(a), PropertyValue::Bool(b)) => assert_eq!(a, b),
        (PropertyValue::Number(a), PropertyValue::Number(b)) => assert_eq!(a.to_bits(), b.to_bits()),
        (PropertyValue::String(a), PropertyValue::String(b)) => assert_eq!(a, b),
        (PropertyValue::Array(a), PropertyValue::Array(b)) => {
            assert_eq!(a.len(), b.len());
            for (a, b) in a.iter().zip(b) {
                assert_property(a, b)
            }
        }
        (PropertyValue::Object(a), PropertyValue::Object(b)) => {
            assert_eq!(a.len(), b.len());
            for ((ka, a), (kb, b)) in a.iter().zip(b) {
                assert_eq!(ka, kb);
                assert_property(a, b)
            }
        }
        _ => panic!("property domain changed"),
    }
}
fn assert_full(a: &RewritingSnapshot, b: &RewritingSnapshot) {
    assert_eq!(view_intrinsic(&semio_framework_value::ToValue::to_value(&a.working_graph)),view_intrinsic(&semio_framework_value::ToValue::to_value(&b.working_graph)));
    assert_eq!(view_intrinsic(&semio_framework_value::ToValue::to_value(&a.lhs)),view_intrinsic(&semio_framework_value::ToValue::to_value(&b.lhs)));
    assert_eq!(view_intrinsic(&semio_framework_value::ToValue::to_value(&a.rhs)),view_intrinsic(&semio_framework_value::ToValue::to_value(&b.rhs)));
    assert_eq!(a.parameter_bindings.len(), b.parameter_bindings.len());
    for ((ka, a), (kb, b)) in a.parameter_bindings.iter().zip(&b.parameter_bindings) {
        assert_eq!(ka, kb);
        assert_property(a, b)
    }
    assert_eq!(a.rule_layout.len(), b.rule_layout.len());
    for ((ka, a), (kb, b)) in a.rule_layout.iter().zip(&b.rule_layout) {
        assert_eq!(ka, kb);
        assert_eq!(a.x.to_bits(), b.x.to_bits());
        assert_eq!(a.y.to_bits(), b.y.to_bits())
    }
}
#[test]
fn sqlite_snapshot_rewriting_actual_bare_parent_owns_capability() {
    assert!(RewritingSnapshot::sqlite_snapshot_codec().is_some(), "Rewriting parent lacks semantic SQLite capability");
}
#[test]
fn sqlite_snapshot_rewriting_binary_keeps_all_authored_strings_and_property_words() {
    for word in words() {
        let expected = Owned::new(full(word));
        let actual = Owned::new(RewritingSnapshot::decode_pack(&expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap());
        assert_full(&actual, &expected)
    }
}
#[test]
fn sqlite_snapshot_rewriting_text_keeps_all_authored_strings_and_property_words() {
    for word in words() {
        let expected = Owned::new(full(word));
        let actual = Owned::new(RewritingSnapshot::parse_dsl(&expected.print_dsl()).unwrap());
        assert_full(&actual, &expected)
    }
}
#[test]
fn sqlite_snapshot_rewriting_genuine_controlled_record_covers_actual_recursive_property_domain() {
    for word in words() {
        let expected = Owned::new(full(word));
        let producer = RewritingSnapshot::__dsl_spec_producer();
        let mut yes = |_| true;
        let mut output = semio_framework_value::NativeEncodeControl::new(32 << 20, &mut yes);
        (producer.encoding)(&mut output).unwrap();
        let record = expected.__dsl_to_record_controlled(&mut output).unwrap();
        let mut yes = |_| true;
        let mut input = semio_framework_value::NativeDecodeControl::new(32 << 20, &mut yes);
        (producer.decoding)(&mut input).unwrap();
        let actual = Owned::new(RewritingSnapshot::__dsl_from_record_controlled(&record, &mut input).unwrap());
        assert_full(&actual, &expected)
    }
}
#[test]
fn sqlite_snapshot_rewriting_declared_json_retains_layout_words_and_all_property_states() {
    use crate::standards::v1::subsets::any::io::{export::serializers::artifacts::json::v_rfc8259::any as writer, import::deserializers::artifacts::json::v_rfc8259::any as reader};
    for word in words() {
        let expected = Owned::new(full(word));
        let bytes = writer::serialize_bytes(&expected).unwrap();
        let oracle: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for key in ["x", "y"] {
            assert_eq!(oracle["ruleLayout"][""][key]["bits"].as_str(), Some(format!("{word:016x}").as_str()))
        }
        let actual = Owned::new(reader::deserialize_bytes(&bytes).unwrap());
        assert_full(&actual, &expected)
    }
}
#[test]
fn sqlite_snapshot_rewriting_independent_sqlite_interprets_all_thirty_domain_tables() {
    let script = r#"import{Database}from'bun:sqlite';const d=new Database(':memory:',{safeIntegers:true});d.exec(process.argv[1]);const l=JSON.parse(process.argv[2]),s=l.nativeCase,tables=d.query("SELECT name FROM sqlite_schema WHERE type='table'").all();if(tables.length!==30)throw Error('table count');for(const{name}of tables)if(d.query('PRAGMA table_info('+name+')').all().length!==l.tableWidths[name])throw Error(name);
const seq=new Map(),put=(table,...cells)=>{const id=(seq.get(table)||0)+1;seq.set(table,id);d.query('INSERT INTO '+table+' VALUES('+[id,...cells].map(()=>'?').join(',')+')').run(id,...cells);return id;};
const num=hex=>{const bits=BigInt('0x'+hex),dv=new DataView(new ArrayBuffer(8));dv.setBigUint64(0,bits);const value=dv.getFloat64(0),kind=Number.isNaN(value)?'nan':value===Infinity?'positiveInfinity':value===-Infinity?'negativeInfinity':'finite';return[Number.isFinite(value)?value:null,BigInt.asIntN(64,bits),kind];};
const property=v=>{const family=v.variant||v.kind,id=put('rewriting_value',family);if(family==='bool')put('rewriting_boolean',id,v.value?1:0);if(family==='number')put('rewriting_number',id,...num(v.bits||v.value.bits));if(family==='string')put('rewriting_string',id,v.value);if(family==='array')(v.elements||v.values).forEach((child,n)=>put('rewriting_array_element',id,n,property(child)));if(family==='object'){const entries=v.members?.map(row=>[row.key,row.value])||Object.entries(v.values);entries.sort((a,b)=>Buffer.compare(Buffer.from(a[0]),Buffer.from(b[0])));entries.forEach(([key,child],n)=>put('rewriting_object_member',id,n,key,property(child)));}return id;};
const type=v=>{const id=put('jack_value_type',v.kind);if(v.kind==='list')put('jack_value_type_list',id,type(v.of));if(v.kind==='schema')put('jack_value_type_schema',id,v.of);return id;};
const jack=s.workingGraph,jid=put('jack_document',jack.schema,jack.name,jack.manifestId??null,jack.rootNodeId??null,jack.query),cam=jack.camera,x=num(cam.x.bits),y=num(cam.y.bits),z=num(cam.zoom.bits);put('jack_camera',jid,x[0],y[0],z[0],...x.slice(1),...y.slice(1),...z.slice(1));const child=jack.content;put('jack_content_child',jid,child.childId,child.target.artifactId,child.target.dialect.artifactKind,child.target.dialect.standard,child.target.dialect.subset);
for(const[role,table,propTable]of[['nodeKinds','jack_node_kind','jack_node_property'],['edgeKinds','jack_edge_kind','jack_edge_property'],['portKinds','jack_port_kind','jack_port_property']])jack.manifest[role].forEach((kind,n)=>{const id=role==='portKinds'?put(table,jid,n,kind.name,kind.direction):put(table,jid,n,kind.name);kind.properties.forEach((value,n)=>put(propTable,id,n,value.name,value.kind,value.expr??null,type(value.valueType)));if(role==='nodeKinds')kind.portKinds.forEach((port,n)=>put('jack_node_kind_port',id,n,port));});
const doc=put('rewriting_document',jid),pattern=v=>put('rewriting_pattern',v.leftVar,v.leftKind,v.edgeVar??null,v.edgeKind??null,v.rightVar??null,v.rightKind??null);put('rewriting_lhs',doc,pattern(s.lhs.pattern),s.lhs.whereClause??null);const rhs=put('rewriting_rhs',doc);for(const[role,table]of[['create','rewriting_create'],['merge','rewriting_merge']])s.rhs[role].forEach((value,n)=>put(table,rhs,n,pattern(value)));s.rhs.delete.forEach((value,n)=>put('rewriting_delete',rhs,n,value));s.rhs.set.forEach((value,n)=>put('rewriting_assignment',rhs,n,value.var,value.prop,property(value.value)));s.rhs.parameters.forEach((value,n)=>put('rewriting_parameter',rhs,n,value.name,value.kind,property(value.default)));
s.parameterBindings.forEach(value=>put('rewriting_binding',doc,value.key,property(value.value)));s.ruleLayout.forEach(value=>{const x=num(value.xBits),y=num(value.yBits);put('rewriting_layout',doc,value.key,x[0],y[0],...x.slice(1),...y.slice(1));});
if(d.query('SELECT left_var,edge_var,right_var FROM rewriting_pattern WHERE id=1').get().left_var!==s.lhs.pattern.leftVar)throw Error('typed pattern');if(d.query('SELECT query FROM jack_document').get().query!==jack.query)throw Error('literal query');if(d.query('SELECT count(*) AS n FROM rewriting_assignment').get().n!==BigInt(s.rhs.set.length))throw Error('assignments');
for(const hex of l.binary64Bits){const word=BigInt('0x'+hex),v=num(hex);d.query('UPDATE rewriting_layout SET x=?,y=?,x_ieee754_bits=?,x_numeric_class=?,y_ieee754_bits=?,y_numeric_class=? WHERE id=1').run(v[0],v[0],v[1],v[2],v[1],v[2]);const row=d.query('SELECT x_ieee754_bits,x_numeric_class FROM rewriting_layout WHERE id=1').get();if(BigInt.asUintN(64,row.x_ieee754_bits).toString(16).padStart(16,'0')!==hex||row.x_numeric_class!==v[2])throw Error('word');}
if(d.query('PRAGMA foreign_key_check').all().length)throw Error('fk');d.close();"#;
    let out = std::process::Command::new("bun").args(["-e", script, include_str!("../../🪶️sqlite/🗄️.sql"), &laws().to_string()]).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
}
#[test]
fn sqlite_snapshot_rewriting_real_large_authored_string_control_is_interior() {
    let mut value = full(0);
    value.working_graph.query = "😀".repeat(32768);
    let expected = Owned::new(value);
    let mut saw = false;
    let mut callback = |e: semio_framework_value::native_encoding::NativeEncodeProgress| {
        if e.total >= 65536 && e.completed >= 65536 && e.completed < e.total {
            saw = true;
            false
        } else {
            true
        }
    };
    let mut output = semio_framework_value::NativeEncodeControl::new(32 << 20, &mut callback);
    assert!(expected.__dsl_to_record_controlled(&mut output).is_err());
    drop(output);
    assert!(saw);
}
#[test]
fn sqlite_snapshot_rewriting_actual_erased_parent_queries_independent_authored_strings() {
    use store::sqlite_snapshot::*;
    let codec = store::ArtifactCodec::bare::<RewritingSnapshot, crate::RewriteRuleMutation>(crate::REWRITE_RULE_SCHEMA).snapshot_sqlite.expect("missing parent capability");
    let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.trinity.rewriting".into(), standard: "1".into(), subset: "*".into() };
    let expected = Owned::new(full(0));
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = match encoding {
            SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()),
            SnapshotEncoding::Text => store::io_schema::IoPayload::Text(expected.print_dsl()),
        };
        let database = (codec.export)(crate::REWRITE_RULE_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        assert_eq!(database.tables.len(), 30);
        let root = database.table("rewriting_document").unwrap().single_row().unwrap();
        let graph=database.table("jack_document").unwrap().single_row().unwrap();
        assert_eq!(root.integer(1).unwrap(),graph.rowid);
        assert_eq!(graph.text(2).unwrap(),expected.working_graph.name);
        assert_eq!(graph.text(5).unwrap(),expected.working_graph.query);
        let lhs=database.table("rewriting_lhs").unwrap().single_row().unwrap();
        assert_eq!(lhs.optional_text(3).unwrap(),expected.lhs.where_clause.as_deref());
        let payload = (codec.import)(crate::REWRITE_RULE_SCHEMA, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let actual = Owned::new(match payload {
            store::io_schema::IoPayload::Text(text) => RewritingSnapshot::parse_dsl(&text).unwrap(),
            store::io_schema::IoPayload::Binary(bytes) => RewritingSnapshot::decode_pack(&bytes).unwrap(),
        });
        assert_full(&actual, &expected)
    }
}

#[test]
fn sqlite_snapshot_rewriting_composed_child_retains_actual_rich_graph_fields() {
    use semio_framework_value::{DslValue, FromValue, Number, ToValue};
    use semio_framework_os_kernel::{ArtifactSqliteSnapshot, sqlite_snapshot::*};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
    fn read(value: &serde_json::Value) -> DslValue {
        match value {
            serde_json::Value::Object(fields) if fields.len() == 1 && fields.contains_key("bits") => {
                DslValue::Number(Number::Float(f64::from_bits(u64::from_str_radix(fields["bits"].as_str().unwrap(), 16).unwrap())))
            }
            serde_json::Value::Object(fields) => DslValue::object(fields.iter().map(|(key, value)| (key.clone(), read(value)))),
            serde_json::Value::Array(values) => DslValue::Array(values.iter().map(read).collect()),
            value => DslValue::from(value),
        }
    }
    fn view(value: &DslValue) -> serde_json::Value {
        match value {
            DslValue::Number(Number::Float(value)) => serde_json::json!({"bits":format!("{:016x}",value.to_bits())}),
            DslValue::Number(Number::UInt(value)) => serde_json::json!(value),
            DslValue::Number(Number::Int(value)) => serde_json::json!(value),
            DslValue::Null => serde_json::Value::Null,
            DslValue::Bool(value) => serde_json::json!(value),
            DslValue::String(value) => serde_json::json!(value),
            DslValue::Bytes(value) => serde_json::json!(value),
            DslValue::Array(values) => serde_json::Value::Array(values.iter().map(view).collect()),
            DslValue::Object(fields) => serde_json::Value::Object(fields.iter().map(|(key, value)| (key.clone(), view(value))).collect()),
        }
    }
    let vector: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json")).unwrap();
    let expected = &vector["childSnapshot"];
    let snapshot = SemioGraphSnapshot::from_value(read(expected)).unwrap();
    assert_eq!(view(&ToValue::to_value(&snapshot)), *expected);
    let mut callback = |_| true;
    let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default());
    let database = snapshot.to_sqlite_database(&mut control).unwrap();
    let restored = SemioGraphSnapshot::from_sqlite_database(&database, &mut control).unwrap();
    assert_eq!(view(&ToValue::to_value(&restored)), *expected);
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = restored.encode_sqlite_snapshot_native(encoding, &mut control).unwrap();
        let output = SemioGraphSnapshot::decode_sqlite_snapshot_native(&payload, &mut control).unwrap();
        assert_eq!(view(&ToValue::to_value(&output)), *expected);
    }
}
