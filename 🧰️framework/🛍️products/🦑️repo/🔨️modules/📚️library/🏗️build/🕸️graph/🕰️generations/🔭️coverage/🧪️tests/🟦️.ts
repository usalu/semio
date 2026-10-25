/** 🔭️ Contract: the verified stamp of a graph generation pointer is a bitmap over the owners of its manifest; encoding, decoding, merging with the reused part of the parent and the coverage of a later request equal the independent Python model on closed vectors, and every malformed stamp is refused by both. */
import{expect,test}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{COVERAGE_POLICY_V1,coverageOfV1,decodeVerifiedScopeV1,encodeVerifiedScopeV1,mergeVerifiedStampV1,readVerifiedStampV1,uncoveredOfV1}from"../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py");
const model=(command:string,input:unknown)=>{const run=Bun.spawnSync([python!,"-I",oracle,command],{stdin:Buffer.from(JSON.stringify(input))});expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString());};
const sorted=(names:Iterable<string>)=>[...names].sort();

test("the fixture and the policy follow the schema file",()=>{
 expect(new Ajv2020({strict:false}).validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture)).toBe(true);
 expect(COVERAGE_POLICY_V1).toEqual(schema.definitions.Policy.const as typeof COVERAGE_POLICY_V1);
});

for(const row of fixture.scopes)test("scope bitmap round trip and Python model: "+row.id,()=>{
 const encoded=encodeVerifiedScopeV1(row.owners,new Set(row.covered)),stamp=readVerifiedStampV1({atMs:5,...encoded});
 expect(sorted(decodeVerifiedScopeV1(row.owners,stamp))).toEqual(sorted(row.covered));
 expect(Buffer.from(encoded.owners,"base64")).toHaveLength(Math.ceil(row.owners.length/8));
 expect(model("scope",{owners:row.owners,covered:row.covered})).toEqual(encoded);
 expect(model("decode",{owners:row.owners,stamp})).toEqual({covered:row.owners.filter(name=>row.covered.includes(name))});
});

for(const row of fixture.merges)test("merged stamp equals the Python model: "+row.id,()=>{
 const carried=row.carried?{atMs:row.carried.atMs,names:new Set(row.carried.names)}:undefined,stamp=mergeVerifiedStampV1({owners:row.owners,scope:row.scope===null?undefined:new Set(row.scope),observedAtMs:row.observedAtMs,carried});
 expect(stamp).toEqual(model("merge",row));
 const names=decodeVerifiedScopeV1(row.owners,stamp);
 if(row.scope===null)expect(names.size).toBe(row.owners.length);
 else for(const name of row.scope)expect(names.has(name)).toBe(row.owners.includes(name));
 if(row.scope!==null&&row.carried)expect(stamp.atMs).toBe(Math.min(row.carried.atMs,row.observedAtMs));
 else expect(stamp.atMs).toBe(row.observedAtMs);
});

for(const row of fixture.coverage)test("coverage of a later request equals the Python model: "+row.id,()=>{
 const stamp=row.stamp?readVerifiedStampV1({atMs:row.stamp.atMs,...encodeVerifiedScopeV1(row.owners,new Set(row.stamp.covered))}):undefined;
 const coverage=coverageOfV1({stamp,owners:row.owners,implementation:row.requestedImplementation,manifestImplementation:row.implementation,requestedAtMs:row.requestedAtMs}),expected=model("cover",row);
 expect(coverage!==undefined).toBe(expected.covered);
 if(coverage===undefined)return;
 expect(coverage.atMs).toBe(expected.atMs);
 expect(sorted(uncoveredOfV1({needed:row.needed===null?undefined:new Set(row.needed),owners:row.owners,coverage}))).toEqual(expected.open);
});

for(const row of fixture.refusals)test("a malformed stamp is refused by both implementations: "+row.id,()=>{
 const refuses=()=>{const stamp=readVerifiedStampV1(row.stamp);decodeVerifiedScopeV1(row.owners,stamp);};
 expect(refuses).toThrow();
 const read=model("read",{stamp:row.stamp}),decode=model("decode",{owners:row.owners,stamp:row.stamp});
 expect(read.accepted===false||decode.refused!==undefined).toBe(true);
});

test("a time can only move to the older side when a part of the parent is reused",()=>{
 const owners=["a","b","c"],carried={atMs:10,names:new Set(["a"])};
 expect(mergeVerifiedStampV1({owners,scope:new Set(["b"]),observedAtMs:99,carried}).atMs).toBe(10);
 expect(mergeVerifiedStampV1({owners,scope:new Set(["b"]),observedAtMs:5,carried}).atMs).toBe(5);
 expect(mergeVerifiedStampV1({owners,scope:undefined,observedAtMs:99,carried}).atMs).toBe(99);
});
