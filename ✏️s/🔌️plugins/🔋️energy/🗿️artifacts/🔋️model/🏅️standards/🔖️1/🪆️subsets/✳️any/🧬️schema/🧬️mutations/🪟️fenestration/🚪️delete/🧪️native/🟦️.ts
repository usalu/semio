import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import law from "./🧫️fixtures/🔣️.json";
import createSchema from "../../🪟️create/🧬️schema/🔣️.json";
import replaceSchema from "../../🔶️replace-vertices/🧬️schema/🔣️.json";
test("aperture SQL deletion restores every original field by popping complete authored inverse intents",()=>{
 const create=new Ajv({strict:false}).compile(createSchema),replace=new Ajv({strict:false}).compile(replaceSchema),db=new Database(":memory:");db.run("CREATE TABLE apertures(rank INTEGER PRIMARY KEY,id INTEGER UNIQUE,payload TEXT)");law.existingRows.forEach((r,i)=>db.run("INSERT INTO apertures VALUES(?,?,?)",i,r.id,JSON.stringify(r)));
 for(const c of law.cases){if(c.outcome!=="changed")continue;db.run("BEGIN");db.run("DELETE FROM apertures WHERE id=?",c.payload.id);expect(db.query<{id:number},[]>("SELECT id FROM apertures ORDER BY rank").all().map(r=>r.id)).toEqual(c.after);expect(c.inverseSteps.length).toBe(c.inverseCount);for(const item of c.inverseSteps.toReversed()){if(item.Create){const p=item.Create;expect(create({mutation:"createFenestration",...p})).toBe(true);const original=Object.fromEntries(createSchema["x-semio-energy-row-birth"].fields.map(f=>[f.field,f.storage==="empty-vector"?[]:p[f.input.replace(/_([a-z])/g,(_:string,c:string)=>c.toUpperCase())]]));db.run("INSERT INTO apertures VALUES(?,?,?)",p.index,p.id,JSON.stringify(original));}else{const p=item.Vertices!;expect(replace({mutation:"replaceFenestrationVertices",...p})).toBe(true);db.run("UPDATE apertures SET payload=json_set(json(payload),'$.vertices_m',json(?)) WHERE id=?",JSON.stringify(p.newVerticesM),p.id);}}expect(db.query<{payload:string},[]>("SELECT payload FROM apertures ORDER BY rank").all().map(r=>JSON.parse(r.payload))).toEqual(law.existingRows);db.run("ROLLBACK");}db.close();
});
