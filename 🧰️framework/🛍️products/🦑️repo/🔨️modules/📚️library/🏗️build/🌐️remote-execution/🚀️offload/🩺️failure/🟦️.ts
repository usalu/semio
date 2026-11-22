import {GRPC_CODES_V1} from "../../📡️wire/🚪️grpc/🟦️.ts";
import type {OffloadReasonV1} from "../📐️policy/🟦️.ts";

/** 🩺️ The prefix with which a worker authors the refusal of an action by its own policy: the protocol's marker for the one `FAILED_PRECONDITION` that no other machine will answer differently. The worker writes it and the client reads it from this one definition. */
export const OFFLOAD_REFUSAL_PREFIX_V1="Action refused by the worker policy";

/** 🩺️ What a non-OK status of the execution service means for the requester: the reason the unit compiles here and whether the same action may be submitted again (only a lost transport). Classified by the gRPC code, never by compiler text. */
export function classifyOffloadStatusV1(status:Readonly<{code:number;message?:string}>):Readonly<{reason:OffloadReasonV1;transient:boolean}>{
 if(status.code===GRPC_CODES_V1.RESOURCE_EXHAUSTED)return {reason:"no-idle-machine",transient:false};
 if(status.code===GRPC_CODES_V1.FAILED_PRECONDITION)return {reason:(status.message??"").startsWith(OFFLOAD_REFUSAL_PREFIX_V1)?"worker-policy":"no-idle-machine",transient:false};
 return {reason:"infrastructure",transient:status.code===GRPC_CODES_V1.UNAVAILABLE};
}

const INCOMPLETE_INPUT_CODES=new Set(["E0460","E0463","E0464","E0583","E0786"]);

/** 🩺️ What a compiler exit means: `compile-result` (the compiler's own answer, handed to the caller byte for byte and never compiled twice), `incomplete-inputs` (a structured rustc diagnostic whose error code says an input of the action was missing) or `infrastructure` (the shell could not run the compiler, exit 126 or 127, or the compiler was killed: exit above 128 or negative). Typed signals only: the text of stderr is never matched. */
export function classifyOffloadExitV1(input:Readonly<{exitCode:number;stderr:string}>):"compile-result"|"incomplete-inputs"|"infrastructure"{
 if(input.exitCode===126||input.exitCode===127||input.exitCode>128||input.exitCode<0)return "infrastructure";
 for(const line of input.stderr.split("\n")){
  if(!line.startsWith("{"))continue;
  try{const code=(JSON.parse(line) as {code?:{code?:unknown}|null}).code?.code;if(typeof code==="string"&&INCOMPLETE_INPUT_CODES.has(code))return "incomplete-inputs";}catch{}
 }
 return "compile-result";
}
