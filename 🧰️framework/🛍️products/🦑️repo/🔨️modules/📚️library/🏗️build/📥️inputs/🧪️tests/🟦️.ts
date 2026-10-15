import {expect,test} from "bun:test";
import Ajv from "ajv";
import {minimatch} from "minimatch";
import {buildInputPathMatchesV1,compactSourceInputs} from "../🟨️.mjs";
import fixture from "./🧫️fixtures/🔣️.json";

test("source input compaction prepares exact patterns once per operation without changing lazy precedence",()=>{
  const validate=new Ajv({strict:true}).compile(fixture.contract);
  for(const row of fixture.matches){expect(minimatch(row.path,row.pattern,{dot:true,nonegate:true})).toBe(row.expected);expect(buildInputPathMatchesV1(row.path,row.pattern)).toBe(row.expected);}
  for(const row of fixture.cases){
    expect(validate(row),row.id).toBe(true);
    const normalize=(input:string)=>input.replaceAll("{projectRoot}",row.root).replaceAll("{workspaceRoot}/",""),exact=row.inputs.filter(input=>typeof input==="string"&&input.startsWith("{workspaceRoot}/")&&!/[!*?{}[\]]/u.test(input.slice("{workspaceRoot}/".length))) as string[],negatives=row.inputs.filter(input=>typeof input==="string"&&input.startsWith("!")&&!input.startsWith("!^")) as string[];
    expect(exact.every(path=>negatives.some(pattern=>minimatch(normalize(path),normalize(pattern.slice(1)),{dot:true,nonegate:true}))),row.id).toBe(true);
    for(let attempt=0;attempt<2;attempt++){
      const original=globalThis.RegExp,compiled:string[]=[];let result:unknown;
      globalThis.RegExp=new Proxy(original,{construct(target,args,newTarget){compiled.push(String(args[0]));return Reflect.construct(target,args,newTarget);}});
      try{result=compactSourceInputs(row.inputs,process.cwd(),row.root);}finally{globalThis.RegExp=original;}
      expect(result,row.id).toEqual(row.expected);
      expect(compiled.length,row.id+":"+attempt).toBe(row.compilations);
      if(!exact.length)expect(result).toBe(row.inputs);
    }
  }
  for(const row of fixture.errors)expect(()=>compactSourceInputs(row.inputs,process.cwd())).toThrow(row.message);
  console.log("[DEBUG] Source compaction operation-local compilation counts and lazy negation/path results agree with independent Ajv/minimatch");
});
