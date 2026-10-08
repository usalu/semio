import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync,mkdirSync,writeFileSync} from "node:fs";
import {resolve} from "node:path";
test("ownership group roots match SQLite cycle and atomic publication oracle",()=>{
 const f=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/📐️schema.json"),"utf8"));expect(new Ajv({strict:true}).compile(schema)(f)).toBe(true);
 const db=new Database(":memory:");db.exec("create table owns(child text primary key,parent text not null,slot text not null)");
 const put=db.query("insert into owns(child,parent,slot) values(?3,?1,?2)");
 const cycle=db.query("with recursive ancestors(id) as (select ?1 union select parent from owns join ancestors on owns.child=ancestors.id) select count(*) as total from ancestors where id=?2");
 for(const edge of f.siblings.additions){expect((cycle.get(edge[0],edge[2]) as {total:number}).total).toBe(0);put.run(...edge);}
 const rows=db.query("select parent,slot,child from owns order by child").all().map((r:any)=>[r.parent,r.slot,r.child]);expect(rows).toEqual(f.siblings.expected);
 db.exec("delete from owns");for(const edge of f.cycle.initial)put.run(...edge);expect((cycle.get(f.cycle.addition[0],f.cycle.addition[2]) as {total:number}).total>0).toBe(f.cycle.expectedRefusal);
 db.exec("delete from owns");put.run(...f.pendingCycle.additions[0]);expect((cycle.get(f.pendingCycle.additions[1][0],f.pendingCycle.additions[1][2]) as {total:number}).total>0).toBe(f.pendingCycle.expectedRefusal);
 db.close();
 const source=readFileSync(resolve(import.meta.dir,"../../../../🦀️.rs"),"utf8");expect(source.includes("mod composition_group;")).toBe(true);const group=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");expect(group.includes("pub fn begin_owns_group")).toBe(true);expect(group.includes("pub fn commit_owns_group")).toBe(true);
 console.log("[DEBUG] Ajv/SQLite forest oracle: atomic two siblings, 65-hop cycle, pending sibling cycle; commit metadata and original roots unchanged before visibility");
});

function compileProductionForest(){
 const f=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const ticket=process.env.SEMIO_TICKET_DIR;if(!ticket)throw new Error("SEMIO_TICKET_DIR is required for generated graph compiler outputs");
 const output=resolve(ticket,"🗑️generated","composition-group-rust-isolation");mkdirSync(output,{recursive:true});
 const tuple=(row:string[])=>`(${row.map(value=>JSON.stringify(value)).join(",")})`;
 const main=readFileSync(resolve(import.meta.dir,"../../../../🦀️.rs"),"utf8"),start=main.indexOf("    pub fn close_step(",main.indexOf("impl CompositionGraph {")),end=main.indexOf("    /// 🧺",start);expect(start>=0&&end>start).toBe(true);
 const source=readFileSync(resolve(import.meta.dir,"🔬️isolated/🦀️.rs"),"utf8").replace("@COLD_CLOSE@",main.slice(start,end)).replace("@MODULE@",JSON.stringify(resolve(import.meta.dir,"../🦀️.rs"))).replace("@SIBLINGS@",`&[${f.siblings.additions.map(tuple).join(",")}]`).replace("@INITIAL@",`&[${f.cycle.initial.map(tuple).join(",")}]`).replace("@CYCLE@",tuple(f.cycle.addition));
 const input=resolve(output,"graph.rs"),binary=resolve(output,process.platform==="win32"?"graph.exe":"graph");writeFileSync(input,source);
 const compiler=Bun.spawnSync(["rustc","--edition=2021","--crate-name","composition_graph_isolation",input,"-o",binary]);writeFileSync(resolve(output,"compiler.log"),new TextDecoder().decode(compiler.stderr));expect(compiler.exitCode).toBe(0);
 return binary;
}
const productionForest=compileProductionForest();
test("actual production ownership root executes the neutral forest in isolation",()=>{
 const output=resolve(process.env.SEMIO_TICKET_DIR!,"🗑️generated","composition-group-rust-isolation");
 const result=Bun.spawnSync([productionForest]);writeFileSync(resolve(output,"runtime.log"),new TextDecoder().decode(result.stdout)+new TextDecoder().decode(result.stderr));expect(result.exitCode).toBe(0);console.log(new TextDecoder().decode(result.stdout));
});
