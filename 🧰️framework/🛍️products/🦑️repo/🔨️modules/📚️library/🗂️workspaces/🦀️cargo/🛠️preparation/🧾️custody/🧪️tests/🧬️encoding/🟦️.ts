import{test,expect}from"bun:test";
import{isAbsolute}from"node:path";
import{loadProductFunctionsSyncV1}from"../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts";
import Ajv from"ajv";
import schema from"../../🧬️schema/🔣️.json";
import cases from"./🔣️.json";
import{readOwnedCommandEncodingContextV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
const names=["annotateCargoPreparationProgramFailureV1","readCargoPreparationProgramContextV1"],functions=()=>loadProductFunctionsSyncV1(new URL("../../🟦️.ts",import.meta.url),names,{custodySchema:schema,isAbsolute,readOwnedCommandEncodingContextV1});
test("custody failure context owners exist",()=>expect(Object.keys(functions())).toEqual(names));
for(const row of cases.cases)test("bounded custody operation "+row.id,()=>{
 const {annotateCargoPreparationProgramFailureV1:annotate,readCargoPreparationProgramContextV1:read}=functions(),encoding={version:1,kind:"output-encoding",program:"/owned/bun",programTruncated:false,stream:"stdout",phase:"final",streamBytes:2,observedBytes:2},reason=new TypeError("decoder [owned-capture-encoding]"+JSON.stringify(encoding));Object.defineProperty(reason,"cause",{value:encoding});
 annotate(reason,row.program,row.operation);
 expect(reason.cause).toBe(encoding);expect(readOwnedCommandEncodingContextV1(reason.message)).toEqual(encoding);
 const context=read(reason.message+"\n"+"x".repeat(6000));
 if(row.accepted){expect(new Ajv({strict:true}).validate(schema.$defs.CargoPreparationProgramContextV1,context)).toBe(true);expect(context).toEqual({version:1,kind:"cargo-preparation-program",program:row.program,operation:row.operation});expect(Object.isFrozen(context)).toBe(true);}
 else expect(context).toBeUndefined();
 const ordinary=new Error("ordinary failure");annotate(ordinary,row.program,row.operation);expect(ordinary.message).toBe("ordinary failure");
});
