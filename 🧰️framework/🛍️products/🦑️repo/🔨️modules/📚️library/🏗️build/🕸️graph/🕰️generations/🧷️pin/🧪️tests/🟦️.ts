/** 🧷️ Contract: the generation pin that an Original run hands to its nested commands is a closed document; encoding, reading, the binding to the invocation chain and the owners a pin leaves open equal the independent Python model on closed vectors, every malformed pin is refused by both, and the constants follow the schema and the lease protocol. */
import{expect,test}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import{readFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{GENERATION_PIN_MESSAGES_V1,GENERATION_PIN_POLICY_V1,encodeGenerationPinV1,generationPinOpenV1,generationPinOwnerV1,generationPinStateV1,readGenerationPinV1,renderGenerationPinMessageV1}from"../🟨️.mjs";
import leases from"../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🧬️schema/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py");
const model=(command:string,input:unknown)=>{const run=Bun.spawnSync([python!,"-I",oracle,command],{stdin:Buffer.from(JSON.stringify(input))});expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString());};
const refused=(body:()=>unknown)=>{try{body();}catch(error){return error as Error&{code?:string;reason?:string};}throw Error("refusal expected");};

test("the fixture, the policy and the messages follow the schema file",()=>{
 expect(new Ajv2020({strict:false}).validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture)).toBe(true);
 expect(GENERATION_PIN_POLICY_V1).toEqual(schema.definitions.Policy.const as typeof GENERATION_PIN_POLICY_V1);
 expect(JSON.parse(JSON.stringify(GENERATION_PIN_MESSAGES_V1))).toEqual(schema.definitions.Messages.const);
 expect(Object.keys(GENERATION_PIN_MESSAGES_V1.en)).toEqual(Object.keys(GENERATION_PIN_MESSAGES_V1.de));
});

test("the environment names are the ones of the lease protocol that every nested process already carries",()=>{
 const text=JSON.stringify(leases);
 expect(text).toContain(GENERATION_PIN_POLICY_V1.invocationEnvironment);
 expect(text).toContain(GENERATION_PIN_POLICY_V1.chainEnvironment);
});

for(const row of fixture.encode)test("encoded pin equals the Python model and reads back: "+row.id,()=>{
 const text=encodeGenerationPinV1({owner:row.owner,generation:row.generation,manifest:row.manifest,owners:row.owners,covered:row.covered===null?undefined:new Set(row.covered),atMs:row.atMs});
 expect(text).toBe(model("encode",row).text);
 const pin=readGenerationPinV1(text);
 expect(pin.owner).toBe(row.owner);expect(pin.generation).toBe(row.generation);expect(pin.manifest).toBe(row.manifest);expect(pin.covered.count).toBe(row.owners.length);
 const wanted=row.covered===null?row.owners:row.covered;
 expect(generationPinOpenV1({pin,owners:row.owners})).toEqual(row.owners.filter(name=>!wanted.includes(name)));
});

for(const row of fixture.accepted)test("an admitted pin is admitted by both: "+row.id,()=>{
 expect(()=>readGenerationPinV1(row.text)).not.toThrow();expect(model("read",row)).toEqual({accepted:true});
});

for(const row of fixture.refusals)test("a malformed pin is refused by both, in both languages: "+row.id,()=>{
 expect(model("read",row)).toEqual({accepted:false});
 const error=refused(()=>readGenerationPinV1(row.text));
 expect(error.code).toBe("GENERATION_PIN_REFUSED");expect(error.reason).toBe("malformed");
 expect(error.message).toContain("does not plan a graph of its own");expect(error.message).toContain("plant keinen eigenen Graphen");
});

for(const row of fixture.states)test("binding to the invocation chain equals the Python model: "+row.id,()=>{
 const alive=new Set(row.alive),expected=model("state",row).state;
 expect(expected).toBe(row.expected);
 if(expected==="refused"){expect(refused(()=>generationPinStateV1(row.environment,{alive:pid=>alive.has(pid),self:-1,now:(row as {now:number}).now})).reason).toBe("malformed");return;}
 expect(generationPinStateV1(row.environment,{alive:pid=>alive.has(pid),self:(row as {self?:number}).self??-1,now:(row as {now:number}).now}).state).toBe(expected);
});

for(const row of fixture.opens)test("the owners a pin leaves open equal the Python model: "+row.id,()=>{
 const pin=readGenerationPinV1(encodeGenerationPinV1({owner:1,generation:1,manifest:"a".repeat(64),owners:row.owners,covered:new Set(row.covered),atMs:1}));
 expect(generationPinOpenV1({pin,owners:row.owners,...(row.needed===null?{}:{needed:new Set(row.needed)})})).toEqual(model("open",row).open);
});

for(const row of fixture.owners)test("the owner invocation of a command equals the Python model: "+row.id,()=>{
 expect(generationPinOwnerV1(row.environment,row.pid)).toBe(model("owner",row).owner);
});

test("an owner list that differs from the manifest of the pin refuses",()=>{
 const pin=readGenerationPinV1(encodeGenerationPinV1({owner:1,generation:1,manifest:"a".repeat(64),owners:["a","b"],covered:new Set(["a"]),atMs:1}));
 expect(refused(()=>generationPinOpenV1({pin,owners:["a","b","c"]})).reason).toBe("malformed");
});

test("the refusal texts are filled in both languages and name the unreadable key",()=>{
 expect(renderGenerationPinMessageV1("uncovered",{generation:4,count:2,names:"x, y"},"en")).toContain("generation 4 does not cover 2");
 expect(renderGenerationPinMessageV1("uncovered",{generation:4,count:2,names:"x, y"},"de")).toContain("Generation 4 2 der benötigten Projekte nicht abdeckt (x, y)");
 expect(()=>renderGenerationPinMessageV1("nope" as never,{},"en")).toThrow("Known generation pin message required");
});

test("only an orchestrated task keeps the pin: the environment issuer drops it for every other task, with the name of the policy",()=>{
 const source=readFileSync(join(dirname(fileURLToPath(import.meta.url)),"../../../../📥️caller/🟦️.ts"),"utf8");
 expect(source).toContain(`if(orchestration===undefined)delete result.${GENERATION_PIN_POLICY_V1.environment};`);
});

test("the schema file of the pin is read, not copied",()=>{
 expect(readFileSync(join(dirname(fileURLToPath(import.meta.url)),"../🟨️.mjs"),"utf8")).toContain('from"./🧬️schema/🔣️.json"');
});
