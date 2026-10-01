import { binary64,binary32,binary64Value,binary32Value,type Binary64 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🧫️ Shared OBJ geometry and membership fixture with independent SQL edits. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import ieee from "../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json";
import {objSnapshotValidateSqliteSubset}from"../../🪶️sqlite/🟦️.ts";
import integerFixture from "../../🧫️fixtures/🪶️sqlite/🧮️int64/🔣️.json";
import { objSnapshotToSqliteDatabase, objSnapshotFromSqliteDatabase, OBJ_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import type { ObjSnapshot } from "../../🟦️.ts";
import { parseObjUnknownStatement as parseArtifactStatement } from "../../../🟦️.ts";
import { parseObjUnknownStatement as parseDiffStatement } from "../../../🔺️diff/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input: ObjSnapshot = { ...fixture, vertices:fixture.vertices.map(v=>({...v,x:binary64(v.x),y:binary64(v.y),z:binary64(v.z),w:"w" in v?binary64(v.w as number):undefined})),texcoords:fixture.texcoords.map(v=>({...v,u:binary64(v.u),v:binary64(v.v),w:"w" in v?binary64(v.w as number):undefined})),normals:fixture.normals.map(v=>({x:binary64(v.x),y:binary64(v.y),z:binary64(v.z)})), unknownStatements: fixture.unknownStatements.map(statement => ({ ...statement, lineIndex: BigInt(statement.lineIndex) })) };


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
  expect(OBJ_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const database = await objSnapshotToSqliteDatabase(input);
  expect(await objSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT f.ordinal AS face,c.ordinal AS corner,v.ordinal AS vertex,t.ordinal AS texcoord,n.ordinal AS normal FROM obj_face f JOIN obj_face_vertex c ON c.face_id=f.id JOIN obj_vertex v ON v.id=c.vertex_id LEFT JOIN obj_texcoord t ON t.id=c.texcoord_id LEFT JOIN obj_normal n ON n.id=c.normal_id ORDER BY f.ordinal,c.ordinal").all()).toEqual(input.faces.flatMap((face, faceOrdinal) => face.vertices.map((vertex, corner) => ({ face: faceOrdinal, corner, vertex: vertex.vertex, texcoord: vertex.texcoord ?? null, normal: vertex.normal ?? null }))));
    expect(db.query("SELECT b.ordinal AS face_from,r.material FROM obj_material_range r JOIN obj_face_boundary b ON b.id=r.first_boundary_id ORDER BY r.ordinal").all()).toEqual(input.usemtl.map(range => ({ face_from: range.faceIndexFrom, material: range.material })));
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
    "DELETE FROM obj_face_vertex WHERE face_id=2 AND ordinal=2",
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
    { ...input, usemtl: [{ faceIndexFrom: input.faces.length + 1, material: "bad" }] },
    { ...input, smoothingGroups: [{ faceIndexFrom: 0, group: 4294967296 }] },
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
  await expect(objSnapshotToSqliteDatabase({ ...input, unknownStatements: Array.from({ length: 1000 }, (_, lineIndex) => ({ lineIndex: BigInt(lineIndex), raw: "source" })) }, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ name: "AbortError" });
  const database = await objSnapshotToSqliteDatabase(input);
  const reconstruct = new AbortController();
  await expect(objSnapshotFromSqliteDatabase(database, { signal: reconstruct.signal, onProgress: () => reconstruct.abort() })).rejects.toMatchObject({ name: "AbortError" });
});
test("OBJ exact scalar words survive independent SQLite affinity, edits and malformed companions", async () => {
  for(const [index,hex] of ieee.binary64Bits.entries()){
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
  }
  console.log("[DEBUG] OBJ exact binary words and independent SQLite identity laws");
});
test("OBJ exact owned dialect, document identity and cancellation laws", async()=>{
  const database=await objSnapshotToSqliteDatabase(input);
  await objSnapshotValidateSqliteSubset(input,ieee.sqliteDialect,database);
  for(const dialect of ieee.invalidSqliteDialects)await expect(objSnapshotValidateSqliteSubset(input,dialect,database)).rejects.toThrow("dialect");
  await expect(objSnapshotValidateSqliteSubset({...input,schema:"different"},ieee.sqliteDialect,database)).rejects.toThrow("identity");
  const controller=new AbortController();controller.abort();
  await expect(objSnapshotValidateSqliteSubset(input,ieee.sqliteDialect,database,{signal:controller.signal})).rejects.toMatchObject({name:"AbortError"});
});
