import {parse as parseGraphql,print as printGraphql,buildASTSchema,coerceInputValue,Kind} from "graphql";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import * as owner from "../../../../../../../../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {readdirSync} from "node:fs";
test("Rewriting editor initial owner preserves the independent nine-node six-edge Jack demo and original query",async()=>{
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,import.meta.url),"utf8")),initial=read("../../../../🧫️fixtures/🪆️child/🏢️initial/🔣️.json");
 const ajv=semioSchemaAjvV1({allErrors:true}),childSchema=read("../../🌳️typed/🪆️child/🔣️.json");ajv.addSchema(childSchema);const validate=ajv.compile(childSchema),initialSchema=ajv.compile(read("../../../../🧫️fixtures/🪆️child/🏢️initial/🧬️schema/🔣️.json"));
 expect(initialSchema(initial)).toBe(true);expect(initialSchema({...initial,opaqueJson:"{}"})).toBe(false);expect(initial.child).toEqual(read("../../../../🧫️fixtures/🪆️child/🏢️initial/🪆️content/🔣️.json"));
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
 const graph=await import("./../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🟦️.ts"),sql=await import("./../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts");
 const typed=graph.parseSemioGraphJsonValue(initial.child),database=Database.deserialize(await exportSqliteDatabase(await sql.semioGraphSnapshotToSqliteDatabase(typed)));
 try{expect((database.query("SELECT COUNT(*) AS count FROM semio_graph_node").get()as{count:number}).count).toBe(9);expect((database.query("SELECT COUNT(*) AS count FROM semio_graph_edge").get()as{count:number}).count).toBe(6);expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(graph.semioGraphJsonValue(await sql.semioGraphSnapshotFromSqliteDatabase(await importSqliteDatabase(database.serialize())))).toEqual(initial.child);}
 finally{database.close();}
});
test("Rewriting actual document retirement vectors own typed parent and every mutation",async()=>{const api=await import("../../../♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts");expect(()=>api.testRewritingDocumentRetirementOracle()).not.toThrow();});

import {parseRewritingDiff} from "../../../🔺️diff/🟦️.ts";
test("Rewriting Nakagin scenario parents retain complete separately owned typed Semio children",async()=>{
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,import.meta.url),"utf8"));
 const ajv=semioSchemaAjvV1({allErrors:true});
 for(const relative of read("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json").schemaDocuments)ajv.addSchema(read(relative));
 const parentSchema=ajv.compile(read("../../../🔣️.json"));
 const graphSchema=ajv.compile(read("../../🌳️typed/🪆️child/🔣️.json"));
 const graph=await import("./../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🟦️.ts");
 const graphSql=await import("./../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts");
 for(const[folder,id,nodes,edges,ports]of[
  ["","nakagin-rewriting-content",180,179,364],
  ["🏢️nakagin-ground-floor/","nakagin-ground-floor-rewriting-content",2,1,2]
 ]as const){
  const root="../../../../🧫️fixtures/♻️mutate-rewrite-1/"+folder,parent=read(root+"🔣️.snapshot.json");
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
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,import.meta.url),"utf8"));
 const contract=read("../../🧫️fixtures/🪆️child/🧵️lifetime/🔣️.json");
 const ajv=semioSchemaAjvV1({allErrors:true}),graphSpec=read("../../🌳️typed/🪆️child/🔣️.json");ajv.addSchema(graphSpec);
 const validate=ajv.compile(read("../../🧫️fixtures/🪆️child/🧵️lifetime/🧬️schema/🔣️.json"));
 expect(validate(contract)).toBe(true);
 expect(validate({...contract,bodyJson:"{}"})).toBe(false);
 const vector=read("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json"),graphSchema=ajv.getSchema(graphSpec.$id)!;
 const graph=await import("./../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🟦️.ts");
 const graphSql=await import("./../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts");
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

const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️sqlite/🔣️.json",import.meta.url),"utf8"));
const sql=readFileSync(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url),"utf8");
function value(input:any):unknown{switch(input.variant){case"null":return{kind:"null"};case"bool":return{kind:"bool",value:input.value};case"number":return{kind:"number",value:{bits:BigInt("0x"+input.bits)}};case"string":return{kind:"string",value:input.value};case"array":return{kind:"array",values:input.elements.map(value)};case"object":return{kind:"object",values:Object.fromEntries(input.members.map((row:any)=>[row.key,value(row.value)]))};default:throw Error("fixture variant");}}
export function specimen(word=fixture.binary64Bits[0]):unknown{const typed=JSON.parse(JSON.stringify({workingGraph:fixture.nativeCase.workingGraph,lhs:fixture.nativeCase.lhs,rhs:fixture.nativeCase.rhs}),(key,value)=>key==="bits"?BigInt("0x"+value):value);for(const parameter of typed.rhs.parameters)if(parameter.default.kind==="number")parameter.default.value.bits=BigInt("0x"+word);return{...typed,parameterBindings:Object.fromEntries(fixture.nativeCase.parameterBindings.map((row:any)=>[row.key,value(row.value)])),ruleLayout:Object.fromEntries(fixture.nativeCase.ruleLayout.map((row:any)=>[row.key,{x:{bits:BigInt("0x"+word)},y:{bits:BigInt("0x"+word)}}]))};}
test("Rewriting actual typed map delta delegates full scalar words to owning parsers",()=>{const number={kind:"number",value:{bits:0x7ff0000000000001n}};const point={x:{bits:0x8000000000000000n},y:{bits:0x7ff0000000000000n}};const input={parameterBindings:{entries:[{key:"",precondition:"any",operation:{kind:"set",value:number}}]},ruleLayout:{entries:[{key:"",precondition:"any",operation:{kind:"set",value:point}}]}};expect(parseRewritingDiff(input).parameterBindings).toEqual(input.parameterBindings as any);expect(parseRewritingDiff(input).ruleLayout).toEqual(input.ruleLayout as any);});
test("Rewriting canonical parent retains literal authored strings, six variants and exact words",()=>{for(const word of fixture.binary64Bits){const input=specimen(word);expect(owner.parseRewritingArtifact(input) as unknown).toEqual(input);}});
test("Rewriting shared handcrafted schema is independently valid SQLite",()=>{const db=new Database(":memory:");try{db.exec(sql);expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);for(const [table,width]of Object.entries(fixture.tableWidths))expect(db.query("PRAGMA table_info("+table+")").all().length).toBe(Number(width));}finally{db.close();}});
test("Rewriting actual declared JSON schema admits exact owned scalar words",()=>{const ajv=semioSchemaAjvV1({allErrors:true});for(const url of JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url),"utf8")).schemaDocuments)ajv.addSchema(JSON.parse(readFileSync(new URL(url,import.meta.url),"utf8")));const schema=JSON.parse(readFileSync(new URL("../../../🔣️.json",import.meta.url),"utf8"));const validate=ajv.compile(schema);const wire=JSON.parse(JSON.stringify(specimen(),(_,v)=>typeof v==="bigint"?v.toString(16).padStart(16,"0"):v));expect(validate(wire)).toBe(true);});
test("Rewriting published semantic facade retains complete parent through independent SQLite",async()=>{const api=owner as unknown as {rewritingSnapshotToSqliteDatabase:(v:unknown)=>Promise<unknown>;rewritingSnapshotFromSqliteDatabase:(v:unknown)=>Promise<unknown>};expect(typeof api.rewritingSnapshotToSqliteDatabase).toBe("function");const input=specimen();expect(await api.rewritingSnapshotFromSqliteDatabase(await api.rewritingSnapshotToSqliteDatabase(input))).toEqual(input);});

const project=(input:unknown,options:Record<string,unknown>={})=>owner.rewritingSnapshotToSqliteDatabase(input as owner.RewritingArtifact,options);
const restore=(database:SqliteDatabase,options:Record<string,unknown>={})=>owner.rewritingSnapshotFromSqliteDatabase(database,options);
async function oracle(input:unknown):Promise<Database>{return Database.deserialize(Buffer.from(await exportSqliteDatabase(await project(input))));}
async function edited(db:Database):Promise<SqliteDatabase>{return importSqliteDatabase(db.serialize());}
for(const word of fixture.binary64Bits)test("Rewriting independently reserialized complete IEEE word "+word,async()=>{const input=specimen(word),db=await oracle(input);try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);const row=(db.query("SELECT x_ieee754_bits,x_numeric_class,x FROM rewriting_layout LIMIT 1") as unknown as {safeIntegers(on:boolean):{get():{x_ieee754_bits:bigint;x_numeric_class:string;x:number|null}}}).safeIntegers(true).get();expect(BigInt.asUintN(64,row.x_ieee754_bits)).toBe(BigInt("0x"+word));const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+word));const number=view.getFloat64(0);if(Number.isNaN(number))expect(row.x).toBeNull();else if(Object.is(number,-0))expect(row.x_numeric_class).toBe("finite");db.exec("UPDATE jack_document SET name='independently edited literal'");const expected=owner.parseRewritingArtifact(input);expected.workingGraph.name="independently edited literal";expect(await restore(await edited(db))).toEqual(expected);}finally{db.close();}});
test("Rewriting complete independent SQL entity census",async()=>{const db=await oracle(specimen());try{for(const [table,count]of Object.entries(fixture.tableRowCounts))expect((db.query("SELECT COUNT(*) AS count FROM "+table).get() as {count:number}).count).toBe(Number(count));}finally{db.close();}});
const malformed=[
"INSERT INTO rewriting_document VALUES(999,1)",
"UPDATE rewriting_layout SET document_id=999 WHERE id=1",
"UPDATE rewriting_layout SET map_key=(SELECT map_key FROM rewriting_layout WHERE id=1) WHERE id=2",
"UPDATE rewriting_binding SET map_key=(SELECT map_key FROM rewriting_binding WHERE id=1) WHERE id=2",
"UPDATE rewriting_value SET variant='unknown' WHERE id=1",
"UPDATE rewriting_boolean SET value=2 WHERE id=1",
"DELETE FROM rewriting_number WHERE id=1",
"UPDATE rewriting_array_element SET ordinal=99 WHERE id=1",
"UPDATE rewriting_array_element SET value_id=array_id WHERE id=1",
"UPDATE rewriting_binding SET value_id=(SELECT value_id FROM rewriting_binding WHERE id=1) WHERE id=2",
"UPDATE rewriting_object_member SET ordinal=99 WHERE id=1",
"UPDATE rewriting_object_member SET map_key=(SELECT map_key FROM rewriting_object_member WHERE id=1) WHERE id=2",
"INSERT INTO rewriting_value VALUES(999,'null')",
"UPDATE rewriting_layout SET x_numeric_class='nan',x=42",
"UPDATE rewriting_number SET value_numeric_class='finite',value=42"
];
for(const mutation of malformed)test("Rewriting refuses independent malformed owner SQL: "+mutation,async()=>{const db=await oracle(specimen());try{db.exec(mutation);expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(restore(await edited(db))).rejects.toThrow();}finally{db.close();}});
test("Rewriting admits independently renumbered structural surrogate keys",async()=>{const input=specimen(),db=await oracle(input);try{db.exec("PRAGMA foreign_keys=OFF");for(const table of Object.keys(fixture.tableRowCounts)){db.exec("UPDATE "+table+" SET id=id+1000");for(const row of db.query("PRAGMA foreign_key_list("+table+")").all() as {from:string}[])if(row.from!=="id")db.exec("UPDATE "+table+" SET "+row.from+"="+row.from+"+1000");}expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);const database=await edited(db);expect(await restore(database)).toEqual(owner.parseRewritingArtifact(input));expect(await owner.validateRewritingSnapshotSqliteDialect(owner.parseRewritingArtifact(input),{artifactKind:"s.trinity.rewriting",standard:"1",subset:"*"},database)).toEqual([]);}finally{db.close();}});
test("Rewriting exact row/schema frontier precedes semantic ownership",async()=>{const input=specimen(),database=await project(input,{maxRows:89});expect(database.tables.reduce((n,t)=>n+t.rows.length,0)).toBe(89);await expect(project(input,{maxRows:88})).rejects.toThrow();await expect(restore(database,{maxRows:88})).rejects.toThrow();const bytes=Math.max(Buffer.byteLength(sql),database.tables.reduce((n,t)=>n+Buffer.byteLength(t.sql)+Buffer.byteLength(t.name),0));await expect(project(input,{maxSchemaBytes:bytes-1})).rejects.toThrow();expect(await restore(database,{maxSchemaBytes:bytes})).toEqual(owner.parseRewritingArtifact(input));});
test("Rewriting iterative retained property depth and interior cancellation",async()=>{let value:owner.PropertyValue={kind:"null"};for(let i=0;i<1024;i++)value={kind:"array",values:[value]};const input=owner.parseRewritingArtifact(specimen());input.parameterBindings={deep:value};const database=await project(input),output=await restore(database);let depth=0,node=output.parameterBindings.deep!;while(node.kind==="array"){depth++;node=node.values[0]!;}expect(depth).toBe(1024);for(const phase of["projectSnapshot","reconstructSnapshot"]){let observed=0;const controller=new AbortController();const options={signal:controller.signal,onProgress:(p:{phase:string;completed:number})=>{if(p.phase===phase&&p.completed>=256){observed=p.completed;controller.abort();}return true;}};await expect(phase==="projectSnapshot"?project(input,options):restore(database,options)).rejects.toThrow();expect(observed).toBeGreaterThanOrEqual(256);}});
test("Rewriting declared JSON word boundary is the actual published parent",()=>{const api=owner as unknown as {parseRewritingJsonValue:(v:unknown)=>unknown;rewritingToJsonValue:(v:owner.RewritingArtifact)=>unknown};expect(typeof api.parseRewritingJsonValue).toBe("function");for(const word of fixture.binary64Bits){const input=owner.parseRewritingArtifact(specimen(word)),wire=api.rewritingToJsonValue(input);expect(api.parseRewritingJsonValue(JSON.parse(JSON.stringify(wire)))).toEqual(input);}});
test("Rewriting committed snapshots use the closed canonical declared JSON scalar schema",()=>{const root=new URL("../../../../🧫️fixtures/🧬️mutations/",import.meta.url),ajv=semioSchemaAjvV1({allErrors:true});for(const url of JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url),"utf8")).schemaDocuments)ajv.addSchema(JSON.parse(readFileSync(new URL(url,import.meta.url),"utf8")));const validate=ajv.compile(JSON.parse(readFileSync(new URL("../../../🔣️.json",import.meta.url),"utf8")));const files=readdirSync(root,{recursive:true}).map(String).filter(p=>p.endsWith("/📸️snapshot/⬅️before/🔣️.json")||p.endsWith("/📸️snapshot/➡️after/🔣️.json"));expect(files.length).toBe(18);expect(files).toContain('🫳️drag-rule/🫳️moves/📸️snapshot/⬅️before/🔣️.json');expect(files).toContain('🫳️drag-rule/🫳️moves/📸️snapshot/➡️after/🔣️.json');for(const path of files)expect(validate(JSON.parse(readFileSync(new URL(path,root),"utf8")))).toBe(true);});

test("Rewriting typed domain contract closes rule and composed graph semantics",async()=>{
 const vector=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url)).json();const schema=await Bun.file(new URL("../../🌳️typed/🔣️.json",import.meta.url)).json();const ajv=semioSchemaAjvV1({allErrors:true});
 for(const url of vector.schemaDocuments)ajv.addSchema(JSON.parse(readFileSync(new URL(url,import.meta.url),"utf8")));const validate=ajv.compile(schema);
 expect(validate(vector.snapshot)).toBe(true);expect(validate({...vector.snapshot,lhsJson:"{}"})).toBe(false);expect(validate({...vector.snapshot,lhs:"{}"})).toBe(false);expect(validate({...vector.snapshot,lhs:{pattern:{leftVar:"a",leftKind:"Piece",unknown:0}}})).toBe(false);expect(validate({...vector.snapshot,rhs:{...vector.snapshot.rhs,parameters:[{name:"caption",kind:"json",default:{kind:"null"}}]}})).toBe(false);
 const db=new Database(":memory:");try{db.exec(await Bun.file(new URL("../../🌳️typed/🪶️sqlite/🗄️.sql",import.meta.url)).text());db.run("INSERT INTO rewriting_pattern VALUES(1,'lhs',0,?,?,?,?,?,?)",[...["leftVar","leftKind","edgeVar","edgeKind","rightVar","rightKind"].map(key=>vector.snapshot.lhs.pattern[key]??null)]);db.run("INSERT INTO rewriting_lhs VALUES(1,1,?)",[vector.snapshot.lhs.whereClause??null]);for(const [ordinal,assignment]of vector.snapshot.rhs.set.entries())db.run("INSERT INTO rewriting_assignment VALUES(?,1,?,?,?)",[ordinal+1,ordinal,assignment.var,assignment.prop]);for(const [ordinal,parameter]of vector.snapshot.rhs.parameters.entries())db.run("INSERT INTO rewriting_parameter VALUES(?,1,?,?,?)",[ordinal+1,ordinal,parameter.name,parameter.kind]);expect(db.query("SELECT p.left_var,p.edge_var,p.right_var,l.where_clause FROM rewriting_lhs l JOIN rewriting_pattern p ON p.id=l.pattern_id").get()).toEqual(vector.oracle.pattern);expect(db.query("SELECT ordinal,var,prop FROM rewriting_assignment ORDER BY ordinal").all()).toEqual(vector.oracle.assignments);expect(db.query("SELECT ordinal,name,kind FROM rewriting_parameter ORDER BY ordinal").all()).toEqual(vector.oracle.parameters);expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(vector.snapshot.workingGraph.content.target.dialect).toEqual({artifactKind:"s.stdio.semio",standard:"v1",subset:"graph"});expect(vector.snapshot.workingGraph.query).toBe(vector.oracle.query);expect(vector.snapshot.parameterBindings.literal.value).toBe(vector.oracle.literal);}finally{db.close()}
});

test("Rewriting published JSON boundary owns normalized rule and working graph fields",async()=>{
 const vector=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url)).json();const typed=JSON.parse(JSON.stringify(vector.snapshot),(key,value)=>key==="bits"?BigInt("0x"+value):value);expect(owner.parseRewritingJsonValue(vector.snapshot)).toEqual(typed);
});

test("Rewriting composed graph retains rich typed node port and edge ownership",async()=>{
 const vector=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url)).json();expect(vector.childSnapshot).toBeDefined();const schema=await Bun.file(new URL("../../🌳️typed/🪆️child/🔣️.json",import.meta.url)).json(),ajv=semioSchemaAjvV1({allErrors:true}),validate=ajv.compile(schema);expect(validate(vector.childSnapshot)).toBe(true);expect(validate({...vector.childSnapshot,nodes:[{...vector.childSnapshot.nodes[0],bodyJson:"{}"}]})).toBe(false);expect(validate({...vector.childSnapshot,edges:[{...vector.childSnapshot.edges[0],metadataJson:"{}"}]})).toBe(false);
 const typed=JSON.parse(JSON.stringify(vector.childSnapshot),(key,value)=>key==="bits"?BigInt("0x"+value):value),api=await import("./../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts");const db=Database.deserialize(await exportSqliteDatabase(await api.semioGraphSnapshotToSqliteDatabase(typed)));
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

test("Rewriting published normalized SQL owns complete typed parent and inline Jack manifest",async()=>{
 const vector=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url)).json(),typed=owner.parseRewritingJsonValue(vector.snapshot),database=await project(typed);
 expect(database.tables.length).toBe(30);
 const db=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("SELECT p.left_var,p.edge_var,p.right_var,l.where_clause FROM rewriting_lhs l JOIN rewriting_pattern p ON p.id=l.pattern_id").get()).toEqual(vector.oracle.pattern);
  expect(db.query("SELECT ordinal,var,prop FROM rewriting_assignment ORDER BY ordinal").all()).toEqual(vector.oracle.assignments);
  expect(db.query("SELECT ordinal,name,kind FROM rewriting_parameter ORDER BY ordinal").all()).toEqual(vector.oracle.parameters);
  expect((db.query("SELECT c.child_id,c.artifact_kind,c.standard,c.subset FROM rewriting_document d JOIN jack_document j ON j.id=d.working_graph_id JOIN jack_content_child c ON c.document_id=j.id").get() as any)).toEqual({child_id:typed.workingGraph.content.childId,artifact_kind:"s.stdio.semio",standard:"v1",subset:"graph"});
  expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(typed);
 }finally{db.close();}
});

test("Rewriting inline manifest owns all seven declared value type families",async()=>{
 const vector=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url)).json(),typed=owner.parseRewritingJsonValue(vector.snapshot);
 expect(typed.workingGraph.manifest.nodeKinds.length).toBe(1);
 const properties=typed.workingGraph.manifest.nodeKinds[0]!.properties;
 expect(properties.map(value=>value.valueType.kind)).toEqual(["boolean","integer","decimal","text","list","any","schema"]);
 expect(properties.map(value=>value.kind)).toEqual(["data","data","data","data","data","data","derived"]);
 const db=Database.deserialize(await exportSqliteDatabase(await project(typed)));
 try{
  expect(db.query("SELECT p.name,p.property_kind,v.variant,p.expression FROM jack_node_property p JOIN jack_value_type v ON v.id=p.value_type_id ORDER BY p.ordinal").all()).toEqual(vector.manifestOracle.properties);
  expect(db.query("SELECT s.schema FROM jack_node_property p JOIN jack_value_type_schema s ON s.value_type_id=p.value_type_id").get()).toEqual({schema:vector.manifestOracle.schema});
  expect(db.query("SELECT v.variant FROM jack_node_property p JOIN jack_value_type_list l ON l.value_type_id=p.value_type_id JOIN jack_value_type v ON v.id=l.child_type_id").get()).toEqual({variant:"text"});
  expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(typed);
 }finally{db.close();}
});

test("Rewriting committed mutation children are exact typed graphs whose edges join held nodes",()=>{
 const root=new URL("../../../../🧫️fixtures/🧬️mutations/",import.meta.url),files=readdirSync(root,{recursive:true}).map(String).filter(p=>p.endsWith("/📸️snapshot/⬅️before/🪆️child/🔣️.json")||p.endsWith("/📸️snapshot/➡️after/🪆️child/🔣️.json"));
 expect(files.length).toBe(18);
 const ajv=semioSchemaAjvV1({allErrors:true}),validate=ajv.compile(JSON.parse(readFileSync(new URL("../../🌳️typed/🪆️child/🔣️.json",import.meta.url),"utf8")));
 for(const path of files){const child=JSON.parse(readFileSync(new URL(path,root),"utf8"));expect(validate(child)).toBe(true);const ids=new Set(child.nodes.map((node:any)=>node.id.value));for(const edge of child.edges){expect(ids.has(edge.source.value)).toBe(true);expect(ids.has(edge.target.value)).toBe(true);}}
});
test("Rewriting rule layout closed ownership contract keeps literal sorted slots and words",async()=>{
 const input=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️sqlite/🗂️layout/🔣️.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️sqlite/🗂️layout/🧬️schema/🔣️.json",import.meta.url),"utf8"));
 const validate=semioSchemaAjvV1({allErrors:true}).compile(schema);expect(validate(input)).toBe(true);
 for(const example of input.cases){const source=owner.parseRewritingArtifact(specimen());source.ruleLayout=Object.fromEntries(example.members.map((row:any)=>[row.key,{x:{bits:BigInt("0x"+row.xBits)},y:{bits:BigInt("0x"+row.yBits)}}]));const db=await oracle(source);try{const rows=(db.query("SELECT map_key,x_ieee754_bits,y_ieee754_bits FROM rewriting_layout ORDER BY map_key COLLATE BINARY") as any).safeIntegers(true).all();expect(rows.map((row:any)=>[row.map_key,BigInt.asUintN(64,row.x_ieee754_bits).toString(16).padStart(16,"0"),BigInt.asUintN(64,row.y_ieee754_bits).toString(16).padStart(16,"0")])).toEqual(example.expected);expect((await restore(await edited(db))).ruleLayout).toEqual(source.ruleLayout);}finally{db.close();}}
});

test("Jack published inference propagates retained child refusal and reads full raw graph",async()=>{
 const contract=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/💡️inference/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/💡️inference/🧬️schema/🔣️.json",import.meta.url)).json();expect(semioSchemaAjvV1({allErrors:true}).compile(schema)(contract)).toBe(true);
 const vector=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url)).json(),parent=owner.parseRewritingJsonValue(vector.snapshot).workingGraph,raw=JSON.parse(JSON.stringify(vector.childSnapshot),(key,value)=>key==="bits"?BigInt("0x"+value):value);
 const api=await import("../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🟦️.ts") as unknown as {inferJackTopology:(parent:unknown,children:ReadonlyMap<string,unknown>)=>unknown};expect(typeof api.inferJackTopology).toBe("function");
 const save=(value:unknown)=>JSON.stringify(value,(_,value)=>typeof value==="bigint"?value.toString():value);
 for(const example of contract.cases){const child={...raw,edges:example.edgeOrdinals.map((ordinal:number)=>raw.edges[ordinal])},before=save(child),db=new Database(":memory:");try{db.exec("CREATE TABLE node(id INTEGER PRIMARY KEY,literal TEXT NOT NULL UNIQUE); CREATE TABLE edge(source INTEGER NOT NULL REFERENCES node(id),target INTEGER NOT NULL REFERENCES node(id))");for(const[index,node]of child.nodes.entries())db.run("INSERT INTO node VALUES(?,?)",[index+1,node.id.value]);for(const edge of child.edges)db.run("INSERT INTO edge SELECT s.id,t.id FROM node s,node t WHERE s.literal=? AND t.literal=?",[edge.source.value,edge.target.value]);const rows=db.query("WITH RECURSIVE walk(id,depth) AS (SELECT id,0 FROM node n WHERE NOT EXISTS(SELECT 1 FROM edge e WHERE e.target=n.id) UNION ALL SELECT e.target,w.depth+1 FROM walk w JOIN edge e ON e.source=w.id) SELECT n.literal,MAX(w.depth) AS depth FROM walk w JOIN node n ON n.id=w.id GROUP BY n.id ORDER BY depth,n.literal COLLATE BINARY").all() as {literal:string;depth:number}[];expect(rows.map(row=>row.literal)).toEqual(example.expected.topoOrder);expect(Object.fromEntries(rows.map(row=>[row.literal,row.depth]))).toEqual(example.expected.depth);expect(api.inferJackTopology(parent,new Map([[parent.content.childId,child]]))).toEqual(example.expected);expect(save(child)).toBe(before);}finally{db.close();}}
 const refused=(parent:unknown,children:ReadonlyMap<string,unknown>)=>{try{api.inferJackTopology(parent,children);throw Error("accepted refused child");}catch(error){expect((error as {kind:string}).kind).toBe(contract.refusalKind);}};
 refused(parent,new Map());refused({...parent,content:{...parent.content,target:{...parent.content.target,dialect:{artifactKind:"s.stdio.semio",standard:"v1",subset:"base"}}}},new Map([[parent.content.childId,raw]]));refused(parent,new Map([[parent.content.childId,{...raw,edges:[{...raw.edges[0],target:{value:"absent"}}]}]]));
});

test("Jack published flattened positions retain typed child and refuse invalid offset family",async()=>{
 const contract=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/💡️inference/🎛️flat/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/💡️inference/🎛️flat/🧬️schema/🔣️.json",import.meta.url)).json();expect(semioSchemaAjvV1({allErrors:true}).compile(schema)(contract)).toBe(true);
 const vector=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url)).json(),parent=owner.parseRewritingJsonValue(vector.snapshot).workingGraph,raw=JSON.parse(JSON.stringify(vector.childSnapshot),(key,value)=>key==="bits"?BigInt("0x"+value):value);
 const api=await import("../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🟦️.ts") as unknown as {inferJackFlatPosition:(parent:unknown,children:ReadonlyMap<string,unknown>)=>unknown};expect(typeof api.inferJackFlatPosition).toBe("function");
 for(const example of contract.cases){const child={...raw,edges:[{...raw.edges[0],properties:example.properties}]},before=JSON.stringify(child,(_,value)=>typeof value==="bigint"?value.toString():value),db=new Database(":memory:");try{db.exec("CREATE TABLE offset(id TEXT PRIMARY KEY,u REAL NOT NULL,v REAL NOT NULL)");db.run("INSERT INTO offset VALUES('a',0,0)");db.run("INSERT INTO offset SELECT 'b',u+?,v+? FROM offset WHERE id='a'",[Number(example.properties.find((entry:any)=>entry.key==="u")?.value.lexeme??0),Number(example.properties.find((entry:any)=>entry.key==="v")?.value.lexeme??0)]);const expected=Object.fromEntries((db.query("SELECT id,u,v FROM offset ORDER BY id COLLATE BINARY").all() as any[]).map(({id,u,v})=>[id,{u,v}]));expect(expected).toEqual(example.expected);expect(api.inferJackFlatPosition(parent,new Map([[parent.content.childId,child]]))).toEqual({positions:expected});expect(JSON.stringify(child,(_,value)=>typeof value==="bigint"?value.toString():value)).toBe(before);}finally{db.close();}}
 try{api.inferJackFlatPosition(parent,new Map([[parent.content.childId,{...raw,edges:[{...raw.edges[0],properties:[contract.refusal]}]}]]));throw Error("accepted invalid typed offset");}catch(error){expect((error as {kind:string}).kind).toBe(contract.refusalKind);}
});

test("Rewriting composed working slot declares the actual Semio child owner",()=>{
 const vector=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url),"utf8"));
 const parent=vector.snapshot.workingGraph,child=vector.childSnapshot;
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE slot(name TEXT PRIMARY KEY,kind TEXT NOT NULL); CREATE TABLE retained_child(slot TEXT NOT NULL REFERENCES slot(name),child_id TEXT NOT NULL,artifact_id TEXT NOT NULL,kind TEXT NOT NULL,standard TEXT NOT NULL,subset TEXT NOT NULL,PRIMARY KEY(slot,child_id));");
  for(const source of ["../../../🔣️.json","../../🔣️.json"]){
   const declared=JSON.parse(readFileSync(new URL(source,import.meta.url),"utf8")).properties.workingGraph;
   expect(declared["x-semio-child-kind"]).toBe(parent.content.target.dialect.artifactKind);
   db.run("INSERT OR REPLACE INTO slot VALUES('workingGraph',?)",[declared["x-semio-child-kind"]]);
   const address=parent.content;
   db.run("INSERT OR REPLACE INTO retained_child VALUES('workingGraph',?,?,?,?,?)",[address.childId,address.target.artifactId,address.target.dialect.artifactKind,address.target.dialect.standard,address.target.dialect.subset]);
   const row=db.query("SELECT c.child_id,c.artifact_id,c.kind,c.standard,c.subset FROM retained_child c JOIN slot s ON s.name=c.slot AND s.kind=c.kind").get() as any;
   expect(row).toEqual({child_id:address.childId,artifact_id:address.target.artifactId,kind:"s.stdio.semio",standard:"v1",subset:"graph"});
   expect(child.schema).toBe(row.kind+"."+row.subset);
   expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  }
 }finally{db.close();}
});

test("Rewriting committed sparse deltas own typed fields and exact inverse assets",()=>{
 const root=new URL("../../../../🧫️fixtures/🧬️mutations/",import.meta.url),ajv=semioSchemaAjvV1({allErrors:true});
 const vector=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url),"utf8"));
 for(const url of vector.schemaDocuments)ajv.addSchema(JSON.parse(readFileSync(new URL(url,import.meta.url),"utf8")));
 ajv.addSchema(JSON.parse(readFileSync(new URL("../../../🔣️.json",import.meta.url),"utf8")));
 ajv.addSchema(JSON.parse(readFileSync(new URL("../../../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/🔣️.json",import.meta.url),"utf8")));
 const validate=ajv.compile(JSON.parse(readFileSync(new URL("../../../🔺️diff/🔣️.json",import.meta.url),"utf8")));
 const files=readdirSync(root,{recursive:true}).map(String).filter(path=>path.endsWith("/🔺️diff/🔣️.json"));
 expect(files.length).toBe(9);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE delta(case_name TEXT PRIMARY KEY,role TEXT NOT NULL,payload TEXT NOT NULL CHECK(json_valid(payload)));");
  for(const path of files){
   const diff=JSON.parse(readFileSync(new URL(path,root),"utf8"));
   expect(validate(diff)).toBe(true);
   expect(Object.keys(diff).sort()).toEqual(["lhs","parameterBindings","rhs","ruleLayout","workingGraph"]);
   const after=JSON.parse(readFileSync(new URL(path.replace("/🔺️diff/🔣️.json","/📸️snapshot/➡️after/🔣️.json"),root),"utf8"));
   for(const role of ["lhs","rhs","workingGraph"]){if(diff[role]!==null){db.run("INSERT INTO delta VALUES(?,?,?)",[path,role,JSON.stringify(diff[role])]);expect(diff[role]).toEqual(after[role]);}}
  }
  for(const row of db.query("SELECT case_name,role,payload FROM delta ORDER BY case_name COLLATE BINARY").all() as {case_name:string;role:string;payload:string}[]){
   const after=JSON.parse(readFileSync(new URL(row.case_name.replace("/🔺️diff/🔣️.json","/📸️snapshot/➡️after/🔣️.json"),root),"utf8"));expect(JSON.parse(row.payload)).toEqual(after[row.role]);
  }
 }finally{db.close();}
});

test("Rewriting published mutation union accepts all nine typed committed inputs",()=>{
 const root=new URL("../../../../🧫️fixtures/🧬️mutations/",import.meta.url),ajv=semioSchemaAjvV1({allErrors:true}),read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,import.meta.url),"utf8"));
 for(const relative of read("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json").schemaDocuments)ajv.addSchema(read(relative));
 ajv.addSchema(read("../../../🔣️.json"));
 const declared=read("../../../🧬️mutations/🔣️.json");
 const schemas=readdirSync(new URL("../../../🧬️mutations/",import.meta.url),{recursive:true}).map(String).filter(path=>path.endsWith("/🧬️schema/🔣️.json"));
 expect(schemas.length).toBe(9);for(const path of schemas)ajv.addSchema(read("../../../🧬️mutations/"+path));
 const validate=ajv.compile(declared),files=readdirSync(root,{recursive:true}).map(String).filter(path=>path.endsWith("/🦠️mutation/🔣️.json"));
 expect(files.length).toBe(9);
 for(const path of files){const input=JSON.parse(readFileSync(new URL(path,root),"utf8"));expect(validate(input)).toBe(true);expect(validate({...input,legacyBody:"{}"})).toBe(false);expect(JSON.parse(JSON.stringify(input))).toEqual(input);}
});

test("Rewriting GraphQL body inputs own full typed rules and composed parent",()=>{
 const read=(relative:string)=>readFileSync(new URL(relative,import.meta.url),"utf8");
 const roots=["../../../🔗️.graphql","../../../🧬️mutations/🔗️.graphql","../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql","../../../../../../../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🌱️value/🧬️schema/🔗️.graphql","../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔗️.graphql","../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔗️.graphql","../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔗️.graphql"];
 const documents=roots.map(relative=>parseGraphql(read(relative)));
 const types=new Map(documents.flatMap(document=>document.definitions).filter((node:any)=>node.name).map((node:any)=>[node.name.value,node]));
 for(const [slug,type,field,payload]of [["👈️edit-lhs","EditLhsInput","newLhs","LhsInput"],["👉️edit-rhs","EditRhsInput","newRhs","RhsInput"],["🖼️edit-before-fixture","EditBeforeFixtureInput","newWorkingGraph","JackSnapshotInput"]]){
  const leaf:any=parseGraphql(read("../../../🧬️mutations/"+slug+"/🔗️.graphql")).definitions[0];
  expect(leaf.fields.map((item:any)=>item.name.value)).toEqual([field]);
  expect(printGraphql(leaf.fields[0].type)).toBe(payload+"!");
  const aggregate:any=types.get(type);expect(printGraphql(aggregate)).toBe(printGraphql(leaf));
 }
 const definitions=documents.flatMap(document=>document.definitions).filter(node=>node.kind==="InputObjectTypeDefinition"||node.kind==="EnumTypeDefinition"||node.kind==="ScalarTypeDefinition");
 const graph=buildASTSchema({kind:Kind.DOCUMENT,definitions});
 const vector=JSON.parse(read("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json"));
 const property=(value:any):any=>{switch(value.kind){case"null":return{null:"NULL"};case"bool":return{boolean:value.value};case"number":return{number:value.value};case"string":return{string:value.value};case"array":return{array:value.values.map(property)};case"object":return{object:Object.entries(value.values).map(([key,value])=>({key,value:property(value)}))};default:throw Error("fixture property family");}};
 const valueType=(value:any):any=>value.kind==="list"?{list:valueType(value.of)}:value.kind==="schema"?{schema:value.of}:{scalar:value.kind.toUpperCase()};
 const kind=(value:any)=>({...value,properties:value.properties.map((item:any)=>({...item,kind:item.kind.toUpperCase(),valueType:valueType(item.valueType)}))});
 const parent={...vector.snapshot.workingGraph,manifest:{nodeKinds:vector.snapshot.workingGraph.manifest.nodeKinds.map(kind),edgeKinds:vector.snapshot.workingGraph.manifest.edgeKinds.map(kind),portKinds:vector.snapshot.workingGraph.manifest.portKinds.map(kind)}};
 const rhs={...vector.snapshot.rhs,set:vector.snapshot.rhs.set.map((item:any)=>({...item,value:property(item.value)})),parameters:vector.snapshot.rhs.parameters.map((item:any)=>({...item,kind:item.kind.toUpperCase(),default:property(item.default)}))};
 const accepted=(name:string,input:unknown)=>{const output=coerceInputValue(input,graph.getType(name) as any);expect(output).toBeDefined();expect(output).toEqual(input);};
 accepted("EditLhsInput",{newLhs:vector.snapshot.lhs});accepted("EditRhsInput",{newRhs:rhs});accepted("EditBeforeFixtureInput",{newWorkingGraph:parent});
 for(const sample of[{kind:"null"},{kind:"bool",value:false},{kind:"number",value:{bits:"7ff8000000000011"}},{kind:"string",value:"quote\\\" NUL\u0000世界😀"},{kind:"array",values:[{kind:"null"}]},{kind:"object",values:{"":{kind:"bool",value:true}}}])accepted("PropertyValueInput",property(sample));
 for(const name of["PropertyValueInput","ValueTypeInput"])expect((graph.getType(name) as any).isOneOf).toBe(true);
 for(const [name,input]of[["EditLhsInput",{newLhs:"{}"}],["EditRhsInput",{newRhs:"{}"}],["EditBeforeFixtureInput",{newWorkingGraph:"{}"}],["PropertyValueInput",{string:"literal",number:{bits:"0000000000000000"}}],["ValueTypeInput",{scalar:"TEXT",schema:"literal"}]] as const){expect(coerceInputValue(input,graph.getType(name) as any)).toBeUndefined();}

 const mutationRoot=new URL("../../../../🧫️fixtures/🧬️mutations/",import.meta.url);
 const mutations=readdirSync(mutationRoot,{recursive:true}).map(String).filter(path=>path.endsWith("/🦠️mutation/🔣️.json"));expect(mutations.length).toBe(9);
 expect((graph.getType("RewriteRuleMutationInput") as any).isOneOf).toBe(true);expect(Object.keys((graph.getType("RewriteRuleMutationInput") as any).getFields()).length).toBe(9);
 for(const path of mutations){
  const raw=JSON.parse(readFileSync(new URL(path,mutationRoot),"utf8")),{mutation,...input}=raw;
  if(input.newValue)input.newValue=property(input.newValue);
  if(input.newRhs)input.newRhs={...input.newRhs,set:input.newRhs.set.map((item:any)=>({...item,value:property(item.value)})),parameters:input.newRhs.parameters.map((item:any)=>({...item,kind:item.kind.toUpperCase(),default:property(item.default)}))};
  if(input.newWorkingGraph)input.newWorkingGraph={...input.newWorkingGraph,manifest:{nodeKinds:input.newWorkingGraph.manifest.nodeKinds.map(kind),edgeKinds:input.newWorkingGraph.manifest.edgeKinds.map(kind),portKinds:input.newWorkingGraph.manifest.portKinds.map(kind)}};
  const type=mutation[0].toUpperCase()+mutation.slice(1)+"Input";
  const leaf:any=parseGraphql(read("../../../🧬️mutations/"+path.split("/")[0]+"/🔗️.graphql")).definitions[0];expect(printGraphql(types.get(type) as any)).toBe(printGraphql(leaf));
  accepted(type,input);accepted("RewriteRuleMutationInput",{[mutation]:input});
 }
 expect(coerceInputValue({editLhs:{newLhs:vector.snapshot.lhs},editRhs:{newRhs:rhs}},graph.getType("RewriteRuleMutationInput") as any)).toBeUndefined();

 expect((graph.getType("JackSnapshotInput") as any).getFields().camera.type.toString()).toBe("CameraInput!");
 expect(Object.keys((graph.getType("ManifestNodeKindInput") as any).getFields())).toEqual(["name","properties","portKinds"]);
 const artifact=parseGraphql(read("../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql"));
 const artifactTypes=artifact.definitions.filter((node:any)=>node.name).map((node:any)=>node.name.value);
 expect(artifactTypes).toEqual(["JackArtifact"]);
 const artifactRoot:any=artifact.definitions[0],snapshotRoot:any=types.get("JackSnapshot");
 expect(artifactRoot.fields.map((field:any)=>[field.name.value,printGraphql(field.type)])).toEqual(snapshotRoot.fields.map((field:any)=>[field.name.value,printGraphql(field.type)]));
 expect(read("../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql")).toContain('# import Manifest, Camera from');
});

test("Rewriting Proto field roles retain typed bodies and complete Jack manifests",()=>{
 const read=(relative:string)=>readFileSync(new URL(relative,import.meta.url),"utf8");
 for(const [slug,name,field,type]of [["👈️edit-lhs","EditLhs","new_lhs","semio.s.trinity.rewriting.artifact.Lhs"],["👉️edit-rhs","EditRhs","new_rhs","semio.s.trinity.rewriting.artifact.Rhs"],["🖼️edit-before-fixture","EditBeforeFixture","new_working_graph","semio.s.trinity.jack.snapshot.JackSnapshot"]]){
  const source=read("../../../🧬️mutations/"+slug+"/🛰️.proto"),body=source.match(new RegExp("message\\s+"+name+"\\s*\\{([^}]+)\\}"))![1]!;
  expect(body.trim().replace(/\s+/g," ")).toBe(type+" "+field+" = 1;");
  expect(read("../../../🧬️mutations/🛰️.proto")).toContain(type+" "+field+" = 1;");
 }
 const jack=read("../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto");
 const fields=(name:string)=>jack.match(new RegExp("message\\s+"+name+"\\s*\\{([^}]+)\\}"))![1]!.trim().replace(/\s+/g," ");
 expect(fields("Camera")).toBe("semio.framework.value.Binary64Word x = 1; semio.framework.value.Binary64Word y = 2; semio.framework.value.Binary64Word zoom = 3;");
 expect(fields("ManifestNodeKind")).toBe("string name = 1; repeated ManifestProperty properties = 2; repeated string port_kinds = 3;");
 expect(fields("ManifestEdgeKind")).toBe("string name = 1; repeated ManifestProperty properties = 2;");
 expect(fields("ManifestPortKind")).toBe("string name = 1; string direction = 2; repeated ManifestProperty properties = 3;");
 expect(fields("ManifestProperty")).toContain("semio.framework.value.ValueType value_type = 3;");
 const value=read("../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🛰️.proto");
 expect(value).toContain("message Binary64Word { fixed64 bits = 1; }");
 const db=new Database(":memory:",{safeIntegers:true});
 try{db.exec("CREATE TABLE exact_word(bits INTEGER NOT NULL) STRICT");
  for(const hex of fixture.binary64Bits){const bits=BigInt("0x"+hex),bytes=new DataView(new ArrayBuffer(8));bytes.setBigUint64(0,bits,true);db.run("INSERT INTO exact_word VALUES(?)",[BigInt.asIntN(64,bytes.getBigUint64(0,true))]);const row=db.query("SELECT bits FROM exact_word ORDER BY rowid DESC LIMIT 1").get() as {bits:bigint};expect(BigInt.asUintN(64,row.bits)).toBe(bits);expect(bytes.getBigUint64(0,true).toString(16).padStart(16,"0")).toBe(hex);}
 }finally{db.close();}
});

test("Rewriting actual Semio child public JSON codec preserves all words and intrinsic families",async()=>{
 const api=await import(new URL("../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🟦️.ts",import.meta.url).href) as {parseSemioGraphJsonValue:(value:unknown)=>any;semioGraphJsonValue:(value:any)=>unknown};
 expect(Object.hasOwn(api,"parseSemioGraphJsonValue")).toBe(true);expect(Object.hasOwn(api,"semioGraphJsonValue")).toBe(true);
 const vector=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url)).json(),validate=semioSchemaAjvV1({allErrors:true}).compile(await Bun.file(new URL("../../🌳️typed/🪆️child/🔣️.json",import.meta.url)).json());
 const neutral=await Bun.file(new URL("../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🧫️fixtures/🔣️json/🔣️.json",import.meta.url)).json();
 const contract=await Bun.file(new URL("../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🧫️fixtures/🔣️json/🧬️schema/🔣️.json",import.meta.url)).json();
 const fixtureAjv=semioSchemaAjvV1({allErrors:true});fixtureAjv.addSchema(await Bun.file(new URL("../../🌳️typed/🪆️child/🔣️.json",import.meta.url)).json());expect(fixtureAjv.compile(contract)(neutral)).toBe(true);
 for(const word of fixture.binary64Bits){
  const raw=structuredClone(vector.childSnapshot);raw.nodes[0].properties=structuredClone(neutral.properties);for(const node of raw.nodes){node.position={x:{bits:word},y:{bits:word}};node.width={bits:word};node.height={bits:word};}
  expect(validate(raw)).toBe(true);const decoded=api.parseSemioGraphJsonValue(raw);for(const node of decoded.nodes){expect(node.position.x.bits).toBe(BigInt("0x"+word));expect(node.position.y.bits).toBe(BigInt("0x"+word));expect(node.width.bits).toBe(BigInt("0x"+word));expect(node.height.bits).toBe(BigInt("0x"+word));}
  expect(api.semioGraphJsonValue(decoded)).toEqual(raw);
  const kinds=decoded.nodes[0].properties.map((entry:any)=>entry.value.kind);expect(kinds).toEqual(["null","bool","int","float","str","bytes","list","map","ref"]);
  const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+word));expect(view.getBigUint64(0)).toBe(decoded.nodes[0].width.bits);
 }
 const snapshotRoot=new URL("../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧫️fixtures/🧬️mutations/",import.meta.url);
 const owners=["✂️delete-edge/✂️removes","✋️drag-nodes/✋️drags","🌉️create-edge/🌉️connects","🏗️create-node/🔎️appends","🏷️rename-node/🏷️renames","📍move-node/📍️moves","📐resize-node/📐️resizes","🔌add-node-port/🔌️inserts","🔚remove-node-port/🔚️detaches","🔧change-node-kind/🔧️retypes","🖍️change-node-label/🔤️relabels","🗑️delete-node/🚫️removes","➕add-edge-property/⚖️inserts","➕add-node-property/⚖️inserts","➖remove-edge-property/➖️detaches","➖remove-node-property/➖️detaches","🎚️set-edge-property/🎚️sets","🎛️set-node-property/🎛️sets"];
 const expected=owners.flatMap(owner=>[`${owner}/📸️snapshot/⬅️before/🔣️.json`,`${owner}/📸️snapshot/➡️after/🔣️.json`]).sort();
 const snapshots=readdirSync(snapshotRoot,{recursive:true}).map(String).filter(path=>path.endsWith("/📸️snapshot/⬅️before/🔣️.json")||path.endsWith("/📸️snapshot/➡️after/🔣️.json")).sort();expect(snapshots).toEqual(expected);expect(snapshots.length).toBe(36);
 for(const path of snapshots){const raw=JSON.parse(readFileSync(new URL(path,snapshotRoot),"utf8"));expect({path,errors:validate(raw)?null:validate.errors}).toEqual({path,errors:null});expect(api.semioGraphJsonValue(api.parseSemioGraphJsonValue(raw))).toEqual(raw);}
 const invalid=structuredClone(vector.childSnapshot);invalid.nodes[0].width={bits:"INVALID"};expect(()=>api.parseSemioGraphJsonValue(invalid)).toThrow();invalid.nodes[0].width={bits:"0000000000000000",extra:true};expect(()=>api.parseSemioGraphJsonValue(invalid)).toThrow();
});

test("Rewriting retained child capture contract preserves exact publication and release authority",async()=>{
 const base=new URL("../../🧫️fixtures/🪆️child/👁️capture/",import.meta.url);
 const contract=await Bun.file(new URL("🔣️.json",base)).json(),schema=await Bun.file(new URL("🧬️schema/🔣️.json",base)).json();
 expect(semioSchemaAjvV1({allErrors:true}).compile(schema)(contract)).toBe(true);
 const vector=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json",import.meta.url)).json();
 const graph=structuredClone(vector.childSnapshot),before=structuredClone(graph),db=new Database(":memory:");
 try{
  db.exec("PRAGMA foreign_keys=ON;CREATE TABLE entry(id INTEGER PRIMARY KEY,slot TEXT NOT NULL,child_id TEXT NOT NULL,kind TEXT NOT NULL,standard TEXT NOT NULL,subset TEXT NOT NULL,owner_schema TEXT NOT NULL,label TEXT NOT NULL);CREATE TABLE live(slot TEXT PRIMARY KEY,entry_id INTEGER NOT NULL REFERENCES entry(id));CREATE TABLE capture(id INTEGER PRIMARY KEY,entry_id INTEGER NOT NULL REFERENCES entry(id));");
  for(const [index,owner]of contract.owners.entries())db.run("INSERT INTO entry VALUES(?,?,?,?,?,?,?,?)",[index+1,contract.slot,contract.childId,contract.dialect.artifactKind,contract.dialect.standard,contract.dialect.subset,vector.childSnapshot.schema,owner.label]);
  db.run("INSERT INTO live VALUES(?,1)",[contract.slot]);
  db.run("INSERT INTO capture SELECT 1,entry_id FROM live WHERE slot=?",[contract.slot]);
  expect(contract.ownerSchema).toBe(vector.childSnapshot.schema);db.run("UPDATE live SET entry_id=2 WHERE slot=?",[contract.slot]);
  const captured=db.query("SELECT e.label FROM entry e JOIN capture c ON c.entry_id=e.id").get() as {label:string};expect(captured.label).toBe(contract.owners[0].label);
  expect(graph).toEqual(before);expect(db.query("SELECT COUNT(*) n FROM capture WHERE entry_id=1").get()).toEqual({n:contract.blockedCaptureCount});
  for(const refusal of contract.refusals){const slot=refusal==="missingSlot"?"absent":contract.slot,id=refusal==="missingChild"?"absent":contract.childId,kind=refusal==="wrongDialect"?"wrong":contract.dialect.artifactKind,ownerSchema=refusal==="wrongType"?"wrong":contract.ownerSchema;const found=db.query("SELECT e.id FROM entry e JOIN live l ON l.entry_id=e.id WHERE l.slot=? AND e.child_id=? AND e.kind=? AND e.owner_schema=?").get(slot,id,kind,ownerSchema);expect(found).toBeNull();}
  expect(()=>db.run("DELETE FROM entry WHERE id=1")).toThrow();db.run("DELETE FROM capture WHERE id=1");expect(db.query("SELECT COUNT(*) n FROM capture WHERE entry_id=1").get()).toEqual({n:contract.releasedCaptureCount});
  db.run("DELETE FROM entry WHERE id=1");expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("SELECT label FROM entry").all()).toEqual([{label:contract.owners[1].label}]);expect(graph).toEqual(before);
 }finally{db.close()}
});
test("Rewriting exact twenty-eight scenario roster owns all eighteen closed typed inline payloads",async()=>{
 const {parseFeature}=await import("../../../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/🥒️gherkin/🟦️.ts");
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,import.meta.url),"utf8"));
 const feature=parseFeature(readFileSync(new URL("../../../../🧪️tests/♻️mutate-rewrite-1/🥒️.feature",import.meta.url),"utf8"));
 const kinds=["edit-before-fixture","edit-lhs","edit-rhs","change-parameter-binding","remove-parameter-binding","change-rule-layout-point","remove-rule-layout-point","drag-rule-nodes","set-rule-layout-points"];
 expect(feature.errors).toEqual([]);
 expect(feature.scenarios.map(row=>row.id)).toEqual(["mutate","inverse","spec-vector"].flatMap(group=>kinds.map(kind=>group+"-"+kind)).concat("identity-round-trip"));
 const ajv=semioSchemaAjvV1({allErrors:true});
 for(const relative of read("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json").schemaDocuments)ajv.addSchema(read(relative));
 ajv.addSchema(read("../../../🔣️.json"));
 const schemas=readdirSync(new URL("../../../🧬️mutations/",import.meta.url),{recursive:true}).map(String).filter(path=>path.endsWith("/🧬️schema/🔣️.json"));
 expect(schemas.length).toBe(9);for(const path of schemas)ajv.addSchema(read("../../../🧬️mutations/"+path));
 const validate=ajv.compile(read("../../../🧬️mutations/🔣️.json"));
 const inline=feature.scenarios.filter(row=>row.id.startsWith("mutate-")||row.id.startsWith("inverse-"));
 expect(inline.length).toBe(18);
 const invalid=inline.flatMap(row=>{const payloads=row.steps.filter(step=>step.docString!==undefined);expect(payloads.length).toBe(1);return validate(JSON.parse(payloads[0]!.docString!))?[]:[row.id]});
 expect(invalid).toEqual([]);
 const childSchema=ajv.compile(read("../../🌳️typed/🪆️child/🔣️.json"));
 const declaredChild="shared://♻️mutate-rewrite-1/🪆️content/🔣️.snapshot.json";
 for(const row of inline){
  expect(row.steps.some(step=>step.text==="the real derived child "+declaredChild)).toBe(true);
  const child=read("../../../../🧫️fixtures/♻️mutate-rewrite-1/🪆️content/🔣️.snapshot.json");expect(childSchema(child)).toBe(true);
  const payload=JSON.parse(row.steps.find(step=>step.docString!==undefined)!.docString!);
  if(payload.mutation==="editBeforeFixture"){
   const uri="shared://♻️mutate-rewrite-1/🔁️core-only/🪆️content/🔣️.snapshot.json";
   expect(row.steps.some(step=>step.text==="the replacement working child "+uri)).toBe(true);
   expect(payload.newWorkingGraph.content).toEqual({childId:"nakagin-core-only-content",target:{artifactId:"nakagin-core-only-content",dialect:{artifactKind:"s.stdio.semio",standard:"v1",subset:"graph"}}});
   expect(payload.newWorkingGraph.camera).toEqual({x:{bits:"0000000000000000"},y:{bits:"0000000000000000"},zoom:{bits:"3ff0000000000000"}});
   const replacement=read("../../../../🧫️fixtures/♻️mutate-rewrite-1/🔁️core-only/🪆️content/🔣️.snapshot.json");expect(childSchema(replacement)).toBe(true);expect(replacement).toEqual({schema:"s.stdio.semio.graph",nodes:[],edges:[]});
  }
 }
});

test("Rewriting all twenty-eight scenario owners declare complete before after and replacement child inputs",async()=>{
 const {parseFeature}=await import("../../../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/🥒️gherkin/🟦️.ts");
 const feature=parseFeature(readFileSync(new URL("../../../../🧪️tests/♻️mutate-rewrite-1/🥒️.feature",import.meta.url),"utf8"));
 expect(feature.errors).toEqual([]);expect(feature.scenarios.length).toBe(28);
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,import.meta.url),"utf8")),ajv=semioSchemaAjvV1({allErrors:true});
 for(const relative of read("../../🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json").schemaDocuments)ajv.addSchema(read(relative));
 const parentSchema=ajv.compile(read("../../../🔣️.json")),childSchema=ajv.compile(read("../../🌳️typed/🪆️child/🔣️.json"));
 const input=(row:typeof feature.scenarios[number],role:string)=>{const steps=row.steps.filter(step=>step.text.startsWith(role+" shared://"));expect({scenario:row.id,role,count:steps.length}).toEqual({scenario:row.id,role,count:1});const uri=steps[0]!.text.slice(role.length+1);return{uri,value:read("../../../../🧫️fixtures/"+uri.slice("shared://".length))}};
 const children=new Map<string,any>();let pairs=0;
 const bind=(row:typeof feature.scenarios[number],parentRole:string,childRole:string)=>{const parent=input(row,parentRole),child=input(row,childRole);expect(parentSchema(parent.value)).toBe(true);expect(childSchema(child.value)).toBe(true);expect(parent.value.workingGraph.content.target.dialect).toEqual({artifactKind:"s.stdio.semio",standard:"v1",subset:"graph"});expect(parent.value.workingGraph.content.childId.length).toBeGreaterThan(0);expect(parent.value.workingGraph.content.target.artifactId.length).toBeGreaterThan(0);children.set(child.uri,child.value);pairs++;return parent.value};
 for(const row of feature.scenarios){
  if(row.id.startsWith("spec-vector-")){bind(row,"the committed before-rule","the committed before-child");bind(row,"the committed after-rule","the committed after-child");}
  else if(row.id==="identity-round-trip"){bind(row,"the real derived rule","the real derived child");bind(row,"the two-node ground-floor rule this case used to rest on","the two-node ground-floor child");}
  else{
   const parent=bind(row,"the real derived rule","the real derived child");
   const mutation=JSON.parse(row.steps.find(step=>step.docString!==undefined)!.docString!);
   if(mutation.mutation==="editBeforeFixture"){const child=input(row,"the replacement working child");expect(parentSchema({...parent,workingGraph:mutation.newWorkingGraph})).toBe(true);expect(childSchema(child.value)).toBe(true);children.set(child.uri,child.value);pairs++;}
  }
 }
 expect(pairs).toBe(40);expect(children.size).toBe(21);
 const api=await import("./../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🟦️.ts");
 const sql=await import("./../../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts");
 await Promise.all(Array.from(children.values(),async child=>{
  const typed=api.parseSemioGraphJsonValue(child),db=Database.deserialize(await exportSqliteDatabase(await sql.semioGraphSnapshotToSqliteDatabase(typed)));
  try{expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(api.semioGraphJsonValue(await sql.semioGraphSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize())))).toEqual(child);}
  finally{db.close();}
 }));
});

test("Rewriting member membership identity remains separate from its exact addressed target",async()=>{
 const contract=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪆️child/🧵️lifetime/🔣️.json",import.meta.url),"utf8"));
 expect(contract.childId).toBe("logical !@/\0引用😀");expect(contract.childId).not.toBe(contract.target.artifactId);
 const schema=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪆️child/🧵️lifetime/🧬️schema/🔣️.json",import.meta.url),"utf8")),ajv=semioSchemaAjvV1({allErrors:true});ajv.addSchema(JSON.parse(readFileSync(new URL("../../🌳️typed/🪆️child/🔣️.json",import.meta.url),"utf8")));const validate=ajv.compile(schema);
 expect(validate(contract)).toBe(true);expect(validate({...contract,childId:null})).toBe(false);
 const closure=await import("../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🟦️.ts");closure.testOwnedDocumentClosureOracle();
 const db=new Database(":memory:");try{db.exec("CREATE TABLE member (parent TEXT NOT NULL,slot TEXT NOT NULL,child_id TEXT NOT NULL,target TEXT NOT NULL UNIQUE,PRIMARY KEY(parent,slot,child_id));");db.run("INSERT INTO member VALUES (?,?,?,?)",["parent",contract.slot,contract.childId,contract.target.artifactId]);expect(db.query("SELECT target FROM member WHERE parent=? AND slot=? AND child_id=?").get("parent",contract.slot,contract.childId)).toEqual({target:contract.target.artifactId});expect(()=>db.run("INSERT INTO member VALUES (?,?,?,?)",["other","other","other",contract.target.artifactId])).toThrow();}finally{db.close();}
 const plugin=readFileSync(new URL("../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",import.meta.url),"utf8");
 const admission=plugin.slice(plugin.indexOf("fn declared_child_reference("),plugin.indexOf("async fn prepare_child_member("));expect(admission).toContain("fields.artifact_id");expect(admission).toContain("fields.child_id == child_id");expect(admission).not.toContain("artifact_id: child_id.clone()");
 const store=readFileSync(new URL("../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",import.meta.url),"utf8"),sync=store.slice(store.indexOf("pub async fn sync_member<P: ArtifactRefs>"),store.indexOf("/// 🧹️ One grant-sized retirement turn"));
 expect(sync).not.toContain("child.child_id != child.target.artifact_id");expect(sync).toContain("next_owns.insert(child.target.artifact_id");
});
