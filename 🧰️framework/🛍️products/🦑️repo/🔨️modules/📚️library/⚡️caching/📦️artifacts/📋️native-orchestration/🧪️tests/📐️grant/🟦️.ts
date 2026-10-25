import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import publication from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🧬️schema/🔣️json/🟦️.ts";
import storage from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️json/🟦️.ts";
import policy from"../../🔣️policy.json"with{type:"json"};

/** 📐️ Pins every received Native grant ceiling of the process module to the central child and transport policy that every real build hands out, so a policy change can never turn the whole publication into a refusal again. */
test("the central native execution policy is admitted by every received Native grant schema and an independent SQLite bound",()=>{
 const db=new Database(":memory:"),grant={maximumElapsedMilliseconds:policy.child.maximumElapsedMilliseconds,maximumBytes:policy.transport.maximumBytes,maximumLines:policy.transport.maximumLines};
 try{
  for(const[name,document,definition]of[["publication",publication,"NativeGrant"],["storage",storage,"ProtectedParentGrant"]]as const){
   const ajv=new Ajv({strict:false}),validate=ajv.compile({...document,$ref:"#/definitions/"+definition}),bounds=(document as unknown as{definitions:Record<string,{properties:Record<string,{minimum:number;maximum:number}>}>}).definitions[definition]!.properties;
   for(const[label,value,accepted]of[["policy",grant,true],["elapsed above",{...grant,maximumElapsedMilliseconds:bounds.maximumElapsedMilliseconds!.maximum+1},false],["elapsed zero",{...grant,maximumElapsedMilliseconds:0},false],["bytes above",{...grant,maximumBytes:bounds.maximumBytes!.maximum+1},false],["lines above",{...grant,maximumLines:bounds.maximumLines!.maximum+1},false]]as const){
    const oracle=db.query("SELECT (?1 BETWEEN ?2 AND ?3 AND ?4 BETWEEN ?5 AND ?6 AND ?7 BETWEEN ?8 AND ?9) accepted").get(value.maximumElapsedMilliseconds,bounds.maximumElapsedMilliseconds!.minimum,bounds.maximumElapsedMilliseconds!.maximum,value.maximumBytes,bounds.maximumBytes!.minimum,bounds.maximumBytes!.maximum,value.maximumLines,bounds.maximumLines!.minimum,bounds.maximumLines!.maximum)as{accepted:number};
    expect(validate(value),name+" "+label).toBe(accepted);expect(Boolean(oracle.accepted),name+" "+label).toBe(accepted);
   }
  }
 }finally{db.close();}
});
