import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{createRequire}from"node:module";
import{createHash}from"node:crypto";
import{Database}from"bun:sqlite";
import corpus from"../🧫️fixtures/📂️descriptors.json";
import schema from"../../🧬️schema/📂️descriptors.json";
import{provePhysicalFileLocksV1}from"../🟦️.ts";
import{PhysicalLockErrorV1}from"../../🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";

test("physical descriptor transfer closes every refused opened handle",()=>{
 const require=createRequire(import.meta.url),ts=require("typescript"),esbuild=require("esbuild"),database=new Database(":memory:"),source=new URL("../../🟦️.ts",import.meta.url),name="retainPhysicalDescriptorV1";
 expect(new(require("ajv").default)({strict:true}).validate(schema,corpus)).toBe(true);
 try{for(const row of corpus.cases){
  const tree=ts.createSourceFile(source.pathname,readFileSync(source,"utf8"),ts.ScriptTarget.Latest,true),node=tree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text===name);expect(node).toBeDefined();
  const emitted=esbuild.transformSync(node.getText(tree),{loader:"ts",format:"cjs"}).code,events:{kind:string;fd:number}[]=[],held:{fd:number;path:string;dev:number;ino:number}[]=[],fault=(phase:string)=>{if(row.failure===phase)throw Error("fault:"+phase);},stat={dev:1,ino:2,size:4,nlink:1,mtimeMs:3,ctimeMs:4,isFile:()=>row.failure!=="identity",isSymbolicLink:()=>false};let stats=0;
  if(row.failure==="retain")Object.defineProperty(held,"push",{value:()=>{fault("retain");}});
  const parameters={openSync:()=>{fault("open");events.push({kind:"open",fd:17});return 17;},closeSync:(fd:number)=>events.push({kind:"close",fd}),constants:{O_RDONLY:1,O_RDWR:2,O_NOFOLLOW:4},fstatSync:()=>{fault(stats++===0?"before":"after");return row.failure==="mutated"&&stats>1?{...stat,ino:3}:stat;},lstatSync:()=>{fault("named");return stat;},readSync:(_fd:number,bytes:Buffer)=>{fault("read");if(row.failure==="short")return 0;bytes.write("data");return 4;},createHash:(algorithm:string)=>{fault("hash");const hash=createHash(algorithm);return{update:(bytes:Buffer)=>{fault("update");hash.update(bytes);},digest:(encoding:"hex")=>{fault("digest");return hash.digest(encoding);}};},Buffer:{alloc:(bytes:number)=>{fault("buffer");return Buffer.alloc(bytes);}},PhysicalLockErrorV1};
  const implementation=new Function(...Object.keys(parameters),emitted+";return "+name+";")(...Object.values(parameters));let error:string|null=null;
  try{implementation("fixture:descriptor",held);}catch(cause){error=(cause as Error).message;}
  const actual={opened:events.filter(event=>event.kind==="open").length,closed:events.filter(event=>event.kind==="close").length,retained:held.length,error,digest:null};expect(actual,row.owner+":"+row.failure).toEqual(row.expected);
  const oracle=database.query("SELECT CASE WHEN ?2='open' THEN 0 ELSE 1 END opened,CASE WHEN ?1='physical' AND ?2='none' THEN 1 ELSE 0 END retained,CASE WHEN ?2='open' OR (?1='physical' AND ?2='none') THEN 0 ELSE 1 END closed").get(row.owner,row.failure)as{opened:number;retained:number;closed:number};expect({opened:actual.opened,retained:actual.retained,closed:actual.closed}).toEqual(oracle);
  expect(events.every(event=>event.fd===17)).toBe(true);expect(actual.closed+actual.retained).toBe(actual.opened);
  const owner=tree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text==="createPosixPhysicalLockBackendV1");expect(owner.getText(tree)).toContain("retainPhysicalDescriptorV1(path,held)");
 }}finally{database.close();}
});

test.skipIf(!["darwin","linux","win32"].includes(process.platform))("qualified kernel file lock retains and releases the actual original descriptor",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Descriptor kernel test artifact directory required");await provePhysicalFileLocksV1(join(artifacts,"descriptor-kernel"),createIssuedResourceControlV1(original,"test:descriptor-kernel",10000));
}),15000);
