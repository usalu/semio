import {expect,test} from "bun:test";
import {GRPC_CODES_V1} from "../../../📡️wire/🚪️grpc/🟦️.ts";
import {classifyOffloadExitV1,classifyOffloadStatusV1,offloadIncompleteDetailV1,OFFLOAD_REFUSAL_PREFIX_V1} from "../../🩺️failure/🟦️.ts";

const diagnostic=(code:string|null,level="error")=>JSON.stringify({$message_type:"diagnostic",message:"m",code:code===null?null:{code,explanation:null},level,spans:[],children:[],rendered:"r"});

test("a status code of the execution service is classified by its code alone: capacity and platform refusals decline, the worker's policy refusal is told by the protocol's own marker, transport faults retry, everything else is infrastructure",()=>{
 const table:[number,string,string,boolean][]=[
  [GRPC_CODES_V1.RESOURCE_EXHAUSTED,"No other machine has headroom: run the action locally","no-idle-machine",false],
  [GRPC_CODES_V1.RESOURCE_EXHAUSTED,"anything else at all","no-idle-machine",false],
  [GRPC_CODES_V1.FAILED_PRECONDITION,"No worker offers the platform x; 0 worker(s) are registered","no-idle-machine",false],
  [GRPC_CODES_V1.FAILED_PRECONDITION,`${OFFLOAD_REFUSAL_PREFIX_V1}: -o is not allowed`,"worker-policy",false],
  [GRPC_CODES_V1.UNAVAILABLE,"worker gone","infrastructure",true],
  [GRPC_CODES_V1.DEADLINE_EXCEEDED,"","infrastructure",false],
  [GRPC_CODES_V1.INTERNAL,"Worker returned a malformed result","infrastructure",false],
  [GRPC_CODES_V1.UNAUTHENTICATED,"","infrastructure",false],
  [GRPC_CODES_V1.PERMISSION_DENIED,"","infrastructure",false],
  [GRPC_CODES_V1.CANCELLED,"Cancelled","infrastructure",false],
  [GRPC_CODES_V1.ABORTED,"","infrastructure",false],
  [GRPC_CODES_V1.UNKNOWN,"couldn't read `x.rs`","infrastructure",false],
 ];
 for(const [code,message,reason,transient] of table)expect<unknown>(classifyOffloadStatusV1({code,message}),`${code} ${message}`).toEqual({reason,transient});
 expect(classifyOffloadStatusV1({code:GRPC_CODES_V1.UNAVAILABLE}).transient).toBe(true);
});

const rendered=(message:string,code:string|null=null)=>JSON.stringify({$message_type:"diagnostic",message,code:code===null?null:{code,explanation:null},level:"error",spans:[],children:[],rendered:`error: ${message}\n`});
const E0425=rendered("cannot find value `DRAW_MOUNTED_OWNER_POLICY` in crate `semio_hub_draw`","E0425")+"\n"+rendered("aborting due to 1 previous error");

test("a compiler result is a result when its stderr holds ordinary diagnostics about the crate's own code; only killed or unrunnable compilers, a missing-input code and environmental failures are not",()=>{
 const table:[number,string,string,string?][]=[
  [1,E0425,"compile-result"],
  [1,diagnostic("E0308"),"compile-result"],
  [1,diagnostic("E0599")+"\n"+diagnostic(null),"compile-result"],
  [1,diagnostic(null),"compile-result"],
  [101,diagnostic(null),"compile-result"],
  [1,diagnostic("E0463"),"incomplete-inputs"],
  [1,rendered("can't find crate for `missing_crate`","E0463"),"incomplete-inputs"],
  [1,diagnostic("E0583"),"incomplete-inputs"],
  [1,diagnostic("E0460"),"incomplete-inputs"],
  [1,diagnostic("E0308")+"\n"+diagnostic("E0463"),"incomplete-inputs"],
  [127,"","infrastructure"],
  [126,"","infrastructure"],
  [137,"","infrastructure"],
  [139,diagnostic("E0308"),"infrastructure"],
  [-9,"","infrastructure"],
  [1,diagnostic("E0514"),"environmental","missing-crate"],
  [1,"error: extern location for serde does not exist: /work/deps/libserde-1.rlib\n"+diagnostic("E0308"),"environmental","missing-crate"],
  [1,rendered("found crate `a` compiled by an incompatible version of rustc","E0514"),"environmental","missing-crate"],
  [1,rendered("couldn't read `src/data.txt`: No such file or directory (os error 2)"),"environmental","unreadable"],
  [1,rendered("proc macro panicked: called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }"),"environmental","unreadable"],
  [1,rendered("file not found for module `generated`","E0583").replace("E0583","E0000"),"environmental","unreadable"],
  [1,rendered("linking with `rust-lld` failed: exit status: 1"),"environmental","linker"],
  [1,"wasm-component-ld: error: failed to read input\n"+diagnostic(null),"environmental","linker"],
  [1,"rust-lld: error: unknown file type: x.rcgu.o\n"+rendered("aborting due to 1 previous error"),"environmental","linker"],
  [1,rendered("could not exec the linker `cc`"),"environmental","linker"],
  [101,"error: internal compiler error: unexpected panic\n"+diagnostic(null),"environmental","crash"],
  [1,"error: could not exec the compiler\n","environmental","crash"],
  [1,rendered("No space left on device (os error 28)"),"environmental","disk"],
  [1,"","environmental","no-diagnostic"],
  [1,"not json {\n"+diagnostic("E0308").slice(0,20),"environmental","no-diagnostic"],
  [1,diagnostic("W0001","warning"),"environmental","no-diagnostic"],
  [101,"thread 'rustc' panicked at x.rs:1:1\n","environmental","no-diagnostic"],
 ];
 for(const [exitCode,stderr,kind,name] of table){
  const verdict=classifyOffloadExitV1({exitCode,stderr});
  expect<unknown>(verdict.kind,`${exitCode} ${stderr.slice(0,60)}`).toBe(kind);
  if(name!==undefined)expect(verdict.kind==="environmental"?verdict.class:"",`${exitCode} ${stderr.slice(0,60)}`).toBe(name);
 }
});

test("the head of a remote failure names the first diagnostic with its source file, else the first line, in at most the schema's characters",()=>{
 expect(offloadIncompleteDetailV1(E0425)).toBe("cannot find value `DRAW_MOUNTED_OWNER_POLICY` in crate `semio_hub_draw`");
 expect(offloadIncompleteDetailV1("first line\nsecond")).toBe("first line");
 expect(offloadIncompleteDetailV1("x".repeat(500)).length).toBe(240);
});

test("a status that names missing blobs is read from its precondition failure detail (every blob), else from its message, and is an incomplete input, never an idle-machine refusal",async()=>{
 const {encodeProtobufV1,REAPI_V1}=await import("../../../📡️wire/📦️protobuf/🟦️.ts"),{offloadMissingBlobsV1}=await import("../../🩺️failure/🟦️.ts");
 const a={hash:"a".repeat(64),size_bytes:5},b={hash:"b".repeat(64),size_bytes:4223571},detail=(rows:readonly {hash:string;size_bytes:number}[])=>({type_url:`type.googleapis.com/${REAPI_V1.preconditionFailure}`,value:encodeProtobufV1(REAPI_V1.preconditionFailure,{violations:rows.map(row=>({type:"MISSING",subject:`blobs/${row.hash}/${row.size_bytes}`}))})});
 expect(offloadMissingBlobsV1({message:"Missing blob x",details:[detail([a,b])]})).toEqual([a,b]);
 expect(offloadMissingBlobsV1({message:`Missing blob ${a.hash}/5`})).toEqual([a]);
 expect(offloadMissingBlobsV1({message:"No worker offers the platform x"})).toEqual([]);
 expect(offloadMissingBlobsV1({message:"x",details:[{type_url:"type.googleapis.com/other",value:new Uint8Array(0)}]})).toEqual([]);
 expect(classifyOffloadStatusV1({code:GRPC_CODES_V1.FAILED_PRECONDITION,message:`Missing blob ${a.hash}/5 and 3 more`})).toEqual({reason:"incomplete-inputs",transient:false});
});
