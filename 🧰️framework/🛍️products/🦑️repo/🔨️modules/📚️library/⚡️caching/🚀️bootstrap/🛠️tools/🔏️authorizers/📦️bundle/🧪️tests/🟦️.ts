/** 🔏️ Contract: one compiled authorizer bundle is its own directory beside the dependency generation, holds only while marker, recomputed key, receipt and bytes agree, and retirement removes only released bundles behind the newest ones exactly as an independent Python plan does. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {chmodSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {withResourceLeases} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {compileJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {nxAuthorizerBundleIdentityV1} from "../../../📦️sdk/🪪️identity/🟦️.ts";
import {supersedingNxAuthorizerBundleV1,newestNxAuthorizerBundleV1,nxAuthorizerBundleDirectoryV1,nxAuthorizerBundleHoldsV1,nxAuthorizerBundlePolicyV1,nxAuthorizerBundleToolkitV1,readNxAuthorizerBundleMarkerV1,touchNxAuthorizerBundleV1} from "../🟦️.ts";
import {planNxAuthorizerBundleRetirementV1,retireNxAuthorizerBundlesV1} from "../🧹️retire/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracle=join(import.meta.dir,"./🐍️oracle/🐍️.py"),hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");
const scratch=mkdtempSync(join(realpathSync(tmpdir()),"nxb-"));

type Parts={dependencies:string;source:string;compiler:string;body:string};
const parts=(dependencies:string,tag=""):Parts=>({dependencies,source:JSON.stringify({version:1,mode:"release",entry:"a"+tag+".ts",resolutions:[],sources:[{path:"a"+tag+".ts",sha256:hash("a"+tag)}]})+"\n",compiler:JSON.stringify({version:1,mode:"release"})+"\n",body:"export const authorizers=1"+tag+";\n"});
function build(tools:string,row:Parts,change:(directory:string,marker:Record<string,unknown>)=>void=()=>{}):string{
 const manifests=new Map([[".semio-authorizer-source.json",Buffer.from(row.source)],[".semio-authorizer-compiler.json",Buffer.from(row.compiler)]]),identity=nxAuthorizerBundleIdentityV1(row.dependencies,manifests),directory=nxAuthorizerBundleDirectoryV1(tools,identity);
 mkdirSync(directory,{recursive:true});
 for(const[name,bytes]of manifests)writeFileSync(join(directory,name),bytes,{mode:0o444});writeFileSync(join(directory,"semio-authorizers.mjs"),row.body,{mode:0o444});
 const marker:Record<string,unknown>={version:1,digest:identity,deps:row.dependencies,authorizers:{version:1,mode:"release",sourceSha256:hash(row.source),entry:"semio-authorizers.mjs",outputs:[{path:"semio-authorizers.mjs",sha256:hash(row.body)}]}};
 change(directory,marker);writeFileSync(join(directory,nxAuthorizerBundlePolicyV1.marker),JSON.stringify(marker),{mode:0o444});
 return directory;
}
const fresh=(label:string)=>{const tools=join(scratch,label);mkdirSync(join(tools,"nx"),{recursive:true});return tools;};

test("the schema and corpus are closed and both validators agree",()=>{
 const ajv=new Ajv({strict:false}),plan=compileJsonSchemaSubset({$ref:"#/$defs/RetirementCorpus"},schema),marker=compileJsonSchemaSubset({$ref:"#/$defs/Marker"},schema);
 ajv.addSchema(schema,"s");expect(ajv.validate({$ref:"s#/$defs/RetirementCorpus"},corpus),JSON.stringify(ajv.errors)).toBe(true);expect(plan.validate(corpus)).toEqual([]);
 const bad={...corpus,cases:[{...corpus.cases[0],rows:[{identity:"x",touchedAt:1,held:false}]}]};expect(ajv.validate({$ref:"s#/$defs/RetirementCorpus"},bad)).toBe(false);expect(plan.validate(bad).length).toBeGreaterThan(0);
 const good={version:1,digest:"a".repeat(64),deps:"b".repeat(64),authorizers:{}};
 for(const value of[good,{...good,extra:1},{...good,deps:"c"},{...good,version:2},{version:1,digest:good.digest,deps:good.deps}])expect(marker.validate(value).length===0,JSON.stringify(value)).toBe(ajv.validate({$ref:"s#/$defs/Marker"},value));
 expect(marker.validate(good)).toEqual([]);expect(marker.validate({...good,extra:1}).length).toBeGreaterThan(0);
});

test("a bundle holds beside its dependency generation and every part of it is bound",()=>{
 const tools=fresh("holds"),dependencies="d".repeat(64),directory=build(tools,parts(dependencies));
 expect(nxAuthorizerBundleHoldsV1(directory,dependencies)?.deps).toBe(dependencies);expect(nxAuthorizerBundleHoldsV1(directory,"e".repeat(64))).toBeNull();
 expect(readNxAuthorizerBundleMarkerV1(directory).digest).toBe(nxAuthorizerBundleIdentityV1(dependencies,new Map([[".semio-authorizer-source.json",Buffer.from(parts(dependencies).source)],[".semio-authorizer-compiler.json",Buffer.from(parts(dependencies).compiler)]])));
 const toolkit=nxAuthorizerBundleToolkitV1(directory);expect(toolkit.identity).toBe(dependencies);expect(toolkit.toolkitRoot).toBe(join(tools,"nx",dependencies));expect(toolkit.bundle).toBe(join(directory,"semio-authorizers.mjs"));
 const faults:[string,(directory:string,marker:Record<string,unknown>)=>void][]=[
  ["marker digest",(_d,marker)=>{marker.digest="f".repeat(64);}],
  ["marker deps",(_d,marker)=>{marker.deps="f".repeat(64);}],
  ["marker extra field",(_d,marker)=>{marker.extra=true;}],
  ["receipt source digest",(_d,marker)=>{(marker.authorizers as {sourceSha256:string}).sourceSha256="0".repeat(64);}],
  ["receipt output digest",(_d,marker)=>{(marker.authorizers as {outputs:{sha256:string}[]}).outputs[0]!.sha256="0".repeat(64);}],
 ];
 for(const[name,change]of faults){const broken=build(fresh("fault-"+name.replaceAll(" ","-")),parts(dependencies),change);expect(nxAuthorizerBundleHoldsV1(broken,dependencies),name).toBeNull();}
 const tampered=build(fresh("tamper"),parts(dependencies));chmodSync(join(tampered,"semio-authorizers.mjs"),0o644);writeFileSync(join(tampered,"semio-authorizers.mjs"),"export const authorizers=2;\n");expect(nxAuthorizerBundleHoldsV1(tampered,dependencies)).toBeNull();
 const compiler=build(fresh("compiler"),parts(dependencies));chmodSync(join(compiler,".semio-authorizer-compiler.json"),0o644);writeFileSync(join(compiler,".semio-authorizer-compiler.json"),"{}\n");expect(nxAuthorizerBundleHoldsV1(compiler,dependencies)).toBeNull();
 const linked=build(fresh("linked"),parts(dependencies));rmSync(join(linked,"semio-authorizers.mjs"));writeFileSync(join(scratch,"outside.mjs"),"export const authorizers=1;\n");symlinkSync(join(scratch,"outside.mjs"),join(linked,"semio-authorizers.mjs"));expect(nxAuthorizerBundleHoldsV1(linked,dependencies)).toBeNull();
 expect(()=>nxAuthorizerBundleToolkitV1(join(scratch,"holds"))).toThrow();
});

test("the newest holding bundle beside a dependency generation is found and others are ignored",async()=>{
 const tools=fresh("newest"),dependencies="d".repeat(64),first=build(tools,parts(dependencies));await Bun.sleep(25);
 const second=build(tools,parts(dependencies,"2")),other=build(tools,parts("e".repeat(64)));
 expect([first,second,other].map(directory=>directory.length).every(length=>length>0)).toBe(true);
 expect(newestNxAuthorizerBundleV1(tools,dependencies)).toBe(second);expect(newestNxAuthorizerBundleV1(tools,"9".repeat(64))).toBeUndefined();expect(newestNxAuthorizerBundleV1(join(scratch,"missing"),dependencies)).toBeUndefined();
});

test("a bundle used after the one a daemon was born with supersedes it, a bundle that does not hold or belongs to another generation never does",async()=>{
 const tools=fresh("superseding"),dependencies="d".repeat(64),name=(directory:string)=>directory.split("/").at(-1)!;
 const first=build(tools,parts(dependencies,"1"));await Bun.sleep(25);
 expect(supersedingNxAuthorizerBundleV1(tools,dependencies,name(first))).toBeUndefined();
 const second=build(tools,parts(dependencies,"2"));
 expect(supersedingNxAuthorizerBundleV1(tools,dependencies,name(first))).toBe(second);expect(supersedingNxAuthorizerBundleV1(tools,dependencies,name(second))).toBeUndefined();
 await Bun.sleep(25);touchNxAuthorizerBundleV1(first);
 expect(supersedingNxAuthorizerBundleV1(tools,dependencies,name(second))).toBe(first);expect(supersedingNxAuthorizerBundleV1(tools,dependencies,name(first))).toBeUndefined();
 await Bun.sleep(25);build(tools,parts("e".repeat(64),"3"));build(tools,parts(dependencies,"4"),(_directory,marker)=>{marker.deps="f".repeat(64);});
 expect(supersedingNxAuthorizerBundleV1(tools,dependencies,name(first))).toBeUndefined();
 expect(supersedingNxAuthorizerBundleV1(tools,dependencies,"9".repeat(64))).toBeUndefined();expect(supersedingNxAuthorizerBundleV1(join(scratch,"missing"),dependencies,name(first))).toBeUndefined();
});

test("the retirement plan equals the Python plan on every corpus case",()=>{
 const run=spawnSync(python,["-I",oracle],{input:JSON.stringify({cases:corpus.cases}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const answers=JSON.parse(run.stdout);for(const[index,row]of corpus.cases.entries()){expect(planNxAuthorizerBundleRetirementV1(row.rows,row.nowMs,row.keepNewest,row.minimumAgeMilliseconds),row.name).toEqual(row.expected);expect(answers[index],row.name).toEqual(row.expected);}
});

test("retirement removes only released bundles behind the newest ones and keeps a bundle a reader holds",async()=>{
 const tools=fresh("retire"),leases=join(scratch,"leases"),signal=new AbortController().signal,made:string[]=[];mkdirSync(leases,{recursive:true});
 for(let index=0;index<6;index++){made.push(build(tools,parts("d".repeat(64),String(index))));await Bun.sleep(25);}
 const names=made.map(directory=>directory.split("/").at(-1)!),held=names[1]!;
 const retired=await withResourceLeases({directory:leases,signal,resources:[{resource:nxAuthorizerBundlePolicyV1.lease+held,mode:"shared"}]},()=>retireNxAuthorizerBundlesV1(tools,leases,undefined,{keepNewest:2,minimumAgeMilliseconds:0,signal}));
 const remaining=readdirSync(join(tools,nxAuthorizerBundlePolicyV1.directory)).sort();
 expect([...retired].sort()).toEqual([names[0]!,names[2]!,names[3]!].sort());expect(remaining).toEqual([names[1]!,names[4]!,names[5]!].sort());
 expect(await retireNxAuthorizerBundlesV1(tools,leases,undefined,{keepNewest:2,minimumAgeMilliseconds:0,signal})).toEqual([names[1]!]);
 expect(await retireNxAuthorizerBundlesV1(join(scratch,"absent"),leases,undefined,{signal})).toEqual([]);
 const young=await retireNxAuthorizerBundlesV1(tools,leases,undefined,{keepNewest:0,signal});expect(young).toEqual([]);
});

test("marking a bundle used keeps it ahead of newer bundles",async()=>{
 const tools=fresh("touch"),leases=join(scratch,"leases-touch"),signal=new AbortController().signal,made:string[]=[];mkdirSync(leases,{recursive:true});
 for(let index=0;index<3;index++){made.push(build(tools,parts("d".repeat(64),"t"+index)));await Bun.sleep(25);}
 touchNxAuthorizerBundleV1(made[0]!);
 const names=made.map(directory=>directory.split("/").at(-1)!),retired=await retireNxAuthorizerBundlesV1(tools,leases,undefined,{keepNewest:1,minimumAgeMilliseconds:0,signal});
 expect([...retired].sort()).toEqual([names[1]!,names[2]!].sort());expect(readdirSync(join(tools,nxAuthorizerBundlePolicyV1.directory))).toEqual([names[0]!]);
 expect(()=>touchNxAuthorizerBundleV1(join(scratch,"absent"))).not.toThrow();
});

test("cleanup",()=>{rmSync(scratch,{recursive:true,force:true});});
