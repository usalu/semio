import refusalCorpus from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧫️fixtures/🔣️.json";
const canceledKind=refusalCorpus.cases.find(c=>c.id==="canceled-projection")!.expectedKind;
import {binary64,binary32,binary64Value,binary32Value,type Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🧫️ Shared OBJ geometry and membership fixture with independent SQL edits. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import ieee from "../🧫️fixtures/🔢️ieee754/🔣️.json";
import sourceIndices from "../🧫️fixtures/🔗️source-indices.json";
import surrogateIndices from "../🧫️fixtures/🔗️surrogate-indices.json";
import {objSnapshotValidateSqliteSubset}from"../🟦️.ts";
import integerFixture from "../🧫️fixtures/🧮️int64/🔣️.json";
import { objSnapshotToSqliteDatabase, objSnapshotFromSqliteDatabase, OBJ_SQLITE_SCHEMA } from "../🟦️.ts";
import type { ObjSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { parseObjArtifact,parseObjGroup,parseObjObject,parseObjUsemtlRange,parseObjSmoothingRange,parseObjFaceVertex,parseObjUnknownStatement as parseArtifactStatement } from "../../../../🧬️schema/🟦️.ts";
import { parseObjUnknownStatement as parseDiffStatement } from "../../../../🧬️schema/🔺️diff/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input: ObjSnapshot = { ...fixture, groups:fixture.groups.map(value=>({...value,faces:value.faces.map(BigInt)})),objects:fixture.objects.map(value=>({...value,faces:value.faces.map(BigInt)})),usemtl:fixture.usemtl.map(value=>({...value,faceIndexFrom:BigInt(value.faceIndexFrom)})),smoothingGroups:fixture.smoothingGroups.map(value=>({...value,faceIndexFrom:BigInt(value.faceIndexFrom)})), vertices:fixture.vertices.map(v=>({...v,x:binary64(v.x),y:binary64(v.y),z:binary64(v.z),w:"w" in v?binary64(v.w as number):undefined})),texcoords:fixture.texcoords.map(v=>({...v,u:binary64(v.u),v:binary64(v.v),w:"w" in v?binary64(v.w as number):undefined})),normals:fixture.normals.map(v=>({x:binary64(v.x),y:binary64(v.y),z:binary64(v.z)})), unknownStatements: fixture.unknownStatements.map(statement => ({ ...statement, lineIndex: BigInt(statement.lineIndex) })) };

test("OBJ canonical membership and range indices own full unsigned64 words",()=>{
  for(const value of sourceIndices.unsigned64Indices.map(BigInt)){
    for(const parse of[parseObjGroup,parseObjObject])expect<unknown>(parse({name:"exact",faces:[value]}).faces).toEqual([value]);
    expect<unknown>(parseObjUsemtlRange({faceIndexFrom:value,material:"exact"}).faceIndexFrom).toBe(value);
    expect<unknown>(parseObjSmoothingRange({faceIndexFrom:value}).faceIndexFrom).toBe(value);
  }
  for(const value of[0,"0",-1n,18446744073709551616n]){
    for(const parse of[parseObjGroup,parseObjObject])expect(()=>parse({name:"exact",faces:[value]})).toThrow("unsigned 64-bit");
    expect(()=>parseObjUsemtlRange({faceIndexFrom:value,material:"exact"})).toThrow("unsigned 64-bit");
    expect(()=>parseObjSmoothingRange({faceIndexFrom:value})).toThrow("unsigned 64-bit");
  }
  expect(parseObjFaceVertex({vertex:4294967295,texcoord:4294967295,normal:4294967295})).toEqual({vertex:4294967295,texcoord:4294967295,normal:4294967295});
  for(const vertex of[-1,4294967296,1.5])expect(()=>parseObjFaceVertex({vertex})).toThrow();
});

test("OBJ independently persisted unresolved source indices and short faces remain complete semantic states",async()=>{
  const value:ObjSnapshot={schema:sourceIndices.schema,vertices:[],texcoords:[],normals:[],faces:sourceIndices.faces,groups:sourceIndices.groups.map(value=>({...value,faces:value.faces.map(BigInt)})),objects:sourceIndices.objects.map(value=>({...value,faces:value.faces.map(BigInt)})),usemtl:sourceIndices.usemtl.map(value=>({...value,faceIndexFrom:BigInt(value.faceIndexFrom)})),smoothingGroups:sourceIndices.smoothingGroups.map(value=>({...value,faceIndexFrom:BigInt(value.faceIndexFrom)})),unknownStatements:[]};
  expect(parseObjArtifact(value)).toMatchObject(value);
  const database=await objSnapshotToSqliteDatabase(value);
  const sqlite=Database.deserialize(await exportSqliteDatabase(database));
  try{
    expect(sqlite.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(sqlite.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(await objSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sqlite.serialize())))).toEqual(value);
  }finally{sqlite.close();}
});

test("OBJ unsigned64 source words and optional resolved relationships survive independent SQL edits",async()=>{
 const indices=sourceIndices.unsigned64Indices.map(BigInt),snapshot:ObjSnapshot={...input,groups:[{name:"indices",faces:indices}],objects:[{name:"indices",faces:indices}],usemtl:indices.map(faceIndexFrom=>({faceIndexFrom,material:"indices"})),smoothingGroups:indices.map(faceIndexFrom=>({faceIndexFrom}))};
 const database=await objSnapshotToSqliteDatabase(snapshot),sqlite=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(sqlite.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(sqlite.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const expected=indices.map(index=>({high:Number(index>>32n),low:Number(index&4294967295n)}));
  for(const table of["obj_group_face","obj_object_face"])expect(sqlite.query(`SELECT face_source_index_high AS high,face_source_index_low AS low FROM ${table} ORDER BY ordinal`).all()).toEqual(expected);
  for(const table of["obj_material_range","obj_smoothing_range"])expect(sqlite.query(`SELECT first_face_source_index_high AS high,first_face_source_index_low AS low FROM ${table} ORDER BY ordinal`).all()).toEqual(expected);
  sqlite.run("UPDATE obj_group_face SET face_id=NULL WHERE ordinal=0");sqlite.run("UPDATE obj_material_range SET first_boundary_id=NULL WHERE ordinal=0");
  expect(await objSnapshotFromSqliteDatabase(await importSqliteDatabase(sqlite.serialize()))).toEqual(snapshot);
  sqlite.run("UPDATE obj_group_face SET face_source_index_high=4294967295,face_source_index_low=4294967294,face_id=NULL WHERE ordinal=1");
  const edited=await objSnapshotFromSqliteDatabase(await importSqliteDatabase(sqlite.serialize()));expect(edited.groups[0]!.faces[1]).toBe(18446744073709551614n);
  sqlite.run("UPDATE obj_face_vertex SET vertex_id=2 WHERE vertex_source_index=0");await expect(objSnapshotFromSqliteDatabase(await importSqliteDatabase(sqlite.serialize()))).rejects.toThrow("source index");
 }finally{sqlite.close();}
});

test("OBJ typed boundaries require owned unsigned64 BigInts without compatibility unions", () => {
  for (const parse of [parseArtifactStatement, parseDiffStatement]) {
    for (const lineIndex of [0n, 9007199254740993n, 18446744073709551615n]) expect(parse({ lineIndex, raw: "source" })).toEqual({ lineIndex, raw: "source" });
    for (const lineIndex of [0, "0", -1n, 18446744073709551616n]) expect(() => parse({ lineIndex, raw: "source" })).toThrow("unsigned 64-bit");
  }
});

test("OBJ source line ordinals retain the full unsigned64 domain through independent SQLite", async () => {
  const value: ObjSnapshot = { ...input, unknownStatements: integerFixture.sourceLineOrdinals.map(line => ({ lineIndex: BigInt(line), raw: "exact " + line })) };
  const database = await objSnapshotToSqliteDatabase(value as unknown as ObjSnapshot);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT source_line_ordinal_high AS high,source_line_ordinal_low AS low FROM obj_unknown_statement ORDER BY ordinal").all()).toEqual(integerFixture.sourceLineOrdinals.map(value => ({ high: Number(BigInt(value) >> 32n), low: Number(BigInt(value) & 4294967295n) })));
    db.run("UPDATE obj_unknown_statement SET source_line_ordinal_high=4294967295,source_line_ordinal_low=4294967294 WHERE ordinal=0");
    const edited = await objSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited.unknownStatements[0]!.lineIndex).toBe(18446744073709551614n);
    expect(edited.unknownStatements[2]!.lineIndex).toBe(9007199254740993n);
    expect(edited.unknownStatements[4]!.lineIndex).toBe(18446744073709551615n);
  } finally { db.close(); }
});

test("OBJ edited unsigned64 words reject values beyond either declared scalar width", async () => {
  for (const edit of ["UPDATE obj_unknown_statement SET source_line_ordinal_high=-1", "UPDATE obj_unknown_statement SET source_line_ordinal_low=4294967296"]) {
    const db = Database.deserialize(await exportSqliteDatabase(await objSnapshotToSqliteDatabase(input)));
    try {
      db.run("PRAGMA ignore_check_constraints=ON");
      db.run(edit);
      await expect(objSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("unsigned 32-bit");
    } finally { db.close(); }
  }
});

test("OBJ native geometry and ordered memberships expose shared handcrafted relational SQL", async () => {
  expect(OBJ_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  const database = await objSnapshotToSqliteDatabase(input);
  expect(await objSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT f.ordinal AS face,c.ordinal AS corner,v.ordinal AS vertex,t.ordinal AS texcoord,n.ordinal AS normal FROM obj_face f JOIN obj_face_vertex c ON c.face_id=f.id JOIN obj_vertex v ON v.id=c.vertex_id LEFT JOIN obj_texcoord t ON t.id=c.texcoord_id LEFT JOIN obj_normal n ON n.id=c.normal_id ORDER BY f.ordinal,c.ordinal").all()).toEqual(input.faces.flatMap((face, faceOrdinal) => face.vertices.map((vertex, corner) => ({ face: faceOrdinal, corner, vertex: vertex.vertex, texcoord: vertex.texcoord ?? null, normal: vertex.normal ?? null }))));
    expect(db.query("SELECT b.ordinal AS face_from,r.material FROM obj_material_range r JOIN obj_face_boundary b ON b.id=r.first_boundary_id ORDER BY r.ordinal").all()).toEqual(input.usemtl.map(range => ({ face_from: Number(range.faceIndexFrom), material: range.material })));
    expect(db.query("SELECT source_line_ordinal_high AS high,source_line_ordinal_low AS low,raw FROM obj_unknown_statement ORDER BY ordinal").all()).toEqual(input.unknownStatements.map(statement => ({ high: Number(statement.lineIndex >> 32n), low: Number(statement.lineIndex & 4294967295n), raw: statement.raw })));
    db.run("UPDATE obj_vertex SET x=6.125,x_ieee754_bits=4618582155356798976 WHERE ordinal=1");
    db.run("UPDATE obj_group SET name='SQL Gruppe 🌠' WHERE ordinal=0");
    const edited = await objSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited.vertices[1]!.x).toEqual(binary64(6.125));
    expect(edited.groups[0]!.name).toBe("SQL Gruppe 🌠");
    expect(edited.faces).toEqual(input.faces);
    expect(edited.smoothingGroups).toEqual(input.smoothingGroups);
  } finally { db.close(); }
});

test("OBJ independently edited dangling geometry, ownership, boundaries and ordinals reject", async () => {
  for (const edit of [
    "UPDATE obj_face_vertex SET vertex_id=999 WHERE id=1",
    "UPDATE obj_face_vertex SET texcoord_id=999 WHERE id=1",
    "UPDATE obj_face_vertex SET face_id=999 WHERE id=1",
    "UPDATE obj_face_vertex SET ordinal=99 WHERE id=1",
    "UPDATE obj_face_boundary SET face_id=1 WHERE ordinal=2",
    "UPDATE obj_material_range SET first_boundary_id=999 WHERE id=1",
    "UPDATE obj_group_face SET face_id=999 WHERE id=1",
    "UPDATE obj_object_face SET object_id=999 WHERE id=1",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await objSnapshotToSqliteDatabase(input)));
    try { db.run(edit); await expect(objSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("OBJ numeric, source-position and aggregate resource constraints apply before projection", async () => {
  for (const snapshot of [
    { ...input, vertices: [{ x: NaN, y: 0, z: 0 }] },
    { ...input, usemtl: [{ faceIndexFrom: -1n, material: "bad" }] },
    { ...input, smoothingGroups: [{ faceIndexFrom: 0n, group: 4294967296 }] },
    { ...input, unknownStatements: [{ lineIndex: -1n, raw: "bad" }] },
  ]) await expect(objSnapshotToSqliteDatabase(snapshot as unknown as ObjSnapshot)).rejects.toThrow();
  const database = await objSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 }, { maxValueBytes: 0 }]) {
    await expect(objSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(objSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
  const empty: ObjSnapshot = { schema: "empty.obj", vertices: [], texcoords: [], normals: [], faces: [], groups: [], objects: [], usemtl: [], smoothingGroups: [], unknownStatements: [] };
  expect(await objSnapshotFromSqliteDatabase(await objSnapshotToSqliteDatabase(empty))).toEqual(empty);
});

test("OBJ counting scans and reconstruction observe cancellation", async () => {
  const controller = new AbortController();
  let events = 0;
  await expect(objSnapshotToSqliteDatabase({ ...input, unknownStatements: Array.from({ length: 1000 }, (_, lineIndex) => ({ lineIndex: BigInt(lineIndex), raw: "source" })) }, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ kind: canceledKind });
  const database = await objSnapshotToSqliteDatabase(input);
  const reconstruct = new AbortController();
  await expect(objSnapshotFromSqliteDatabase(database, { signal: reconstruct.signal, onProgress: () => reconstruct.abort() })).rejects.toMatchObject({ kind: canceledKind });
});
for(const [index,hex] of ieee.binary64Bits.entries())test(`OBJ scalar word ${hex} survives independent SQLite edits and companion validation`, async () => {
    const bits=BigInt("0x"+hex);
    const scalar={bits};const snapshot:ObjSnapshot={...input,vertices:input.vertices.map(v=>({...v,x:scalar,y:scalar,z:scalar,w:scalar})),texcoords:[{u:scalar,v:scalar,w:undefined},{u:scalar,v:scalar,w:scalar}],normals:input.normals.map(()=>({x:scalar,y:scalar,z:scalar}))};
    const db=Database.deserialize(await exportSqliteDatabase(await objSnapshotToSqliteDatabase(snapshot)));
    try{
      expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
      expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
      expect(db.query("SELECT CAST(x_ieee754_bits AS TEXT) AS bits,x_numeric_class AS kind FROM obj_vertex WHERE id=1").get()).toEqual({bits:BigInt.asIntN(64,bits).toString(),kind:ieee.classes[index]});
      const restored=await objSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
      expect(restored.vertices[0]!.x.bits).toBe(bits);
      expect(restored).toEqual(snapshot);
      db.run("UPDATE obj_vertex SET x=0,x_ieee754_bits=-9223372036854775808,x_numeric_class='finite' WHERE id=1");
      const edited=await objSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
      expect(edited.vertices[0]!.x.bits).toBe(0x8000000000000000n);
      db.run("UPDATE obj_vertex SET x=NULL,x_numeric_class='nan' WHERE id=1");
      await expect(objSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();
    }finally{db.close();}
});
test("OBJ exact owned dialect retains the complete independent snapshot", async()=>{
  const database=await objSnapshotToSqliteDatabase(input);
  await objSnapshotValidateSqliteSubset(input,ieee.sqliteDialect,database);
});
test("OBJ consistently renumbered signed surrogate identities preserve the same logical snapshot",async()=>{
 const sqlite=Database.deserialize(await exportSqliteDatabase(await objSnapshotToSqliteDatabase(input)));
 try{
  sqlite.run("PRAGMA foreign_keys=OFF");
  for(const statement of surrogateIndices.statements)sqlite.run(statement);
  expect(sqlite.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(sqlite.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const database=await importSqliteDatabase(sqlite.serialize());
  expect(await objSnapshotFromSqliteDatabase(database)).toEqual(input);
  await objSnapshotValidateSqliteSubset(input,ieee.sqliteDialect,database);
 }finally{sqlite.close();}
});
test("OBJ invalid dialect coordinates refuse before materialization",async()=>{
  const database=await objSnapshotToSqliteDatabase(input);
  for(const dialect of ieee.invalidSqliteDialects)await expect(objSnapshotValidateSqliteSubset(input,dialect,database)).rejects.toThrow("dialect");
});
test("OBJ complete state mismatch and cancellation refuse",async()=>{
  const database=await objSnapshotToSqliteDatabase(input);
  await expect(objSnapshotValidateSqliteSubset({...input,schema:"different"},ieee.sqliteDialect,database)).rejects.toThrow("identity");
  const controller=new AbortController();controller.abort();
  await expect(objSnapshotValidateSqliteSubset(input,ieee.sqliteDialect,database,{signal:controller.signal})).rejects.toMatchObject({kind:canceledKind});
});

import NormSemanticAjv from "ajv/dist/2020.js";
import normSemanticContract from "../🧫️fixtures/🎛️semantic.json";

import {objSnapshotToSqliteDatabase as normSemanticProject,objSnapshotFromSqliteDatabase as normSemanticRestore} from "../🟦️.ts";
import {exportSqliteDatabase as normSemanticExport,importSqliteDatabase as normSemanticImport} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {independentSqliteExtent as normIndependentExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";
test("OBJ complete independent cells preserve copied limits and required empty metadata",async()=>{
 expect(normSemanticContract["schemaBytes"]).toEqual(6231);expect(normSemanticContract["tableWidths"]).toEqual({"obj_document":3,"obj_face":3,"obj_face_boundary":4,"obj_face_vertex":9,"obj_group":4,"obj_group_face":6,"obj_material_range":7,"obj_normal":12,"obj_object":4,"obj_object_face":6,"obj_smoothing_range":7,"obj_texcoord":12,"obj_unknown_statement":6,"obj_vertex":15});expect(normSemanticContract["cases"]).toEqual([{"id":"originalSharedSourceNative","rows":34,"valueBytes":1830},{"id":"emptyCollectionsRetainMetadata","rows":2,"valueBytes":60},{"id":"originalSourceEmpty","rows":2,"valueBytes":41}]);
 for(const item of normSemanticContract.cases){const source=structuredClone(input);if(item.id!=="originalSharedSourceNative"){source.vertices=[];source.texcoords=[];source.normals=[];source.faces=[];source.groups=[];source.objects=[];source.usemtl=[];source.smoothingGroups=[];source.unknownStatements=[];if(item.id==="originalSourceEmpty"){source.schema="empty.obj";source.mtllib=undefined;}}const database=await normSemanticProject(source),bytes=await normSemanticExport(database);expect(normIndependentExtent(bytes)).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:normSemanticContract.schemaBytes,tableWidths:normSemanticContract.tableWidths});const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:normSemanticContract.schemaBytes,maxTables:14,maxColumns:15};expect(await normSemanticProject(source,limits)).toEqual(database);expect(await normSemanticRestore(await normSemanticImport(bytes),limits)).toEqual(source);
  for(const short of[{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:13},{...limits,maxColumns:14}]){await expect(normSemanticProject(source,short)).rejects.toThrow();await expect(normSemanticRestore(database,short)).rejects.toThrow();}
 }console.log("[DEBUG] OBJ complete independent SQLite cells and metadata preserve full and empty semantic limits");
});
