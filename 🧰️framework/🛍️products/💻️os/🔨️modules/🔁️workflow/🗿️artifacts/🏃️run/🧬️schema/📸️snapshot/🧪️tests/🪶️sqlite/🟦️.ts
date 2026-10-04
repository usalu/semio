import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import fixture from"../../🧫️fixtures/🪶️sqlite/🔣️.json";
import lawsSchema from"../../🧫️fixtures/🪶️sqlite/🧬️schema/🔣️.json";
import nativeSchema from"../../🔣️.json";
import * as owner from"../../🟦️.ts";
import{binary64}from"../../../../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase,sqliteOperation,type SqliteDatabase}from"../../../../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type{ArtifactSqliteOptions}from"../../../../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
type Snapshot=owner.RunSnapshot;
type Ports={runSnapshotToSqliteDatabase(value:Snapshot,options?:ArtifactSqliteOptions):Promise<SqliteDatabase>;runSnapshotFromSqliteDatabase(database:SqliteDatabase,options?:ArtifactSqliteOptions):Promise<Snapshot>};
const ports=owner as unknown as Ports;
function snapshot():Snapshot{return{...structuredClone(fixture.snapshot),status:"canceled",trigger:{...fixture.snapshot.trigger,kind:"automation"},nodeRecords:fixture.snapshot.nodeRecords.map((node,index)=>({...structuredClone(node),status:fixture.nodeStatuses[index]as owner.RunNodeStatus,durationMs:binary64(node.durationMs)}))}}
const bytes=async(value:Snapshot)=>exportSqliteDatabase(await ports.runSnapshotToSqliteDatabase(value));
const restore=async(value:Uint8Array,options:ArtifactSqliteOptions={})=>ports.runSnapshotFromSqliteDatabase(await importSqliteDatabase(value),options);
test("Run neutral schema independently admits complete root ownership and both trigger branches",()=>{const validator=new Ajv({strict:false,validateFormats:false}).addSchema(nativeSchema);expect(validator.validate(lawsSchema,fixture)).toBe(true)});
test("Run independent SQLite DDL declares seven semantic tables and exact duration columns",async()=>{const db=new Database(":memory:");try{db.run(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());expect(db.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").all()).toEqual(["run_document","run_log","run_node","run_output_artifact","run_parameter_value","run_port_fingerprint","run_trigger"].map(name=>({name})));expect(db.query("PRAGMA table_info(run_document)").all().length).toBe(11);expect(db.query("PRAGMA table_info(run_node)").all().length).toBe(10);expect(db.query("SELECT name FROM sqlite_master WHERE type='index'").all()).toEqual([])}finally{db.close()}});
test("Run public persisted facade exposes both complete relational directions",()=>{expect(Object.hasOwn(owner,"runSnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(owner,"runSnapshotFromSqliteDatabase")).toBe(true)});
test("Run semantic files preserve duplicate literal identities and all ordered ownership",async()=>{const expected=snapshot(),data=await bytes(expected),db=Database.deserialize(data,{safeIntegers:true});try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT status FROM run_node ORDER BY ordinal").all()).toEqual([{status:0n},{status:1n},{status:2n}]);expect(db.query("SELECT port_id,fingerprint FROM run_port_fingerprint WHERE direction='input' ORDER BY ordinal").all()).toEqual([{port_id:"",fingerprint:"one"},{port_id:"",fingerprint:"two"}]);expect(db.query("SELECT value FROM run_parameter_value ORDER BY ordinal").all()).toEqual(fixture.snapshot.parameterValues.map(row=>({value:row.value})));expect(await restore(data)).toEqual(expected)}finally{db.close()}});
for(const hex of fixture.binary64Words)test("Run every duration retains exact binary64 "+hex,async()=>{const expected=snapshot(),word={bits:BigInt("0x"+hex)};for(const node of expected.nodeRecords)node.durationMs=word;const db=Database.deserialize(await bytes(expected),{safeIntegers:true});try{const buffer=Buffer.alloc(8);buffer.writeBigUInt64BE(word.bits);const value=buffer.readDoubleBE(),kind=Number.isNaN(value)?"nan":value===Infinity?"positiveInfinity":value===-Infinity?"negativeInfinity":"finite";expect(db.query("SELECT duration_ms_bits,duration_ms_class FROM run_node ORDER BY ordinal").all()).toEqual(expected.nodeRecords.map(()=>({duration_ms_bits:BigInt.asIntN(64,word.bits),duration_ms_class:kind})));expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected)}finally{db.close()}});
test("Run independent surrogate renumbering and scalar edits preserve semantic ownership",async()=>{const expected=snapshot(),db=Database.deserialize(await bytes(expected));try{db.run("UPDATE run_node SET id=id+100,config_fingerprint='edited'");db.run("UPDATE run_port_fingerprint SET node_id=node_id+100");db.run("UPDATE run_output_artifact SET node_id=node_id+100");db.run("UPDATE run_trigger SET id=id+101");db.run("UPDATE run_parameter_value SET id=id+103");db.run("UPDATE run_log SET id=id+107");for(const node of expected.nodeRecords)node.configFingerprint="edited";expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected)}finally{db.close()}});
for(const[index,sql]of fixture.malformedSql.entries())test("Run independent malformed semantic edit rejects "+index,async()=>{const db=Database.deserialize(await bytes(snapshot()));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(restore(new Uint8Array(db.serialize()))).rejects.toThrow()}finally{db.close()}});
test("Run every status and trigger branch retains present empty and absent finished time",async()=>{for(const status of fixture.statuses){for(const trigger of fixture.triggers){for(const finishedAt of[undefined,"","\0😀"]){const expected=snapshot();expected.status=status as owner.RunStatus;expected.trigger=trigger as owner.RunTrigger;if(finishedAt===undefined)delete expected.finishedAt;else expected.finishedAt=finishedAt;expect(await restore(await bytes(expected))).toEqual(expected)}}}});
test("Run exact known row admission accepts equality and refuses one below",async()=>{const expected=snapshot(),db=await ports.runSnapshotToSqliteDatabase(expected),count=db.tables.reduce((sum,table)=>sum+table.rows.length,0);expect(count).toBe(13);await expect(ports.runSnapshotToSqliteDatabase(expected,{maxRows:count})).resolves.toBeDefined();await expect(ports.runSnapshotToSqliteDatabase(expected,{maxRows:count-1})).rejects.toThrow();await expect(ports.runSnapshotFromSqliteDatabase(db,{maxRows:count-1})).rejects.toThrow()});
for(const phase of["projectSnapshot","reconstructSnapshot"]as const)test("Run caller cancellation interrupts known ordered ownership in "+phase,async()=>{const expected=snapshot();expected.parameterValues=Array.from({length:fixture.control.collectionLength},()=>({parameterId:"",value:""}));const db=await ports.runSnapshotToSqliteDatabase(expected),controller=new AbortController();let reached=false;const options:ArtifactSqliteOptions={signal:controller.signal,onProgress:event=>{if(event.phase===phase&&event.completed>=256&&event.completed<event.total){reached=true;controller.abort()}}};await expect(phase==="projectSnapshot"?ports.runSnapshotToSqliteDatabase(expected,options):ports.runSnapshotFromSqliteDatabase(db,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);const initial=new AbortController();initial.abort();await expect(phase==="projectSnapshot"?ports.runSnapshotToSqliteDatabase(snapshot(),{signal:initial.signal}):ports.runSnapshotFromSqliteDatabase(db,{signal:initial.signal})).rejects.toHaveProperty("kind","canceled")});
test("Run small cells retain exact concrete ordinal backing and cumulative prior debt",async()=>{
 const expected=snapshot();expected.parameterValues=Array.from({length:fixture.control.collectionLength},()=>({parameterId:"",value:""}));
 const db=await ports.runSnapshotToSqliteDatabase(expected),frontierBytes=(db.tables.reduce((sum,table)=>sum+table.rows.length,0)-2)*8;let cellBytes=0;
 for(const table of db.tables)for(const row of table.rows)for(const value of row.values)cellBytes+=typeof value==="string"?Buffer.byteLength(value,"utf8"):value===null?0:value instanceof Uint8Array?value.byteLength:8;
 expect(cellBytes).toBeLessThan(fixture.control.maxOwnedBytes);
 for(const maximum of[frontierBytes,frontierBytes-1]){const operation=sqliteOperation({maxValueBytes:fixture.control.maxOwnedBytes,maxAllocationBytes:maximum});if(maximum===frontierBytes){expect(await ports.runSnapshotFromSqliteDatabase(db,operation)).toEqual(expected);expect(operation.ownedBytes).toBe(frontierBytes);}else await expect(ports.runSnapshotFromSqliteDatabase(db,operation)).rejects.toHaveProperty("kind","ownershipLimit");}
 const priorDebt=19,operation=sqliteOperation({maxValueBytes:fixture.control.maxOwnedBytes,maxAllocationBytes:priorDebt+frontierBytes*2});operation.allocateBytes(priorDebt);
 for(let replay=1;replay<=2;replay++){expect(await ports.runSnapshotFromSqliteDatabase(db,operation)).toEqual(expected);expect(operation.ownedBytes).toBe(priorDebt+frontierBytes*replay);}
 await expect(ports.runSnapshotFromSqliteDatabase(db,operation)).rejects.toHaveProperty("kind","ownershipLimit");
});
test("Run requested file and value ceilings refuse oversized ownership",async()=>{const expected=snapshot(),db=await ports.runSnapshotToSqliteDatabase(expected);await expect(exportSqliteDatabase(db,{maxFileBytes:fixture.control.tinyFileBytes})).rejects.toThrow();await expect(ports.runSnapshotFromSqliteDatabase(db,{maxValueBytes:1})).rejects.toThrow();expected.logs[0]!.message="😀".repeat(fixture.control.largeCharacters);await expect(ports.runSnapshotToSqliteDatabase(expected,{maxValueBytes:fixture.control.maxOwnedBytes})).rejects.toThrow()});
for(const phase of["projectSnapshot","reconstructSnapshot"]as const)test("Run long Unicode measurement cancels inside its known code-unit frontier in "+phase,async()=>{const expected=snapshot(),text="😀".repeat(fixture.control.largeCharacters);expect(Buffer.byteLength(text,"utf8")).toBe(text.length*2);expected.logs[0]!.message=text;const database=await ports.runSnapshotToSqliteDatabase(expected),controller=new AbortController();let reached=false;const options:ArtifactSqliteOptions={signal:controller.signal,onProgress:event=>{if(event.phase===phase&&event.total===text.length&&event.completed>=fixture.control.cancelAfterBytes/2&&event.completed<event.total){reached=true;controller.abort()}}};await expect(phase==="projectSnapshot"?ports.runSnapshotToSqliteDatabase(expected,options):ports.runSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true)});
import requestSettlement from '../../🧫️fixtures/🪶️sqlite/💰️backing/🔬️requests/🔣️.json';
import requestSettlementSchema from '../../🧫️fixtures/🪶️sqlite/💰️backing/🔬️requests/🧬️schema/🔣️.json';
test('Run full concrete request contract retains all literal field and IEEE owners in physical SQLite', async () => {
  expect(new Ajv({strict: true}).validate(requestSettlementSchema, requestSettlement)).toBe(true);
  expect(Object.keys(fixture.snapshot)).toEqual(requestSettlement.fieldOrder);
  const expected = snapshot(), database = await ports.runSnapshotToSqliteDatabase(expected);
  expect(database.tables.length).toBe(requestSettlement.tableCount);
  for (const hex of fixture.binary64Words) {
    const word = {bits: BigInt('0x' + hex)};
    for (const node of expected.nodeRecords) node.durationMs = word;
    const sql = Database.deserialize(await bytes(expected), {safeIntegers: true});
    try {
      expect(sql.query('SELECT duration_ms_bits FROM run_node ORDER BY ordinal').all()).toEqual(expected.nodeRecords.map(() => ({duration_ms_bits: BigInt.asIntN(64, word.bits)})));
      expect(await restore(new Uint8Array(sql.serialize()))).toEqual(expected);
    } finally { sql.close(); }
  }
  for (const operation of [() => ports.runSnapshotToSqliteDatabase(expected, {maxAllocationBytes: 0}), () => ports.runSnapshotFromSqliteDatabase(database, {maxAllocationBytes: 0})]) await expect(operation()).rejects.toHaveProperty('kind', 'ownershipLimit');
});
test('Run exact semantic scalar allowance is measured independently of reconstruction workspace', async () => {
  const expected = snapshot(), database = await ports.runSnapshotToSqliteDatabase(expected), sql = Database.deserialize(await exportSqliteDatabase(database), {safeIntegers: true});
  let scalarBytes = 0;
  try {
    for (const table of database.tables) for (const row of sql.query('SELECT * FROM "' + table.name.replaceAll('"', '""') + '"').all() as Record<string, unknown>[]) for (const value of Object.values(row)) scalarBytes += value === null ? 0 : typeof value === 'string' ? Buffer.byteLength(value, 'utf8') : value instanceof Uint8Array ? value.byteLength : 8;
  } finally { sql.close(); }
  await expect(ports.runSnapshotFromSqliteDatabase(database, {maxValueBytes: scalarBytes})).resolves.toEqual(expected);
  await expect(ports.runSnapshotFromSqliteDatabase(database, {maxValueBytes: scalarBytes - 1})).rejects.toHaveProperty('kind', 'ownershipLimit');
});
for (const phase of ['projectSnapshot', 'reconstructSnapshot'] as const) test('Run immutable entry controls retain the original cancellation authority in ' + phase, async () => {
  const expected = snapshot(), database = await ports.runSnapshotToSqliteDatabase(expected), aborted = new AbortController(); aborted.abort();
  let changed = false;
  const options: {signal?:AbortSignal;onProgress:NonNullable<ArtifactSqliteOptions['onProgress']>} = {onProgress(event) {if (event.phase === phase && !changed) {changed = true; options.signal = aborted.signal;}}};
  const result = phase === 'projectSnapshot' ? await ports.runSnapshotToSqliteDatabase(expected, options) : await ports.runSnapshotFromSqliteDatabase(database, options);
  expect(changed).toBe(true);
  expect(result).toEqual(phase === 'projectSnapshot' ? database : expected);
});

import boundedFrontiers from '../../🧫️fixtures/🪶️sqlite/🎛️frontiers/🔣️.json';
import boundedFrontiersSchema from '../../🧫️fixtures/🪶️sqlite/🎛️frontiers/🧬️schema/🔣️.json';
function boundedSnapshot(role:string):Snapshot {
 const value=snapshot(),node=value.nodeRecords[0]!;value.parameterValues=[];value.logs=[];value.nodeRecords=[node];node.inputFingerprints=[];node.outputFingerprints=[];node.outputs=[];
 const count=boundedFrontiers.collectionLength;
 if(role==='artifactMaterialization')node.outputs=Array.from({length:count},(_,index)=>({portId:'p'+index,artifactId:'a'+index,path:''}));
 else node[role==='outputMaterialization'?'outputFingerprints':'inputFingerprints']=Array.from({length:count},(_,index)=>({portId:'p'+index,fingerprint:'f'+index}));
 return value;
}
test('Run bounded frontier contract independently preserves every child domain in SQLite',async()=>{
 expect(new Ajv({strict:true}).validate(boundedFrontiersSchema,boundedFrontiers)).toBe(true);
 for(const role of boundedFrontiers.roles){const expected=boundedSnapshot(role),database=await ports.runSnapshotToSqliteDatabase(expected),sql=Database.deserialize(await exportSqliteDatabase(database),{safeIntegers:true});try{
  expect(database.tables.length).toBe(boundedFrontiers.tableCount);
  expect(sql.query('PRAGMA foreign_key_check').all()).toEqual([]);
  const table=role==='artifactMaterialization'?'run_output_artifact':'run_port_fingerprint';
  expect(sql.query('SELECT count(*) AS count FROM '+table).get()).toEqual({count:BigInt(boundedFrontiers.collectionLength)});
  expect(sql.query('SELECT port_id FROM '+table+' ORDER BY ordinal').all()).toEqual(Array.from({length:boundedFrontiers.collectionLength},(_,index)=>({port_id:'p'+index})));
  expect(await ports.runSnapshotFromSqliteDatabase(database)).toEqual(expected);
 }finally{sql.close();}}
});
for(const role of boundedFrontiers.roles)test('Run later borrowed child frontier is cancellable within its exact read bound in '+role,async()=>{
 const expected=boundedSnapshot(role),source=await ports.runSnapshotToSqliteDatabase(expected),controller=new AbortController();let armed=false,reads=0,placementEnds=0;
 const tableName=role==='artifactMaterialization'?'run_output_artifact':'run_port_fingerprint',column=role==='directionPartition'?2:role==='artifactMaterialization'?3:4;
 const database:SqliteDatabase={tables:source.tables.map(table=>table.name!==tableName?table:{...table,rows:table.rows.map(row=>{const values=[...row.values],scalar=values[column];Object.defineProperty(values,column,{get(){if(armed)reads++;return scalar;},enumerable:true,configurable:true});return{...row,values};})})};
 await expect(ports.runSnapshotFromSqliteDatabase(database,{signal:controller.signal,onProgress(event){
  if(event.phase!=='reconstructSnapshot')return;
  if(role==='directionPartition'&&event.total===1&&event.completed===0)armed=true;
  if(role!=='directionPartition'&&event.total===boundedFrontiers.collectionLength&&event.completed===768&&++placementEnds===2)armed=true;
  if(armed&&reads>0)controller.abort();
 }})).rejects.toHaveProperty('kind','canceled');
 expect(armed).toBe(true);expect(reads).toBeGreaterThan(0);expect(reads).toBeLessThanOrEqual(boundedFrontiers.maxReadsBetweenControls);
});

import intrinsicRefusals from '../../🧫️fixtures/🪶️sqlite/⚠️refusals/🔣️.json';
import intrinsicRefusalsSchema from '../../🧫️fixtures/🪶️sqlite/⚠️refusals/🧬️schema/🔣️.json';
function replaceLiteral(value:Record<string,unknown>,path:readonly(string|number)[],replacement:unknown):void {
 let parent=value;for(const key of path.slice(0,-1))parent=parent[key]as Record<string,unknown>;parent[path.at(-1)!]=replacement;
}
for(const refusal of intrinsicRefusals.cases)test('Run intrinsic validation preserves its canonical refusal for '+refusal.path.join('.'),async()=>{
 expect(new Ajv({strict:true}).validate(intrinsicRefusalsSchema,intrinsicRefusals)).toBe(true);
 const validator=new Ajv({strict:false,validateFormats:false});expect(validator.validate(nativeSchema,fixture.snapshot)).toBe(true);
 const json=structuredClone(fixture.snapshot)as Record<string,unknown>;replaceLiteral(json,refusal.path,refusal.value);expect(validator.validate(nativeSchema,json)).toBe(false);
 const invalid=snapshot()as unknown as Record<string,unknown>;replaceLiteral(invalid,refusal.path,refusal.value);
 await expect(ports.runSnapshotToSqliteDatabase(invalid as unknown as Snapshot)).rejects.toHaveProperty('kind',intrinsicRefusals.kind);
});
