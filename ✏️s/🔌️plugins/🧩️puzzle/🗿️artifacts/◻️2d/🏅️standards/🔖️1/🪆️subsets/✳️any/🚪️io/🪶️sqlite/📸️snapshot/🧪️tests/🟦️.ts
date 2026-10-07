import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
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
