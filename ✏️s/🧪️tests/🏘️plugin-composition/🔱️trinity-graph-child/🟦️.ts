/** 🔱️ Rewriting parents compose the Semio graph child; this integration law needs both plugins, so it lives where plugins may meet. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync,readdirSync} from "node:fs";
import {semioSchemaAjvV1} from "../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import * as owner from "../../../🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🟦️.ts";
import artifactReferenceSchema from "../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
const trinityTests=new URL("../../../🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/",import.meta.url);
const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",trinityTests),"utf8"));
const sql=readFileSync(new URL("../🗄️.sql",trinityTests),"utf8");
function value(input:any):unknown{switch(input.variant){case"null":return{kind:"null"};case"bool":return{kind:"bool",value:input.value};case"number":return{kind:"number",value:{bits:BigInt("0x"+input.bits)}};case"string":return{kind:"string",value:input.value};case"array":return{kind:"array",values:input.elements.map(value)};case"object":return{kind:"object",values:Object.fromEntries(input.members.map((row:any)=>[row.key,value(row.value)]))};default:throw Error("fixture variant");}}
function specimen(word=fixture.binary64Bits[0]):unknown{const typed=JSON.parse(JSON.stringify({workingGraph:fixture.nativeCase.workingGraph,lhs:fixture.nativeCase.lhs,rhs:fixture.nativeCase.rhs}),(key,value)=>key==="bits"?BigInt("0x"+value):value);for(const parameter of typed.rhs.parameters)if(parameter.default.kind==="number")parameter.default.value.bits=BigInt("0x"+word);return{...typed,parameterBindings:Object.fromEntries(fixture.nativeCase.parameterBindings.map((row:any)=>[row.key,value(row.value)])),ruleLayout:Object.fromEntries(fixture.nativeCase.ruleLayout.map((row:any)=>[row.key,{x:{bits:BigInt("0x"+word)},y:{bits:BigInt("0x"+word)}}]))};}
const project=(input:unknown,options:Record<string,unknown>={})=>owner.rewritingSnapshotToSqliteDatabase(input as owner.RewritingArtifact,options);
const restore=(database:SqliteDatabase,options:Record<string,unknown>={})=>owner.rewritingSnapshotFromSqliteDatabase(database,options);
async function oracle(input:unknown):Promise<Database>{return Database.deserialize(Buffer.from(await exportSqliteDatabase(await project(input))));}
async function edited(db:Database):Promise<SqliteDatabase>{return importSqliteDatabase(db.serialize());}
test("Rewriting editor initial owner preserves the independent nine-node six-edge Jack demo and original query",async()=>{
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,trinityTests),"utf8")),initial=read("../../../../🧪️tests/🧫️fixtures/🪆️child/🏢️initial/🔣️.json");
 const ajv=semioSchemaAjvV1({allErrors:true}),childSchema=read("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json");ajv.addSchema(artifactReferenceSchema);ajv.addSchema(childSchema);const validate=ajv.compile(childSchema);
 expect(initial["query"]).toEqual("MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label");expect(initial["rootNodeId"]).toEqual("7dc5b737-3b6b-4068-b315-b7bacc91c2e1");expect(initial.child).toEqual(read("../../../../🧪️tests/🧫️fixtures/🪆️child/🏢️initial/🪆️content/🔣️.json"));
 expect(validate(initial.child)).toBe(true);
 expect(initial.query).toBe("MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label");
 expect(initial.rootNodeId).toBe("7dc5b737-3b6b-4068-b315-b7bacc91c2e1");
 expect(initial.child.nodes.map((node:any)=>node.label)).toEqual(["b","t_f0_b_c0","t_f0_b_c1","t_f1_b_c0","t_f8_b_c0","ci_t_f8_b_c0","jack_orphan","jack_prune","jack_spare"]);
 expect(initial.child.nodes.map((node:any)=>node.id.value)).toEqual(["7dc5b737-3b6b-4068-b315-b7bacc91c2e1","5f0266bc-856b-4ef2-9eb0-16ef5e1fb952","17d5dec8-87b2-44a9-84ff-93b7e7419bdd","596f3f60-b634-467b-8acc-7d5b649ae0d1","f2edea5c-960e-46bd-932d-aae5ecad72d8","6947a41b-8c6d-4291-bdd8-96cd535c78fc","a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6","b9f4e3d2-5c6d-7e8f-90a1-b2c3d4e5f6a7","caf5e4d3-6d7e-8f90-a1b2-c3d4e5f6a7b8"]);
 expect(initial.child.edges.map((edge:any)=>edge.id.value)).toEqual(["cc0d14f5-11b2-4258-ac52-287d0d46b23b","8684b51c-94f5-4a50-be16-526ac8e0892a","e-shaft-1","e-shaft-8","fdae9f64-398f-45ad-8496-453e72c31512","e-jack-prune"]);
 const number=(word:{bits:string})=>{const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+word.bits));return view.getFloat64(0)};
 expect(initial.child.nodes.map((node:any)=>[number(node.position.x),number(node.position.y),number(node.width),number(node.height)])).toEqual([[0,0,96,48],[-140,80,88,40],[140,80,88,40],[-140,180,88,40],[-140,280,88,40],[-140,380,96,44],[300,200,88,40],[300,80,88,40],[300,300,88,40]]);
 expect(initial.child.nodes.map((node:any)=>node.ports.length)).toEqual([2,2,2,2,2,1,1,1,1]);
 expect(initial.child.nodes.map((node:any)=>node.properties.find((property:any)=>property.key==="label").value.value)).toEqual(["tower-core","floor0-left","floor0-right","floor1-shaft","floor8-shaft","capsule-instance","jack-orphan","jack-prune","jack-spare"]);
 expect(initial.child.nodes.map((node:any)=>node.properties.find((property:any)=>property.key==="tier").value.lexeme)).toEqual(["0.0","0.0","0.0","1.0","8.0","8.0","0.0","0.0","0.0"]);
 expect(initial.child.nodes.map((node:any)=>node.properties.map((property:any)=>property.key))).toEqual(Array.from({length:9},(_,n)=>n<6?["label","position","tier"]:["label","tier"]));
 expect(initial.child.nodes.slice(0,6).map((node:any)=>node.properties.find((property:any)=>property.key==="position").value.entries.map((property:any)=>[property.key,property.value.kind,property.value.lexeme]))).toEqual([["0.0","0.0"],["-1.2","1.2"],["1.2","1.2"],["-1.2","2.4"],["-1.2","9.6"],["-1.2","11.2"]].map(([x,y])=>[["x","float",x],["y","float",y],["z","float","0.0"]]));
 expect(initial.child.nodes.flatMap((node:any)=>node.ports).every((port:any)=>port.category==="Connector"&&port.properties.length===0)).toBe(true);
 expect(initial.child.nodes.map((node:any)=>node.ports.map((port:any)=>[port.name,port.kind]))).toEqual([[ ["c5465220-19ba-4443-8f1d-617c832dd13c","out"],["d25a91ed-b124-4e5b-8e7e-af832541c953","out"] ],[ ["7be6cfda-db1b-47e4-bcea-cd011e516e2e","in"],["e0fb5d49-9286-4a6d-88b0-043f389c6898","out"] ],[ ["7be6cfda-db1b-47e4-bcea-cd011e516e2e","in"],["f1a2b3c4-d5e6-7890-abcd-ef1234567890","out"] ],[ ["7be6cfda-db1b-47e4-bcea-cd011e516e2e","in"],["e0fb5d49-9286-4a6d-88b0-043f389c6898","out"] ],[ ["e0fb5d49-9286-4a6d-88b0-043f389c6898","in"],["c5465220-19ba-4443-8f1d-617c832dd13c","out"] ],[["4ba51a88-2a7f-4b78-b119-0f02eacdb702","in"]],[["d4e5f6a7-b8c9-0123-def4-567890abcdef","in"]],[["e5f6a7b8-c9d0-1234-ef56-7890abcdef01","in"]],[["f6a7b8c9-d0e1-2345-f678-90abcdef0123","out"]]]);
 expect(initial.child.edges.map((edge:any)=>[edge.source.value+"@"+edge.sourcePort,edge.target.value+"@"+edge.targetPort])).toEqual([["7dc5b737-3b6b-4068-b315-b7bacc91c2e1@c5465220-19ba-4443-8f1d-617c832dd13c","5f0266bc-856b-4ef2-9eb0-16ef5e1fb952@7be6cfda-db1b-47e4-bcea-cd011e516e2e"],["7dc5b737-3b6b-4068-b315-b7bacc91c2e1@d25a91ed-b124-4e5b-8e7e-af832541c953","17d5dec8-87b2-44a9-84ff-93b7e7419bdd@7be6cfda-db1b-47e4-bcea-cd011e516e2e"],["5f0266bc-856b-4ef2-9eb0-16ef5e1fb952@e0fb5d49-9286-4a6d-88b0-043f389c6898","596f3f60-b634-467b-8acc-7d5b649ae0d1@7be6cfda-db1b-47e4-bcea-cd011e516e2e"],["596f3f60-b634-467b-8acc-7d5b649ae0d1@e0fb5d49-9286-4a6d-88b0-043f389c6898","f2edea5c-960e-46bd-932d-aae5ecad72d8@e0fb5d49-9286-4a6d-88b0-043f389c6898"],["f2edea5c-960e-46bd-932d-aae5ecad72d8@c5465220-19ba-4443-8f1d-617c832dd13c","6947a41b-8c6d-4291-bdd8-96cd535c78fc@4ba51a88-2a7f-4b78-b119-0f02eacdb702"],["17d5dec8-87b2-44a9-84ff-93b7e7419bdd@f1a2b3c4-d5e6-7890-abcd-ef1234567890","b9f4e3d2-5c6d-7e8f-90a1-b2c3d4e5f6a7@e5f6a7b8-c9d0-1234-ef56-7890abcdef01"]]);
 expect(initial.child.edges.every((edge:any)=>edge.kind==="Connection"&&edge.label==="")).toBe(true);
 expect(initial.child.edges.map((edge:any)=>edge.properties.map((property:any)=>[property.key,property.value.kind,property.value.lexeme]))).toEqual([["270.0","-1.2","1.2"],["90.0","1.2","1.2"],["0.0","0.0","1.2"],["0.0","0.0","7.2"],["90.0","0.0","1.6"],["0.0","1.2","0.0"]].map(([rotation,u,v])=>[["gap","float","0.0"],["rise","float","0.0"],["rotation","float",rotation],["shift","float","0.0"],["tilt","float","0.0"],["turn","float","0.0"],["u","float",u],["v","float",v]]));
 const graph=await import("../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts"),sql=await import("../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts");
 const typed=graph.parseSemioGraphJsonValue(initial.child),database=Database.deserialize(await exportSqliteDatabase(await sql.semioGraphSnapshotToSqliteDatabase(typed)));
 try{expect((database.query("SELECT COUNT(*) AS count FROM semio_graph_node").get()as{count:number}).count).toBe(9);expect((database.query("SELECT COUNT(*) AS count FROM semio_graph_edge").get()as{count:number}).count).toBe(6);expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(graph.semioGraphJsonValue(await sql.semioGraphSnapshotFromSqliteDatabase(await importSqliteDatabase(database.serialize())))).toEqual(initial.child);}
 finally{database.close();}
});
test("Rewriting Nakagin scenario parents retain complete separately owned typed Semio children",async()=>{
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,trinityTests),"utf8"));
 const ajv=semioSchemaAjvV1({allErrors:true});
 for(const relative of read("./🧫️fixtures/🌳️typed/🔣️.json").schemaDocuments)ajv.addSchema(read(relative));
 const parentSchema=ajv.compile(read("../../../../🧬️schema/🔣️.json"));
 const graphSchema=ajv.compile(read("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json"));
 const graph=await import("../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts");
 const graphSql=await import("../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts");
 for(const[folder,id,nodes,edges,ports]of[
  ["","nakagin-rewriting-content",180,179,364],
  ["🏢️nakagin-ground-floor/","nakagin-ground-floor-rewriting-content",2,1,2]
 ]as const){
  const root="../../../../🧫️fixtures/♻️mutate-rewrite-1"+folder,parent=read(root+"🔣️.snapshot.json");
  expect(parentSchema(parent)).toBe(true);
  expect(parent.workingGraph.content).toEqual({childId:id,target:{artifactId:id,dialect:{artifactKind:"s.stdio.semio",standard:"v1",subset:"graph"}}});
  const child=read(root+"🪆️content/🔣️.snapshot.json");
  expect(graphSchema(child)).toBe(true);
  expect(Object.keys(parent).sort()).toEqual(["lhs","parameterBindings","rhs","ruleLayout","workingGraph"]);
  expect(Object.keys(child).sort()).toEqual(["edges","nodes","schema"]);
  expect(parent.workingGraph.rootNodeId).toBe("7dc5b737-3b6b-4068-b315-b7bacc91c2e1");
  expect(parent.workingGraph.manifestId).toBe("nakagin");
  expect(parent.workingGraph.manifest.nodeKinds.some((kind:any)=>kind.name==="Piece")).toBe(true);
  expect(parent.workingGraph.manifest.edgeKinds.some((kind:any)=>kind.name==="Connection")).toBe(true);
  expect(parent.lhs.pattern).toEqual({leftVar:"a",leftKind:"Piece",edgeVar:"r",edgeKind:"Connection",rightVar:"b",rightKind:"Piece"});
  expect(parent.lhs.whereClause).toBe("a.name = 'b'");
  expect(parent.rhs.set).toEqual([{var:"a",prop:"label",value:{kind:"string",value:"$label"}}]);
  expect(parent.rhs.parameters).toEqual([{name:"label",kind:"string",default:{kind:"string",value:"nakagin-core"}}]);
  expect(parent.parameterBindings).toEqual({label:{kind:"string",value:"nakagin-core"}});
  const typedParent=owner.parseRewritingJsonValue(parent),parentDb=Database.deserialize(await exportSqliteDatabase(await project(typedParent)));
  try{
   expect(parentDb.query("PRAGMA foreign_key_check").all()).toEqual([]);
   expect(await restore(await importSqliteDatabase(parentDb.serialize()))).toEqual(typedParent);
  }finally{parentDb.close();}
  const typed=graph.parseSemioGraphJsonValue(child),db=Database.deserialize(await exportSqliteDatabase(await graphSql.semioGraphSnapshotToSqliteDatabase(typed)));
  try{
   expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
   for(const[table,count]of[["semio_graph_node",nodes],["semio_graph_edge",edges],["semio_graph_port",ports]]as const)expect((db.query("SELECT COUNT(*) AS count FROM "+table).get()as{count:number}).count).toBe(count);
   expect(child.nodes.reduce((count:number,node:any)=>count+node.ports.length,0)).toBe(ports);
   const ids=new Set(child.nodes.map((node:any)=>node.id.value));
   expect(ids.size).toBe(nodes);
   for(const edge of child.edges){
    expect(ids.has(edge.source.value)).toBe(true);expect(ids.has(edge.target.value)).toBe(true);
    const source=child.nodes.find((node:any)=>node.id.value===edge.source.value),target=child.nodes.find((node:any)=>node.id.value===edge.target.value);
    if(edge.sourcePort!==undefined)expect(source.ports.some((port:any)=>port.name===edge.sourcePort)).toBe(true);
    if(edge.targetPort!==undefined)expect(target.ports.some((port:any)=>port.name===edge.targetPort)).toBe(true);
   }
   for(const node of child.nodes)for(const word of[node.position.x,node.position.y,node.width,node.height])expect(word.bits).toMatch(/^[0-9a-f]{16}$/);
   expect(await graphSql.semioGraphSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(typed);
  }finally{db.close();}
 }
});
test("Rewriting full child lease frame retains rich owner words across publication and bounded return",async()=>{
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,trinityTests),"utf8"));
 const contract=read("../../../../🧪️tests/🧫️fixtures/📸️snapshot/🪆️child/🧵️lifetime/🔣️.json");
 const ajv=semioSchemaAjvV1({allErrors:true}),graphSpec=read("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json");ajv.addSchema(graphSpec);
 
 expect(contract["contractId"]).toEqual("rewriting-full-child-frame-lifetime");expect(contract["slot"]).toEqual("workingGraph");expect(contract["target"]["dialect"]["artifactKind"]).toEqual("s.stdio.semio");expect(contract["target"]["dialect"]["standard"]).toEqual("v1");expect(contract["target"]["dialect"]["subset"]).toEqual("graph");expect(contract["retainedTailBytes"]).toEqual(65536);expect(contract["borrowedCaptureAllocationBytes"]).toEqual(0);expect(contract["closeGrants"]).toEqual([0,1]);expect(contract["refusals"]).toEqual(["wrongParent","missingSlot","missingTarget","wrongDialect"]);expect(contract["intrinsicKinds"]).toEqual(["bool","bytes","float","int","list","map","null","ref","str"]);expect(contract["childId"]).toEqual("logical !@/\u0000引用😀");
 
 const vector=read("./🧫️fixtures/🌳️typed/🔣️.json"),graphSchema=ajv.getSchema(graphSpec.$id)!;
 const graph=await import("../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts");
 const graphSql=await import("../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts");
 const child=structuredClone(vector.childSnapshot);child.nodes[0].properties.push(...contract.intrinsicValues);
 const old=graph.parseSemioGraphJsonValue(child),current=graph.parseSemioGraphJsonValue(child);
 current.nodes[0]!.label=contract.nextLiteral+"x".repeat(contract.retainedTailBytes);
 const oldDb=Database.deserialize(await exportSqliteDatabase(await graphSql.semioGraphSnapshotToSqliteDatabase(old)));
 const currentDb=Database.deserialize(await exportSqliteDatabase(await graphSql.semioGraphSnapshotToSqliteDatabase(current)));
 const leases=new Database(":memory:");
 try{
  leases.exec("CREATE TABLE owner(revision TEXT PRIMARY KEY, parent_generation INTEGER NOT NULL, slot TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL, captures INTEGER NOT NULL CHECK(captures>=0), published INTEGER NOT NULL CHECK(published IN(0,1))); CREATE TABLE frame(parent_generation INTEGER PRIMARY KEY, parent_revision TEXT NOT NULL, child_revision TEXT NOT NULL REFERENCES owner(revision));");
  const identity=contract.target,oldRevision=contract.publications[0].childRevision,nextRevision=contract.publications[1].childRevision;
  leases.run("INSERT INTO owner VALUES(?,?,?,?,?,?,?,0,1)",[oldRevision,contract.publications[0].parentGeneration,contract.slot,identity.artifactId,identity.dialect.artifactKind,identity.dialect.standard,identity.dialect.subset]);
  leases.run("INSERT INTO frame VALUES(?,?,?)",[contract.publications[0].parentGeneration,contract.publications[0].parentRevision,oldRevision]);
  const capture=(generation:number,slot:string,target:any)=>leases.query("SELECT o.revision FROM frame f JOIN owner o ON o.revision=f.child_revision WHERE f.parent_generation=? AND o.slot=? AND o.artifact_id=? AND o.artifact_kind=? AND o.standard=? AND o.subset=?").get(generation,slot,target.artifactId,target.dialect.artifactKind,target.dialect.standard,target.dialect.subset)as{revision:string}|null;
  expect(capture(contract.publications[0].parentGeneration,contract.slot,identity)).toEqual({revision:oldRevision});
  for(const refusal of contract.refusals){const altered=structuredClone(identity);let generation=contract.publications[0].parentGeneration,slot=contract.slot;
   if(refusal==="wrongParent")generation++;else if(refusal==="missingSlot")slot+="missing";else if(refusal==="missingTarget")altered.artifactId+="missing";else altered.dialect.subset+="wrong";
   expect(capture(generation,slot,altered)).toBeNull();
  }
  leases.run("UPDATE owner SET captures=captures+1 WHERE revision=?",[oldRevision]);
  leases.run("UPDATE owner SET published=0 WHERE revision=?",[oldRevision]);
  leases.run("INSERT INTO owner VALUES(?,?,?,?,?,?,?,0,1)",[nextRevision,contract.publications[1].parentGeneration,contract.slot,identity.artifactId,identity.dialect.artifactKind,identity.dialect.standard,identity.dialect.subset]);
  leases.run("INSERT INTO frame VALUES(?,?,?)",[contract.publications[1].parentGeneration,contract.publications[1].parentRevision,nextRevision]);
  expect(capture(contract.publications[0].parentGeneration,contract.slot,identity)).toEqual({revision:oldRevision});
  expect(capture(contract.publications[1].parentGeneration,contract.slot,identity)).toEqual({revision:nextRevision});
  expect(leases.query("SELECT captures FROM owner WHERE revision=? AND published=0").get(oldRevision)).toEqual({captures:1});
  const before=await graphSql.semioGraphSnapshotFromSqliteDatabase(await importSqliteDatabase(oldDb.serialize())),after=await graphSql.semioGraphSnapshotFromSqliteDatabase(await importSqliteDatabase(currentDb.serialize()));
  expect(before).toEqual(old);expect(after).toEqual(current);
  expect(before.nodes[0]!.label).toBe(vector.childOracle.literal);expect(after.nodes[0]!.label).toBe(contract.nextLiteral+"x".repeat(contract.retainedTailBytes));
  expect(before.nodes[0]!.width.bits).toBe(BigInt("0x"+vector.childOracle.widthBits));expect(before.nodes[0]!.height.bits).toBe(BigInt("0x"+vector.childOracle.heightBits));
  expect(graphSchema(child)).toBe(true);
  const kinds=new Set<string>(),pending=[...before.nodes.flatMap(node=>node.properties.map(row=>row.value)),...before.nodes.flatMap(node=>node.ports.flatMap(port=>port.properties.map(row=>row.value))),...before.edges.flatMap(edge=>edge.properties.map(row=>row.value))];
  while(pending.length){const value=pending.pop()!;kinds.add(value.kind);if(value.kind==="list")pending.push(...value.items);if(value.kind==="map")pending.push(...value.entries.map(row=>row.value));}
  expect([...kinds].sort()).toEqual(contract.intrinsicKinds);
  expect(contract.closeGrants).toEqual([0,1]);expect(contract.borrowedCaptureAllocationBytes).toBe(0);
  for(const grant of contract.closeGrants){if(grant===0)expect(leases.query("SELECT captures FROM owner WHERE revision=?").get(oldRevision)).toEqual({captures:1});else leases.run("UPDATE owner SET captures=captures-1 WHERE revision=?",[oldRevision]);}
  expect(leases.query("SELECT captures FROM owner WHERE revision=?").get(oldRevision)).toEqual({captures:0});
  leases.run("DELETE FROM frame WHERE child_revision=?",[oldRevision]);leases.run("DELETE FROM owner WHERE revision=? AND captures=0 AND published=0",[oldRevision]);
  expect(leases.query("SELECT revision FROM owner ORDER BY revision").all()).toEqual([{revision:nextRevision}]);
  expect(leases.query("PRAGMA foreign_key_check").all()).toEqual([]);
 }finally{oldDb.close();currentDb.close();leases.close();}
});
test("Rewriting composed graph retains rich typed node port and edge ownership",async()=>{
 const vector=await Bun.file(new URL("./🧫️fixtures/🌳️typed/🔣️.json",trinityTests)).json();expect(vector.childSnapshot).toBeDefined();const schema=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json",trinityTests)).json(),ajv=semioSchemaAjvV1({allErrors:true}),validate=ajv.compile(schema);expect(validate(vector.childSnapshot)).toBe(true);expect(validate({...vector.childSnapshot,nodes:[{...vector.childSnapshot.nodes[0],bodyJson:"{}"}]})).toBe(false);expect(validate({...vector.childSnapshot,edges:[{...vector.childSnapshot.edges[0],metadataJson:"{}"}]})).toBe(false);
 const typed=JSON.parse(JSON.stringify(vector.childSnapshot),(key,value)=>key==="bits"?BigInt("0x"+value):value),api=await import("../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts");const db=Database.deserialize(await exportSqliteDatabase(await api.semioGraphSnapshotToSqliteDatabase(typed)));
 try{
  expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  for(const[table,key]of [["semio_graph_node","nodeCount"],["semio_graph_port","portCount"],["semio_graph_edge","edgeCount"],["semio_graph_port_property","portPropertyCount"],["semio_graph_edge_property","edgePropertyCount"]])expect((db.query("SELECT COUNT(*) AS count FROM "+table).get() as {count:number}).count).toBe(vector.childOracle[key]);
  expect(db.query("SELECT p.name,p.category,v.integer_lexeme FROM semio_graph_port p JOIN semio_graph_port_property r ON r.port_id=p.id JOIN semio_graph_value v ON v.id=r.value_id WHERE v.kind='int'").get()).toEqual({name:vector.childOracle.sourcePort,category:"Signal",integer_lexeme:"18446744073709551615"});
  const word=(db.query("SELECT width_ieee754_bits,height_ieee754_bits,label FROM semio_graph_node ORDER BY ordinal LIMIT 1") as unknown as {safeIntegers(on:boolean):{get():{width_ieee754_bits:bigint;height_ieee754_bits:bigint;label:string}}}).safeIntegers(true).get();
  expect(BigInt.asUintN(64,word.width_ieee754_bits)).toBe(BigInt("0x"+vector.childOracle.widthBits));expect(BigInt.asUintN(64,word.height_ieee754_bits)).toBe(BigInt("0x"+vector.childOracle.heightBits));expect(word.label).toBe(vector.childOracle.literal);
  expect(db.query("SELECT source_port,target_port FROM semio_graph_edge ORDER BY ordinal").all()).toEqual([{source_port:"write@literal",target_port:"read"}, {source_port:"write@literal",target_port:null}]);
  expect(await api.semioGraphSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(typed);
 }finally{db.close();}
});
test("Rewriting actual Semio child public JSON codec preserves all words and intrinsic families",async()=>{
 const api=await import(new URL("../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts",trinityTests).href) as {parseSemioGraphJsonValue:(value:unknown)=>any;semioGraphJsonValue:(value:any)=>unknown};
 expect(Object.hasOwn(api,"parseSemioGraphJsonValue")).toBe(true);expect(Object.hasOwn(api,"semioGraphJsonValue")).toBe(true);
 const vector=await Bun.file(new URL("./🧫️fixtures/🌳️typed/🔣️.json",trinityTests)).json(),validate=semioSchemaAjvV1({allErrors:true}).compile(await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json",trinityTests)).json());
 const neutral=await Bun.file(new URL("../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧪️tests/🧫️fixtures/📸️snapshot/🔣️json/🔣️.json",trinityTests)).json();
 
 const fixtureAjv=semioSchemaAjvV1({allErrors:true});fixtureAjv.addSchema(await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json",trinityTests)).json());
 for(const word of fixture.binary64Bits){
  const raw=structuredClone(vector.childSnapshot);raw.nodes[0].properties=structuredClone(neutral.properties);for(const node of raw.nodes){node.position={x:{bits:word},y:{bits:word}};node.width={bits:word};node.height={bits:word};}
  expect(validate(raw)).toBe(true);const decoded=api.parseSemioGraphJsonValue(raw);for(const node of decoded.nodes){expect(node.position.x.bits).toBe(BigInt("0x"+word));expect(node.position.y.bits).toBe(BigInt("0x"+word));expect(node.width.bits).toBe(BigInt("0x"+word));expect(node.height.bits).toBe(BigInt("0x"+word));}
  expect(api.semioGraphJsonValue(decoded)).toEqual(raw);
  const kinds=decoded.nodes[0].properties.map((entry:any)=>entry.value.kind);expect(kinds).toEqual(["null","bool","int","float","str","bytes","list","map","ref"]);
  const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+word));expect(view.getBigUint64(0)).toBe(decoded.nodes[0].width.bits);
 }
 const snapshotRoot=new URL("../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations",trinityTests);
 const owners=["✂️delete-edge/✂️removes","✋️drag-nodes/✋️drags","🌉️create-edge/🌉️connects","🏗️create-node/🔎️appends","🏷️rename-node/🏷️renames","📍move-node/📍️moves","📐resize-node/📐️resizes","🔌add-node-port/🔌️inserts","🔚remove-node-port/🔚️detaches","🔧change-node-kind/🔧️retypes","🖍️change-node-label/🔤️relabels","🗑️delete-node/🚫️removes","➕add-edge-property/⚖️inserts","➕add-node-property/⚖️inserts","➖remove-edge-property/➖️detaches","➖remove-node-property/➖️detaches","🎚️set-edge-property/🎚️sets","🎛️set-node-property/🎛️sets"];
 const expected=owners.flatMap(owner=>[`${owner}/📸️snapshot/⬅️before/🔣️.json`,`${owner}/📸️snapshot/➡️after/🔣️.json`]).sort();
 const snapshots=readdirSync(snapshotRoot,{recursive:true}).map(String).filter(path=>path.endsWith("/📸️snapshot/⬅️before/🔣️.json")||path.endsWith("/📸️snapshot/➡️after/🔣️.json")).sort();expect(snapshots).toEqual(expected);expect(snapshots.length).toBe(36);
 for(const path of snapshots){const raw=JSON.parse(readFileSync(new URL(path,snapshotRoot),"utf8"));expect({path,errors:validate(raw)?null:validate.errors}).toEqual({path,errors:null});expect(api.semioGraphJsonValue(api.parseSemioGraphJsonValue(raw))).toEqual(raw);}
 const invalid=structuredClone(vector.childSnapshot);invalid.nodes[0].width={bits:"INVALID"};expect(()=>api.parseSemioGraphJsonValue(invalid)).toThrow();invalid.nodes[0].width={bits:"0000000000000000",extra:true};expect(()=>api.parseSemioGraphJsonValue(invalid)).toThrow();
});
test("Rewriting all twenty-eight scenario owners declare complete before after and replacement child inputs",async()=>{
 const {parseFeature}=await import("../../../../🧰️framework/🔨️modules/🧪️test/🥒️gherkin/🟦️.ts");
 const feature=parseFeature(readFileSync(new URL("../../../../🧪️tests/♻️mutate-rewrite-1/🥒️.feature",trinityTests),"utf8"));
 expect(feature.errors).toEqual([]);expect(feature.scenarios.length).toBe(28);
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,trinityTests),"utf8")),ajv=semioSchemaAjvV1({allErrors:true});
 for(const relative of read("./🧫️fixtures/🌳️typed/🔣️.json").schemaDocuments)ajv.addSchema(read(relative));
 const parentSchema=ajv.compile(read("../../../../🧬️schema/🔣️.json")),childSchema=ajv.compile(read("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json"));
 const input=(row:typeof feature.scenarios[number],role:string)=>{const steps=row.steps.filter(step=>step.text.startsWith(role+" shared://"));expect({scenario:row.id,role,count:steps.length}).toEqual({scenario:row.id,role,count:1});const uri=steps[0]!.text.slice(role.length+1);return{uri,value:read("../../../../🧫️fixtures"+uri.slice("shared://".length))}};
 const children=new Map<string,any>();let pairs=0;
 const bind=(row:typeof feature.scenarios[number],parentRole:string,childRole:string)=>{const parent=input(row,parentRole),child=input(row,childRole);expect(parentSchema(parent.value)).toBe(true);expect(childSchema(child.value)).toBe(true);expect(parent.value.workingGraph.content.target.dialect).toEqual({artifactKind:"s.stdio.semio",standard:"v1",subset:"graph"});expect(parent.value.workingGraph.content.childId.length).toBeGreaterThan(0);expect(parent.value.workingGraph.content.target.artifactId.length).toBeGreaterThan(0);children.set(child.uri,child.value);pairs++;return parent.value};
 for(const row of feature.scenarios){
  if(row.id.startsWith("spec-vector-")){bind(row,"the committed before-rule","the committed before-child");bind(row,"the committed after-rule","the committed after-child");}
  else if(row.id==="identity-round-trip"){bind(row,"the real derived rule","the real derived child");bind(row,"the two-node ground-floor rule this case used to rest on","the two-node ground-floor child");}
  else{
   const parent=bind(row,"the real derived rule","the real derived child");
   const mutation=JSON.parse(row.steps.find(step=>step.docString!==undefined)!.docString!);
   if(mutation.mutation==="editWorkingGraph"){const child=input(row,"the replacement working child");expect(parentSchema({...parent,workingGraph:mutation.newWorkingGraph})).toBe(true);expect(childSchema(child.value)).toBe(true);children.set(child.uri,child.value);pairs++;}
  }
 }
 expect(pairs).toBe(40);expect(children.size).toBe(21);
 const api=await import("../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts");
 const sql=await import("../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts");
 await Promise.all(Array.from(children.values(),async child=>{
  const typed=api.parseSemioGraphJsonValue(child),db=Database.deserialize(await exportSqliteDatabase(await sql.semioGraphSnapshotToSqliteDatabase(typed)));
  try{expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(api.semioGraphJsonValue(await sql.semioGraphSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize())))).toEqual(child);}
  finally{db.close();}
 }));
});
