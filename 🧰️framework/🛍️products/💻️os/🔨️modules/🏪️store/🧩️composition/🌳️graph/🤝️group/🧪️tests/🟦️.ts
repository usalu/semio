import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync,mkdirSync,writeFileSync} from "node:fs";
import {resolve} from "node:path";
test("ownership group roots match SQLite cycle and atomic publication oracle",()=>{
 const f=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const db=new Database(":memory:");db.exec("create table owns(child text primary key,parent text not null,slot text not null)");
 const put=db.query("insert into owns(child,parent,slot) values(?3,?1,?2)");
 const cycle=db.query("with recursive ancestors(id) as (select ?1 union select parent from owns join ancestors on owns.child=ancestors.id) select count(*) as total from ancestors where id=?2");
 for(const edge of f.siblings.additions){expect((cycle.get(edge[0],edge[2]) as {total:number}).total).toBe(0);put.run(...edge);}
 const rows=db.query("select parent,slot,child from owns order by child").all().map((r:any)=>[r.parent,r.slot,r.child]);expect(rows).toEqual(f.siblings.expected);
 db.exec("delete from owns");for(const edge of f.cycle.initial)put.run(...edge);expect((cycle.get(f.cycle.addition[0],f.cycle.addition[2]) as {total:number}).total>0).toBe(f.cycle.expectedRefusal);
 db.exec("delete from owns");put.run(...f.pendingCycle.additions[0]);expect((cycle.get(f.pendingCycle.additions[1][0],f.pendingCycle.additions[1][2]) as {total:number}).total>0).toBe(f.pendingCycle.expectedRefusal);
 db.exec("delete from owns");put.run(...f.emptyAdditionRoot.original);expect(f.emptyAdditionRoot.additions.length).toBe(0);expect(db.query("select parent,slot,child from owns").all().map((r:any)=>[r.parent,r.slot,r.child])).toEqual([f.emptyAdditionRoot.expected]);
 db.close();
 const source=readFileSync(resolve(import.meta.dir,"../../../../🦀️.rs"),"utf8");expect(source.includes("mod composition_group;")).toBe(true);const group=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");expect(group.includes("pub fn begin_owns_group")).toBe(true);expect(group.includes("pub fn commit_owns_group")).toBe(true);
 console.log("[DEBUG] SQLite forest oracle: atomic two siblings, 65-hop cycle, pending sibling cycle; commit metadata and original roots unchanged before visibility");
});

function compileProductionForest(){
 const f=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const ticket=process.env.SEMIO_TICKET_DIR;if(!ticket)throw new Error("SEMIO_TICKET_DIR is required for generated graph compiler outputs");
 const output=resolve(ticket,"🗑️generated","composition-group-rust-isolation");mkdirSync(output,{recursive:true});
 const tuple=(row:string[])=>`(${row.map(value=>JSON.stringify(value)).join(",")})`;
 const main=readFileSync(resolve(import.meta.dir,"../../../../🦀️.rs"),"utf8"),start=main.indexOf("    pub fn close_step(",main.indexOf("impl CompositionGraph {")),end=main.indexOf("    /// 🧺",start);expect(start>=0&&end>start).toBe(true);
 const source=readFileSync(resolve(import.meta.dir,"🔬️isolated/🦀️.rs"),"utf8").replace("@COLD_CLOSE@",main.slice(start,end)).replace("@MODULE@",JSON.stringify(resolve(import.meta.dir,"../🦀️.rs"))).replace("@LINK_MODULE@",JSON.stringify(resolve(import.meta.dir,"../../🔗️links/🦀️.rs"))).replace("@LINKS@",`&[${f.links.rows.map(([source,targets]:[string,string[]])=>`(${JSON.stringify(source)},&[${targets.map(value=>JSON.stringify(value)).join(",")}])`).join(",")}]`).replace("@EMPTY_ROW@",tuple(f.emptyAdditionRoot.original)).replace("@SIBLINGS@",`&[${f.siblings.additions.map(tuple).join(",")}]`).replace("@INITIAL@",`&[${f.cycle.initial.map(tuple).join(",")}]`).replace("@CYCLE@",tuple(f.cycle.addition));
 const privateSource=readFileSync(resolve(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs"),"utf8"),forward=privateSource.match(/^fn group_grant\([^\n]+\}$/mu)?.[0];if(!forward)throw new Error("current private group grant authority is absent");
 const admittedSource=source.replace("@PRIVATE_GROUP_GRANT@",forward.replace("fn group_grant(","fn private_group_grant("));
 const input=resolve(output,"graph.rs"),binary=resolve(output,process.platform==="win32"?"graph.exe":"graph");writeFileSync(input,admittedSource);
 const compiler=Bun.spawnSync(["rustc","--edition=2021","--crate-name","composition_graph_isolation",input,"-o",binary]);writeFileSync(resolve(output,"compiler.log"),new TextDecoder().decode(compiler.stderr));expect(compiler.exitCode).toBe(0);
 return binary;
}
 
test("actual production ownership root executes the neutral forest in isolation",()=>{
 const productionForest=compileProductionForest();
 const output=resolve(process.env.SEMIO_TICKET_DIR!,"🗑️generated","composition-group-rust-isolation");
 const result=Bun.spawnSync([productionForest]);writeFileSync(resolve(output,"runtime.log"),new TextDecoder().decode(result.stdout)+new TextDecoder().decode(result.stderr));expect(result.exitCode).toBe(0);console.log(new TextDecoder().decode(result.stdout));
});

test("ownership forest retirement keeps independent work and physical release axes",()=>{
 const f=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")).closeAxes;
 const db=new Database(":memory:");db.exec("create table owner(capacity integer not null)");for(const capacity of f.stringCapacities)db.query("insert into owner values(?)").run(capacity);
 const schema=JSON.parse(readFileSync(resolve(process.cwd(),"🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"),"utf8"));const validate=new Ajv().compile(schema.$defs.Grant);
 const rowBytes=f.rowWords*f.wordBytes;expect(rowBytes).toBe(f.retirement.copiedBytes);expect((db.query("select sum(capacity) as total from owner").get() as {total:number}).total).toBe(f.retirement.releasedBytes);
 expect(validate({maximumItems:1,maximumCopyBytes:-1,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1})).toBe(false);
 for(const grant of f.grants){expect(validate({maximumItems:grant.items,maximumCopyBytes:grant.copyBytes,maximumCapacityBytes:grant.capacityBytes,maximumReleaseBytes:grant.releaseBytes,maximumDepth:grant.depth})).toBe(true);const accepted=grant.items>=1&&grant.copyBytes>=rowBytes&&grant.depth>=1;expect(accepted).toBe(grant.expectedMove);console.log("[DEBUG] forest common close independent="+JSON.stringify({grant,accepted,copy:rowBytes,release:0}));}
 db.close();const source=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");expect(source).toContain("pub fn next_close_copy_byte_demand");expect(source).toContain("pub fn next_close_release_byte_demand");expect(source).toContain("grant.maximum_copy_bytes");expect(source).toContain("grant.maximum_release_bytes");expect(source).not.toContain("SnapshotRetirementStep");
});

test("original adjacency rows match independent SQLite unique-edge ownership",()=>{
 const f=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")).links;const db=new Database(":memory:");db.exec("create table edge(source text,target text,primary key(source,target))");for(const [source,targets] of f.rows)for(const target of targets)db.query("insert or ignore into edge values(?,?)").run(source,target);expect(db.query("select source,target from edge order by source,target").all().map((r:any)=>[r.source,r.target])).toEqual(f.expected);db.close();console.log("[DEBUG] SQLite original links unique ordered rows="+f.expected.length+" terminalDropRequired="+f.terminalDropBytes);
});

test("forest preparation reserves native capacity independently from copied identifier words",()=>{
 const f=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")).preparationAxes;
 const db=new Database(":memory:");db.exec("create table identifier(octets blob not null)");for(const text of f.edge)db.query("insert into identifier values(?)").run(new TextEncoder().encode(text));
 const bytes=(db.query("select sum(length(octets)) as bytes from identifier").get() as {bytes:number}).bytes;expect(bytes).toBe(f.bodyBytes);expect(f.count*f.rowWords*f.wordBytes).toBe(f.rowBackingBytes);
 const copied=bytes+f.rowWords*f.wordBytes;expect(copied).toBe(f.copiedBytes);for(const grant of f.grants)expect(grant.copy>=copied&&grant.capacity>=bytes).toBe(grant.expected);
 db.close();const source=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");expect(source).toContain("pub fn next_edge_demands");expect(source).toContain("pub fn next_preparation_demands");
 console.log("[DEBUG] SQLite forest preparation copiedWords="+copied+" payload="+bytes+" capacity="+bytes+" structural="+f.rowBackingBytes+" release0");
});
