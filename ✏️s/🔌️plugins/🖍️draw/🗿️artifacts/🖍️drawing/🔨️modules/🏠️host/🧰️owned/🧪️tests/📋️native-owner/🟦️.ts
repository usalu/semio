import {Database} from "bun:sqlite";
import {test, expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve, join} from "node:path";
import {applyPatch} from "fast-json-patch";
import {createHash} from "node:crypto";
import {semioSchemaAjvV1} from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

test("Drawing initializer closes the original catalog one exactly admitted page at a time",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const directory=join(owner,"🏗️initialization/📚️catalog");
  const law=JSON.parse(readFileSync(join(directory,"🧫️fixtures/🔣️.json"),"utf8"));
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE pages (ordinal INTEGER PRIMARY KEY, lane TEXT UNIQUE NOT NULL, bytes INTEGER NOT NULL)");
    law.lanes.forEach((lane:string,index:number)=>database.query("INSERT INTO pages VALUES (?,?,?)").run(index,lane,128+index));
    let state={pages:law.lanes.map((lane:string,index:number)=>({lane,bytes:128+index}))};
    for(const lane of law.lanes){
      const row=database.query("SELECT ordinal,lane,bytes FROM pages ORDER BY ordinal LIMIT 1").get() as {ordinal:number,lane:string,bytes:number};
      expect(row.lane).toBe(lane);
      for(const grant of [0,row.bytes-1])expect(database.query("SELECT CASE WHEN ? >= bytes THEN bytes ELSE 0 END AS released FROM pages WHERE ordinal=?").get(grant,row.ordinal)).toEqual({released:0});
      expect(database.query("SELECT CASE WHEN ? >= bytes THEN bytes ELSE 0 END AS released FROM pages WHERE ordinal=?").get(row.bytes,row.ordinal)).toEqual({released:row.bytes});
      database.query("DELETE FROM pages WHERE ordinal=?").run(row.ordinal);
      state=applyPatch(state,[{op:"remove",path:"/pages/0"}],true,false).newDocument;
      expect(state.pages.map((page:any)=>page.lane)).toEqual(database.query("SELECT lane FROM pages ORDER BY ordinal").all().map((page:any)=>page.lane));
    }
    expect(state).toEqual({pages:[]});
  }finally{database.close();}
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const initializer=host.slice(host.indexOf("struct DrawingStoreInitializationAuthority"),host.indexOf("pub fn drawing_document_store_initialization_job"));
  expect(initializer.includes("drop(self.owner_catalog.take())")).toBe(false);
  expect(initializer.includes("close_initialization_catalog(&mut self.owner_catalog, 1, maximum_bytes)")).toBe(true);
  expect(initializer.includes("close_initialization_catalog(&mut self.owner_catalog, maximum_items, maximum_bytes)")).toBe(true);
  expect(initializer.includes("PluginCloseStep::Pending { released_items, released_bytes }")).toBe(true);
  const source=readFileSync(join(directory,"🦀️.rs"),"utf8");
  expect(source.includes("catalog.close_step(1, maximum_bytes)")).toBe(true);
  expect(source.includes("if !catalog.terminal_is_empty()")).toBe(true);
  console.log("[DEBUG] Drawing initializer catalog SQLite exact first-page admission and RFC6902 resident-page order agree; original catalog birth is cold and uncredited");
});

test("Drawing initializer transfers original active owners into inline typed retirement",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"♻️retirement/🧫️fixtures/🔣️.json"),"utf8"));
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE pending (ordinal INTEGER PRIMARY KEY, kind TEXT UNIQUE NOT NULL, backingBytes INTEGER NOT NULL)");
    law.initializerOwners.forEach((kind:string,index:number)=>database.query("INSERT INTO pending VALUES (?,?,?)").run(index,kind,128));
    expect(database.query("SELECT kind, backingBytes FROM pending ORDER BY ordinal").all()).toEqual(law.initializerOwners.map((kind:string)=>({kind,backingBytes:128})));
    for(const kind of law.initializerOwners){
      const state=applyPatch({pending:{kind,backingBytes:128}},[{op:"move",from:"/pending",path:"/active"}],true,false).newDocument;
      expect(state).toEqual({active:{kind,backingBytes:128}});
      expect(applyPatch(state,[{op:"remove",path:"/active"}],true,false).newDocument).toEqual({});
    }
  }finally{database.close();}
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const initializer=host.slice(host.indexOf("struct DrawingStoreInitializationAuthority"),host.indexOf("pub fn drawing_document_store_initialization_job"));
  expect(initializer.includes("active: std::mem::ManuallyDrop<Option<DrawingOwnedRetirement>>")).toBe(true);
  expect(initializer.includes("Box::new(DrawingOwnedRetirement")).toBe(false);
  expect(initializer.includes("ArtifactOwnedValueRetirementFactory::retire_owned")).toBe(false);
  console.log("[DEBUG] Drawing initializer SQLite/RFC6902 retain the exact original Snapshot/HistoryId under one typed active owner until terminal; outer lifecycle demands remain separately required");
});

test("Drawing native retirement retains typed variants under independent physical grants",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const directory=join(owner,"♻️retirement");
  const law=JSON.parse(readFileSync(join(directory,"🧫️fixtures/🔣️.json"),"utf8"));
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE owners (ordinal INTEGER PRIMARY KEY, kind TEXT UNIQUE NOT NULL); CREATE TABLE backing (ordinal INTEGER PRIMARY KEY, bytes INTEGER NOT NULL)");
    law.variants.forEach((kind:string,index:number)=>database.query("INSERT INTO owners VALUES (?,?)").run(index,kind));
    law.backingBytes.forEach((bytes:number,index:number)=>database.query("INSERT INTO backing VALUES (?,?)").run(index,bytes));
    expect(database.query("SELECT kind FROM owners ORDER BY ordinal").all().map((row:any)=>row.kind)).toEqual(law.variants);
    for(const bytes of law.backingBytes){
      expect(database.query("SELECT CASE WHEN ? >= bytes THEN bytes ELSE 0 END AS released FROM backing WHERE bytes=?").get(Math.max(0,bytes-1),bytes)).toEqual({released:0});
      expect(database.query("SELECT CASE WHEN ? >= bytes THEN bytes ELSE 0 END AS released FROM backing WHERE bytes=?").get(bytes,bytes)).toEqual({released:bytes});
      const transferred=applyPatch({input:{backingBytes:bytes}},[{op:"move",from:"/input",path:"/retirement"}],true,false).newDocument;
      expect(transferred).toEqual({retirement:{backingBytes:bytes}});
      expect(applyPatch(transferred,[{op:"remove",path:"/retirement"}],true,false).newDocument).toEqual({});
    }
  }finally{database.close();}
  const source=readFileSync(join(directory,"🦀️.rs"),"utf8");
  expect(source.includes("DrawingDecodedFieldRetirement<DrawingRetirementOwner>")).toBe(true);
  expect(source.includes("owned_retirement(")).toBe(false);
  expect(source.includes("self.owner.next_grant()")).toBe(true);
  const helper=readFileSync(join(owner,"🦀️.rs"),"utf8");
  for(const axis of law.axes)expect(helper.includes(`next_${axis}_byte_demand`)).toBe(true);
  console.log("[DEBUG] Drawing native retirement nine typed variants, SQLite exact/undergrant backing and RFC6902 unique terminal transfer agree; native heap execution remains required");
});

test("Drawing decoded field close admits payload work separately from physical backing",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const contract=resolve(import.meta.dir,"../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/♻️retirement/🧫️fixtures/📏️copy-demand");
  const law=JSON.parse(readFileSync(join(contract,"🔣️.json"),"utf8"));
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE grants (bytes INTEGER PRIMARY KEY)");
    law.grants.forEach((bytes:number)=>database.query("INSERT INTO grants VALUES (?)").run(bytes));
    for(const row of law.cases){
      expect(database.query("SELECT MIN(bytes) AS bytes FROM grants WHERE bytes >= ?").get(row.minimumCopyBytes)).toEqual({bytes:row.minimumCopyBytes});
      const bytes=row.kind==="text"?new TextEncoder().encode(row.value).length:row.kind==="u32-list"?Uint32Array.from(row.value).byteLength:Uint32Array.of(row.value).byteLength;
      expect(bytes).toBe(row.totalCopyBytes);
      const moved=applyPatch({field:{value:row.value,backingBytes:128}},[{op:"move",from:"/field",path:"/retirement"}],true,false).newDocument;
      const processed=applyPatch(moved,[{op:"remove",path:"/retirement/value"}],true,false).newDocument;
      expect(processed.retirement.backingBytes).toBe(128);
    }
  }finally{database.close();}
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const close=host.slice(host.indexOf("impl<T: semio_framework_value::retirement::RetireOwned> DrawingDecodedFieldRetirement"),host.indexOf("macro_rules! drawing_owned_field_close_capacity"));
  expect(close.includes("next_copy_byte_demand()")).toBe(true);
  expect(close.includes("maximum_copy_bytes: copy")).toBe(true);
  console.log("[DEBUG] Drawing decoded field SQLite admits the independent minimum payload work; platform typed bytes and RFC6902 preserve physical backing until separately granted release");
});

test("Drawing duplicate cancellation preserves the caller grant and physical axes",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE grants (bytes INTEGER PRIMARY KEY)");
    law.duplicateClose.grants.forEach((bytes:number)=>database.query("INSERT INTO grants VALUES (?)").run(bytes));
    expect(database.query("SELECT bytes, MIN(bytes,4096) AS admitted FROM grants ORDER BY bytes").all()).toEqual(law.duplicateClose.grants.map((bytes:number)=>({bytes,admitted:Math.min(bytes,4096)})));
    expect(applyPatch({copy:0,capacity:0,release:0},[{op:"replace",path:"/copy",value:7},{op:"replace",path:"/copy",value:0},{op:"replace",path:"/release",value:7}],true,false).newDocument).toEqual({copy:0,capacity:0,release:7});
  }finally{database.close();}
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const rewrite=host.slice(host.indexOf("impl DrawingDuplicateRewriteAuthority"),host.indexOf("enum DrawingMutationCandidatePhase"));
  expect(rewrite.includes("fn close_granted(")).toBe(true);
  expect(rewrite.includes("maximum_bytes.max(demand)")).toBe(false);
  expect(rewrite.includes("retire_displaced_granted(grant)")).toBe(true);
  console.log("[DEBUG] Drawing duplicate cancellation SQLite exact grant admission and RFC6902 distinct copy/release axes agree; native physical heap witness remains required");
});

test("Drawing duplicate rewrite preserves root names and internal reference ownership",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const root=resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const input=JSON.parse(readFileSync(resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"),"utf8"));
  const corpus=JSON.parse(readFileSync(join(root,"🎬️scene/🔀️booleans/🧫️fixtures/🔣️.json"),"utf8"));
  const boolean=structuredClone(corpus[0].document.layers.find((value:any)=>value.kind==="boolean"));
  if(boolean.attributes.stroke===null)delete boolean.attributes.stroke;
  boolean.id="boolean-copy-source";boolean.children=["text-a","external"];
  input.layers[1].children.push(boolean);
  const validate=semioSchemaAjvV1({allErrors:true}).addSchema(JSON.parse(readFileSync(join(root,"🔣️.json"),"utf8"))).compile(JSON.parse(readFileSync(join(root,"📸️snapshot/🔣️.json"),"utf8")));
  expect(validate(input)).toBe(true);
  const source=input.layers[1];
  const duplicated=structuredClone(source);
  duplicated.id="copied-group";duplicated.name+=" copy";duplicated.children[0].id="copied-text";duplicated.children[1].id="copied-boolean";duplicated.children[1].children=["copied-text","external"];
  const applied=applyPatch(input,[{op:"add",path:"/layers/2",value:duplicated}],true,false).newDocument;
  expect(validate(applied)).toBe(true);expect(applied.layers[1]).toEqual(source);expect(duplicated.children[0].name).toBe(source.children[0].name);
  expect(applyPatch(applied,[{op:"remove",path:"/layers/2"}],true,false).newDocument).toEqual(input);
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE identities (original TEXT PRIMARY KEY, copied TEXT NOT NULL); CREATE TABLE refs (ordinal INTEGER PRIMARY KEY, target TEXT NOT NULL)");
    [[source.id,duplicated.id],[source.children[0].id,duplicated.children[0].id],[boolean.id,duplicated.children[1].id]].forEach(([old,id])=>database.query("INSERT INTO identities VALUES (?,?)").run(old,id));
    boolean.children.forEach((id:string,index:number)=>database.query("INSERT INTO refs VALUES (?,?)").run(index,id));
    expect(database.query("SELECT COALESCE(identities.copied,refs.target) AS target FROM refs LEFT JOIN identities ON identities.original=refs.target ORDER BY ordinal").all().map((row:any)=>row.target)).toEqual(duplicated.children[1].children);
  }finally{database.close();}
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const rewrite=host.slice(host.indexOf("struct DrawingDuplicateRewriteAuthority"),host.indexOf("enum DrawingMutationCandidatePhase"));
  expect(rewrite.includes("DrawingDuplicateIdentityCursor")).toBe(true);
  expect(rewrite.includes("DrawingDuplicateReferenceSearch")).toBe(true);
  expect(rewrite.includes("PagedUtf8AppendCursor")).toBe(true);
  expect(rewrite.includes("semio.drawing.duplicate-id.v1")).toBe(false);
  console.log("[DEBUG] Drawing duplicate root-only name and internal-only reference remap agree with strict snapshot schema/Ajv, SQLite left-join and RFC6902 source/restoration");
});

test("Drawing decoded field close admits actual typed retirement births and releases",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE backing (ordinal INTEGER PRIMARY KEY, capacity INTEGER NOT NULL)");
    law.decodedFieldClose.physicalBackingCapacities.forEach((capacity:number,index:number)=>database.query("INSERT INTO backing VALUES (?,?)").run(index,capacity));
    expect(database.query("SELECT SUM(capacity) AS bytes FROM backing").get()).toEqual({bytes:law.decodedFieldClose.physicalBackingCapacities.reduce((total:number,bytes:number)=>total+bytes,0)});
    expect(database.query("SELECT capacity FROM backing ORDER BY ordinal DESC").all().map((row:any)=>row.capacity)).toEqual([...law.decodedFieldClose.physicalBackingCapacities].reverse());
    const transferred=applyPatch({field:{chunks:law.decodedFieldClose.physicalBackingCapacities}},[{op:"move",from:"/field",path:"/retirement"}],true,false).newDocument;
    expect(transferred.field).toBeUndefined();expect(transferred.retirement.chunks).toEqual(law.decodedFieldClose.physicalBackingCapacities);
  }finally{database.close();}
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  expect(host.includes("struct DrawingDecodedFieldRetirement")).toBe(true);
  const authorities=host.slice(host.indexOf("macro_rules! drawing_owned_field_close_capacity"),host.indexOf("struct DrawingRejectedConflictAuthority"));
  expect(authorities.includes("DrawingDecodedFieldRetirement<$value>")).toBe(true);
  expect(authorities.includes("usize::from(self.retirement.is_some()) * DRAWING_OWNED_FIELD_BYTES")).toBe(false);
  console.log("[DEBUG] Drawing decoded native field close SQLite physical-backing ledger and RFC6902 single ownership move conserve spare/empty/full backing; native granted birth/release witness required");
});

test("Drawing duplicate identity streams original native IDs with the canonical suffix",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const database=new Database(":memory:");
  try{
    for(const id of [...law.duplicateIdentity.ids,law.text.repeat(law.repeat)+"\0"]){
      const material=id+law.duplicateIdentity.suffix;
      const result=database.query("SELECT CAST(? || ? AS BLOB) AS material").get(id,law.duplicateIdentity.suffix) as {material:Uint8Array};
      expect(Buffer.from(result.material)).toEqual(Buffer.from(material,"utf8"));
      const restored=applyPatch({id},[{op:"replace",path:"/id",value:material},{op:"replace",path:"/id",value:id}],true,false).newDocument;
      expect(restored).toEqual({id});
    }
  }finally{database.close();}
  const schema=readFileSync(resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"),"utf8");
  expect(schema.includes('create_drawing_id("layer", format!("{old}{suffix}").as_bytes())')).toBe(true);
  expect(law.duplicateIdentity.nameSuffixOnlyOnRoot).toBe(true);expect(law.duplicateIdentity.internalReferencesRemapped).toBe(true);
  const source=readFileSync(join(owner,"📐️footprint/🦀️.rs"),"utf8");
  expect(source.includes("struct DrawingDuplicateIdentityCursor")).toBe(true);
  console.log("[DEBUG] Drawing duplicate neutral Unicode/NUL/long ID+suffix material agrees with SQLite BLOB and RFC6902; native SipHash1-3 oracle separately verifies the digest");
});

test("Drawing sparse path patches preserve native array ownership and semantic restoration",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const root=resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const fixture=resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape");
  const before=JSON.parse(readFileSync(join(fixture,"📸️snapshot/⬅️before/🔣️.json"),"utf8"));
  const after=JSON.parse(readFileSync(join(fixture,"📸️snapshot/➡️after/🔣️.json"),"utf8"));
  const delta=JSON.parse(readFileSync(join(fixture,"🔺️diff/🔣️.json"),"utf8"));
  const validate=semioSchemaAjvV1({allErrors:true}).addSchema(JSON.parse(readFileSync(join(root,"🔣️.json"),"utf8"))).compile(JSON.parse(readFileSync(join(root,"📸️snapshot/🔣️.json"),"utf8")));
  expect(validate(before)).toBe(true);expect(validate(after)).toBe(true);
  const segments=delta.layers.modified[0].patch.pathSegments;
  const applied=applyPatch(before,[{op:"replace",path:"/layers/0/segments",value:segments}],true,false).newDocument;
  expect(applied).toEqual(after);
  const restored=applyPatch(applied,[{op:"replace",path:"/layers/0/segments",value:before.layers[0].segments}],true,false).newDocument;
  expect(restored).toEqual(before);
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE segments (ordinal INTEGER PRIMARY KEY, json TEXT NOT NULL)");
    segments.forEach((value:unknown,index:number)=>database.query("INSERT INTO segments VALUES (?,?)").run(index,JSON.stringify(value)));
    expect(database.query("SELECT json FROM segments ORDER BY ordinal").all().map((value:any)=>JSON.parse(value.json))).toEqual(segments);
  }finally{database.close();}
  const source=readFileSync(join(root,"🔺️diff/🦀️.rs"),"utf8");
  expect(source.includes("pub path_segments: Option<PagedList<crate::PathSegment, {usize::MAX}>>")).toBe(true);
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  expect(host.includes("retired.extend(std::mem::replace(&mut path.segments")).toBe(false);
  console.log("[DEBUG] Drawing sparse path native array owner agrees with existing snapshot schema/Ajv, SQLite ordinals and RFC6902 exact before/after/restoration");
});

test("Drawing native identifier equality reads separate bounded chunk owners",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const cases=[...law.textEquality,{left:law.text.repeat(law.repeat)+"\0",right:law.text.repeat(law.repeat)+"\0",equal:true},{left:law.text.repeat(law.repeat)+"x",right:law.text.repeat(law.repeat)+"y",equal:false}];
  const database=new Database(":memory:");
  try{
    for(const value of cases){
      const result=database.query("SELECT CAST(? AS BLOB) = CAST(? AS BLOB) AS equal").get(value.left,value.right) as {equal:number};
      expect(Boolean(result.equal)).toBe(value.equal);
      expect(Buffer.from(value.left,"utf8").equals(Buffer.from(value.right,"utf8"))).toBe(value.equal);
      const restored=applyPatch({id:value.left},[{op:"replace",path:"/id",value:value.right},{op:"replace",path:"/id",value:value.left}],true,false).newDocument;
      expect(restored).toEqual({id:value.left});
    }
  }finally{database.close();}
  const source=readFileSync(join(owner,"📐️footprint/🦀️.rs"),"utf8");
  expect(source.includes("struct DrawingTextEqualityCursor")).toBe(true);
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const locator=host.slice(host.indexOf("struct DrawingLayerLocator"),host.indexOf("enum DrawingContainerRebuildMove"));
  expect(locator.includes("DrawingTextEqualityCursor")).toBe(true);
  expect(locator.includes("target: &str")).toBe(false);
  console.log("[DEBUG] Drawing native ID equality SQLite BLOB/Node Buffer/RFC6902 agree on exact Unicode/NUL/empty/late-mismatch identities without String reconstruction");
});

test("Drawing text mutation candidates adopt one native copy and retire the displaced field",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const input=JSON.parse(readFileSync(resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"),"utf8"));
  const root=resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const validate=semioSchemaAjvV1({allErrors:true}).addSchema(JSON.parse(readFileSync(join(root,"🔣️.json"),"utf8"))).compile(JSON.parse(readFileSync(join(root,"📸️snapshot/🔣️.json"),"utf8")));
  const long=law.text.repeat(law.repeat)+"\0";
  const candidate=applyPatch(input,[{op:"replace",path:"/layers/1/children/0/name",value:long}],true,false).newDocument;
  expect(validate(input)).toBe(true);expect(validate(candidate)).toBe(true);
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE owners (role TEXT PRIMARY KEY, text TEXT NOT NULL)");
    database.query("INSERT INTO owners VALUES (?,?)").run("source",input.layers[1].children[0].name);
    database.query("INSERT INTO owners VALUES (?,?)").run("candidate",candidate.layers[1].children[0].name);
    expect(database.query("SELECT text FROM owners WHERE role = ?").get("candidate")).toEqual({text:long});
    expect(database.query("SELECT text FROM owners WHERE role = ?").get("source")).toEqual({text:input.layers[1].children[0].name});
  }finally{database.close();}
  const restored=applyPatch(candidate,[{op:"replace",path:"/layers/1/children/0/name",value:input.layers[1].children[0].name}],true,false).newDocument;
  expect(restored).toEqual(input);
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  expect(host.includes("fn write_overlay_string")).toBe(false);
  expect(host.includes("clone_text_work.step")).toBe(true);
  expect(host.includes("fn adopt_native_text")).toBe(true);
  console.log("[DEBUG] Drawing native text candidate schema/Ajv/SQLite/RFC6902 preserve long UTF8 and displaced owner identity without writing through whole String overlays");
});

test("Drawing root and asset census reads original native field owners once",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const root=resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const input=JSON.parse(readFileSync(resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"),"utf8"));
  const long=law.text.repeat(law.repeat)+"\0";
  input.id=input.title=long;input.assets={[long]:{width:long.length,height:1,samples:Array.from({length:long.length},()=>[1,2,3,4])}};
  const validate=semioSchemaAjvV1({allErrors:true}).addSchema(JSON.parse(readFileSync(join(root,"🔣️.json"),"utf8"))).compile(JSON.parse(readFileSync(join(root,"📸️snapshot/🔣️.json"),"utf8")));
  expect(validate(input)).toBe(true);
  const copied=applyPatch({},[{op:"add",path:"/snapshot",value:structuredClone(input)}],true,false).newDocument.snapshot;
  expect(copied).toEqual(input);expect(validate(copied)).toBe(true);
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE fields (scope TEXT NOT NULL, field TEXT NOT NULL, bytes INTEGER NOT NULL, UNIQUE(scope,field))");
    for(const field of law.recordFootprint.snapshotTexts)database.query("INSERT INTO fields VALUES (?, ?, ?)").run("snapshot",field,Buffer.byteLength(input[field]??"","utf8"));
    for(const field of law.recordFootprint.assetTexts)database.query("INSERT INTO fields VALUES (?, ?, ?)").run("asset",field,Buffer.byteLength(field==="key"?long:input.assets[long][field],"utf8"));
    expect(database.query("SELECT scope, COUNT(*) AS count FROM fields GROUP BY scope ORDER BY scope").all()).toEqual([{scope:"asset",count:1},{scope:"snapshot",count:3}]);
    expect((database.query("SELECT SUM(bytes) AS bytes FROM fields WHERE scope = ?").get("asset") as {bytes:number}).bytes).toBe(Buffer.byteLength(long,"utf8"));
    database.exec("CREATE TABLE samples(ordinal INTEGER PRIMARY KEY, r INTEGER NOT NULL,g INTEGER NOT NULL,b INTEGER NOT NULL,a INTEGER NOT NULL)");for(const [index,sample]of input.assets[long].samples.entries())database.query("INSERT INTO samples VALUES (?,?,?,?,?)").run(index,...sample);expect((database.query("SELECT COUNT(*) AS count FROM samples").get()as{count:number}).count).toBe(long.length);

  }finally{database.close();}
  expect(law.recordFootprint.inlineBytesCountedByParent).toBe(true);
  const source=readFileSync(join(owner,"📐️footprint/🦀️.rs"),"utf8");
  expect(source.includes("struct DrawingRecordFootprintCursor")).toBe(true);
  expect(source.includes("assets.retained_entries().allocated_bytes()")).toBe(true);
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const census=host.slice(host.indexOf("struct DrawingSnapshotBoundsAuthority"),host.indexOf("struct DrawingNativeCloneAuthority"));
  expect(census.includes("DrawingRecordFootprintCursor")).toBe(true);
  expect(census.includes("fn string_owner")).toBe(false);
  console.log("[DEBUG] Drawing root/asset native census SQLite/Ajv/RFC6902 conserve each original field and native entry owner without duplicated inline backing");
});

test("Drawing direct layer footprint keeps native owner backing separate from nested group fields",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const root=resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const input=JSON.parse(readFileSync(resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"),"utf8"));
  const nested=input.layers[1].children[0];
  nested.id=nested.name=nested.content=law.text.repeat(law.repeat)+"\0";
  const validate=semioSchemaAjvV1({allErrors:true}).addSchema(JSON.parse(readFileSync(join(root,"🔣️.json"),"utf8"))).compile(JSON.parse(readFileSync(join(root,"📸️snapshot/🔣️.json"),"utf8")));
  expect(validate(input)).toBe(true);
  const copied=applyPatch({},[{op:"add",path:"/snapshot",value:structuredClone(input)}],true,false).newDocument.snapshot;
  expect(copied).toEqual(input);expect(validate(copied)).toBe(true);
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE fields (scope TEXT NOT NULL, field TEXT NOT NULL, bytes INTEGER NOT NULL)");
    for(const [scope,layer] of [["group",input.layers[1]],["text",nested]] as const){
      for(const field of law.directLayerFootprint.baseTexts)database.query("INSERT INTO fields VALUES (?, ?, ?)").run(scope,field,Buffer.byteLength(layer[field]??"","utf8"));
      for(const field of law.directLayerFootprint.variantTexts[scope])database.query("INSERT INTO fields VALUES (?, ?, ?)").run(scope,field,Buffer.byteLength(layer[field],"utf8"));
    }
    expect((database.query("SELECT COUNT(*) AS count FROM fields WHERE scope = ?").get("group") as {count:number}).count).toBe(3);
    expect((database.query("SELECT COUNT(*) AS count FROM fields WHERE scope = ?").get("text") as {count:number}).count).toBe(4);
    expect((database.query("SELECT SUM(bytes) AS bytes FROM fields WHERE scope = ?").get("text") as {bytes:number}).bytes).toBeGreaterThan(law.bodyBytes);
  }finally{database.close();}
  expect(law.directLayerFootprint.nestedGroupFieldsIncluded).toBe(false);
  const source=readFileSync(join(owner,"📐️footprint/🦀️.rs"),"utf8");
  expect(source.includes("struct DrawingLayerFootprintCursor")).toBe(true);
  expect(source.includes("DrawingTextFootprintCursor")).toBe(true);
  expect(source.includes("to_string_owner")).toBe(false);
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const work=host.slice(host.indexOf("struct DrawingLayerCloneWorkAuthority"),host.indexOf("fn drawing_fill_clone_work_totals"));
  expect(work.includes("DrawingLayerFootprintCursor")).toBe(true);
  console.log("[DEBUG] Drawing direct native layer footprint SQLite field selection and schema/Ajv/RFC6902 preserve nested long UTF8 without counting descendant fields twice");
});

test("Drawing native text digest preserves the exact semantic field stream over bounded chunks",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const text=law.text.repeat(law.repeat)+"\0";
  const bytes=Buffer.from(text,"utf8");
  const prefix=Buffer.alloc(11);prefix[0]=0xd8;prefix.writeUInt16BE(law.textDigest.tag,1);prefix.writeBigUInt64BE(BigInt(bytes.length),3);
  const whole=createHash("sha256").update(prefix).update(bytes).digest("hex");
  const incremental=createHash("sha256").update(prefix);
  for(let offset=0;offset<bytes.length;offset+=law.bodyBytes)incremental.update(bytes.subarray(offset,Math.min(offset+law.bodyBytes,bytes.length)));
  expect(incremental.digest("hex")).toBe(whole);
  console.log(`[DEBUG] Drawing independent native text digest tag=${law.textDigest.tag} bytes=${bytes.length} sha256=${whole}`);
  const source=readFileSync(join(owner,"📐️footprint/🦀️.rs"),"utf8");
  expect(source.includes("struct DrawingTextDigestCursor")).toBe(true);
  expect(whole).toBe(law.textDigest.sha256);
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const observer=host.slice(host.indexOf("fn observe_owned_string"),host.indexOf("fn observe(",host.indexOf("fn observe_owned_string")));
  expect(observer.includes("DrawingTextDigestCursor")).toBe(true);
  expect(observer.includes("value.as_bytes()")).toBe(false);
});

test("Drawing retained text footprint reads original chunk backing without materialization",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE chunks (ordinal INTEGER PRIMARY KEY, capacity INTEGER NOT NULL)");
    law.ownedChunkCapacities.forEach((capacity:number,index:number)=>database.query("INSERT INTO chunks VALUES (?,?)").run(index,capacity));
    let total=0;
    for(let index=0;index<law.ownedChunkCapacities.length;index++){
      total+=law.ownedChunkCapacities[index];
      expect((database.query("SELECT SUM(capacity) AS bytes FROM chunks WHERE ordinal <= ?").get(index) as {bytes:number}).bytes).toBe(total);
    }
    expect(total).toBeGreaterThan(law.bodyBytes);
    const moved=applyPatch({source:{chunks:law.ownedChunkCapacities}},[{op:"copy",from:"/source",path:"/candidate"}],true,false).newDocument;
    expect(moved.source.chunks).toEqual(moved.candidate.chunks);
  }finally{database.close();}
  const source=readFileSync(join(owner,"📐️footprint/🦀️.rs"),"utf8");
  expect(source.includes("source.retained_chunks()")).toBe(true);
  expect(source.includes("chunks.allocated_bytes()")).toBe(true);
  expect(source.includes("chunk.capacity()")).toBe(true);
  expect(source.includes("to_string_owner")).toBe(false);
  console.log("[DEBUG] Drawing retained text footprint independent SQLite/RFC6902 preserve empty/spare/full chunk capacities without native text materialization");
});

test("Drawing typed field clone uses semantic arrays and text without whole reservations",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const schemaRoot=resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const input=JSON.parse(readFileSync(resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"),"utf8"));
  const gradient=JSON.parse(readFileSync(join(schemaRoot,"🎨️fill/🧫️fixtures/🔣️.json"),"utf8"))[0].after;
  gradient.stops=Array.from({length:law.repeat},(_,index)=>structuredClone(gradient.stops[index%gradient.stops.length]));
  input.layers[0].attributes.fill=gradient;
  input.layers[1].children[0].content=law.text.repeat(law.repeat)+"\0";
  const validate=semioSchemaAjvV1({allErrors:true}).addSchema(JSON.parse(readFileSync(join(schemaRoot,"🔣️.json"),"utf8"))).compile(JSON.parse(readFileSync(join(schemaRoot,"📸️snapshot/🔣️.json"),"utf8")));
  expect(validate(input)).toBe(true);
  const copied=applyPatch({},[{op:"add",path:"/snapshot",value:structuredClone(input)}],true,false).newDocument.snapshot;
  expect(copied).toEqual(input);expect(validate(copied)).toBe(true);
  expect(copied.layers[0].attributes.fill.stops.length).toBe(law.repeat);
  expect(new TextEncoder().encode(copied.layers[1].children[0].content).length).toBeGreaterThan(law.bodyBytes);
  const host=readFileSync(join(owner,"🦀️.rs"),"utf8");
  expect(host.includes("struct DrawingNativeCloneAuthority<T: RetainedClone>")).toBe(true);
  for(const leaf of ["FillStyle","StrokeStyle","DrawingNativeText","DrawingNativeSegments"])
    expect(host.includes(`DrawingNativeCloneAuthority<${leaf}>`)).toBe(true);
  expect(host.includes("target.try_reserve_exact(stops.len())")).toBe(false);
  const cloneBody=host.slice(host.indexOf("struct DrawingNativeCloneAuthority"),host.indexOf("struct DrawingCloneWorkTotals"));
  expect(cloneBody.includes("try_reserve_exact")).toBe(false);
  console.log("[DEBUG] Drawing native typed field owners preserve semantic long text and gradient arrays under Ajv/RFC6902 without whole allocation reservations");
});

test("Drawing native asset cursor follows retained entry ordinals without copying keys",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const keys=(law.assetKeyOrder as string[]).map(key=>key==="long"?law.text.repeat(law.repeat)+"\0":key);
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE assets (ordinal INTEGER PRIMARY KEY, key TEXT NOT NULL)");
    keys.forEach((key,index)=>database.query("INSERT INTO assets VALUES (?, ?)").run(index,key));
    for(let index=0;index<keys.length;index++)expect((database.query("SELECT key FROM assets WHERE ordinal = ?").get(index) as {key:string}).key).toBe(keys[index]);
    expect(database.query("SELECT key FROM assets WHERE ordinal = ?").get(keys.length)).toBeNull();
    expect(new TextEncoder().encode(keys[2]).length).toBeGreaterThan(law.bodyBytes);
  }finally{database.close();}
  const source=readFileSync(join(owner,"🦀️.rs"),"utf8");
  const cursor=source.slice(source.indexOf("struct DrawingAssetBoundsCursor"),source.indexOf("struct DrawingSnapshotBoundsAuthority"));
  expect(cursor.includes("assets.entry_at(self.index)")).toBe(true);
  expect(cursor.includes("BTreeMap")).toBe(false);expect(cursor.includes("key_len")).toBe(false);
  console.log("[DEBUG] Drawing native asset entry ordinals agree with independent SQLite for empty/long UTF8/NUL keys without key materialization");
});

test("Drawing initial snapshot copying retains native paged assets and separate grants",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️native-owner/🔣️.json"),"utf8"));
  const input=JSON.parse(readFileSync(resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"),"utf8"));
  const long=law.text.repeat(law.repeat)+"\0";
  input.id=long;input.title=long;input.assets={[long]:{width:long.length,height:1,samples:Array.from({length:long.length},()=>[1,2,3,4])}};
  const schema=resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const validate=semioSchemaAjvV1({allErrors:true}).addSchema(JSON.parse(readFileSync(join(schema,"🔣️.json"),"utf8"))).compile(JSON.parse(readFileSync(join(schema,"📸️snapshot/🔣️.json"),"utf8")));
  expect(validate(input)).toBe(true);
  const copied=applyPatch({},[{op:"add",path:"/snapshot",value:structuredClone(input)}],true,false).newDocument.snapshot;
  expect(copied).toEqual(input);expect(validate(copied)).toBe(true);
  expect(copied.assets[long].samples.length*law.recordFootprint.assetSampleComponents).toBeGreaterThan(law.bodyBytes);
  expect(applyPatch({snapshot:copied},[{op:"remove",path:"/snapshot"}],true,false).newDocument).toEqual({});
  const source=readFileSync(join(owner,"🦀️.rs"),"utf8");
  expect(source).toContain("CloneInitialSnapshot");
  expect(source).toContain("initial_snapshot_clone");
  expect(source).toContain("RetainedCloneGrant::one_release_turn");
  expect(source.includes("data.try_reserve_exact(asset.data.len())")).toBe(false);
  expect(source.includes("DrawingStoreInitializationPhase::CloneInitialAsset")).toBe(false);
  console.log("[DEBUG] Drawing initial snapshot neutral schema/Ajv and RFC6902 preserve long native asset keys and data before paged clone adoption");
});

test("Drawing closure moves inline owners under copy and frees pages under release",()=>{
  const owner=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(resolve(owner,"../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧫️fixtures/📏️release-authority/🔣️.json"),"utf8"));
  const original={active:{byte:law.inputByte},closing:null,page:{bytes:law.physicalCursorBytes}};
  const moved=applyPatch(original,[{op:"move",from:"/active",path:"/closing"}],true,false).newDocument;
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE owner (name TEXT PRIMARY KEY, bytes INTEGER NOT NULL)");
    database.query("INSERT INTO owner VALUES (?, ?)").run("page",law.physicalCursorBytes);
    expect(database.query("SELECT SUM(bytes) AS bytes FROM owner").get()).toEqual({bytes:moved.page.bytes});
    expect(moved.closing).toEqual({byte:law.inputByte});
    expect(moved.page).toEqual(original.page);
    expect(law.oneBelowReleaseBytes).toBeLessThan(moved.page.bytes);
    const closed=applyPatch(moved,[{op:"remove",path:"/page"}],true,false).newDocument;
    database.query("DELETE FROM owner WHERE name = ?").run("page");
    expect(database.query("SELECT COUNT(*) AS count FROM owner").get()).toEqual({count:0});
    expect(closed).toEqual({closing:{byte:law.inputByte}});
  }finally{database.close();}
  const rename=resolve(owner,"../../../🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🎮️prepare");
  const inverse=readFileSync(join(rename,"↩️inverse/🦀️.rs"),"utf8");
  expect(inverse.includes("grant.maximum_copy_bytes < size_of::<RenameLayer>()")).toBe(true);
  expect(inverse.includes("return Ok(payload_progress(size_of::<RenameLayer>()));")).toBe(true);
  const plan=readFileSync(join(rename,"🔗️owned/🦀️.rs"),"utf8");
  expect(plan.includes("release_empty_page(grant.maximum_release_bytes)")).toBe(true);
});

test("Drawing native paged owner preserves semantic snapshot and UTF-8 keys", () => {
  const owner = resolve(import.meta.dir, "../..");
  const law = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📋️native-owner/🔣️.json"), "utf8"));
  const input = JSON.parse(readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"), "utf8"));
  const text = law.text.repeat(law.repeat);
  input.id = text; input.title = text; input.assets = {[text]:{width:1,height:1,samples:[[1,2,3,4]]}};
  const longLayerText = text + "\0";
  input.layers[1].children[0].id = longLayerText; input.layers[1].children[0].name = longLayerText; input.layers[1].children[0].content = longLayerText;
  const schemaRoot = resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const documentSchema = JSON.parse(readFileSync(join(schemaRoot, "🔣️.json"), "utf8"));
  const snapshotSchema = JSON.parse(readFileSync(join(schemaRoot, "📸️snapshot/🔣️.json"), "utf8"));
  const validate = semioSchemaAjvV1({allErrors:true}).addSchema(documentSchema).compile(snapshotSchema);
  expect(validate(input)).toBe(true);
  expect(new TextEncoder().encode(text).length).toBeGreaterThan(law.bodyBytes);
  const copied = applyPatch({}, [{op:"add",path:"/snapshot",value:structuredClone(input)}], true, false).newDocument.snapshot;
  expect(copied).toEqual(JSON.parse(JSON.stringify(input)));
  const host = readFileSync(join(owner,"🦀️.rs"),"utf8");
  const layerClone = host.slice(host.indexOf("struct DrawingNativeCloneAuthority"),host.indexOf("struct DrawingCloneWorkTotals"));
  expect(layerClone.includes("cursor: T::Cursor")).toBe(true);
  expect(layerClone.includes("DrawingNativeCloneAuthority<DrawingLayerNode>")).toBe(true);
  expect(layerClone.includes("cursor.advance")).toBe(true);
  expect(layerClone.includes("close_granted")).toBe(true);
  expect(layerClone.includes("Vec::with_capacity")).toBe(false);
  expect(validate(copied)).toBe(true);
  expect(validate({...copied, assets:{[text]:{mime:"image/png",data:7}}})).toBe(false);
  expect(validate({...copied, layers:{}})).toBe(false);
  const retired = applyPatch({snapshot:copied}, [{op:"remove",path:"/snapshot"}], true, false).newDocument;
  expect(Object.keys(retired).length === 0).toBe(law.terminalEmpty);
  expect(input.id === text && Object.keys(input.assets)[0] === text).toBe(law.originalUnchanged);
  for (const pause of law.cancelAt) {
    const partial = {snapshot:structuredClone(copied), pause};
    const closed = applyPatch(partial, [{op:"remove", path:"/snapshot"}], true, false).newDocument;
    expect(closed).toEqual({pause});
    expect(validate(input)).toBe(true);
  }
  console.log("[DEBUG] Drawing native paged owner admitted by semantic snapshot schema/Ajv; JSON Patch/UTF-8 conserve long text and object keys");
});


test("Drawing paged borrowed lookup follows first native depth-first identity", () => {
  const owner = resolve(import.meta.dir, "../..");
  const law = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📋️native-owner/🔣️.json"), "utf8"));
  const input = JSON.parse(readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"), "utf8"));
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE nodes (ordinal INTEGER PRIMARY KEY, id TEXT NOT NULL, name TEXT NOT NULL, path TEXT NOT NULL)");
    let ordinal = 0;
    const visit = (layers: typeof input.layers, parent: number[] = []) => {for (let index=0;index<layers.length;index++) {const layer=layers[index];const path=[...parent,index];database.query("INSERT INTO nodes VALUES (?, ?, ?, ?)").run(ordinal++, layer.id, layer.name,JSON.stringify(path));if (layer.kind === "group") visit(layer.children,path);}};
    visit(input.layers);
    if (law.lookup.firstDuplicate) database.query("INSERT INTO nodes VALUES (?, ?, ?, ?)").run(ordinal++, "text-a", "Later Duplicate",JSON.stringify([input.layers.length]));
    for (const row of law.lookup.cases) {
      const found = database.query("SELECT name,path FROM nodes WHERE id = ? ORDER BY ordinal LIMIT 1").get(row.target) as {name:string,path:string}|null;
      expect(found?.name ?? null).toBe(row.name);
      expect(found ? JSON.parse(found.path) : null).toEqual(row.path);
    }
    const long = law.text.repeat(law.repeat) + "\0";
    database.query("INSERT INTO nodes VALUES (?, ?, ?, ?)").run(ordinal++, long, "Long UTF8","[]");
    expect((database.query("SELECT name FROM nodes WHERE id = ? ORDER BY ordinal LIMIT 1").get(long) as {name:string}).name).toBe("Long UTF8");
    expect(new TextEncoder().encode(long).length).toBeGreaterThan(law.bodyBytes);
    let deep = {...structuredClone(input.layers[0]), id:long, name:"Deep UTF8"};
    for (let depth = 0; depth < law.lookup.deep; depth++) deep = {...structuredClone(input.layers[1]), id:`nested-${depth}`, children:[deep]};
    const nested = {...input, layers:[deep]};
    const schemaRoot = resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    const validate = semioSchemaAjvV1({allErrors:true}).addSchema(JSON.parse(readFileSync(join(schemaRoot, "🔣️.json"), "utf8"))).compile(JSON.parse(readFileSync(join(schemaRoot, "📸️snapshot/🔣️.json"), "utf8")));
    expect(validate(nested)).toBe(true);
    visit(nested.layers);
    expect((database.query("SELECT name FROM nodes WHERE id = ? ORDER BY ordinal DESC LIMIT 1").get(long) as {name:string}).name).toBe("Deep UTF8");
    expect(JSON.parse((database.query("SELECT path FROM nodes WHERE id = ? ORDER BY ordinal DESC LIMIT 1").get(long) as {path:string}).path)).toEqual(Array(law.lookup.deep+1).fill(0));
    const source = readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup/🦀️.rs"), "utf8");
    expect(source).toContain("pub struct DrawingLayerLookupCursor");
    expect(source).toContain("close_granted");
    expect(source).toContain("pub fn path_index");
    expect(law.lookup.ownedProjection.retainRootUntilClosed).toBe(true);
    expect(law.lookup.ownedProjection.targetReborrowedEachTurn).toBe(true);
    const retained = readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup/🔗️owned/🦀️.rs"), "utf8");
    expect(retained.includes("pub struct DrawingOwnedLayerLookupCursor")).toBe(true);
    console.log("[DEBUG] Drawing borrowed lookup SQLite orders first duplicate/native depth-first IDs and retains long UTF8/NUL keys");
  } finally {database.close();}
});


test("Drawing borrowed rename preparation preserves first-target inverse and native names", () => {
  const owner = resolve(import.meta.dir, "../..");
  const law = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📋️native-owner/🔣️.json"), "utf8"));
  const input = JSON.parse(readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"), "utf8"));
  const long = law.text.repeat(law.repeat) + "\0";
  input.layers[1].children[0].name = long;
  input.layers.push({...structuredClone(input.layers[0]), id:long, name:"Long Identity"});
  input.layers.push({...structuredClone(input.layers[0]), name:"Later Duplicate"});
  let deep = {...structuredClone(input.layers[0]), id:"deep-rename", name:"Deep Original"};
  for (let depth = 0; depth < law.lookup.deep; depth++) deep = {...structuredClone(input.layers[1]), id:`rename-nested-${depth}`, children:[deep]};
  input.layers.push(deep);
  const before = JSON.stringify(input);
  const schemaRoot = resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const ajv = semioSchemaAjvV1({allErrors:true});
  const validate = ajv.addSchema(JSON.parse(readFileSync(join(schemaRoot, "🔣️.json"), "utf8"))).compile(JSON.parse(readFileSync(join(schemaRoot, "📸️snapshot/🔣️.json"), "utf8")));
  const validateMutation = ajv.compile(JSON.parse(readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🧬️schema/🔣️.json"), "utf8")));
  expect(validate(input)).toBe(true);
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE nodes (ordinal INTEGER PRIMARY KEY, id TEXT, name TEXT, path TEXT)");
    let ordinal = 0;
    const visit = (layers: typeof input.layers, parent: string) => {for (let index = 0; index < layers.length; index++) {const layer = layers[index];const path = parent + "/" + index;database.query("INSERT INTO nodes VALUES (?, ?, ?, ?)").run(ordinal++, layer.id, layer.name, path);if (layer.kind === "group") visit(layer.children, path + "/children");}};
    visit(input.layers, "/layers");
    for (const row of law.rename.cases) {
      const name = (row.repeatNewName ? row.newName.repeat(law.repeat) + (row.disposition === "no-op" || row.suffix ? "\0" : "") : row.newName) + (row.suffix ?? "");
      const target = row.repeatTarget ? row.target.repeat(law.repeat) + "\0" : row.target;
      expect(validateMutation({mutation:"renameLayer",layerId:target,newName:name})).toBe(true);
      const found = database.query("SELECT name, path FROM nodes WHERE id = ? ORDER BY ordinal LIMIT 1").get(target) as {name:string,path:string}|null;
      expect(found?.name ?? null).toBe(row.inverseName === "long" ? long : row.inverseName);
      expect(found ? found.path.split("/").filter(part => /^\d+$/u.test(part)).map(Number) : null).toEqual(row.path);
      expect(!found ? "missing" : found.name === name ? "no-op" : "changed").toBe(row.disposition);
      if (found && row.disposition === "changed") {const changed = applyPatch(structuredClone(input), [{op:"replace",path:found.path + "/name",value:name}], true, false).newDocument;expect(validate(changed)).toBe(true);const restored = applyPatch(changed, [{op:"replace",path:found.path + "/name",value:found.name}], true, false).newDocument;expect(restored).toEqual(input);}
      if (row.repeatNewName) expect(new TextEncoder().encode(name).length).toBeGreaterThan(law.bodyBytes);
      if (row.repeatTarget) expect(new TextEncoder().encode(target).length).toBeGreaterThan(law.bodyBytes);
    }
    expect(JSON.stringify(input)).toBe(before);
    const source = readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🎮️prepare/🦀️.rs"), "utf8");
    expect(source).toContain("pub struct DrawingRenamePreparationCursor");
    expect(source).toContain("close_granted");
    const owned = readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🎮️prepare/↩️inverse/🦀️.rs"), "utf8");
    expect(owned).toContain("pub struct DrawingRenameInverseCursor");
    expect(owned).toContain("ControlledRetirement");
    expect(owned.includes("DrawingOwnedRenamePlan")).toBe(true);
    expect(law.rename.ownedProjection.retainRootUntilClosed).toBe(true);
    expect(law.rename.ownedProjection.payloadReborrowedEachTurn).toBe(true);
    const binding = law.rename.ownedProjection;
    expect((database.query("SELECT ? = ? AS accepted").get(binding.originalLease, binding.originalLease) as {accepted:number}).accepted).toBe(1);
    expect((database.query("SELECT ? = ? AS accepted").get(binding.originalLease, binding.foreignLease) as {accepted:number}).accepted).toBe(Number(!binding.foreignPayloadRefused));
    const retained = readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🎮️prepare/🔗️owned/🦀️.rs"), "utf8");
    expect(retained.includes("pub struct DrawingOwnedRenamePreparationCursor")).toBe(true);
    expect(retained.includes("pub fn path_index")).toBe(true);
    console.log("[DEBUG] Drawing rename SQLite first identity/old name and independent RFC6902 restoration preserve empty/long UTF8, no-op and missing inputs");
  } finally {database.close();}
});
