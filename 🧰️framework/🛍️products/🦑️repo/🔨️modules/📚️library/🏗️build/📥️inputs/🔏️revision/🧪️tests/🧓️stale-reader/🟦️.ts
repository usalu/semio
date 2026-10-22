/** 🧓️ Contract: a revision reader whose own implementation or schema was edited while it ran refuses with the stale-reader error, whatever the edit did to the file (appended, shortened, replaced, removed), instead of a size-bound refusal that hides the cause. */
import {expect,test} from "bun:test";
import {appendFileSync,copyFileSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";

const source=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),schema=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url));
const load=async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"stale-reader-")),module=join(root,"revision","🟨️.mjs"),contract=join(root,"revision","🧬️schema","🔣️.json"),target=join(root,"target.txt");
 mkdirSync(join(root,"revision","🧬️schema"),{recursive:true});copyFileSync(source,module);copyFileSync(schema,contract);writeFileSync(target,"data");
 const loaded=await import(module+"?"+Math.random());
 return{root,module,contract,target,loaded};
};
const stale="Stale loaded Source revision reader; restart required";

test("an unchanged reader captures",async()=>{
 const{root,target,loaded}=await load();
 try{expect(loaded.captureBoundedSourceRevisionV1(target,1024).sha256).toMatch(/^[a-f0-9]{64}$/u);}finally{rmSync(root,{recursive:true,force:true});}
});
for(const[name,edit]of Object.entries({
 "bytes appended to the implementation":(row:{module:string})=>appendFileSync(row.module,"\n// edited\n".repeat(50)),
 "implementation shortened":(row:{module:string})=>writeFileSync(row.module,"export {};\n"),
 "implementation replaced by same-size bytes":(row:{module:string})=>{const{readFileSync}=require("node:fs");const text=readFileSync(row.module,"utf8");writeFileSync(row.module,text.slice(0,-1)+(text.endsWith("x")?"y":"x"));},
 "implementation removed":(row:{module:string})=>rmSync(row.module),
}))test("stale reader error when "+name,async()=>{
 const row=await load();
 try{edit(row as never);expect(()=>row.loaded.captureBoundedSourceRevisionV1(row.target,1024)).toThrow(stale);}finally{rmSync(row.root,{recursive:true,force:true});}
});
