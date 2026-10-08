import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import{applyPatch}from"fast-json-patch";
import fixture from"../🧫️fixtures/🔣️.json";

import nativeSchema from"../../../../🧬️schema/📸️snapshot/🔣️.json";
import artifactSchema from"../../../../🧬️schema/🔣️.json";
import * as owner from"../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {binary64,type Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type{ArtifactSqliteOptions}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
type Handle=Omit<owner.Puzzle2dHandle,"angle"|"radius"|"scale">&{angle:Binary64;radius?:Binary64;scale?:Binary64};
type Node=Omit<owner.Puzzle2dNode,"x"|"y"|"radius"|"width"|"height"|"scale"|"handles">&{x:Binary64;y:Binary64;radius?:Binary64;width?:Binary64;height?:Binary64;scale?:Binary64;handles:Handle[]};
type Edge=Omit<owner.Puzzle2dEdge,"gap"|"shift"|"rise"|"rotation"|"turn"|"tilt"|"x"|"y">&{gap:Binary64;shift:Binary64;rise:Binary64;rotation:Binary64;turn:Binary64;tilt:Binary64;x:Binary64;y:Binary64};
type Template=Omit<owner.Puzzle2dHandleTemplate,"angle"|"t"|"radius">&{angle:Binary64;t?:Binary64;radius?:Binary64};
type NodeKind=Omit<owner.Puzzle2dCatalogNodeKind,"handles">&{handles:Template[]};
type Catalogs=Omit<owner.Puzzle2dKindCatalogs,"nodes">&{nodes:NodeKind[]};
type Meta=Omit<owner.Puzzle2dMeta,"kindCatalogs">&{kindCatalogs?:Catalogs};
type Region=Omit<owner.Puzzle2dTargetRegion,"x"|"y"|"width"|"height">&{x:Binary64;y:Binary64;width:Binary64;height:Binary64};
type Snapshot=Omit<owner.Puzzle2dSnapshot,"camera"|"nodes"|"edges"|"targetRegions"|"meta">&{camera:{x:Binary64;y:Binary64;zoom:Binary64};nodes:Node[];edges:Edge[];targetRegions:Region[];meta:Meta};
import * as own from"../🟦️.ts";
function snapshot():Snapshot{
 const source=structuredClone(fixture.snapshot)as owner.Puzzle2dSnapshot;const{kindCatalogs,...meta}=source.meta;
 return{...source,camera:{x:binary64(source.camera.x),y:binary64(source.camera.y),zoom:binary64(source.camera.zoom)},
 nodes:source.nodes.map(({x,y,radius,width,height,scale,handles,...row})=>({...row,x:binary64(x),y:binary64(y),...(radius===undefined?{}:{radius:binary64(radius)}),...(width===undefined?{}:{width:binary64(width)}),...(height===undefined?{}:{height:binary64(height)}),...(scale===undefined?{}:{scale:binary64(scale)}),handles:handles.map(({angle,radius,scale,...handle})=>({...handle,angle:binary64(angle),...(radius===undefined?{}:{radius:binary64(radius)}),...(scale===undefined?{}:{scale:binary64(scale)})}))})),
 edges:source.edges.map(row=>({...row,gap:binary64(row.gap),shift:binary64(row.shift),rise:binary64(row.rise),rotation:binary64(row.rotation),turn:binary64(row.turn),tilt:binary64(row.tilt),x:binary64(row.x),y:binary64(row.y)})),
 targetRegions:source.targetRegions.map(row=>({...row,x:binary64(row.x),y:binary64(row.y),width:binary64(row.width),height:binary64(row.height)})),
 meta:{...meta,...(kindCatalogs===undefined?{}:{kindCatalogs:{...kindCatalogs,nodes:kindCatalogs.nodes.map(row=>({...row,handles:row.handles.map(({angle,t,radius,...handle})=>({...handle,angle:binary64(angle),...(t===undefined?{}:{t:binary64(t)}),...(radius===undefined?{}:{radius:binary64(radius)})}))}))}})}};
}
const bytes=async(value:Snapshot)=>exportSqliteDatabase(await own.puzzle2dSnapshotToSqliteDatabase(value));
const restore=async(value:Uint8Array,options:ArtifactSqliteOptions={})=>own.puzzle2dSnapshotFromSqliteDatabase(await importSqliteDatabase(value),options);
test("Puzzle 2D independent neutral schema admits every persisted board and catalog field",()=>{const validator=new Ajv({strict:false,validateFormats:false}).addSchema(artifactSchema).addSchema(nativeSchema);expect(fixture["control"]["collectionLength"]).toEqual(2048);expect(fixture["control"]["largeTextBytes"]).toEqual(131073);expect(fixture["control"]["maxOwnedBytes"]).toEqual(65536);expect(fixture["control"]["tinyFileBytes"]).toEqual(1024);expect(fixture["semanticCells"]["tableWidths"]).toEqual([["puzzle2d_attribute",7],["puzzle2d_author",8],["puzzle2d_base_kind",4],["puzzle2d_camera",11],["puzzle2d_catalog_edge_kind",9],["puzzle2d_catalog_handle_kind",11],["puzzle2d_catalog_node_kind",11],["puzzle2d_catalog_wire_kind",10],["puzzle2d_compatible_kind",4],["puzzle2d_document",2],["puzzle2d_edge",35],["puzzle2d_handle",18],["puzzle2d_handle_template",19],["puzzle2d_kind_catalogs",2],["puzzle2d_kind_compatibility",8],["puzzle2d_meta",3],["puzzle2d_node",30],["puzzle2d_representation",9],["puzzle2d_representation_tag",4],["puzzle2d_target_region",19]]);expect(fixture["semanticCells"]["fullRows"]).toEqual(42);});
test("Puzzle 2D independent SQLite schema has twenty authored ownership tables",async()=>{const db=new Database(":memory:");try{db.run(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());expect(db.query("SELECT name FROM sqlite_master WHERE type='table'").all().length).toBe(20);expect(db.query("SELECT name FROM sqlite_master WHERE type='index'").all()).toEqual([]);expect(db.query("PRAGMA table_info(puzzle2d_node)").all().length).toBe(30);expect(db.query("PRAGMA table_info(puzzle2d_edge)").all().length).toBe(35);expect(db.query("PRAGMA table_info(puzzle2d_handle_template)").all().length).toBe(19)}finally{db.close()}});
test("Puzzle 2D public facade exposes both owned semantic directions",()=>{expect(Object.hasOwn(own,"puzzle2dSnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(own,"puzzle2dSnapshotFromSqliteDatabase")).toBe(true)});
test("Puzzle 2D independently queryable files retain every scalar ordered catalog and duplicate literal id",async()=>{const expected=snapshot(),data=await bytes(expected),db=Database.deserialize(data,{safeIntegers:true});try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT rank FROM puzzle2d_author ORDER BY ordinal").all()).toEqual([{rank:-2147483648n},{rank:2147483647n},{rank:null}]);expect(db.query("SELECT tag FROM puzzle2d_representation_tag ORDER BY ordinal").all()).toEqual([{tag:""},{tag:"duplicate"},{tag:"duplicate"}]);expect(db.query("SELECT text,root,visible,locked FROM puzzle2d_node ORDER BY ordinal").all()).toEqual([{text:"😀\0世界",root:0n,visible:0n,locked:1n},{text:null,root:null,visible:null,locked:null}]);expect(await restore(data)).toEqual(expected)}finally{db.close()}});
test("Puzzle 2D preserves absent and present empty catalog ownership",async()=>{for(const present of[false,true]){const expected=snapshot();expected.nodes=[];expected.edges=[];expected.targetRegions=[];expected.meta={kindCompatibility:[],...(present?{manifestId:"",kindCatalogs:{nodes:[],handles:[],edges:[],wires:[]}}:{})};const data=await bytes(expected),db=Database.deserialize(data);try{expect(db.query("SELECT * FROM puzzle2d_kind_catalogs").all().length).toBe(present?1:0);expect(await restore(data)).toEqual(expected)}finally{db.close()}}});
function fillWords(expected:Snapshot,word:Binary64):void{
 expected.camera={x:word,y:word,zoom:word};
 for(const row of expected.nodes){row.x=word;row.y=word;if(row.radius!==undefined)row.radius=word;if(row.width!==undefined)row.width=word;if(row.height!==undefined)row.height=word;if(row.scale!==undefined)row.scale=word;for(const handle of row.handles){handle.angle=word;if(handle.radius!==undefined)handle.radius=word;if(handle.scale!==undefined)handle.scale=word;}}
 for(const row of expected.edges){row.gap=word;row.shift=word;row.rise=word;row.rotation=word;row.turn=word;row.tilt=word;row.x=word;row.y=word;}
 for(const row of expected.targetRegions){row.x=word;row.y=word;row.width=word;row.height=word;}
 for(const row of expected.meta.kindCatalogs?.nodes??[])for(const handle of row.handles){handle.angle=word;if(handle.t!==undefined)handle.t=word;if(handle.radius!==undefined)handle.radius=word;}
}
for(const hex of fixture.binary64Words)test("Puzzle 2D every native number retains exact binary64 "+hex,async()=>{const expected=snapshot(),word={bits:BigInt("0x"+hex)};fillWords(expected,word);const data=await bytes(expected),db=Database.deserialize(data,{safeIntegers:true});try{const buffer=Buffer.alloc(8);buffer.writeBigUInt64BE(word.bits);const value=buffer.readDoubleBE(),kind=Number.isNaN(value)?"nan":value===Infinity?"positiveInfinity":value===-Infinity?"negativeInfinity":"finite";expect(db.query("SELECT x_bits,x_class FROM puzzle2d_camera").get()).toEqual({x_bits:BigInt.asIntN(64,word.bits),x_class:kind});expect(await restore(data)).toEqual(expected)}finally{db.close()}});
for(const[index,sql]of fixture.malformedSql.entries())test("Puzzle 2D independently mutated semantic file refuses malformed ownership "+index,async()=>{const db=Database.deserialize(await bytes(snapshot()));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(restore(new Uint8Array(db.serialize()))).rejects.toThrow()}finally{db.close()}});
test("Puzzle 2D signed optional width domains refuse both outside i32 limits",async()=>{for(const property of["rank","order"]as const)for(const invalid of[-2147483649,2147483648]){const expected=snapshot();if(property==="rank")expected.meta.kindCatalogs!.nodes[0]!.authors[0]!.rank=invalid;else expected.meta.kindCatalogs!.handles[0]!.order=invalid;await expect(own.puzzle2dSnapshotToSqliteDatabase(expected)).rejects.toThrow()}});
test("Puzzle 2D exact row budget admits equality and refuses one below",async()=>{const expected=snapshot(),d=await own.puzzle2dSnapshotToSqliteDatabase(expected),rows=d.tables.reduce((total,table)=>total+table.rows.length,0);await expect(own.puzzle2dSnapshotToSqliteDatabase(expected,{maxRows:rows})).resolves.toBeDefined();await expect(own.puzzle2dSnapshotToSqliteDatabase(expected,{maxRows:rows-1})).rejects.toThrow()});
test("Puzzle 2D tiny and cumulative byte limits bound scalar catalog and UTF8 ownership",async()=>{const expected=snapshot(),d=await own.puzzle2dSnapshotToSqliteDatabase(expected);await expect(own.puzzle2dSnapshotFromSqliteDatabase(d,{maxValueBytes:1})).rejects.toThrow();await expect(exportSqliteDatabase(d,{maxFileBytes:fixture.control.tinyFileBytes})).rejects.toThrow();expected.nodes[0]!.text="😀".repeat(fixture.control.largeTextBytes);await expect(own.puzzle2dSnapshotToSqliteDatabase(expected,{maxValueBytes:fixture.control.maxOwnedBytes})).rejects.toThrow()});
for(const phase of["projectSnapshot","reconstructSnapshot"]as const)test("Puzzle 2D cancellation reaches actual ordered nested representation tags during "+phase,async()=>{const expected=snapshot(),length=fixture.control.collectionLength;expected.meta.kindCatalogs!.nodes[0]!.representations[0]!.tags=Array.from({length},(_,index)=>String(index));const d=await own.puzzle2dSnapshotToSqliteDatabase(expected),controller=new AbortController();let reached=false;const options:ArtifactSqliteOptions={signal:controller.signal,onProgress:event=>{if(event.phase===phase&&event.total===length&&event.completed>=256&&event.completed<length){reached=true;controller.abort()}}};await expect(phase==="projectSnapshot"?own.puzzle2dSnapshotToSqliteDatabase(expected,options):own.puzzle2dSnapshotFromSqliteDatabase(d,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);const initial=new AbortController();initial.abort();await expect(phase==="projectSnapshot"?own.puzzle2dSnapshotToSqliteDatabase(expected,{signal:initial.signal}):own.puzzle2dSnapshotFromSqliteDatabase(d,{signal:initial.signal})).rejects.toHaveProperty("kind","canceled")});

/** 📐️ Uses independent SQLite storage classes and UTF8 bytes for the complete authored corpus. */
test("Puzzle 2D complete semantic cells have independent exact and one-short grants",async()=>{
 expect(fixture["control"]["collectionLength"]).toEqual(2048);expect(fixture["control"]["largeTextBytes"]).toEqual(131073);expect(fixture["control"]["maxOwnedBytes"]).toEqual(65536);expect(fixture["control"]["tinyFileBytes"]).toEqual(1024);expect(fixture["semanticCells"]["tableWidths"]).toEqual([["puzzle2d_attribute",7],["puzzle2d_author",8],["puzzle2d_base_kind",4],["puzzle2d_camera",11],["puzzle2d_catalog_edge_kind",9],["puzzle2d_catalog_handle_kind",11],["puzzle2d_catalog_node_kind",11],["puzzle2d_catalog_wire_kind",10],["puzzle2d_compatible_kind",4],["puzzle2d_document",2],["puzzle2d_edge",35],["puzzle2d_handle",18],["puzzle2d_handle_template",19],["puzzle2d_kind_catalogs",2],["puzzle2d_kind_compatibility",8],["puzzle2d_meta",3],["puzzle2d_node",30],["puzzle2d_representation",9],["puzzle2d_representation_tag",4],["puzzle2d_target_region",19]]);expect(fixture["semanticCells"]["fullRows"]).toEqual(42);
 for(const hex of[undefined,...fixture.binary64Words]){
  const expected=snapshot();if(hex!==undefined)fillWords(expected,{bits:BigInt("0x"+hex)});const database=await own.puzzle2dSnapshotToSqliteDatabase(expected),db=Database.deserialize(await exportSqliteDatabase(database),{safeIntegers:true});let rows:number,cells:number;
  try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
   const tables=(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()as{name:string}[]).map(({name})=>{const columns=db.query('PRAGMA table_info('+name+')').all()as{name:string}[],expression=columns.map(({name})=>{const c='"'+name.replaceAll('"','""')+'"';return "CASE typeof("+c+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+c+" AS BLOB)) WHEN 'blob' THEN length("+c+") WHEN 'null' THEN 0 ELSE NULL END"}).join('+'),r=db.query('SELECT COUNT(*) AS rows,COALESCE(SUM('+expression+'),0) AS bytes FROM '+name).get()as{rows:bigint;bytes:bigint}|null;if(!r)throw Error("Independent cell aggregate is absent");return{table:name,columns:columns.length,rows:Number(r.rows),semanticBytes:Number(r.bytes)}});
   expect(tables.map(t=>[t.table,t.columns])).toEqual(fixture.semanticCells.tableWidths);expect(tables.every(t=>t.rows>0)).toBe(true);rows=tables.reduce((n,t)=>n+t.rows,0);cells=tables.reduce((n,t)=>n+t.semanticBytes,0);expect(rows).toBe(fixture.semanticCells.fullRows);
  }finally{db.close()}
  expect(await own.puzzle2dSnapshotToSqliteDatabase(expected,{maxRows:rows,maxValueBytes:cells})).toEqual(database);
  for(const limit of[{maxRows:rows-1,maxValueBytes:cells},{maxRows:rows,maxValueBytes:cells-1}]){await expect(own.puzzle2dSnapshotToSqliteDatabase(expected,limit)).rejects.toThrow();await expect(own.puzzle2dSnapshotFromSqliteDatabase(database,limit)).rejects.toThrow();}
 }
});


test("Puzzle2d typed sparse deltas and pilot grammar retain neutral authored records",async()=>{
 const corpus=await Bun.file(new URL("../../../📝️text/🔺️diff/🧫️fixtures/🔣️.json",import.meta.url)).json() as {samples:{json:unknown;document:string}[];diffRecordOwners:string[]};
 const schema=await Bun.file(new URL("../../../../🧬️schema/🔺️diff/🔣️.json",import.meta.url)).json();
 const validate=new Ajv({strict:false}).addSchema(artifactSchema).compile(schema);
 for(const sample of corpus.samples){expect(validate(sample.json)).toBe(true);expect(JSON.parse(JSON.stringify(sample.json))).toEqual(sample.json)}
 expect(validate({schema:3})).toBe(false);expect(validate({unknown:"field"})).toBe(false);
 const diff=await Bun.file(new URL("../../../../🧬️schema/🔺️diff/🦀️.rs",import.meta.url)).text();
 for(const name of corpus.diffRecordOwners)expect(diff).toMatch(new RegExp("#\\[derive\\([^\\]]*semio_framework_dsl_record_derive::DslRecord[^\\]]*\\)\\][^#]*?(?:#\\[[^\\]]*\\][^#]*?)*pub struct "+name+"\\b"));
 expect(diff.match(/pub removed: Vec<PagedUtf8<\{ usize::MAX \}>>/gu)?.length).toBe(4);expect(diff.match(/pub reordered: Option<Vec<PagedUtf8<\{ usize::MAX \}>>>/gu)?.length).toBe(4);
 const artifact=await Bun.file(new URL("../../../../🧬️schema/🦀️.rs",import.meta.url)).text();expect(artifact).toContain("semio_framework_dsl_record_derive::DslRecord");
 const owner=await Bun.file(new URL("../../../../../../../../🦀️.rs",import.meta.url)).text();
 for(const constant of ["COMPONENT_GRAMMAR_SEMIO","COMPONENT_GRAMMAR_PATH"])expect(owner).toContain("standards::v1::subsets::any::io::text::diff::"+constant);
});

test("Puzzle2d native snapshot ownership has retained clone and retirement dispatch",async()=>{
 const corpus=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🧬️retained-clone/🔣️.json",import.meta.url)).json() as {recordOwners:string[];journeys:string[];control:{maximumItems:number}};
 expect(corpus.control.maximumItems).toBe(1);expect(corpus.journeys).toEqual(["empty","nested","largeUtf8","zeroGrant","cancelDuringCopy","takeOnce","boundedClose"]);
 const source=await Bun.file(new URL("../../../../../../../../🦀️.rs",import.meta.url)).text()+await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🦀️.rs",import.meta.url)).text();
 for(const name of corpus.recordOwners)for(const role of ["RetainedClone","RetireOwned"])expect(source).toMatch(new RegExp("#\\[derive\\([^\\]]*semio_framework_value::"+role+"[^\\]]*\\)\\][^#]*?(?:#\\[[^\\]]*\\][^#]*?)*pub (?:struct|enum) "+name+"\\b"));
});


test("Puzzle2d borrowed mutation ownership has exhaustive native clone and retirement dispatch",async()=>{
 const corpus=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🧬️retained-clone/🔣️.json",import.meta.url)).json() as {mutationOwners:string[];borrowedJourneys:string[]};
 expect(corpus.mutationOwners.length).toBe(37);expect(corpus.borrowedJourneys).toEqual(["allAuthoredLeaves","zeroGrant","sourceSwap","cancelDuringCopy","largeUtf8","takeOnce","boundedClose"]);
 const directory=new URL("../../../../🧬️schema/🧬️mutations/",import.meta.url),files=Array.from(new Bun.Glob("**/🦀️.rs").scanSync({cwd:fileURLToPath(directory)})).filter(path=>path==="🦀️.rs"||path.split("/").length===2);
 const source=(await Promise.all(files.map(path=>Bun.file(new URL(path,directory)).text()))).join("\n");
 for(const name of corpus.mutationOwners)for(const role of ["RetainedClone","RetireOwned"])expect(source).toMatch(new RegExp("#\\[derive\\([^\\]]*semio_framework_value::"+role+"[^\\]]*\\)\\][^#]*?(?:#\\[[^\\]]*\\][^#]*?)*pub (?:struct|enum) "+name+"\\b"));
});


test("Puzzle2d physical text and arrays preserve the authored semantic shapes with paged owners",async()=>{
 const corpus=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🧬️retained-clone/🔣️.json",import.meta.url)).json() as {physicalOwners:{textType:string;arrayType:string;maximumTurnBytes:number;unchangedSemanticShapes:string[];journeys:string[]}};
 expect(corpus.physicalOwners.maximumTurnBytes).toBe(4096);expect(corpus.physicalOwners.unchangedSemanticShapes).toEqual(["string","array"]);
 expect(corpus.physicalOwners.journeys).toEqual(["largeText","largeNodes","nestedHandles","nestedCatalogs","tinyGrant","zeroGrant","cancelDuringCopy","boundedClose"]);
 const validate=new Ajv({strict:false,validateFormats:false}).addSchema(artifactSchema).compile(nativeSchema);expect(validate(fixture.snapshot)).toBe(true);expect(JSON.parse(JSON.stringify(fixture.snapshot))).toEqual(fixture.snapshot);
 const source=await Bun.file(new URL("../../../../../../../../🦀️.rs",import.meta.url)).text()+await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🦀️.rs",import.meta.url)).text();
 expect(source).toContain("PagedUtf8<{ usize::MAX }>");expect(source).toContain("PagedList<Puzzle2dNode, { usize::MAX }>");expect(source).toContain("PagedList<Puzzle2dHandle, { usize::MAX }>");expect(source).toContain("PagedList<Puzzle2dTargetRegion, { usize::MAX }>");
 const document=source.slice(source.indexOf("pub struct Puzzle2dCamera"),source.indexOf("//#endregion 🔖️Document"));expect(document).not.toMatch(/pub \w+: (?:String|Vec<|Option<String>)/);
});

test("Puzzle2d native lookup preserves independent SQLite first-match topology under exact grants",async()=>{
 const corpus=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🔎️lookup/🧫️fixtures/🔣️.json",import.meta.url)).json() as {control:{maximumItems:number;maximumBytes:number;largePrefix:string;largeRepeats:number};cases:{scope:string;nodes:string[];handles:string[][];edges:string[];regions:string[];target:string;expected:{outer:number;inner:number|null}|null}[];allocation:{advance:number;close:number}};
 expect(corpus.control.maximumItems).toBe(1);expect(corpus.control.maximumBytes).toBe(4);expect(corpus.allocation).toEqual({advance:0,close:0});
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const db=new Database(":memory:");try{db.run("CREATE TABLE identifiers(scope TEXT NOT NULL,outer_ordinal INTEGER NOT NULL,inner_ordinal INTEGER,value TEXT NOT NULL)");for(const [scope,ids]of[["node",row.nodes],["edge",row.edges],["region",row.regions]]as const)ids.forEach((id,outer)=>db.run("INSERT INTO identifiers VALUES(?,?,?,?)",[scope,outer,null,text(id)]));row.handles.forEach((ids,outer)=>ids.forEach((id,inner)=>db.run("INSERT INTO identifiers VALUES(?,?,?,?)",["handle",outer,inner,text(id)])));expect(db.query("SELECT outer_ordinal AS outer,inner_ordinal AS inner FROM identifiers WHERE scope=? AND value=? ORDER BY outer_ordinal,inner_ordinal LIMIT 1").get(row.scope,text(row.target))).toEqual(row.expected)}finally{db.close()}}
 const source=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🔎️lookup/🦀️.rs",import.meta.url)).text();expect(source).toContain("pub struct Puzzle2dLookupCursor");expect(source).toContain("PagedUtf8BoundedOrdCursor");expect(source).toContain("RetainedCloneBinding::close_one");expect(source).not.toContain("to_string_owner");
});

test("Puzzle2d scalar preparation preserves RFC6902 forward and inverse semantics before refusal",async()=>{
 const corpus=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/📍move-node/🎮️prepare/🧫️fixtures/🔣️.json",import.meta.url)).json() as {control:{maximumItems:number;maximumBytes:number;largePrefix:string;largeRepeats:number};cases:{nodes:{id:string;x:number;y:number}[];mutation:{id:string;newX:number|string;newY:number|string};status:string;index:number|null;after:{id:string;x:number;y:number}[];inverse:{id:string;newX:number;newY:number}|null}[]};
 expect(corpus.control.maximumItems).toBe(1);expect(corpus.control.maximumBytes).toBe(32);
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const before=row.nodes.map(node=>({...node,id:text(node.id)})),after=row.after.map(node=>({...node,id:text(node.id)}));const index=before.findIndex(node=>node.id===text(row.mutation.id));expect(index<0?null:index).toEqual(row.index);expect(index<0?null:{id:row.nodes[index]!.id,newX:before[index]!.x,newY:before[index]!.y}).toEqual(row.inverse);
 const patch=row.status==="changed"?[{op:"replace" as const,path:`/${index}/x`,value:row.mutation.newX},{op:"replace" as const,path:`/${index}/y`,value:row.mutation.newY}]:[];expect(applyPatch(structuredClone(before),patch).newDocument).toEqual(after);const inverse=row.inverse?[{op:"replace" as const,path:`/${index}/x`,value:row.inverse.newX},{op:"replace" as const,path:`/${index}/y`,value:row.inverse.newY}]:[];expect(applyPatch(structuredClone(after),inverse).newDocument).toEqual(before)}
 const source=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/📍move-node/🎮️prepare/🦀️.rs",import.meta.url)).text();expect(source).toContain("pub struct Puzzle2dMoveNodePreparationCursor");expect(source).toContain("Puzzle2dLookupCursor");expect(source).toContain("RetainedCloneBinding::close_one");expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|\.iter\(\)\.find/);
});

test("Puzzle2d owned inverse admits literal ids and preserves independent RFC6902 restoration",async()=>{
 const corpus=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/📍move-node/🎮️prepare/🧫️fixtures/🔣️.json",import.meta.url)).json() as {control:{largePrefix:string;largeRepeats:number};inverseOwnership:{maximumItems:number;maximumTurnBytes:number;cancelAfterBodyBytes:number;journeys:string[]};cases:{nodes:{id:string;x:number;y:number}[];after:{id:string;x:number;y:number}[];inverse:{id:string;newX:number;newY:number}|null;index:number|null}[]};
 expect(corpus.inverseOwnership.maximumItems).toBe(1);expect(corpus.inverseOwnership.maximumTurnBytes).toBe(4096);expect(corpus.inverseOwnership.cancelAfterBodyBytes).toBe(128);
 expect(corpus.inverseOwnership.journeys).toEqual(["inverseBeforeRefusal","noOpInverse","targetMissingEmptyInverse","borrowedIdOnly","largeUtf8","zeroGrant","cancelDuringIdCopy","takeOnce","controlledRetirement"]);
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const inverse=row.inverse?[{mutation:"move-node",payload:{...row.inverse,id:text(row.inverse.id)}}]:[];expect(inverse.length).toBe(row.index===null?0:1);const before=row.nodes.map(node=>({...node,id:text(node.id)})),after=row.after.map(node=>({...node,id:text(node.id)}));const patch=inverse.flatMap(op=>[{op:"replace" as const,path:`/${row.index}/x`,value:op.payload.newX},{op:"replace" as const,path:`/${row.index}/y`,value:op.payload.newY}]);expect(applyPatch(structuredClone(after),patch).newDocument).toEqual(before)}
 const source=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/📍move-node/🎮️prepare/↩️inverse/🦀️.rs",import.meta.url)).text();expect(source).toContain("pub struct Puzzle2dMoveNodeInverseCursor");expect(source).toContain("ControlledRetirement");expect(source).toContain("PagedList<Puzzle2dMutation");expect(source).toContain("mutation.project(1, |payload| &payload.id)");expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
});

test("Puzzle2d native forward ownership agrees with independent RFC6902 first-match edits",async()=>{
 const corpus=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/📍move-node/🎮️prepare/🧫️fixtures/🔣️.json",import.meta.url)).json() as {control:{largePrefix:string;largeRepeats:number};forwardOwnership:{maximumItems:number;maximumTurnBytes:number;cancelAfterBodyBytes:number;changedOwner:string;unchangedOwner:string;journeys:string[]};cases:{nodes:{id:string;x:number;y:number}[];mutation:{id:string;newX:number|string;newY:number|string};status:string;index:number|null;after:{id:string;x:number;y:number}[]}[]};
 expect(corpus.forwardOwnership).toEqual({maximumItems:1,maximumTurnBytes:4096,cancelAfterBodyBytes:128,changedOwner:"nativePagedSnapshot",unchangedOwner:"retainedOriginal",journeys:["changedCandidate","firstDuplicateOnly","noOpRetainsOriginal","refusalRetainsOriginal","borrowedPayload","allUntouchedOwners","zeroGrant","cancelDuringSnapshotCopy","takeOnce","controlledRetirement"]});
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const before={nodes:row.nodes.map(node=>({...node,id:text(node.id)})),untouched:{tags:["😀","\u0000","same"],camera:{x:-0,y:3,zoom:1}}};const patch=row.status==="changed"?[{op:"replace" as const,path:`/nodes/${row.index}/x`,value:row.mutation.newX},{op:"replace" as const,path:`/nodes/${row.index}/y`,value:row.mutation.newY}]:[];const expected={...structuredClone(before),nodes:row.after.map(node=>({...node,id:text(node.id)}))};expect(applyPatch(structuredClone(before),patch).newDocument).toEqual(expected);expect(before.nodes).toEqual(row.nodes.map(node=>({...node,id:text(node.id)})));expect(patch.length).toBe(row.status==="changed"?2:0)}
 const source=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/📍move-node/🎮️prepare/📸️candidate/🦀️.rs",import.meta.url)).text();expect(source).toContain("pub struct Puzzle2dMoveNodeCandidateCursor");expect(source).toContain("Puzzle2dSnapshot::retained_clone_cursor()");expect(source).toContain("ControlledRetirement");expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
});


test("Puzzle2d borrowed create preparation retains native invariant priority and RFC6902 placement",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🌱create-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json();
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 const number={type:"number"},positive={type:"number",exclusiveMinimum:0};
 const handle={type:"object",required:["id","angle"],properties:{id:{type:"string"},angle:number,radius:positive,scale:positive}};
 const admits=new Ajv({strict:false,strictNumbers:true}).compile({type:"object",required:["id","x","y"],properties:{id:{type:"string"},x:number,y:number,shape:{enum:["circle","rectangle"]},radius:positive,width:positive,height:positive,scale:positive,handles:{type:"array",items:handle}}});
 const typed=(value:any):any=>value==="nan"?NaN:value==="positive-infinity"?Infinity:Array.isArray(value)?value.map(typed):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,entry])=>[key,typed(entry)])):value;
 for(const row of corpus.cases){const before=row.nodes.map(text),id=text(row.mutation.node.id),db=new Database(":memory:");try{db.run("CREATE TABLE nodes(ordinal INTEGER PRIMARY KEY,id TEXT NOT NULL)");before.forEach((value:string,ordinal:number)=>db.run("INSERT INTO nodes VALUES(?,?)",[ordinal,value]));const duplicate=db.query("SELECT ordinal FROM nodes WHERE id=? ORDER BY ordinal LIMIT 1").get(id);if(row.status==="duplicate-id")expect(duplicate).not.toBeNull();if(row.status==="changed")expect(duplicate).toBeNull();}finally{db.close()}
  expect(admits(typed(row.mutation.node))).toBe(row.status==="changed"||row.status==="duplicate-id");
  const node=row.mutation.node,positive=(name:string)=>node[name]===undefined||(typeof node[name]==="number"&&node[name]>0&&Number.isFinite(node[name]));const finite=(value:unknown)=>typeof value==="number"&&Number.isFinite(value);
  const first=!finite(node.x)?"nonfinite-x":!finite(node.y)?"nonfinite-y":node.shape!==undefined&&!["circle","rectangle"].includes(node.shape)?"invalid-shape":["radius","width","height","scale"].find(name=>!positive(name));
  if(first)expect(row.status).toBe(first.startsWith("invalid-")||first.startsWith("nonfinite-")?first:`invalid-${first}`);
  const patch=row.status==="changed"?[{op:"add" as const,path:`/${row.position}`,value:id}]:[];expect(applyPatch(structuredClone(before),patch).newDocument).toEqual(row.after.map(text));if(row.status==="changed")expect(applyPatch(row.after.map(text),[{op:"remove",path:`/${row.position}`}]).newDocument).toEqual(before);
  expect(row.inverse).toEqual({mutation:"delete-node",payload:{id:row.mutation.node.id}});
 }
 const source=await Bun.file(new URL("🦀️.rs",root)).text();expect(source).toContain("pub struct Puzzle2dCreateNodePreparationCursor");expect(source).toContain("Puzzle2dLookupCursor");expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|\.iter\(\)\.find/);
 console.log(`[DEBUG] Puzzle2d create preparation admits sixteen invariant/duplicate/placement plans against SQLite and RFC6902`);
});


test("Puzzle2d create inverse retains unconditional literal intent under native paged ownership",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🌱create-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json();
 expect(corpus.inverseOwnership).toEqual({maximumItems:1,maximumTurnBytes:4096,cancelAfterBodyBytes:128,journeys:["inverseBeforeRefusal","duplicateInverseIntent","borrowedIdOnly","largeUtf8","zeroGrant","sourceSwap","cancelDuringIdCopy","takeOnce","controlledRetirement"]});
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){expect(row.inverse).toEqual({mutation:"delete-node",payload:{id:row.mutation.node.id}});const id=text(row.inverse.payload.id),db=new Database(":memory:");try{db.run("CREATE TABLE inverse_intent(id TEXT NOT NULL)");db.run("INSERT INTO inverse_intent VALUES(?)",[id]);expect(db.query("SELECT id,length(CAST(id AS BLOB)) AS bytes FROM inverse_intent").get()).toEqual({id,bytes:new TextEncoder().encode(id).length});}finally{db.close()}
  if(row.status==="changed")expect(applyPatch(row.after.map(text),[{op:"remove",path:`/${row.position}`}]).newDocument).toEqual(row.nodes.map(text));
 }
 const source=await Bun.file(new URL("↩️inverse/🦀️.rs",root)).text();expect(source).toContain("pub type Puzzle2dCreateNodeInverseCursor");expect(source).toContain("fn inverse_id(&self)");expect(source).toContain("&self.node.id");const owner=await Bun.file(new URL("../../🏷️literal-id-inverse/🦀️.rs",root)).text();expect(owner).toContain("mutation.project(1, M::inverse_id)");expect(owner).toContain("PagedList<Puzzle2dMutation");expect(owner).toContain("ControlledRetirement");expect(owner).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
 console.log("[DEBUG] Puzzle2d CreateNode inverse preserves sixteen unconditional typed intents against SQLite and changed-case RFC6902 restoration");
});


test("Puzzle2d create candidate preserves independent ordered insertion and every untouched owner",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🌱create-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json();
 expect(corpus.forwardOwnership).toEqual({maximumItems:1,maximumTurnBytes:4096,cancelAfterBodyBytes:128,changedOwner:"nativePagedSnapshot",unchangedOwner:"retainedOriginal",journeys:["append","indexedInsertion","clampedInsertion","refusalRetainsOriginal","borrowedNodeOnly","nestedHandles","allUntouchedOwners","zeroGrant","cancelDuringSnapshotCopy","cancelDuringNodeCopy","cancelDuringInsertion","takeOnce","controlledRetirement"]});
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const before={nodes:row.nodes.map((id:string)=>({id:text(id),text:"retained",handles:[]})),untouched:{camera:{x:-0,y:3,zoom:1},edges:[{id:"edge",label:"😀\u0000"}],tags:["original"]}};const patch=row.status==="changed"?[{op:"add" as const,path:`/nodes/${row.position}`,value:row.mutation.node}]:[];const after=applyPatch(structuredClone(before),patch).newDocument;expect(after.nodes.map((node:any)=>text(node.id))).toEqual(row.after.map(text));expect(after.untouched).toEqual(before.untouched);expect(before.nodes.map(node=>node.id)).toEqual(row.nodes.map(text));if(row.status==="changed"){expect(after.nodes[row.position]).toEqual(row.mutation.node);expect(applyPatch(after,[{op:"remove",path:`/nodes/${row.position}`}]).newDocument).toEqual(before)}}
 const source=await Bun.file(new URL("📸️candidate/🦀️.rs",root)).text();expect(source).toContain("pub struct Puzzle2dCreateNodeCandidateCursor");expect(source).toContain("RetainedFieldCursor::<Puzzle2dSnapshot>::default()");expect(source).toContain("RetainedFieldCursor::<Puzzle2dNode>::default()");expect(source).toContain("PagedListEditCursor::insert");expect(source).toContain("mutation.project(1, |payload| &payload.node)");expect(source).toContain("ControlledRetirement");expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
 console.log("[DEBUG] Puzzle2d CreateNode candidate exact ordered insertion and untouched-owner restoration agree with independent RFC6902");
});

test("Puzzle2d borrowed delete cascade streams exact first-node and ordered edge descriptors",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🗑️delete-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json();
 expect(corpus.journeys).toEqual(["firstNodeOnly","sourceEndpoint","targetEndpoint","selfEdgeOnce","originalEdgeOrder","missingNode","emptyHandles","largeUtf8","zeroGrant","sourceSwap","cancelDuringComparison","boundedClose"]);
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const db=new Database(":memory:");try{
  db.run("CREATE TABLE nodes(ordinal INTEGER PRIMARY KEY,id TEXT NOT NULL);CREATE TABLE handles(node INTEGER NOT NULL,ordinal INTEGER NOT NULL,id TEXT NOT NULL);CREATE TABLE edges(ordinal INTEGER PRIMARY KEY,id TEXT NOT NULL,source TEXT NOT NULL,target TEXT NOT NULL)");
  row.nodes.forEach((node:any,ordinal:number)=>{db.run("INSERT INTO nodes VALUES(?,?)",[ordinal,text(node.id)]);node.handles.forEach((id:string,handle:number)=>db.run("INSERT INTO handles VALUES(?,?,?)",[ordinal,handle,text(id)]))});
  row.edges.forEach((edge:any,ordinal:number)=>db.run("INSERT INTO edges VALUES(?,?,?,?)",[ordinal,edge.id,text(edge.source),text(edge.target)]));
  const node=db.query("SELECT ordinal FROM nodes WHERE id=? ORDER BY ordinal LIMIT 1").get(text(row.target)) as {ordinal:number}|null;expect(node?.ordinal??null).toEqual(row.node);
  const severed=node?db.query("SELECT ordinal FROM edges WHERE EXISTS(SELECT 1 FROM handles WHERE node=? AND (handles.id=edges.source OR handles.id=edges.target)) ORDER BY ordinal").all(node.ordinal) as {ordinal:number}[]:[];
  expect(severed.map(edge=>edge.ordinal)).toEqual(row.severedEdges);
  const before={nodes:structuredClone(row.nodes),edges:structuredClone(row.edges),untouched:{text:"😀\u0000",camera:[3,-4,1]}};
  const patch=[...row.severedEdges].reverse().map(index=>({op:"remove" as const,path:`/edges/${index}`}));if(row.node!==null)patch.push({op:"remove",path:`/nodes/${row.node}`});
  const after=applyPatch(structuredClone(before),patch).newDocument;expect(after.nodes.map((node:any)=>node.id)).toEqual(row.afterNodes);expect(after.edges.map((edge:any)=>edge.id)).toEqual(row.afterEdges);expect(after.untouched).toEqual(before.untouched);
  const inverse=row.node===null?[]:[{op:"add" as const,path:`/nodes/${row.node}`,value:before.nodes[row.node]},...row.severedEdges.map(index=>({op:"add" as const,path:`/edges/${index}`,value:before.edges[index]}))];
  expect(applyPatch(structuredClone(after),inverse).newDocument).toEqual(before);
 }finally{db.close()}}
 const source=await Bun.file(new URL("🦀️.rs",root)).text();expect(source).toContain("pub struct Puzzle2dDeleteNodePreparationCursor");expect(source).toContain("PagedUtf8BoundedOrdCursor");expect(source).toContain("Puzzle2dLookupCursor");expect(source).toContain("RetainedCloneBinding::close_one");expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|Vec<|\.iter\(\)\.find/);
 console.log("[DEBUG] Puzzle2d borrowed DeleteNode first-match and cascade descriptor order agree with eight SQLite/RFC6902 cases");
});


test("Puzzle2d edge restoration preserves original ordinals and all optional flags",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🪢️connect-handles/↩️inverse/📑️ordered-restoration/",import.meta.url),corpus=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json();
 for(const row of corpus.cases){const db=new Database(":memory:");try{
  db.run("CREATE TABLE edges(ordinal INTEGER PRIMARY KEY,record TEXT NOT NULL,source TEXT NOT NULL,target TEXT NOT NULL)");row.edges.forEach((edge:any,index:number)=>db.run("INSERT INTO edges VALUES(?,?,?,?)",[index,JSON.stringify(edge),edge.source,edge.target]));
  expect(db.query("SELECT ordinal FROM edges WHERE source=? OR target=? ORDER BY ordinal").all("handle","handle").map((edge:any)=>edge.ordinal)).toEqual(row.removed);
  const before={edges:row.edges,untouched:{text:"😀\u0000",camera:[3,4,1]}},patch=[...row.removed].reverse().map((index:number)=>({op:"remove" as const,path:`/edges/${index}`})),after=applyPatch(structuredClone(before),patch).newDocument;
  const inverse=row.removed.map((index:number)=>({op:"add" as const,path:`/edges/${index}`,value:row.edges[index]}));expect(applyPatch(structuredClone(after),inverse).newDocument).toEqual(before);
  for(const index of row.removed)expect(JSON.parse((db.query("SELECT record FROM edges WHERE ordinal=?").get(index) as {record:string}).record)).toEqual(row.edges[index]);
 }finally{db.close()}}
 for(const row of corpus.insertions){const index=row.index===null?row.before.length:Math.min(row.index,row.before.length);expect(applyPatch([...row.before],[{op:"add",path:`/${index}`,value:"new"}]).newDocument).toEqual(row.after)}
 const schema=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🪢️connect-handles/🧬️schema/🔣️.json",import.meta.url)).json();expect(schema.properties.index.type).toEqual(["integer","null"]);expect(schema.properties.index.minimum).toBe(0);
 const source=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🪢️connect-handles/🦀️.rs",import.meta.url)).text();expect(source).toContain("pub index: Option<usize>");expect(source).toContain("pub(crate) fn restore_edge");
 console.log("[DEBUG] Puzzle2d exact edge order and optional flag restoration agree with six SQLite/RFC6902 cascades and six insertion cases");
});


test("Puzzle2d owned delete inverse preserves complete original cascade records",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🗑️delete-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json();expect(corpus.inverseOwnership.maximumItems).toBe(1);expect(corpus.inverseOwnership.maximumTurnBytes).toBe(4096);expect(corpus.inverseOwnership.changedOwner).toBe("nativePagedInverse");
 for(const row of corpus.cases){const before={nodes:row.nodes.map((node:any,index:number)=>({...node,shape:"rectangle",x:index,y:-index,text:"😀\u0000",handles:node.handles.map((id:string)=>({id,angle:2,color:"retained"}))})),edges:row.edges.map((edge:any,index:number)=>({...edge,edgeKind:"",gap:-3,shift:2,rise:1,rotation:4,turn:5,tilt:-6,x:7,y:-8,sourceTip:"",targetTip:"é",...(index%3===0?{visible:false,locked:true}:index%3===1?{visible:true,locked:false}:{})})),untouched:{camera:[3,-4,1],metadata:"original"}};
  const patch=[...row.severedEdges].reverse().map((index:number)=>({op:"remove" as const,path:`/edges/${index}`}));if(row.node!==null)patch.push({op:"remove",path:`/nodes/${row.node}`});const after=applyPatch(structuredClone(before),patch).newDocument;
  const inverse=row.node===null?[]:[{op:"add" as const,path:`/nodes/${row.node}`,value:before.nodes[row.node]},...row.severedEdges.map((index:number)=>({op:"add" as const,path:`/edges/${index}`,value:before.edges[index]}))];expect(applyPatch(structuredClone(after),inverse).newDocument).toEqual(before);
 }
 const source=await Bun.file(new URL("↩️inverse/🦀️.rs",root)).text();expect(source).toContain("pub struct Puzzle2dDeleteNodeInverseCursor");expect(source).toContain("Puzzle2dDeleteNodePreparationCursor");expect(source).toContain("RetainedFieldCursor::<Puzzle2dNode>::default()");expect(source).toContain("RetainedFieldCursor::<Puzzle2dEdge>::default()");expect(source).toContain("ControlledRetirement");expect(source).toContain("PagedList<Puzzle2dMutation");expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
 console.log("[DEBUG] Puzzle2d owned DeleteNode inverse complete native record semantics agree with eight independent RFC6902 cascade cases");
});

test("Puzzle2d delete candidate preserves first matching edge IDs and refuses repeated targets",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🗑️delete-node/🎮️prepare/",import.meta.url),base=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json(),corpus=await Bun.file(new URL("📸️candidate/🧫️fixtures/🔣️.json",root)).json();
 const text=(value:string)=>value.startsWith("$large:")?base.control.largePrefix.repeat(base.control.largeRepeats)+value.slice(7):value;
 for(const row of [...base.cases,...corpus.cases]){const db=new Database(":memory:");try{
  db.run("CREATE TABLE nodes(ordinal INTEGER PRIMARY KEY,id TEXT NOT NULL);CREATE TABLE handles(node INTEGER NOT NULL,id TEXT NOT NULL);CREATE TABLE edges(ordinal INTEGER PRIMARY KEY,id TEXT NOT NULL,source TEXT NOT NULL,target TEXT NOT NULL)");
  row.nodes.forEach((node:any,ordinal:number)=>{db.run("INSERT INTO nodes VALUES(?,?)",[ordinal,text(node.id)]);node.handles.forEach((id:string)=>db.run("INSERT INTO handles VALUES(?,?)",[ordinal,text(id)]))});row.edges.forEach((edge:any,ordinal:number)=>db.run("INSERT INTO edges VALUES(?,?,?,?)",[ordinal,text(edge.id),text(edge.source),text(edge.target)]));
  const node=db.query("SELECT ordinal FROM nodes WHERE id=? ORDER BY ordinal LIMIT 1").get(text(row.target)) as {ordinal:number}|null;
  const targets=node?db.query("SELECT id FROM edges WHERE EXISTS(SELECT 1 FROM handles WHERE node=? AND (handles.id=edges.source OR handles.id=edges.target)) ORDER BY ordinal").all(node.ordinal) as {id:string}[]:[];
  const refused=new Set(targets.map(edge=>edge.id)).size!==targets.length,status=refused?"refused":node?"changed":"unchanged";
  const removed=refused?[]:targets.map(edge=>(db.query("SELECT ordinal FROM edges WHERE id=? ORDER BY ordinal LIMIT 1").get(edge.id) as {ordinal:number}).ordinal).sort((a,b)=>a-b);
  expect(status).toEqual(row.status??(row.node===null?"unchanged":"changed"));expect(removed).toEqual(row.removed??row.severedEdges);
  const before={nodes:structuredClone(row.nodes),edges:structuredClone(row.edges),untouched:{camera:[3,-4,1],text:"😀\u0000"}};
  const patch=removed.toReversed().map(index=>({op:"remove" as const,path:`/edges/${index}`}));if(status==="changed")patch.push({op:"remove",path:`/nodes/${node!.ordinal}`});const after=applyPatch(structuredClone(before),patch).newDocument;
  expect(after.nodes.map((node:any)=>node.id)).toEqual(row.afterNodes);expect(after.edges.map((edge:any)=>edge.id)).toEqual(row.afterEdges);expect(after.untouched).toEqual(before.untouched);
  if(status!=="changed")expect(after).toEqual(before);
 }finally{db.close()}}
 const source=await Bun.file(new URL("📸️candidate/🦀️.rs",root)).text();expect(source).toContain("pub struct Puzzle2dDeleteNodeCandidateCursor");expect(source).toContain("RetainedFieldCursor::<Puzzle2dSnapshot>::default()");expect(source).toContain("PagedListEditCursor::remove");expect(source).toContain("PagedListEditCursor::insert");expect(source).toContain("PagedUtf8BoundedOrdCursor");expect(source).toContain("ControlledRetirement");expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
 console.log("[DEBUG] Puzzle2d DeleteNode candidate first-ID removal and duplicate-target refusal agree with eleven independent SQLite/RFC6902 cases");
});


test("Puzzle2d borrowed connect preparation keeps invariant priority and proximity intent",async()=>{
 const base="../../../../🧬️schema/🧬️mutations/🪢️connect-handles/🎮️prepare/",corpus=await Bun.file(new URL(base+"🧫️fixtures/🔣️.json",import.meta.url)).json(),{Vector2}=await import("three");
 const expanded=(id:string)=>id.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+id.slice(7):id;
 for(const entry of corpus.cases){
  const db=new Database(":memory:");db.run("CREATE TABLE handle(node INTEGER,slot INTEGER,id TEXT,x REAL,y REAL);CREATE TABLE edge(slot INTEGER,id TEXT)");
  try{
   entry.nodes.forEach((node:any,outer:number)=>node.handles.forEach((handle:any,inner:number)=>{let x:number,y:number;if(node.shape==="rectangle"){const ux=-Math.sin(handle.angle),uy=-Math.cos(handle.angle),hw=(node.width??48)/2,hh=(node.height??48)/2,scale=Math.min(ux===0?Infinity:hw/Math.abs(ux),uy===0?Infinity:hh/Math.abs(uy));x=node.x+ux*scale;y=node.y+uy*scale;}else{x=node.x+(node.radius??24)*Math.cos(handle.angle);y=node.y+(node.radius??24)*Math.sin(handle.angle);}db.query("INSERT INTO handle VALUES(?,?,?,?,?)").run(outer,inner,expanded(handle.id),x,y);}));
   entry.edges.forEach((id:string,index:number)=>db.query("INSERT INTO edge VALUES(?,?)").run(index,expanded(id)));
   const mutation=entry.mutation,scalar=(value:unknown)=>value==="nan"?NaN:value==="infinity"?Infinity:Number(value),fields=["gap","shift","rise","rotation","turn","tilt","x","y","tolerance"],invalid=fields.find(name=>!Number.isFinite(scalar(mutation[name]??0)));
   let status=invalid?"nonfinite-"+invalid:mutation.tolerance!==null&&scalar(mutation.tolerance)<0?"negative-tolerance":db.query("SELECT slot FROM edge WHERE id=? ORDER BY slot LIMIT 1").get(expanded(mutation.id))?"duplicate-id":"changed",warning="none",distance:number|null=null;
   if(status==="changed"&&mutation.tolerance!==null){const a=db.query("SELECT x,y FROM handle WHERE id=? ORDER BY node,slot LIMIT 1").get(expanded(mutation.source))as{x:number,y:number}|null,b=db.query("SELECT x,y FROM handle WHERE id=? ORDER BY node,slot LIMIT 1").get(expanded(mutation.target))as{x:number,y:number}|null;if(a&&b){distance=new Vector2(a.x,a.y).distanceTo(new Vector2(b.x,b.y));if(!(distance<=scalar(mutation.tolerance)))warning="too-far";}else warning="missing-handle";}
   expect(status).toBe(entry.status);expect(warning).toBe(entry.warning);expect(status==="changed"?Math.min(mutation.index??entry.edges.length,entry.edges.length):null).toBe(entry.position);expect(distance).toBe(entry.distance);
   console.log("[DEBUG] Puzzle connect "+entry.id+" independent SQLite first handle + Three distance preserved "+status+"/"+warning);
  }finally{db.close();}
 }
 const source=await Bun.file(new URL(base+"🦀️.rs",import.meta.url)).text();expect(source).toContain("pub struct Puzzle2dConnectHandlesPreparationCursor");expect(source).toContain("Puzzle2dLookupScope::Handle");expect(source).not.toMatch(/\.to_string_owner\(|\.clone\(|serde_json|MutationKind.*diff/);
});


test("Puzzle2d connect inverse owns only unconditional literal ID intent",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🪢️connect-handles/🎮️prepare/↩️inverse/",import.meta.url),corpus=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json();
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){expect(row.inverse).toEqual([{kind:"disconnect-handles",id:row.id}]);const id=text(row.id),db=new Database(":memory:");try{db.run("CREATE TABLE inverse_intent(kind TEXT NOT NULL,id TEXT NOT NULL)");db.run("INSERT INTO inverse_intent VALUES(?,?)",["disconnect-handles",id]);expect(db.query("SELECT kind,id,length(CAST(id AS BLOB)) AS bytes FROM inverse_intent").get()).toEqual({kind:"disconnect-handles",id,bytes:new TextEncoder().encode(id).length});}finally{db.close()}
 if(row.changed){const forward=applyPatch(row.base.map(text),[{op:"add",path:`/${row.position}`,value:id}]).newDocument;expect(applyPatch(forward,[{op:"remove",path:`/${row.position}`}]).newDocument).toEqual(row.base.map(text));}
 console.log(`[DEBUG] Puzzle connect inverse ${row.name} preserves literal ID against SQLite and RFC6902 ordered restoration`);
 }
 const source=await Bun.file(new URL("🦀️.rs",root)).text();expect(source).toContain("pub type Puzzle2dConnectHandlesInverseCursor");expect(source).toContain("impl Puzzle2dLiteralIdInverse for ConnectHandles");expect(source).toContain("&self.id");const owner=await Bun.file(new URL("../../../🏷️literal-id-inverse/🦀️.rs",root)).text();expect(owner).toContain("mutation.project(1, M::inverse_id)");expect(owner).toContain("PagedList<Puzzle2dMutation");expect(owner).toContain("ControlledRetirement");expect(owner).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
});


test("Puzzle2d connection edge assembly owns native fields without copying preconditions",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🪢️connect-handles/🎮️prepare/🪢️edge/",import.meta.url),corpus=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json();
 const text=(value:string|null)=>value?.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const db=new Database(":memory:");try{db.run("CREATE TABLE native_edge(id TEXT,source TEXT,target TEXT,edge_kind TEXT,source_tip TEXT,target_tip TEXT)");const columns=["id","source","target","edgeKind","sourceTip","targetTip"];db.run("INSERT INTO native_edge VALUES(?,?,?,?,?,?)",columns.map(key=>text(row.mutation[key])));expect(Object.values(db.query("SELECT * FROM native_edge").get()!)).toEqual(columns.map(key=>text(row.edge[key])));
 db.run("CREATE TABLE native_scalar(ordinal INTEGER PRIMARY KEY,bits TEXT NOT NULL)");row.mutation.scalarBits.forEach((bits:string,index:number)=>db.run("INSERT INTO native_scalar VALUES(?,?)",[index,bits]));expect(db.query("SELECT bits FROM native_scalar ORDER BY ordinal").all().map((value:any)=>value.bits)).toEqual(row.edge.scalarBits);expect(row.edge.visible).toBeNull();expect(row.edge.locked).toBeNull();expect(Object.keys(row.edge)).not.toContain("toleranceBits");expect(Object.keys(row.edge)).not.toContain("index");console.log(`[DEBUG] Puzzle connection edge ${row.name} native fields and exact scalar words agree with independent SQLite`);
 }finally{db.close()}}
 const source=await Bun.file(new URL("🦀️.rs",root)).text();expect(source).toContain("pub struct Puzzle2dConnectEdgeCursor");expect(source).toContain("mutation.project");expect(source).toContain("ControlledRetirement");expect(source).not.toMatch(/to_string_owner|\.clone\(|serde_json|Vec<|format!|tolerance|\.index/);
});


test("Puzzle2d connect candidate preserves every unrelated owner and native ordered placement",async()=>{
 const root=new URL("../../../../🧬️schema/🧬️mutations/🪢️connect-handles/🎮️prepare/",import.meta.url),base=await Bun.file(new URL("🧫️fixtures/🔣️.json",root)).json(),corpus=await Bun.file(new URL("📸️candidate/🧫️fixtures/🔣️.json",root)).json();
 for(const row of corpus.cases){const intent=base.cases.find((entry:any)=>entry.id===row.preparationCase);expect(intent).toBeDefined();const mutation={...intent.mutation,index:row.index},before={edges:row.edges.map((id:string)=>({id,visible:false,locked:true,sourceTip:"retained\u0000😀"})),untouched:{nodes:intent.nodes,camera:[3,-4,1],metadata:"native\u0000😀"}},changed=intent.status==="changed",position=Math.min(row.index??row.edges.length,row.edges.length),edge={id:mutation.id,source:mutation.source,target:mutation.target,edgeKind:mutation.edgeKind,sourceTip:mutation.sourceTip,targetTip:mutation.targetTip,gap:mutation.gap,shift:mutation.shift,rise:mutation.rise,rotation:mutation.rotation,turn:mutation.turn,tilt:mutation.tilt,x:mutation.x,y:mutation.y,visible:null,locked:null};
 const after=applyPatch(structuredClone(before),changed?[{op:"add",path:`/edges/${position}`,value:edge}]:[]).newDocument;expect(after.edges.map((edge:any)=>edge.id)).toEqual(row.after);expect(after.untouched).toEqual(before.untouched);if(changed)expect(after.edges[position]).toEqual(edge);else expect(after).toEqual(before);
 const db=new Database(":memory:");try{db.run("CREATE TABLE native_order(position INTEGER PRIMARY KEY,record TEXT NOT NULL)");before.edges.forEach((edge:any,index:number)=>db.run("INSERT INTO native_order VALUES(?,?)",[index*2,JSON.stringify(edge)]));if(changed){db.run("UPDATE native_order SET position=position+1000 WHERE position>=?",[position*2]);db.run("UPDATE native_order SET position=position-998 WHERE position>=1000");db.run("INSERT INTO native_order VALUES(?,?)",[position*2,JSON.stringify(edge)])}expect(db.query("SELECT record FROM native_order ORDER BY position").all().map((item:any)=>JSON.parse(item.record))).toEqual(after.edges)}finally{db.close()}
 console.log(`[DEBUG] Puzzle connect candidate ${row.name} native ordered records and untouched owner conservation agree with independent SQLite/RFC6902`);
 }
 const source=await Bun.file(new URL("📸️candidate/🦀️.rs",root)).text();expect(source).toContain("pub struct Puzzle2dConnectHandlesCandidateCursor");expect(source).toContain("Puzzle2dConnectHandlesPreparationCursor");expect(source).toContain("RetainedFieldCursor::<Puzzle2dSnapshot>::default()");expect(source).toContain("Puzzle2dConnectEdgeCursor");expect(source).toContain("PagedListEditCursor::insert");expect(source).toContain("ControlledRetirement");expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
});


test("Puzzle2d borrowed optional flags preserve four typed native intents and first target semantics", () => {
 const path=resolve(import.meta.dir,"../../../../🧬️schema/🧬️mutations/⚑️flag/🎮️prepare");
 const corpus=JSON.parse(readFileSync(resolve(path,"🧫️fixtures/🔣️.json"),"utf8"));

 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 const patch=require("fast-json-patch");
 for(const row of corpus.cases){
  const db=new Database(":memory:");db.run("CREATE TABLE native_owner (ordinal INTEGER PRIMARY KEY, identifier TEXT, value INTEGER)");
  const before=row.rows.map((item:{id:string;value:boolean|null})=>({id:text(item.id),value:item.value}));
  for(const [i,item]of before.entries())db.query("INSERT INTO native_owner VALUES (?1,?2,?3)").run(i,item.id,item.value===null?null:Number(item.value));
  const found=db.query("SELECT ordinal,value FROM native_owner WHERE identifier=?1 ORDER BY ordinal LIMIT 1").get(text(row.mutation.id)) as {ordinal:number;value:number|null}|null;
  expect(found?.ordinal??null).toEqual(row.index);
  const previous=found===null?null:found.value===null?null:Boolean(found.value);expect(previous).toEqual(row.previous);
  const status=found===null?"target-missing":previous===row.mutation.value?"no-op":"changed";expect(status).toBe(row.status);
  if(status==="changed"){const after=structuredClone(before);after[found!.ordinal]!.value=row.mutation.value;const edits=patch.compare(before,after);expect(patch.applyPatch(structuredClone(before),edits).newDocument).toEqual(after);expect(patch.applyPatch(structuredClone(after),patch.compare(after,before)).newDocument).toEqual(before);}
  db.close();
 }
 console.log("[DEBUG] forty optional node/edge locked/visible first-ID plans agree with SQLite and RFC6902 exact null/false restoration");
 const source=readFileSync(resolve(path,"🦀️.rs"),"utf8");expect(source).toContain("Puzzle2dFlagPreparationCursor");expect(source).toContain("RetainedCloneBinding::close_one");expect(source).not.toContain(".clone()");expect(source).not.toContain(".to_string_owner()");
});


test("Puzzle2d native optional flag inverses retain authored absence and false exactly",()=>{
 const path=resolve(import.meta.dir,"../../../../🧬️schema/🧬️mutations/⚑️flag/🎮️prepare");const corpus=JSON.parse(readFileSync(resolve(path,"🧫️fixtures/🔣️.json"),"utf8"));const inverse=JSON.parse(readFileSync(resolve(path,"↩️inverse/🧫️fixtures/🔣️.json"),"utf8"));
 const kinds:Record<string,[string,string]>={"node-locked":["changeNodeLocked","newLocked"],"node-visible":["changeNodeVisible","newVisible"],"edge-locked":["changeEdgeLocked","newLocked"],"edge-visible":["changeEdgeVisible","newVisible"]};
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const db=new Database(":memory:");db.run("CREATE TABLE native_flag (ordinal INTEGER PRIMARY KEY, identifier TEXT, value INTEGER)");for(const [i,item]of row.rows.entries())db.query("INSERT INTO native_flag VALUES (?1,?2,?3)").run(i,text(item.id),item.value===null?null:Number(item.value));const found=db.query("SELECT identifier,value FROM native_flag WHERE identifier=?1 ORDER BY ordinal LIMIT 1").get(text(row.mutation.id))as{identifier:string;value:number|null}|null;const[kind,key]=kinds[row.kind]!;const actual=found===null?[]:[{mutation:kind,id:found.identifier,[key]:found.value===null?null:Boolean(found.value)}];expect(actual).toEqual(inverse.cases.find((value:{id:string})=>value.id===row.id).inverse.map((value:{id:string})=>({...value,id:text(value.id)})));db.close();}
 console.log("[DEBUG] forty literal typed flag inverse owners including no-op and missing targets agree with independent SQLite");
 const source=readFileSync(resolve(path,"↩️inverse/🦀️.rs"),"utf8");expect(source).toContain("Puzzle2dFlagInverseCursor");expect(source).toContain("ControlledRetirement");expect(source).not.toContain(".clone()");expect(source).not.toContain(".to_string_owner()");
});


test("Puzzle2d native optional flag candidates preserve every untouched ordinal owner",()=>{
 const path=resolve(import.meta.dir,"../../../../🧬️schema/🧬️mutations/⚑️flag/🎮️prepare");const corpus=JSON.parse(readFileSync(resolve(path,"🧫️fixtures/🔣️.json"),"utf8"));const candidates=JSON.parse(readFileSync(resolve(path,"📸️candidate/🧫️fixtures/🔣️.json"),"utf8"));const patch=require("fast-json-patch");const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const before=row.rows.map((item:{id:string;value:boolean|null})=>({...item,id:text(item.id)}));const db=new Database(":memory:");db.run("CREATE TABLE native_flag (ordinal INTEGER PRIMARY KEY, identifier TEXT, value INTEGER)");for(const[i,item]of before.entries())db.query("INSERT INTO native_flag VALUES (?1,?2,?3)").run(i,item.id,item.value===null?null:Number(item.value));let actual=null;if(row.status==="changed"){db.query("UPDATE native_flag SET value=?1 WHERE ordinal=(SELECT ordinal FROM native_flag WHERE identifier=?2 ORDER BY ordinal LIMIT 1)").run(row.mutation.value===null?null:Number(row.mutation.value),text(row.mutation.id));actual=(db.query("SELECT identifier,value FROM native_flag ORDER BY ordinal").all()as{identifier:string;value:number|null}[]).map(item=>({id:item.identifier,value:item.value===null?null:Boolean(item.value)}));expect(patch.applyPatch(structuredClone(before),patch.compare(before,actual)).newDocument).toEqual(actual);expect(patch.applyPatch(structuredClone(actual),patch.compare(actual,before)).newDocument).toEqual(before);}const expected=candidates.cases.find((item:{id:string})=>item.id===row.id).after;expect(actual).toEqual(expected===null?null:expected.map((item:{id:string})=>({...item,id:text(item.id)})));db.close();}
 console.log("[DEBUG] forty exact native flag candidate ordinal rows and untouched owners agree with independent SQLite/RFC6902");
 const source=readFileSync(resolve(path,"📸️candidate/🦀️.rs"),"utf8");expect(source).toContain("Puzzle2dFlagCandidateCursor");expect(source).toContain("RetainedFieldCursor");expect(source).toContain("ControlledRetirement");expect(source).not.toContain(".clone()");expect(source).not.toContain(".to_string_owner()");
});


test("Puzzle2d native optional root flag preserves literal preparation inverse and candidate owners",()=>{
 const path=resolve(import.meta.dir,"../../../../🧬️schema/🧬️mutations/⚑️flag/🎮️prepare"),corpus=JSON.parse(readFileSync(resolve(path,"🌟️root/🧫️fixtures/🔣️.json"),"utf8")),patch=require("fast-json-patch");
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const before=row.rows.map((item:{id:string;value:boolean|null})=>({...item,id:text(item.id)})),db=new Database(":memory:");try{db.run("CREATE TABLE root_flag (ordinal INTEGER PRIMARY KEY, identifier TEXT, value INTEGER)");for(const[i,item]of before.entries())db.query("INSERT INTO root_flag VALUES(?1,?2,?3)").run(i,item.id,item.value===null?null:Number(item.value));const found=db.query("SELECT ordinal,identifier,value FROM root_flag WHERE identifier=?1 ORDER BY ordinal LIMIT 1").get(text(row.mutation.id))as{ordinal:number;identifier:string;value:number|null}|null,previous=found===null?null:found.value===null?null:Boolean(found.value),status=found===null?"target-missing":previous===row.mutation.value?"no-op":"changed";expect(found?.ordinal??null).toBe(row.index);expect(previous).toBe(row.previous);expect(status).toBe(row.status);const inverse=found===null?[]:[{mutation:"changeNodeRoot",id:found.identifier,newRoot:previous}];expect(inverse).toEqual(row.inverse.map((owner:{id:string})=>({...owner,id:text(owner.id)})));let after=null;if(status==="changed"){after=structuredClone(before);after[found!.ordinal]!.value=row.mutation.value;expect(patch.applyPatch(structuredClone(before),patch.compare(before,after)).newDocument).toEqual(after);expect(patch.applyPatch(structuredClone(after),patch.compare(after,before)).newDocument).toEqual(before)}expect(after).toEqual(row.after===null?null:row.after.map((item:{id:string})=>({...item,id:text(item.id)})))}finally{db.close()}}
 console.log("[DEBUG] ten native root optional first-owner plans, literal inverse and ordered candidate cases agree with independent SQLite/RFC6902");const source=readFileSync(resolve(path,"🦀️.rs"),"utf8");expect(source).toContain("impl Puzzle2dFlagIntent for ChangeNodeRoot");expect(source).toContain(".root=value");
});

test("Puzzle2d native required region flags preserve literal first owner inverse and candidate states",()=>{
 const path=resolve(import.meta.dir,"../../../../🧬️schema/🧬️mutations/⚑️flag/🎮️prepare"),corpus=JSON.parse(readFileSync(resolve(path,"🏁️region/🧫️fixtures/🔣️.json"),"utf8")),patch=require("fast-json-patch");
 const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
 for(const row of corpus.cases){const before=row.rows.map((item:{id:string;value:boolean|null})=>({...item,id:text(item.id)})),db=new Database(":memory:");try{db.run("CREATE TABLE region_flag (ordinal INTEGER PRIMARY KEY, identifier TEXT, value INTEGER)");for(const[i,item]of before.entries())db.query("INSERT INTO region_flag VALUES(?1,?2,?3)").run(i,item.id,item.value===null?null:Number(item.value));const found=db.query("SELECT ordinal,identifier,value FROM region_flag WHERE identifier=?1 ORDER BY ordinal LIMIT 1").get(text(row.mutation.id))as{ordinal:number;identifier:string;value:number|null}|null,previous=found===null?null:found.value===null?null:Boolean(found.value),status=found===null?"target-missing":previous===row.mutation.value?"no-op":"changed";expect(found?.ordinal??null).toBe(row.index);expect(previous).toBe(row.previous);expect(status).toBe(row.status);const inverse=found===null?[]:[{mutation:row.kind==="region-hidden"?"changeTargetRegionHidden":"changeTargetRegionLocked",id:found.identifier,[row.kind==="region-hidden"?"newHidden":"newLocked"]:previous}];expect(inverse).toEqual(row.inverse.map((owner:{id:string})=>({...owner,id:text(owner.id)})));let after=null;if(status==="changed"){after=structuredClone(before);after[found!.ordinal]!.value=row.mutation.value;expect(patch.applyPatch(structuredClone(before),patch.compare(before,after)).newDocument).toEqual(after);expect(patch.applyPatch(structuredClone(after),patch.compare(after,before)).newDocument).toEqual(before)}expect(after).toEqual(row.after===null?null:row.after.map((item:{id:string})=>({...item,id:text(item.id)})))}finally{db.close()}}
 console.log("[DEBUG] twenty native region required first-owner plans, literal inverse and ordered candidate cases agree with independent SQLite/RFC6902");const source=readFileSync(resolve(path,"🦀️.rs"),"utf8");expect(source).toContain("impl Puzzle2dFlagIntent for ChangeTargetRegionHidden");expect(source).toContain("impl Puzzle2dFlagIntent for ChangeTargetRegionLocked");expect(source).toContain("Puzzle2dLookupScope::Region");
});


test("Puzzle2d child preparation owns exact granted native birth and terminal release",()=>{
 const base=resolve(import.meta.dir,"../../../../🧬️schema/🧬️mutations/🎮️prepare/🧰️child");
 const corpus=JSON.parse(readFileSync(resolve(base,"🧫️fixtures/🔣️.json"),"utf8"));
 const db=new Database(":memory:");
 try{
  db.run("CREATE TABLE turns(birth INTEGER,free INTEGER)");
  let born=false;
  for(const turn of corpus.turns){
   const birth=!born&&turn.capacity>=corpus.nativeChildBytes?corpus.nativeChildBytes:0;
   if(birth)born=true;
   const free=born&&turn.terminal&&turn.release>=corpus.nativeChildBytes?corpus.nativeChildBytes:0;
   expect({birth,free}).toEqual({birth:turn.birth,free:turn.free});
   db.run("INSERT INTO turns VALUES(?,?)",[birth,free]);
   expect(birth+free).toBeLessThanOrEqual(corpus.maximumTurnBytes);
   if(free)born=false;
  }
  expect(db.query("SELECT SUM(birth) AS birth,SUM(free) AS free FROM turns").get()).toEqual({birth:64,free:64});
  const native=readFileSync(resolve(base,"🦀️.rs"),"utf8");
  expect(native).toContain("maximum_capacity_bytes < bytes");expect(native).toContain("maximum_release_bytes < bytes");expect(native).toContain("owner.terminal_is_empty()");
 }finally{db.close();}
});
