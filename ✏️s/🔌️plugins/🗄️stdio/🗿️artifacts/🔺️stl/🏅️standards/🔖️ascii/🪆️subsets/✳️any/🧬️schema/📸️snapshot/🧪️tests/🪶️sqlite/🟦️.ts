import refusalCorpus from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧫️fixtures/🔣️.json";
const canceledKind=refusalCorpus.cases.find(c=>c.id==="canceled-projection")!.expectedKind;
import { binary64,binary32,binary64Value,binary32Value,type Binary64 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🧫️ Shared STL triangle fixture and independent scalar SQL interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import ieee from "../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json";
import {parseStlTriangle,parseStlArtifact}from"../../../🟦️.ts";
import {parseStlTriangleDiff,parseStlTriangleAdded}from"../../../🔺️diff/🟦️.ts";
import {stlSnapshotValidateSqliteSubset}from"../../🪶️sqlite/🟦️.ts";
import type { StlSnapshot } from "../../🟦️.ts";
import { stlSnapshotToSqliteDatabase, stlSnapshotFromSqliteDatabase, STL_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const tuple=(values:readonly number[]):[Binary64,Binary64,Binary64]=>[binary64(values[0]!),binary64(values[1]!),binary64(values[2]!)];
const input:StlSnapshot={...fixture,triangles:fixture.triangles.map(triangle=>({normal:tuple(triangle.normal),vertices:[tuple(triangle.vertices[0]!),tuple(triangle.vertices[1]!),tuple(triangle.vertices[2]!)]}))};

test("STL facet normals and vertex entities expose the shared handcrafted schema", async () => {
  expect(STL_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const database = await stlSnapshotToSqliteDatabase(input);
  expect(await stlSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT f.ordinal AS facet,v.ordinal AS vertex,v.x,v.y,v.z FROM stl_facet f JOIN stl_vertex v ON v.facet_id=f.id ORDER BY f.ordinal,v.ordinal").all()).toEqual(input.triangles.flatMap((triangle, facet) => triangle.vertices.map(([x,y,z], vertex) => ({ facet,vertex,x:binary64Value(x),y:binary64Value(y),z:binary64Value(z) }))));
    db.run("UPDATE stl_vertex SET x=5.125,x_ieee754_bits=4617456255449956352 WHERE facet_id=1 AND ordinal=1");
    const edited = await stlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited.triangles[0]!.vertices[1][0]).toEqual(binary64(5.125));
    expect(edited.triangles[0]!.normal).toEqual([binary64(0),binary64(0),binary64(0)]);
  } finally { db.close(); }
});

test("STL independent missing vertices, references and ordinals reject", async () => {
  for (const edit of ["DELETE FROM stl_vertex WHERE id=1", "UPDATE stl_vertex SET facet_id=999 WHERE id=1", "UPDATE stl_vertex SET ordinal=1 WHERE id=1", "UPDATE stl_facet SET solid_id=999 WHERE id=1"]) {
    const db = Database.deserialize(await exportSqliteDatabase(await stlSnapshotToSqliteDatabase(input)));
    try { db.run(edit); await expect(stlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("STL empty solids, owned scalar words and aggregate resource budgets", async () => {
  const empty = { ...input, triangles: [] };
  expect(await stlSnapshotFromSqliteDatabase(await stlSnapshotToSqliteDatabase(empty))).toEqual(empty);
  const database = await stlSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 }, { maxValueBytes: 0 }]) {
    await expect(stlSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(stlSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
});

test("STL projection and reconstruction cancel at bounded semantic intervals", async () => {
  const controller = new AbortController();
  let events = 0;
  await expect(stlSnapshotToSqliteDatabase({ ...input, triangles: Array.from({ length: 512 }, () => input.triangles[0]!) }, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ kind: canceledKind });
  const database = await stlSnapshotToSqliteDatabase(input);
  const reconstruct = new AbortController();
  await expect(stlSnapshotFromSqliteDatabase(database, { signal: reconstruct.signal, onProgress: () => reconstruct.abort() })).rejects.toMatchObject({ kind: canceledKind });
});
test("STL exact scalar words survive independent SQLite affinity, edits and malformed companions", async () => {
  for(const [index,hex] of ieee.binary64Bits.entries()){
    const bits=BigInt("0x"+hex);
    const scalar={bits}; const snapshot:StlSnapshot={schema:"exact",solidName:"",triangles:[{normal:[scalar,scalar,scalar],vertices:[[scalar,scalar,scalar],[scalar,scalar,scalar],[scalar,scalar,scalar]]}]};
    const db=Database.deserialize(await exportSqliteDatabase(await stlSnapshotToSqliteDatabase(snapshot)));
    try{
      expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
      expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
      expect(db.query("SELECT CAST(normal_x_ieee754_bits AS TEXT) AS bits,normal_x_numeric_class AS kind FROM stl_facet WHERE id=1").get()).toEqual({bits:BigInt.asIntN(64,bits).toString(),kind:ieee.classes[index]});
      const restored=await stlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
      expect(restored.triangles[0]!.normal[0].bits).toBe(bits);
      expect(restored).toEqual(snapshot);
      db.run("UPDATE stl_facet SET normal_x=0,normal_x_ieee754_bits=-9223372036854775808,normal_x_numeric_class='finite' WHERE id=1");
      const edited=await stlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
      expect(edited.triangles[0]!.normal[0].bits).toBe(0x8000000000000000n);
      db.run("UPDATE stl_facet SET normal_x=NULL,normal_x_numeric_class='nan' WHERE id=1");
      await expect(stlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();
    }finally{db.close();}
  }
  console.log("[DEBUG] STL exact binary words and independent SQLite identity laws");
});
test("STL exact owned dialect, document identity and cancellation laws", async()=>{
  const database=await stlSnapshotToSqliteDatabase(input);
  await stlSnapshotValidateSqliteSubset(input,ieee.sqliteDialect,database);
  for(const dialect of ieee.invalidSqliteDialects)await expect(stlSnapshotValidateSqliteSubset(input,dialect,database)).rejects.toThrow("dialect");
  await expect(stlSnapshotValidateSqliteSubset({...input,schema:"different"},ieee.sqliteDialect,database)).rejects.toThrow("identity");
  const controller=new AbortController();controller.abort();
  await expect(stlSnapshotValidateSqliteSubset(input,ieee.sqliteDialect,database,{signal:controller.signal})).rejects.toMatchObject({kind:canceledKind});
});
test("STL artifact, snapshot and sparse diff callers share exact owned coordinate words",()=>{
  const triangle=input.triangles[0]!;
  expect(parseStlTriangle(triangle)).toEqual(triangle);
  expect(parseStlArtifact({schema:"empty"})).toEqual({schema:"empty",solidName:"",triangles:[]});
  expect(parseStlTriangleDiff({normal:triangle.normal,vertices:triangle.vertices})).toEqual(triangle);
  expect(parseStlTriangleAdded({index:0,triangle})).toEqual({index:0,triangle});
  expect(()=>parseStlTriangle({normal:[0,0,0],vertices:[[0,0,0],[0,0,0],[0,0,0]]})).toThrow("binary64");
});

test("STL same-schema changed semantic state refuses while surrogate renumbering remains valid",async()=>{
 const db=Database.deserialize(await exportSqliteDatabase(await stlSnapshotToSqliteDatabase(input)));
 try{db.run("UPDATE stl_solid SET name='changed'");expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(stlSnapshotValidateSqliteSubset(input,ieee.sqliteDialect,await importSqliteDatabase(db.serialize()))).rejects.toThrow("identity");}finally{db.close();}
 const renumbered=Database.deserialize(await exportSqliteDatabase(await stlSnapshotToSqliteDatabase(input)));
 try{renumbered.run("UPDATE stl_solid SET id=-19");renumbered.run("UPDATE stl_facet SET solid_id=-19,id=id+100");renumbered.run("UPDATE stl_vertex SET facet_id=facet_id+100,id=id+300");expect(renumbered.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(renumbered.query("PRAGMA foreign_key_check").all()).toEqual([]);await stlSnapshotValidateSqliteSubset(input,ieee.sqliteDialect,await importSqliteDatabase(renumbered.serialize()));}finally{renumbered.close();}
});
