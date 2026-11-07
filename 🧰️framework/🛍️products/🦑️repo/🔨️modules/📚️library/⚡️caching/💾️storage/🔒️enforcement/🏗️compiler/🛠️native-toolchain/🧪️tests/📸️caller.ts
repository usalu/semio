import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import fixture from"./📸️caller.json" with{type:"json"};
import{snapshotNativeCallerEnvironmentV1}from"../🟦️.ts";
import{portableMsvcEligibilityV1}from"../../../../../../../../../../🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/🟦️.ts";
test("caller choices remain stable through managed preparation",()=>{const db=new Database(":memory:");try{for(const row of fixture.cases){const environment:Record<string,string>={...row.before},caller=snapshotNativeCallerEnvironmentV1(environment);Object.assign(environment,row.after);const oracle=db.query("select case when ? is null then 'eligible' else 'override' end reason").get(row.before.CC??null) as{reason:string};expect(portableMsvcEligibilityV1(caller,{platform:"win32",architecture:"x64",target:null}),row.name).toBe(oracle.reason);expect(oracle.reason).toBe(row.reason);}}finally{db.close();}});
