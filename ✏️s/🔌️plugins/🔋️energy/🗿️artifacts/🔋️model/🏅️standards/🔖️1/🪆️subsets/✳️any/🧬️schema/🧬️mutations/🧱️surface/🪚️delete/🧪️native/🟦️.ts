import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import law from "./🧫️fixtures/🔣️.json";
import surfaceSchema from "../../🟫️create/🧬️schema/🔣️.json";
import fenSchema from "../../../🪟️fenestration/🪟️create/🧬️schema/🔣️.json";
import verticesSchema from "../../../🪟️fenestration/🔶️replace-vertices/🧬️schema/🔣️.json";
import pairSchema from "../../../🔲️boundary/🤝️connect/🧬️schema/🔣️.json";
test("surface cascade SQL oracle preserves restrictions, child ownership, complete inverse payloads and every original rank",()=>{
 for(const c of law.targetCases)expect(c.kind==="Pair"?[String(c.payload.surfaceAId),String(c.payload.surfaceBId)]:[String(c.payload.id)]).toEqual(c.segments);
 const ajv=new Ajv({strict:false}),validators={Surface:ajv.compile(surfaceSchema),Fenestration:ajv.compile(fenSchema),Vertices:ajv.compile(verticesSchema),Pair:ajv.compile(pairSchema)},db=new Database(":memory:");
 db.run("CREATE TABLE rows(kind TEXT,rank INTEGER,payload TEXT,PRIMARY KEY(kind,rank))");const groups={surfaces:law.existingRows,fenestrations:law.model.fenestrations,adjacency_pairs:law.model.adjacency_pairs};for(const[k,rows]of Object.entries(groups))rows.forEach((r,i)=>db.run("INSERT INTO rows VALUES(?,?,?)",k,i,JSON.stringify(r)));
 const read=(kind:string)=>db.query<{payload:string},[string]>("SELECT payload FROM rows WHERE kind=? ORDER BY rank").all(kind).map(r=>JSON.parse(r.payload));
 const insert=(kind:string,rank:number,payload:unknown)=>{db.run("UPDATE rows SET rank=-rank-1 WHERE kind=? AND rank>=?",kind,rank);db.run("UPDATE rows SET rank=-rank WHERE kind=? AND rank<0",kind);db.run("INSERT INTO rows VALUES(?,?,?)",kind,rank,JSON.stringify(payload));};
 for(const c of law.cases){db.run("BEGIN");const id=c.payload.id,found=db.query("SELECT 1 FROM rows WHERE kind='surfaces' AND json_extract(payload,'$.id')=?").get(id),restricted=db.query("SELECT 1 FROM rows WHERE kind='surfaces' AND json_extract(payload,'$.outside_boundary_condition.Interzone')=?").get(id);expect(found?restricted?"referenced":"changed":"missing").toBe(c.outcome);if(c.outcome==="changed"){
 db.run("DELETE FROM rows WHERE kind='surfaces' AND json_extract(payload,'$.id')=?",id);db.run("DELETE FROM rows WHERE kind='fenestrations' AND json_extract(payload,'$.surface_id')=?",id);db.run("DELETE FROM rows WHERE kind='adjacency_pairs' AND (json_extract(payload,'$.surface_a_id')=? OR json_extract(payload,'$.surface_b_id')=?)",id,id);const after=[...read("surfaces").map(r=>r.id),...read("fenestrations").map(r=>r.id),...read("adjacency_pairs").flatMap(r=>[r.surface_a_id,r.surface_b_id])];expect(after).toEqual(c.after);
 for(const kind of Object.keys(groups)){const rows=read(kind);db.run("DELETE FROM rows WHERE kind=?",kind);rows.forEach((r,i)=>db.run("INSERT INTO rows VALUES(?,?,?)",kind,i,JSON.stringify(r)));}
 expect(c.inverseSteps.length).toBe(c.inverseCount);for(const item of c.inverseSteps.toReversed()){const[k,p]=Object.entries(item)[0] as [keyof typeof validators,any];expect(validators[k]({mutation:{Surface:"createSurface",Fenestration:"createFenestration",Vertices:"replaceFenestrationVertices",Pair:"connectSurfaces"}[k],...p})).toBe(true);if(k==="Vertices"){db.run("UPDATE rows SET payload=json_set(payload,'$.vertices_m',json(?)) WHERE kind='fenestrations' AND json_extract(payload,'$.id')=?",JSON.stringify(p.newVerticesM),p.id);}else if(k==="Pair"){insert("adjacency_pairs",p.index,{surface_a_id:p.surfaceAId,surface_b_id:p.surfaceBId});}else{const schema=k==="Surface"?surfaceSchema:fenSchema;const original=Object.fromEntries(schema["x-semio-energy-row-birth"].fields.map(f=>[f.field,f.storage==="empty-vector"?[]:f.storage==="surface-boundary"?p.boundary==="Interzone"?{Interzone:p.interzoneSurfaceId}:p.boundary:p[f.input.replace(/_([a-z])/g,(_:string,c:string)=>c.toUpperCase())]]));insert(k==="Surface"?"surfaces":"fenestrations",p.index,original);}}
 for(const[k,rows]of Object.entries(groups))expect(read(k)).toEqual(rows);
 }db.run("ROLLBACK");}db.close();
});
