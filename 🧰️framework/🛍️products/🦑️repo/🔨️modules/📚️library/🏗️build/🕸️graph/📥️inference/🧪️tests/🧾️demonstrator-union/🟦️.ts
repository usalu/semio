/** 🧾️ Contract: the component closures of the demonstrator activation lanes (generator, energy, fem3d) together equal exactly the closure over the demonstrator roots, and the lanes named by `project.json` are exactly the lanes the activation gate derives. The independent oracle is a breadth-first search over `@iarna/toml`-parsed manifests. */
import{expect,test}from"bun:test";
import{execFileSync}from"node:child_process";
import{existsSync,readFileSync}from"node:fs";
import{join}from"node:path";
import iarna from"@iarna/toml";

const workspace=join(import.meta.dir,"../../../../../../../../../..");
const runtime=await import(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs"));
type Row={pluginId:string;dependsOn:string[];consumes:string[];contributes:string[];extends?:string;host?:unknown};
const components:Row[]=[],playgrounds:{variant:string;pluginId:string;app?:string}[]=[],oracleRows=new Map<string,Row>();
for(const path of execFileSync("git",["ls-files","-z","--","*Cargo.toml"],{cwd:workspace,maxBuffer:1<<28}).toString().split("\0")){
 if(!path||path.includes(".🧬semio")||!existsSync(join(workspace,path)))continue;
 const metadata=(Bun.TOML.parse(readFileSync(join(workspace,path),"utf8")) as any).package?.metadata;
 if(!metadata?.component?.package||!["plugin","extension"].includes(metadata.semio?.["component-kind"]))continue;
 const semio=metadata.semio,pluginId=(metadata.component.package as string).slice(6);
 components.push({pluginId,dependsOn:[...(semio.extends?[semio.extends]:[]),...(semio["depends-on"]??[])],consumes:semio.consumes??[],contributes:semio.contributes??[],extends:semio.extends,host:semio.host});
 const oracle=(iarna.parse(readFileSync(join(workspace,path),"utf8")) as any).package.metadata.semio;
 oracleRows.set(pluginId,{pluginId,dependsOn:[...(oracle.extends?[oracle.extends]:[]),...(oracle["depends-on"]??[])],consumes:oracle.consumes??[],contributes:oracle.contributes??[],extends:oracle.extends,host:oracle.host});
 for(const row of semio.playground??[])playgrounds.push({...row,pluginId});
}
const catalog=JSON.parse(readFileSync(join(workspace,"♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json"),"utf8"));
const rowOf=(variant:string)=>playgrounds.find(row=>row.variant===variant)!;
const oracleClosure=(roots:string[])=>{
 const seen=new Set<string>(),pending=roots.map(id=>({id,shallow:false})),contributors=new Map<string,string[]>();
 for(const row of oracleRows.values())for(const topic of row.contributes)contributors.set(topic,[...(contributors.get(topic)??[]),row.pluginId]);
 while(pending.length){const{id,shallow}=pending.pop()!;if(seen.has(id))continue;const row=oracleRows.get(id);if(!row)throw Error(`Unknown component ${id}`);seen.add(id);if(shallow)continue;for(const dependency of row.dependsOn)pending.push({id:dependency,shallow:row.extends===dependency});for(const topic of row.consumes)pending.push(...(contributors.get(topic)??[]).map(id=>({id,shallow:false})));}
 return[...seen].sort();
};
const variants=[...new Set<string>(catalog.panes.flatMap((pane:{variant:string;runtimeVariant:string})=>[pane.variant,pane.runtimeVariant]))];

test("every demonstrator runtime variant is a declared playground row",()=>{
 expect(variants.filter(variant=>!rowOf(variant))).toEqual([]);
});

test("the lane closures partition exactly the union closure of the demonstrator roots",()=>{
 const roots=[...new Set(variants.map(variant=>rowOf(variant).pluginId))],union=runtime.runtimeComponentClosure(components,roots);
 const lanes=["generator","energy","fem3d"].map(lane=>runtime.runtimeComponentClosure(components,[{id:rowOf(lane).pluginId,appScoped:rowOf(lane).app!==undefined}]) as string[]);
 expect(lanes.flat().sort()).toEqual(union);
 expect(new Set(lanes.flat()).size).toBe(union.length);
 expect(oracleClosure(roots)).toEqual(union);
});

test("the lanes the activation gate derives are the lanes project.json activates",()=>{
 const primary="generator",covered=new Set<string>(runtime.runtimeComponentClosure(components,[rowOf(primary).pluginId])),derived=[primary];
 for(const pane of catalog.panes){const id=rowOf(pane.runtimeVariant).pluginId;if(covered.has(id))continue;for(const member of runtime.runtimeComponentClosure(components,[id]))covered.add(member);derived.push(pane.runtimeVariant);}
 const project=JSON.parse(readFileSync(join(workspace,"♻️mit-bestand/🧺️demonstrator/📋️project.json"),"utf8"));
 const activated=(project.targets["activate-lanes-prototype"].dependsOn as string[]).map(id=>/:activate-(.+)-react-prototype$/u.exec(id)![1]);
 expect(activated.sort()).toEqual(derived.sort());
});
