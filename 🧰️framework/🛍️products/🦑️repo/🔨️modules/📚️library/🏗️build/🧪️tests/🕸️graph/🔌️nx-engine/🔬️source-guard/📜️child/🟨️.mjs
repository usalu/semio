import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const [library,scratch,rows]=JSON.parse(process.argv[2]),api=await import(join(library,"🟨️.mjs")),identity=api.nxBuildDependencyImplementationV1(),results=[];
for(const [index,row] of rows.entries()){
 const path=join(library,row.source),copy=join(scratch,String(index));
 writeFileSync(copy,Buffer.concat([readFileSync(path),Buffer.from(row.mutation)]));
 globalThis.semioSourceMutationV1={path,mutation:row.mutation,copy};
 let refusal=null;try{api.nxBuildDependencyImplementationV1();}catch(error){refusal=error.message;}
 globalThis.semioSourceMutationV1=undefined;
 results.push({source:row.source,refusal,restored:api.nxBuildDependencyImplementationV1()===identity});
}
console.log(JSON.stringify(results));
