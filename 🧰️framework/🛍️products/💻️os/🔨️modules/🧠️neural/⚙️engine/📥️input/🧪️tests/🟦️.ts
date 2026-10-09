import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch,type Operation} from "fast-json-patch";
import engineContract from "../../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("original neural input merge agrees with independent JSONPatch",()=>{
 const domain=new Ajv({strict:true,allErrors:true});domain.addSchema(engineContract);const validateValue=domain.compile({$ref:engineContract.$id+"#/$defs/Value"});
 for(const row of fixture.cases){for(const value of [row.base,row.overlay,row.expected])expect(validateValue(value)).toBe(true);const base=structuredClone(row.base);const patch:Operation[]=Object.entries(row.overlay).map(([key,value])=>({op:"add",path:`/${key.replaceAll("~","~0").replaceAll("/","~1")}`,value}));expect(applyPatch(base,patch,true,false).newDocument).toEqual(row.expected);expect(row.base).toEqual(fixture.cases.find(item=>item.id===row.id)!.base);}
 for(const row of fixture.plans){expect(validateValue(row.input)).toBe(true);const members=row.channels.filter(name=>name!=="*").map(name=>((row.input as Record<string,{$schema?:string}>)[name]??{}).$schema??"");const validators=row.signatures.map(signature=>new Ajv({strict:true}).compile({type:"array",items:{type:"string"},const:signature}));const exact=validators.findIndex((validate,index)=>row.signatures[index].length>0&&validate(members));expect(exact<0?row.signatures.findIndex(signature=>signature.length===0):exact).toBe(row.selected);}
 console.log("[DEBUG] Original Neural input domainValueAjv=true wholeCorpusAuthority=false JSONPatch=3 exactSignatureAjv=3 nativeConservation=unqualified");
});
