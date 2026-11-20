import {writeFile} from "node:fs/promises";
import {join} from "node:path";
import {loadGenerationGraphV1} from "../../🟨️.mjs";
import {createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";

export type Info={phases?:Record<string,number>};

/** ✏️ Gives the following graph build actual edited owners. */
export async function edit(dir:string,count:number){
 await Bun.sleep(20);
 for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const carried=${at};\n`);
}

if(import.meta.main){
 const dir=process.argv[2];
 if(!dir)throw Error("Private graph test directory required");
 const first:Info[]=[],later:Info[]=[];
 await seed(dir,ringFiles(8));
 const cold=createMockHost(dir);
 await loadGenerationGraphV1({...cold.host,published:async(_generation:number,info:Info)=>{first.push(info);}} as never);
 await edit(dir,3);
 const mock=createMockHost(dir,{prefetch:async()=>{}});
 await loadGenerationGraphV1({...mock.host,builderIdle:async()=>false,published:async(_generation:number,info:Info)=>{later.push(info);}} as never);
 process.stdout.write(JSON.stringify({first,later,events:mock.events}));
}
