/** 🧩️ Play release job order laws: every driver job depends only on jobs (the driver finishes them before any Cargo pass, so the pages follow the driver) and the bundle job follows every engine producer job the framework catalog declares. */
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import catalog from"../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/📇️catalog.json"with{type:"json"};
const jobs=():{id:string;root:string;command:readonly string[];after:readonly string[]}[]=>JSON.parse(readFileSync(resolve(import.meta.dir,"../../📋️project.json"),"utf8")).targets.build.metadata.semio.release.jobs;
test("neutral Play driver jobs depend only on jobs and leave the pages to the driver's completion",()=>{
 const all=jobs();expect(all.find(row=>row.id==="pages")).toBeUndefined();for(const row of all)for(const dependency of row.after)expect(dependency,row.id).toMatch(/^job:/u);
});

test("neutral Play bundle job reads after every engine producer job the framework catalog declares",()=>{
 const all=jobs(),bundle=all.find(row=>row.id==="bundle");expect(bundle).toBeDefined();
 for(const row of catalog.packages){const producer=all.find(job=>job.root===row.sourceOwner&&job.command.at(-1)==="wasm");expect(producer,row.id+" actual authored job").toBeDefined();expect(bundle?.after,row.id+" before bundle").toContain("job:"+producer?.id);}
});
