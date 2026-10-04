/** ♻️ Handwritten Rewriting SQL interpreted independently before provider opt-in. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import corpus from "../../../🧫️fixtures/🪶️sqlite/🔣️.json";
import sql from "../../../🪶️sqlite/🗄️.sql" with { type: "text" };

type PropertyCase = { variant: string; value?: boolean | string; elements?: readonly PropertyCase[]; members?: readonly { key: string; value: PropertyCase }[] };
function scalar(hex: string): [number | null, bigint, string] {
  const word = BigInt("0x" + hex), view = new DataView(new ArrayBuffer(8));
  view.setBigUint64(0, word);
  const value = view.getFloat64(0);
  return [Number.isNaN(value) ? null : value, BigInt.asIntN(64, word), Number.isNaN(value) ? "nan" : value === Infinity ? "positiveInfinity" : value === -Infinity ? "negativeInfinity" : "finite"];
}
function independent(hex: string): Database {
  const db = new Database(":memory:", { safeIntegers: true });
  db.exec(sql);
  db.query("INSERT INTO rewriting_document VALUES(1,1)").run();
  const [value, bits, kind] = scalar(hex);
  for (const [index, layout] of corpus.nativeCase.ruleLayout.entries()) db.query("INSERT INTO rewriting_layout VALUES(?,1,?,?,?,?,?,?,?)").run(index + 1, layout.key, value, value, bits, kind, bits, kind);
  let nodes = 0, booleans = 0, numbers = 0, strings = 0, elements = 0, members = 0;
  function insert(v: PropertyCase): number {
    const id = ++nodes;
    db.query("INSERT INTO rewriting_value VALUES(?,?)").run(id, v.variant);
    switch (v.variant) {
      case "null": break;
      case "bool": db.query("INSERT INTO rewriting_boolean VALUES(?,?,?)").run(++booleans, id, v.value === true ? 1 : 0); break;
      case "number": db.query("INSERT INTO rewriting_number VALUES(?,?,?,?,?)").run(++numbers, id, value, bits, kind); break;
      case "string": db.query("INSERT INTO rewriting_string VALUES(?,?,?)").run(++strings, id, v.value as string); break;
      case "array": for (const [ordinal, child] of (v.elements ?? []).entries()) db.query("INSERT INTO rewriting_array_element VALUES(?,?,?,?)").run(++elements, id, ordinal, insert(child)); break;
      case "object": for (const [ordinal, member] of (v.members ?? []).entries()) db.query("INSERT INTO rewriting_object_member VALUES(?,?,?,?,?)").run(++members, id, ordinal, member.key, insert(member.value)); break;
      default: throw new Error("Neutral Rewriting variant");
    }
    return id;
  }
  for (const [index, binding] of corpus.nativeCase.parameterBindings.entries()) db.query("INSERT INTO rewriting_binding VALUES(?,1,?,?)").run(index + 1, binding.key, insert(binding.value));
  const document = corpus.nativeCase;
  let patterns = 0;
  function pattern(p: any): number {
    const id = ++patterns;
    db.query("INSERT INTO rewriting_pattern VALUES(?,?,?,?,?,?,?)").run(id,p.leftVar,p.leftKind,p.edgeVar??null,p.edgeKind??null,p.rightVar??null,p.rightKind??null);
    return id;
  }
  function property(v: any): PropertyCase {
    switch(v.kind){
      case "null":return {variant:"null"};
      case "bool":case "string":return {variant:v.kind,value:v.value};
      case "number":return {variant:"number"};
      case "array":return {variant:"array",elements:v.values.map(property)};
      case "object":return {variant:"object",members:Object.entries(v.values).sort(([a],[b])=>a<b?-1:a>b?1:0).map(([key,value])=>({key,value:property(value)}))};
      default:throw Error("Neutral rule property variant");
    }
  }
  db.query("INSERT INTO rewriting_lhs VALUES(1,1,?,?)").run(pattern(document.lhs.pattern),document.lhs.whereClause??null);
  db.query("INSERT INTO rewriting_rhs VALUES(1,1)").run();
  for(const table of ["create","merge"] as const)for(const[ordinal,p]of document.rhs[table].entries())db.query("INSERT INTO rewriting_"+table+" VALUES(?,1,?,?)").run(ordinal+1,ordinal,pattern(p));
  for(const[ordinal,name]of document.rhs.delete.entries())db.query("INSERT INTO rewriting_delete VALUES(?,1,?,?)").run(ordinal+1,ordinal,name);
  for(const[ordinal,a]of document.rhs.set.entries())db.query("INSERT INTO rewriting_assignment VALUES(?,1,?,?,?,?)").run(ordinal+1,ordinal,a.var,a.prop,insert(property(a.value)));
  for(const[ordinal,p]of document.rhs.parameters.entries())db.query("INSERT INTO rewriting_parameter VALUES(?,1,?,?,?,?)").run(ordinal+1,ordinal,p.name,p.kind,insert(property(p.default)));
  const graph=document.workingGraph;
  db.query("INSERT INTO jack_document VALUES(1,?,?,?,?,?)").run(graph.schema,graph.name,graph.manifestId??null,graph.rootNodeId??null,graph.query);
  const camera=[graph.camera.x,graph.camera.y,graph.camera.zoom].map(v=>scalar(v.bits));
  db.query("INSERT INTO jack_camera VALUES(1,1,?,?,?,?,?,?,?,?,?)").run(...camera.map(v=>v[0]),...camera.flatMap(v=>[v[1],v[2]]));
  const child=graph.content;
  db.query("INSERT INTO jack_content_child VALUES(1,1,?,?,?,?,?)").run(child.childId,child.target.artifactId,child.target.dialect.artifactKind,child.target.dialect.standard,child.target.dialect.subset);
  let types=0,lists=0,schemas=0;
  function valueType(v:any):number{
    const id=++types;db.query("INSERT INTO jack_value_type VALUES(?,?)").run(id,v.kind);
    if(v.kind==="list")db.query("INSERT INTO jack_value_type_list VALUES(?,?,?)").run(++lists,id,valueType(v.of));
    if(v.kind==="schema")db.query("INSERT INTO jack_value_type_schema VALUES(?,?,?)").run(++schemas,id,v.of);
    return id;
  }
  for(const family of ["node","edge","port"] as const){
    let properties=0,ports=0;
    for(const[ordinal,kind]of graph.manifest[family+"Kinds"].entries()){
      const id=ordinal+1;
      if(family==="port")db.query("INSERT INTO jack_port_kind VALUES(?,1,?,?,?)").run(id,ordinal,kind.name,kind.direction);
      else db.query("INSERT INTO jack_"+family+"_kind VALUES(?,1,?,?)").run(id,ordinal,kind.name);
      if(family==="node")for(const[index,name]of kind.portKinds.entries())db.query("INSERT INTO jack_node_kind_port VALUES(?,?,?,?)").run(++ports,id,index,name);
      for(const[index,p]of kind.properties.entries())db.query("INSERT INTO jack_"+family+"_property VALUES(?,?,?,?,?,?,?)").run(++properties,id,index,p.name,p.kind,p.expr??null,valueType(p.valueType));
    }
  }
  return db;
}
test("Rewriting thirty authored tables preserve every persisted domain relationship", () => {
  const db = independent(corpus.binary64Bits[0]!);
  try {
    expect(db.query("SELECT name FROM sqlite_schema WHERE type='table'").all().length).toBe(30);
    for (const [table, width] of Object.entries(corpus.tableWidths)) expect(db.query("PRAGMA table_info(" + table + ")").all().length).toBe(width);
    for (const [table, count] of Object.entries(corpus.tableRowCounts)) expect((db.query("SELECT count(*) AS n FROM " + table).get() as { n: bigint }).n).toBe(BigInt(count));
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect((db.query("SELECT DISTINCT variant FROM rewriting_value ORDER BY variant").all() as { variant: string }[]).map(v => v.variant)).toEqual([...corpus.propertyValueVariants].sort());
    expect(db.query("SELECT j.name,j.query,l.where_clause FROM rewriting_document d JOIN jack_document j ON j.id=d.working_graph_id JOIN rewriting_lhs l ON l.document_id=d.id").get()).toEqual({name:corpus.nativeCase.workingGraph.name,query:corpus.nativeCase.workingGraph.query,where_clause:corpus.nativeCase.lhs.whereClause});
    expect((db.query("SELECT map_key FROM rewriting_binding ORDER BY id").all() as { map_key: string }[]).map(v => v.map_key)).toEqual(corpus.nativeCase.parameterBindings.map(v => v.key));
    expect((db.query("SELECT value FROM rewriting_boolean ORDER BY id").all() as { value: bigint }[]).map(v => v.value)).toEqual([0n, 1n, 1n, 0n]);
  } finally { db.close(); }
});
for (const word of corpus.binary64Bits) test("Rewriting queryable exact scalar word " + word, () => {
  const db = independent(word);
  try {
    const [query, bits, kind] = scalar(word);
    for (const row of db.query("SELECT value,value_ieee754_bits,value_numeric_class FROM rewriting_number").all() as { value: number | null; value_ieee754_bits: bigint; value_numeric_class: string }[]) {
      expect(row.value).toBe(query === 0 ? 0 : query);
      expect(row.value_ieee754_bits).toBe(bits);
      expect(row.value_numeric_class).toBe(kind);
    }
    for (const row of db.query("SELECT x,y,x_ieee754_bits,y_ieee754_bits,x_numeric_class,y_numeric_class FROM rewriting_layout").all() as { x: number | null; y: number | null; x_ieee754_bits: bigint; y_ieee754_bits: bigint; x_numeric_class: string; y_numeric_class: string }[]) {
      expect(row.x).toBe(query === 0 ? 0 : query);
      expect(row.y).toBe(query === 0 ? 0 : query);
      expect(row.x_ieee754_bits).toBe(bits);
      expect(row.y_ieee754_bits).toBe(bits);
      expect(row.x_numeric_class).toBe(kind);
      expect(row.y_numeric_class).toBe(kind);
    }
    const published = Database.deserialize(db.serialize(), { safeIntegers: true });
    try { expect(published.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" }); expect(published.query("PRAGMA foreign_key_check").all()).toEqual([]); }
    finally { published.close(); }
  } finally { db.close(); }
});
