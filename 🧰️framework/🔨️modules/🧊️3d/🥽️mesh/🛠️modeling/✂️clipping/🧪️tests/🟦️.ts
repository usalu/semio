/** 🧪️ Clipping: 13 tests covering retained clipping edge vectors agree with schema Three and SQLite; real mesh packages consume an acyclic canonical contract and retained clipping owner; actual extracted Rust Source parses and each n… */
import "../../../🧪️tests/📦️targets/🧪️packaging/🟦️.ts";
import Parser from"web-tree-sitter";
import{readModuleTreeSource}from"@semio-tech/framework-compiler-ts/syntax/module-tree";
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{Line3,Plane,Vector3,BufferGeometry,Float32BufferAttribute}from"three";
import{mergeVertices}from"three/examples/jsm/utils/BufferGeometryUtils.js";
import intentSchema from"../../🔁️reconstruction/🧬️schema/🎛️intents.json";
import intentRows from"../../🔁️reconstruction/🧪️tests/🎛️intents.json";
import reconstructionSchema from"../../🔁️reconstruction/🧬️schema/🔣️.json";
import weldRows from"../../🔁️reconstruction/🧪️tests/🔣️.json";
import{meshWeldKeyV1}from"../../🔁️reconstruction/🟦️.ts";
import documentSchema from"../../../📄️document/🧬️schema/🔣️.json";
import documents from"../../../📄️document/🧪️tests/🔣️.json";
import{meshTriangleTopologyV1}from"../../../📄️document/🟦️.ts";
import{readFileSync}from"node:fs";
import{resolve,dirname}from"node:path";
import callers from"../../../📄️document/🧪️tests/📥️callers.json";
import consumers from"../../../📄️document/🧪️tests/📚️consumers.json";
import{parse}from"smol-toml";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
import{meshClipEdgeV1}from"../🟦️.ts";
test("retained clipping edge vectors agree with schema Three and SQLite",()=>{
 const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 const tuple=(value:readonly number[]):readonly[number,number,number]=>{const[a,b,c]=value;if(value.length!==3||a===undefined||b===undefined||c===undefined)throw Error("Three coordinates required");return[a,b,c];};
 try{for(const row of corpus){expect(validate(row)).toBe(true);const actual=meshClipEdgeV1({...row,a:tuple(row.a),b:tuple(row.b),normal:tuple(row.normal)});expect(JSON.stringify(actual)).toBe(JSON.stringify(row.expected));
 const plane=new Plane(new Vector3(...tuple(row.normal)),-row.distance),line=new Line3(new Vector3(...tuple(row.a)),new Vector3(...tuple(row.b))),intersection=plane.intersectLine(line,new Vector3());
 const startDistance=plane.distanceToPoint(line.start),endDistance=plane.distanceToPoint(line.end),crossed=startDistance>row.epsilon&&endDistance< -row.epsilon||startDistance< -row.epsilon&&endDistance>row.epsilon;expect(Boolean(actual.position)).toBe(crossed);
 if(actual.position){expect(intersection?.toArray()).toEqual(actual.position);}
 const sql=db.query(`select (?1*?7+?2*?8+?3*?9-?10)<=?11 as retained`).get(...row.a,...row.b,...row.normal,row.distance,row.epsilon);expect(sql).toEqual({retained:Number(actual.retained)});
 }expect(validate({...corpus[0],epsilon:-1})).toBe(false);
 }finally{db.close();}
});
test("real mesh packages consume an acyclic canonical contract and retained clipping owner",()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),mesh=resolve(root,"🧰️framework/🔨️modules/🧊️3d/🥽️mesh"),read=(path:string)=>path.endsWith(".rs")?readModuleTreeSource(path):readFileSync(path,"utf8");
 const model=read(resolve(mesh,"🛠️modeling/🦀️.rs"));expect(model.includes("struct Clip {")).toBe(false);
 const facade=parse(read(resolve(mesh,"../📦️packages/🦀️rust/Cargo.toml"))),clip=parse(read(resolve(mesh,"🛠️modeling/✂️clipping/📦️packages/🦀️rust/Cargo.toml"))),contract=parse(read(resolve(mesh,"🧬️contract/📦️packages/🦀️rust/Cargo.toml")));
 const document=read(resolve(mesh,"🦀️.rs")),algorithm=read(resolve(mesh,"🛠️modeling/✂️clipping/🦀️.rs"));
 expect(model.includes("struct Clip {")).toBe(false);expect(model.includes("MeshClipJob::new(")).toBe(true);expect(document).not.toContain("pub struct Vec3(");expect(read(resolve(mesh,"📄️document/🦀️.rs"))).toContain("semio_framework_3d_mesh_contract");expect(document).toContain("semio_framework_3d_mesh_document");expect(model.includes("take_source_faces()")).toBe(true);expect(algorithm).toContain("mesh_clip_edge(");
 for(const value of [facade,clip,contract])expect(value.package).toBeDefined();
 const dependencies=(value:unknown)=>{if(!value||typeof value!=="object"||!("dependencies"in value)||!value.dependencies||typeof value.dependencies!=="object")throw Error("Actual dependency table required");return Object.keys(value.dependencies);};
 expect(dependencies(facade)).toContain("semio-framework-3d-mesh-clipping");expect(dependencies(facade)).toContain("semio-framework-3d-mesh-contract");expect(dependencies(clip)).toContain("semio-framework-3d-mesh-contract");expect(dependencies(contract)).not.toContain("semio-framework-3d");expect(dependencies(clip)).not.toContain("semio-framework-3d");
 for(const text of [algorithm,read(resolve(mesh,"🧬️contract/🦀️.rs"))]){expect(Buffer.byteLength(text)).toBeLessThanOrEqual(262144);expect(text.split("\n").filter(line=>line.trim()).length).toBeLessThanOrEqual(4000);}
});

test("actual extracted Rust Source parses and each new compilation root stays finite",async()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),mesh=resolve(root,"🧰️framework/🔨️modules/🧊️3d/🥽️mesh");
 await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const parser=new Parser();
 try{parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));
 for(const files of [["🧬️contract/🦀️.rs"],["🛠️modeling/✂️clipping/🦀️.rs","🛠️modeling/✂️clipping/🧪️tests/🦀️.rs"]]){
  let bytes=0,lines=0;for(const file of files){const source=readFileSync(resolve(mesh,file),"utf8"),tree=parser.parse(source);try{expect(tree.rootNode.hasError()).toBe(false);}finally{tree.delete();}bytes+=Buffer.byteLength(source);lines+=source.split("\n").filter(line=>line.trim()).length;}
  expect(bytes).toBeLessThanOrEqual(262144);expect(lines).toBeLessThanOrEqual(4000);expect(files.length).toBeLessThanOrEqual(32);
 }
 for(const file of ["🦀️.rs","🛠️modeling/🦀️.rs"]){const tree=parser.parse(readFileSync(resolve(mesh,file),"utf8"));try{expect(tree.rootNode.hasError()).toBe(false);}finally{tree.delete();}}
 const model=readFileSync(resolve(mesh,"🛠️modeling/🦀️.rs"),"utf8"),algorithm=readFileSync(resolve(mesh,"🛠️modeling/✂️clipping/🦀️.rs"),"utf8");
 expect(model).not.toContain("RetireOwned for Clip");expect(model).not.toContain("RetireOwned for Soup");expect(model).not.toContain("artifact_retire_leaf!(SourceVertex,Ranked)");
 expect(algorithm).toContain("if !self.completed||self.transferred&bit!=0");expect(algorithm).toContain("self.transferred|=bit");expect(algorithm).toContain("self.completed=true");
 for(const field of ["intersections","cap","cap_ids","clipped","corners","sorted","vertex_sources","source_faces","corner_faces","face_weights","cap_owners","cap_weights"])expect(algorithm).toContain("deferred(self."+field+")");
 }finally{parser.delete();}
});

test("real document owner defines topology and conversion below modeling extension",()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),mesh=resolve(root,"🧰️framework/🔨️modules/🧊️3d/🥽️mesh");
 const facade=readModuleTreeSource(resolve(mesh,"🦀️.rs")),model=readModuleTreeSource(resolve(mesh,"🛠️modeling/🦀️.rs"));
 expect(facade.includes("pub struct HalfedgeMesh")).toBe(false);
 const document=readModuleTreeSource(resolve(mesh,"📄️document/🦀️.rs"));
 expect(document).toContain("pub struct HalfedgeMesh");expect(document.includes("mod modeling;")).toBe(false);
 expect(model.includes("impl MeshModelingOps for HalfedgeMesh")).toBe(true);expect(model.includes("impl HalfedgeMesh {")).toBe(false);
 expect(document.includes("pub fn box_prim(")).toBe(false);expect(model.includes("fn box_prim(")).toBe(true);
});

test("document topology corpus agrees with Three and independent SQLite edge incidence",()=>{
 expect(new Ajv({strict:true}).validate(documentSchema,documents)).toBe(true);const db=new Database(":memory:");
 try{db.exec("create table edges(a integer,b integer)");for(const row of documents){const actual=meshTriangleTopologyV1(row.positions,row.indices);expect(JSON.stringify(actual)).toBe(JSON.stringify(row.expected));
 const geometry=new BufferGeometry();try{geometry.setAttribute("position",new Float32BufferAttribute(row.positions.flat(),3));geometry.setIndex(row.indices);geometry.computeVertexNormals();const normal=geometry.getAttribute("normal");expect([normal.getX(0),normal.getY(0),normal.getZ(0)]).toEqual(row.expected.normal);expect(geometry.getAttribute("position").count).toBe(actual.vertices);}finally{geometry.dispose();}
 db.exec("delete from edges");for(let offset=0;offset<row.indices.length;offset+=3)for(let corner=0;corner<3;corner++){const a=row.indices[offset+corner],b=row.indices[offset+(corner+1)%3];if(a===undefined||b===undefined)throw Error("Complete corpus triangle required");db.query("insert into edges values (?,?)").run(a,b);}
 expect(db.query("select count(*) as count from (select min(a,b),max(a,b) from edges group by min(a,b),max(a,b))").get()).toEqual({count:actual.edges});
 }expect(()=>meshTriangleTopologyV1([[0,0,0]], [0,1,2])).toThrow();expect(()=>meshTriangleTopologyV1([[0,0,0]], [0,0])).toThrow();
 }finally{db.close();}
});

test("all cfg-possible document mounts and actual extension callers parse below finite document bounds",async()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),mesh=resolve(root,"🧰️framework/🔨️modules/🧊️3d/🥽️mesh"),seen=new Set<string>();
 const visit=(path:string)=>{if(seen.has(path))return;seen.add(path);const source=readFileSync(path,"utf8");for(const match of source.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)){const child=match[1];if(!child)throw Error("Literal Rust mount required");visit(resolve(dirname(path),child));}};
 visit(resolve(mesh,"📄️document/🦀️.rs"));let bytes=0,lines=0;
 await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const parser=new Parser();
 try{parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));
 for(const path of seen){const source=readFileSync(path,"utf8");bytes+=Buffer.byteLength(source);lines+=source.split("\n").filter(line=>line.trim()).length;const tree=parser.parse(source);try{expect(tree.rootNode.hasError()).toBe(false);}finally{tree.delete();}for(const match of source.matchAll(/include_str!\("([^"]+)"\)/g)){const included=match[1];if(!included)throw Error("Literal fixture path required");expect(readFileSync(resolve(dirname(path),included)).length).toBeGreaterThan(0);}}
 expect(bytes).toBeLessThanOrEqual(262144);expect(lines).toBeLessThanOrEqual(4000);expect(seen.size).toBeLessThanOrEqual(32);
 for(const relative of callers){const source=readFileSync(resolve(root,relative),"utf8"),tree=parser.parse(source);try{expect(tree.rootNode.hasError()).toBe(false);}finally{tree.delete();}expect(readModuleTreeSource(resolve(root,relative)).includes("MeshModelingOps")).toBe(true);const inner=source.lastIndexOf("#!["),imported=source.indexOf("use semio_framework_3d::mesh");if(inner>=0&&imported>=0)expect(imported).toBeGreaterThan(inner);}
 const document=readModuleTreeSource(resolve(mesh,"📄️document/🦀️.rs")),model=readModuleTreeSource(resolve(mesh,"🛠️modeling/🦀️.rs")),facade=readModuleTreeSource(resolve(mesh,"🦀️.rs"));
 expect(document.includes("semio_framework_3d::")).toBe(false);expect(document.includes("semio_framework_3d_mesh_clipping")).toBe(false);expect(facade.includes("#[path = \"🧩️tessellation")).toBe(false);
 expect(model.includes("trait MeshModelingInternals")).toBe(true);expect(model.includes("pub trait MeshModelingOps: Sized")).toBe(true);
 const manifest=parse(readFileSync(resolve(mesh,"📄️document/📦️packages/🦀️rust/Cargo.toml"),"utf8"));
 const dependencies=manifest.dependencies;if(!dependencies||typeof dependencies!=="object"||Array.isArray(dependencies))throw Error("Document dependency table required");
 expect(Object.keys(dependencies)).not.toContain("semio-framework-3d");expect(Object.keys(dependencies)).not.toContain("semio-framework-3d-mesh-clipping");
 for(const dependency of Object.values(dependencies)){if(dependency&&typeof dependency==="object"&&!Array.isArray(dependency)&&typeof dependency.path==="string")expect(readFileSync(resolve(mesh,"📄️document/📦️packages/🦀️rust",dependency.path,"Cargo.toml"),"utf8").length>0).toBe(true);}
 }finally{parser.delete();}
});

test("actual Rust extension declarations and implementations have identical contracts",async()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),mesh=resolve(root,"🧰️framework/🔨️modules/🧊️3d/🥽️mesh");
 await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const parser=new Parser();
 try{parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));
 const source=readFileSync(resolve(mesh,"🛠️modeling/🧰️modeling-operations/🦀️.rs"),"utf8"),tree=parser.parse(source);
 try{const trait=tree.rootNode.descendantsOfType("trait_item").find(node=>node.childForFieldName("name")?.text==="MeshModelingOps"),implementation=tree.rootNode.descendantsOfType("impl_item").find(node=>node.childForFieldName("trait")?.text==="MeshModelingOps");if(!trait||!implementation)throw Error("Actual trait and implementation required");
 const normalize=(text:string)=>text.replace(/(?<!&)\bmut (\w+)/g,"$1").replace(/\s+/g,"");
 const contract=(node:Parser.SyntaxNode)=>["name","type_parameters","parameters","return_type","where_clause"].map(field=>normalize(node.childForFieldName(field)?.text??"")).join("|");
 const declarations=trait.descendantsOfType("function_signature_item"),definitions=implementation.descendantsOfType("function_item");expect(declarations.length).toBe(93);expect(definitions.length).toBe(93);
 expect(declarations.map(contract).sort()).toEqual(definitions.map(contract).sort());
 const privateTrait=tree.rootNode.descendantsOfType("trait_item").find(node=>node.childForFieldName("name")?.text==="MeshModelingInternals"),privateImpl=tree.rootNode.descendantsOfType("impl_item").find(node=>node.childForFieldName("trait")?.text==="MeshModelingInternals");if(!privateTrait||!privateImpl)throw Error("Actual private helper contract required");const privateDeclarations=privateTrait.descendantsOfType("function_signature_item"),privateDefinitions=privateImpl.descendantsOfType("function_item");expect(privateDeclarations.length).toBe(7);expect(privateDefinitions.length).toBe(7);expect(privateDeclarations.map(contract).sort()).toEqual(privateDefinitions.map(contract).sort());
 for(const node of definitions)expect(node.namedChildren.some(child=>child.type==="visibility_modifier")).toBe(false);
 for(const node of declarations)expect(/\b(?:Work|Transform|PositionPolicy|SourceAttributeRemap|Affine3)\b/.test(node.text)).toBe(false);
 const inputTree=parser.parse(readFileSync(resolve(mesh,"🛠️modeling/📥️input/🦀️.rs"),"utf8"));try{expect(inputTree.rootNode.hasError()).toBe(false);const inputTrait=inputTree.rootNode.descendantsOfType("trait_item").find(node=>node.childForFieldName("name")?.text==="MeshTranslationSource"),inputImpl=inputTree.rootNode.descendantsOfType("impl_item").find(node=>node.childForFieldName("trait")?.text==="MeshTranslationSource");if(!inputTrait||!inputImpl)throw Error("Actual cold input extension required");expect(inputTrait.descendantsOfType("function_signature_item").map(contract)).toEqual(inputImpl.descendantsOfType("function_item").map(contract));}finally{inputTree.delete();}
 const methods=declarations.map(node=>node.childForFieldName("name")?.text);
 for(const relative of consumers){const text=readFileSync(resolve(root,relative),"utf8");const calls=methods.some(name=>name!==undefined&&new RegExp("(?:\\.|::)"+name+"\\s*\\(").test(text));if(calls&&text.includes("HalfedgeMesh"))expect(text.includes("MeshModelingOps")||text.includes("mesh::*")).toBe(true);}
 }finally{tree.delete();}
 }finally{parser.delete();}
});

test("actual retained reconstruction family is owned below the existing aggregate",()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),mesh=resolve(root,"🧰️framework/🔨️modules/🧊️3d/🥽️mesh"),model=readFileSync(resolve(mesh,"🛠️modeling/🦀️.rs"),"utf8");
 expect(model.includes("struct SourceAttributeRemap {")).toBe(false);expect(model.includes("struct Coplanar {")).toBe(false);expect(model.includes("struct Import {")).toBe(false);expect(model.includes("semio_framework_3d_mesh_reconstruction")).toBe(true);
});

test("consumed weld-grid vectors agree with Ajv SQLite and independent Three dedup",()=>{
 expect(new Ajv({strict:true}).validate(reconstructionSchema,weldRows)).toBe(true);const db=new Database(":memory:");
 try{for(const row of weldRows){if(row.expected===null){expect(()=>meshWeldKeyV1(row.point,row.precision)).toThrow();continue;}const actual=meshWeldKeyV1(row.point,row.precision);expect(JSON.stringify(actual)).toBe(JSON.stringify(row.expected));
 const scale=Math.fround(1/Math.max(Math.fround(row.precision),Math.fround(1e-9)));for(const[index,value]of row.point.entries()){const expected=row.expected[index];if(expected===undefined)throw Error("Three corpus keys required");expect(db.query("select printf('%lld',round(?)) as key").get(Math.fround(value)*scale)).toEqual({key:expected});}
 }const geometry=new BufferGeometry();try{geometry.setAttribute("position",new Float32BufferAttribute([0,0,0,1,0,0,0,0,0,1.00000001,0,0],3));const output=mergeVertices(geometry,0.001);try{expect(output.getAttribute("position").count).toBe(new Set([[0,0,0],[1,0,0],[0,0,0],[1.00000001,0,0]].map(point=>meshWeldKeyV1(point,0.001).join(":"))).size);}finally{output.dispose();}}finally{geometry.dispose();}
 expect(()=>meshWeldKeyV1([0,0,0],0)).toThrow();expect(()=>meshWeldKeyV1([0,0],1)).toThrow();
 }finally{db.close();}
});

test("actual reconstruction closure is bounded private and fully retired without facade return edge",async()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),mesh=resolve(root,"🧰️framework/🔨️modules/🧊️3d/🥽️mesh"),family=resolve(mesh,"🛠️modeling/🔁️reconstruction"),read=(path:string)=>readFileSync(path,"utf8");
 const closure=(entry:string,tests:boolean)=>{const seen=new Set<string>();const visit=(path:string)=>{if(seen.has(path))return;seen.add(path);for(const match of read(path).matchAll(/((?:#\[[^\n]+\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;/g)){const attrs=match[1]??"",child=/#\[path\s*=\s*"([^"]+)"\]/.exec(attrs)?.[1];if(child&&(tests||!attrs.includes("#[cfg(test)]")))visit(resolve(dirname(path),child));}};visit(entry);return seen;};
 await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const parser=new Parser();
 try{parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));
 const files=closure(resolve(family,"🦀️.rs"),true);let bytes=0,lines=0;for(const path of files){const text=read(path),tree=parser.parse(text);try{expect(tree.rootNode.hasError()).toBe(false);}finally{tree.delete();}bytes+=Buffer.byteLength(text);lines+=text.split("\n").filter(line=>line.trim()).length;for(const match of text.matchAll(/include_str!\("([^"]+)"\)/g)){const child=match[1];if(!child)throw Error("Fixture path required");expect(readFileSync(resolve(dirname(path),child)).length).toBeGreaterThan(0);}}
 expect(bytes).toBeLessThanOrEqual(262144);expect(lines).toBeLessThanOrEqual(4000);expect(files.size).toBeLessThanOrEqual(32);
 const parentFiles=closure(resolve(mesh,"../📦️packages/🦀️rust/🦀️.rs"),false),parentBytes=Array.from(parentFiles).reduce((n,path)=>n+Buffer.byteLength(read(path)),0);expect(parentBytes).toBeLessThanOrEqual(262144);expect(parentFiles.size).toBeLessThanOrEqual(32);expect(Array.from(parentFiles).reduce((n,path)=>n+read(path).split("\n").filter(line=>line.trim()).length,0)).toBeLessThanOrEqual(4000);
 const source=read(resolve(family,"🦀️.rs")),tree=parser.parse(source);try{
 const structures=tree.rootNode.descendantsOfType("struct_item"),find=(name:string)=>{const value=structures.find(node=>node.childForFieldName("name")?.text===name);if(!value)throw Error("Defining struct required: "+name);return value;};
 for(const name of["MeshMergeJob","MeshImportJob","MeshSourceRemap","MeshTopologyBuild","MeshPolygonNormal"]){const fields=find(name).descendantsOfType("field_declaration");expect(fields.some(field=>field.namedChildren.some(node=>node.type==="visibility_modifier"))).toBe(false);}
 expect(find("MeshSourceSnapshot").descendantsOfType("field_declaration").filter(field=>field.namedChildren.some(node=>node.type==="visibility_modifier")).map(field=>field.childForFieldName("name")?.text)).toEqual(["source","soup"]);
 const retirement=(name:string)=>{const item=tree.rootNode.descendantsOfType("impl_item").find(node=>node.childForFieldName("trait")?.text==="semio_framework_value::retirement::RetireOwned"&&node.childForFieldName("type")?.text===name);if(!item)throw Error("Actual defining retirement required: "+name);return item.text;};
 const dynamic=/Vec\s*<|HistoryFold|HalfedgeMesh|Soup|String|PolygonMeshSource|MeshSourceSnapshot|MeshTopologyBuild|MeshSourceRemap|Coplanar|SourceValueEquality|SourceEqualityTask/;
 for(const name of["MeshMergeJob","MeshImportJob","MeshSourceRemap","MeshTopologyBuild","MeshSourceSnapshot","Coplanar","SourceValueEquality"]){const body=retirement(name);for(const field of find(name).descendantsOfType("field_declaration")){const type=field.childForFieldName("type")?.text??"",key=field.childForFieldName("name")?.text;if(!key)throw Error("Actual field key required");if(type==="MeshSourceRemapData"){for(const nested of find(type).descendantsOfType("field_declaration")){if(dynamic.test(nested.childForFieldName("type")?.text??""))expect([body.includes("deferred(self.data."+nested.childForFieldName("name")?.text+")"),body.includes("deferred_birth_bytes_for(&self.data."+nested.childForFieldName("name")?.text+")")]).toEqual([true,true]);}}else if(dynamic.test(type))expect([body.includes("deferred(self."+key+")"),body.includes("deferred_birth_bytes_for(&self."+key+")")]).toEqual([true,true]);}}
 const equalityRetirement=retirement("SourceEqualityTask");for(const variant of["Node","Sequence","Text"])expect(equalityRetirement.includes("Self::"+variant+"(path")).toBe(true);expect(equalityRetirement.includes("deferred(path)")).toBe(true);expect(equalityRetirement.includes("deferred_birth_bytes_for(path)")).toBe(true);
 expect(source.includes("let point=self.snapshot.soup.positions[self.cursor];let key=mesh_weld_key(point,self.threshold)?;")).toBe(true);expect(source.includes("if self.phase!=0")).toBe(true);for(const name of["MeshMergeJob","MeshImportJob","MeshSourceRemap","MeshTopologyBuild"]){const owner=tree.rootNode.descendantsOfType("impl_item").filter(node=>node.childForFieldName("type")?.text===name);const advance=owner.flatMap(node=>node.descendantsOfType("function_item")).find(node=>node.childForFieldName("name")?.text==="advance");if(!advance)throw Error("Actual advance required");expect(advance.text.includes("self.phase>=254")).toBe(true);expect(advance.text.includes("advance_owned(")).toBe(true);}expect(source.includes("semio_framework_3d::")).toBe(false);
 }finally{tree.delete();}
 const parent=readModuleTreeSource(resolve(mesh,"🛠️modeling/🦀️.rs"));expect(parent.includes("work.snapshot_frontier()")).toBe(true);expect(parent.includes("work.transfer_remaining_assets(&mut mesh)?")).toBe(true);expect(parent.includes("Import::indexed(")).toBe(true);expect(parent.includes("configuration_mut()?")).toBe(true);
 const jobs=read(resolve(mesh,"🧪️tests/🔬️jobs/🦀️.rs")),jobsTree=parser.parse(jobs);try{expect(jobsTree.rootNode.hasError()).toBe(false);}finally{jobsTree.delete();}expect(jobs.includes("snapshot()()")).toBe(false);expect(jobs).toContain("snapshot.state()");expect(jobs).toContain("snapshot.scratch_corners()");expect(jobs).toContain("build.state().used_empty");
 const manifest=parse(read(resolve(family,"📦️packages/🦀️rust/Cargo.toml"))),deps=manifest.dependencies;if(!deps||typeof deps!=="object"||Array.isArray(deps))throw Error("Family dependencies required");expect(Object.keys(deps).sort()).toEqual(["semio-framework-3d-mesh-contract","semio-framework-3d-mesh-document","semio-framework-mesh-engine","semio-framework-value"].sort());
 }finally{parser.delete();}
});

test("closed reconstruction intentions match actual Rust variants and independent SQLite admission",async()=>{
 const validate=new Ajv({strict:true}).compile(intentSchema),db=new Database(":memory:");try{for(const row of intentRows){expect(validate(row.value)).toBe(row.accepted);const value=JSON.stringify(row.value);const sql=db.query(`select case json_extract(?1,'$.kind') when 'Coplanar' then (select count(*)=1 from json_each(?1)) when 'DissolveEdges' then json_array_length(?1,'$.edges')>0 when 'DissolveVertices' then json_array_length(?1,'$.vertices')>0 when 'Vertices' then json_array_length(?1,'$.vertices')>0 and json_extract(?1,'$.mode') in ('Center','First','ByDistance') and (json_extract(?1,'$.mode')!='ByDistance' or json_extract(?1,'$.threshold')>=0) when 'Grid' then json_extract(?1,'$.precision')>0 else 0 end as accepted`).get(value);expect(sql).toEqual({accepted:Number(row.accepted)});}}finally{db.close();}
 const root=resolve(import.meta.dir,"../../../../../../.."),source=readFileSync(resolve(root,"🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🔁️reconstruction/🦀️.rs"),"utf8");await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const parser=new Parser();try{parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));const tree=parser.parse(source);try{const variants=(name:string)=>{const item=tree.rootNode.descendantsOfType("enum_item").find(node=>node.childForFieldName("name")?.text===name);if(!item)throw Error("Actual closed intent required");const actual=item.descendantsOfType("enum_variant");if(name==="MeshPrimitiveKind")expect(actual.map(node=>node.childForFieldName("value")?.text)).toEqual(["0","1","2","3","4"]);return actual.map(node=>node.childForFieldName("name")?.text);};expect(variants("MeshMergeIntent")).toEqual(intentSchema.oneOf.map(row=>row.properties.kind.const));expect(variants("MeshPrimitiveKind")).toEqual(intentSchema.$defs.MeshPrimitiveKind.enum);}finally{tree.delete();}}finally{parser.delete();}
});

test("every actual higher 3D test compilation target has a bounded canonical Source closure",async()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),base=resolve(root,"🧰️framework/🔨️modules/🧊️3d"),schema=JSON.parse(readFileSync(resolve(base,"🥽️mesh/🧪️tests/📦️targets/🔣️.json"),"utf8")),rows:unknown=JSON.parse(readFileSync(resolve(base,"🥽️mesh/🧪️tests/📦️targets/🧪️.json"),"utf8"));
 if(!Array.isArray(rows))throw Error("Budget corpus required");const db=new Database(":memory:");try{for(const row of rows){expect(new Ajv({strict:true}).validate(schema,row)).toBe(true);if(!row||typeof row!=="object"||typeof row.bytes!=="number"||typeof row.files!=="number"||typeof row.lines!=="number"||typeof row.accepted!=="boolean")throw Error("Budget record required");expect(row.bytes<=262144&&row.files<=32&&row.lines<=4000).toBe(row.accepted);expect(db.query("select ?1<=262144 and ?2<=32 and ?3<=4000 as accepted").get(row.bytes,row.files,row.lines)).toEqual({accepted:Number(row.accepted)});}}finally{db.close();}
 const read=(path:string)=>readFileSync(path,"utf8"),closure=(entry:string)=>{const files=new Set<string>();const visit=(path:string)=>{if(files.has(path))return;files.add(path);for(const match of read(path).matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)){const child=match[1];if(!child)throw Error("Literal mount required");visit(resolve(dirname(path),child));}};visit(entry);return files;};
 const manifestPath=resolve(base,"🥽️mesh/🧪️tests/📦️targets/📦️packages/🦀️rust/Cargo.toml"),manifest=parse(read(manifestPath));
 if(!manifest.package||typeof manifest.package!=="object"||!("autotests"in manifest.package)||manifest.package.autotests!==false||!Array.isArray(manifest.test))throw Error("Explicit integration targets required");
 const entries=[resolve(base,"📦️packages/🦀️rust/🦀️.rs"),resolve(base,"🥽️mesh/📄️document/🦀️.rs"),resolve(base,"🥽️mesh/🛠️modeling/🔁️reconstruction/🦀️.rs"),resolve(base,"🥽️mesh/🛠️modeling/✂️clipping/🦀️.rs"),resolve(base,"🥽️mesh/🧬️contract/🦀️.rs"),resolve(base,"🌀️rigid/🦀️.rs"),resolve(base,"🧿️collision/🦀️.rs")];
 for(const relative of["🌀️rigid/📦️packages/🦀️rust/Cargo.toml","🧿️collision/📦️packages/🦀️rust/Cargo.toml","🥽️mesh/🧪️tests/📦️targets/📦️packages/🦀️rust/Cargo.toml"]){const path=resolve(base,relative),data=parse(read(path));if(!data.lib||typeof data.lib!=="object"||!("path"in data.lib)||typeof data.lib.path!=="string"||!data.package||typeof data.package!=="object"||!("workspace"in data.package)||typeof data.package.workspace!=="string")throw Error("Actual Cargo root required");expect(readFileSync(resolve(dirname(path),data.lib.path)).length).toBeGreaterThan(0);expect(resolve(dirname(path),data.package.workspace)).toBe(resolve(base,"../.."));}
 const targets:string[]=[];for(const row of manifest.test){if(!row||typeof row!=="object"||!("path"in row)||!("name"in row)||typeof row.path!=="string"||typeof row.name!=="string")throw Error("Closed test target required");const path=resolve(dirname(manifestPath),row.path);targets.push(path);entries.push(path);expect(read(path).includes("use super::*;")).toBe(false);expect(read(path).includes("semio_framework_3d::")).toBe(true);}
 expect(targets.length).toBe(5);expect(new Set(targets).size).toBe(5);
 await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const parser=new Parser();try{parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));
 for(const entry of entries){const files=closure(entry),bytes=Array.from(files).reduce((n,path)=>n+readFileSync(path).length,0),lines=Array.from(files).reduce((n,path)=>n+read(path).split("\n").filter(line=>line.trim()).length,0);expect(bytes).toBeLessThanOrEqual(262144);expect(files.size).toBeLessThanOrEqual(32);expect(lines).toBeLessThanOrEqual(4000);for(const path of files){const text=read(path),tree=parser.parse(text);try{expect(tree.rootNode.hasError()).toBe(false);}finally{tree.delete();}for(const match of text.matchAll(/include_str!\("([^"]+)"\)/g)){const child=match[1];if(!child)throw Error("Fixture name required");expect(readFileSync(resolve(dirname(path),child)).length).toBeGreaterThan(0);}}}
 }finally{parser.delete();}
});

test("integration targets preserve every original law and private observation never exposes an effect",async()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),base=resolve(root,"🧰️framework/🔨️modules/🧊️3d"),mesh=resolve(base,"🥽️mesh"),read=(path:string)=>readFileSync(path,"utf8"),records:unknown=JSON.parse(read(resolve(mesh,"🧪️tests/📦️targets/📋️preserved.json")));
 if(!Array.isArray(records))throw Error("Preserved law table required");const moved=read(resolve(mesh,"🛠️modeling/🔁️reconstruction/🧪️tests/🦀️.rs"));let actualAssertions=0,expectedAssertions=0;const allSources:string[]=[];
 for(const row of records){if(!row||typeof row!=="object"||typeof row.path!=="string"||!Array.isArray(row.tests)||typeof row.assertions!=="number")throw Error("Preserved law record required");const text=read(resolve(root,row.path));allSources.push(text);actualAssertions+=(text.match(/\bassert(?:_eq|_ne)?!/g)??[]).length;expectedAssertions+=row.assertions;for(const name of row.tests){if(typeof name!=="string")throw Error("Declared law name required");const matches=[text,moved].flatMap(source=>Array.from(source.matchAll(/\bfn\s+(\w+)\s*\(/g))).filter(match=>match[1]===name);expect(matches.length).toBe(1);}}
 await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const parser=new Parser();try{parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));const tree=parser.parse(moved);try{const fn=tree.rootNode.descendantsOfType("function_item").find(node=>node.childForFieldName("name")?.text==="original_modeling_snapshot_frontier_preserves_same_source_and_actual_receipts");if(!fn)throw Error("Defining snapshot owner law required");actualAssertions+=(fn.text.match(/\bassert(?:_eq|_ne)?!/g)??[]).length;}finally{tree.delete();}
 expect(actualAssertions).toBe(expectedAssertions);
 for(const text of allSources){const tree=parser.parse(text);try{expect(tree.rootNode.hasError()).toBe(false);}finally{tree.delete();}}
 const inspection=read(resolve(mesh,"🛠️modeling/🔬️inspection/🦀️.rs"));expect(inspection.includes("&mut ")).toBe(false);expect(inspection.includes("pub use super::*")).toBe(false);const observationTree=parser.parse(inspection);try{expect(observationTree.rootNode.hasError()).toBe(false);const methods=observationTree.rootNode.descendantsOfType("function_item").map(node=>node.childForFieldName("name")?.text);expect(methods.sort()).toEqual(["inspection","inspection_cancelled","snapshot"]);}finally{observationTree.delete();}
 const parent=read(resolve(mesh,"🛠️modeling/🦀️.rs")),facade=read(resolve(mesh,"🦀️.rs"));expect(parent).toContain('#[cfg(feature="mesh-test-observation")]');expect(facade).toContain('#[cfg(feature="mesh-test-observation")]');expect(parent.includes("pub enum Work")).toBe(false);expect(parent.includes("pub work:")).toBe(false);
 const parentManifest=parse(read(resolve(base,"📦️packages/🦀️rust/Cargo.toml"))),collisionManifest=parse(read(resolve(base,"🧿️collision/📦️packages/🦀️rust/Cargo.toml"))),rigidManifest=parse(read(resolve(base,"🌀️rigid/📦️packages/🦀️rust/Cargo.toml")));
 const dependencyNames=(value:unknown)=>{if(!value||typeof value!=="object"||!("dependencies"in value))return[];if(!value.dependencies||typeof value.dependencies!=="object"||Array.isArray(value.dependencies))throw Error("Actual dependency table required");return Object.keys(value.dependencies);};
 expect(dependencyNames(parentManifest)).toContain("semio-framework-3d-rigid");expect(dependencyNames(parentManifest)).toContain("semio-framework-3d-collision");expect(dependencyNames(collisionManifest)).toEqual(["semio-framework-3d-rigid","semio-framework-value","semio-framework-value-derive"]);expect(dependencyNames(rigidManifest)).toEqual(["semio-framework-value","semio-framework-value-derive"]);
 const parentRoot=read(resolve(base,"📦️packages/🦀️rust/🦀️.rs"));expect(parentRoot).toContain("pub use semio_framework_3d_collision as collision;");expect(parentRoot).toContain("pub use semio_framework_3d_rigid as rigid;");
 const workspace=read(resolve(base,"../../Cargo.toml"));for(const path of["🌀️rigid/📦️packages/🦀️rust","🧿️collision/📦️packages/🦀️rust","🥽️mesh/🧪️tests/📦️targets/📦️packages/🦀️rust"]){const member='"🔨️modules/🧊️3d/'+path+'"';expect(workspace.split(member).length-1).toBe(1);}
 const enumTree=parser.parse(inspection);try{const item=enumTree.rootNode.descendantsOfType("enum_item").find(node=>node.childForFieldName("name")?.text==="MeshModelingInspection");if(!item)throw Error("Immutable observation variants required");expect(item.descendantsOfType("enum_variant").map(node=>node.childForFieldName("name")?.text).sort()).toEqual(["Bevel","Decimate","Mirror","Merge","Subdivide","LoopCut","Orient","FillHoles","Inflate","Import","Knife","Transform"].sort());expect(inspection.includes(".clone(")).toBe(false);expect(inspection.includes(".step(")).toBe(false);expect(inspection.includes(".advance(")).toBe(false);}finally{enumTree.delete();}
 const packageText=read(resolve(mesh,"🧪️tests/📦️targets/📦️packages/🦀️rust/Cargo.toml"));expect(packageText).toContain('features = ["mesh-test-observation"]');expect(packageText.includes('[[bin]]')).toBe(false);
 for(const text of allSources){for(const match of text.matchAll(/include_str!\("([^"]+)"\)/g)){const fixture=match[1];if(!fixture)throw Error("Fixture required");const index=allSources.indexOf(text),record=records[index];if(!record||typeof record!=="object"||typeof record.path!=="string")throw Error("Fixture owner required");expect(readFileSync(resolve(dirname(resolve(root,record.path)),fixture)).length).toBeGreaterThan(0);}}
 }finally{parser.delete();}
});
