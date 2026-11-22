import {expect,test} from "bun:test";
import {GRPC_CODES_V1} from "../../../📡️wire/🚪️grpc/🟦️.ts";
import {classifyOffloadExitV1,classifyOffloadStatusV1,OFFLOAD_REFUSAL_PREFIX_V1} from "../../🩺️failure/🟦️.ts";

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
 for(const [code,message,reason,transient] of table)expect(classifyOffloadStatusV1({code,message}),`${code} ${message}`).toEqual({reason,transient});
 expect(classifyOffloadStatusV1({code:GRPC_CODES_V1.UNAVAILABLE}).transient).toBe(true);
});

test("a compiler result is a result: only typed signals make it infrastructure (killed or not runnable, or a diagnostic code that names a missing input), and no stderr text ever does",()=>{
 const text=(message:string)=>`error: ${message}\n`;
 const table:[number,string,string][]=[
  [1,diagnostic("E0308"),"compile-result"],
  [1,diagnostic("E0599")+"\n"+diagnostic(null),"compile-result"],
  [1,diagnostic(null),"compile-result"],
  [101,diagnostic(null),"compile-result"],
  [1,diagnostic("E0463"),"incomplete-inputs"],
  [1,JSON.stringify({$message_type:"diagnostic",message:"can't find crate for `semio_offload_inputs_incomplete`",code:{code:"E0463",explanation:null},level:"error",spans:[],children:[],rendered:"r"}),"incomplete-inputs"],
  [1,diagnostic("E0583"),"incomplete-inputs"],
  [1,diagnostic("E0460"),"incomplete-inputs"],
  [1,diagnostic("E0308")+"\n"+diagnostic("E0463"),"incomplete-inputs"],
  [127,"","infrastructure"],
  [126,"","infrastructure"],
  [137,"","infrastructure"],
  [139,diagnostic("E0308"),"infrastructure"],
  [-9,"","infrastructure"],
  [1,text("couldn't read `src/lib.rs`: No such file or directory"),"compile-result"],
  [1,text("linking with `cc` failed"),"compile-result"],
  [1,"error[E0463]: can't find crate for `serde`\n","compile-result"],
  [1,text("No space left on device"),"compile-result"],
  [1,"not json {\n"+diagnostic("E0463").slice(0,20),"compile-result"],
  [1,"","compile-result"],
 ];
 for(const [exitCode,stderr,expected] of table)expect(classifyOffloadExitV1({exitCode,stderr}),`${exitCode} ${stderr.slice(0,60)}`).toBe(expected);
});
