/** 🧩️ Play release job order laws: the pages job follows the all-component success barrier and the bundle job follows every engine producer job the framework catalog declares. */
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import catalog from"../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/📇️catalog.json"with{type:"json"};
const jobs=():{id:string;root:string;command:readonly string[];after:readonly string[]}[]=>JSON.parse(readFileSync(resolve(import.meta.dir,"../../📋️project.json"),"utf8")).targets.build.metadata.semio.release.jobs;
test("neutral Play pages job follows the all-component success barrier and never the publish stage",()=>{
 const job=jobs().find(row=>row.id==="pages");expect(job).toBeDefined();expect(job?.after).toContain("components:complete");expect(job?.after).not.toContain("components:publish");
});

test("neutral Play bundle job reads after every engine producer job the framework catalog declares",()=>{
 const all=jobs(),bundle=all.find(row=>row.id==="bundle");expect(bundle).toBeDefined();
 for(const row of catalog.packages){const producer=all.find(job=>job.root===row.sourceOwner&&job.command.at(-1)==="wasm");expect(producer,row.id+" actual authored job").toBeDefined();expect(bundle?.after,row.id+" before bundle").toContain("job:"+producer?.id);}
});
