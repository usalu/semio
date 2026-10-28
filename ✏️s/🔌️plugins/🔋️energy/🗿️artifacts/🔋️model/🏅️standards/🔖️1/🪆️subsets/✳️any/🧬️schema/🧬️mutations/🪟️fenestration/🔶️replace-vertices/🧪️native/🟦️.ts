import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import recipeSchema from "../../../✨️derive/🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("aperture schema and independent SQL cross-product plane agree for finite original points and exact inverses",()=>{
 expect(new Ajv2020({strict:true}).compile(recipeSchema)(schema["x-semio-energy-operation"])).toBe(true);const valid=new Ajv({strict:false,strictNumbers:true}).compile(schema),db=new Database(":memory:");db.run("CREATE TABLE points(rank INTEGER PRIMARY KEY,x REAL,y REAL,z REAL)");const [a,b,c]=law.host.vertices_m;const u=b.map((v,i)=>v-a[i]),v=c.map((v,i)=>v-a[i]);const n=db.query<{x:number,y:number,z:number},number[]>("SELECT ?*?-?*? x,?*?-?*? y,?*?-?*? z").get(u[1],v[2],u[2],v[1],u[2],v[0],u[0],v[2],u[0],v[1],u[1],v[0])!;const norm=Math.sqrt(n.x*n.x+n.y*n.y+n.z*n.z);expect(norm).toBeGreaterThan(0);
 for(const test of law.cases){const points=test.value.map(p=>p.slice());if(test.nonfiniteAt)points[test.nonfiniteAt[0]][test.nonfiniteAt[1]]=NaN;const accepted=valid({mutation:"replaceFenestrationVertices",id:test.id,newVerticesM:points});db.run("DELETE FROM points");if(accepted)points.forEach((p,i)=>db.run("INSERT INTO points VALUES(?,?,?,?)",i,...p));const outside=accepted&&db.query<{found:number},number[]>("SELECT count(*) found FROM points WHERE abs((x-?)*?+(y-?)*?+(z-?)*?)/?>?").get(a[0],n.x,a[1],n.y,a[2],n.z,norm,law.toleranceM)!.found>0;const outcome=test.id!==law.original.id?"missing":!accepted?"invalid":outside?"invalid-context":JSON.stringify(points)===JSON.stringify(law.original.vertices_m)?"no-op":"changed";expect(outcome).toBe(test.outcome);if(outcome==="changed"){expect(db.query<{x:number,y:number,z:number},[]>("SELECT x,y,z FROM points ORDER BY rank").all().map(p=>[p.x,p.y,p.z])).toEqual(points);db.run("DELETE FROM points");law.original.vertices_m.forEach((p,i)=>db.run("INSERT INTO points VALUES(?,?,?,?)",i,...p));expect(db.query<{x:number,y:number,z:number},[]>("SELECT x,y,z FROM points ORDER BY rank").all().map(p=>[p.x,p.y,p.z])).toEqual(law.original.vertices_m);}}
 db.close();
});
