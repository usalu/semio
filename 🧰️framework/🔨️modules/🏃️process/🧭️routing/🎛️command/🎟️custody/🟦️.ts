import {resolve,isAbsolute} from "node:path";
import {checkScriptInvocation,type ScriptInvocation} from "../../📥️invocation/🟦️.ts";
import {readCargoExecutionContractV1} from "../../../🧪️testing/🦀️cargo/🏃️execution/🟦️.ts";
import {readCargoTestPolicyV1,type CargoTestPolicyV1} from "../../../🧪️testing/🦀️cargo/🟦️.ts";
import type {CommandRequestV1} from "../📥️request/🟦️.ts";
import {isDeepStrictEqual} from "node:util";

/** 🎟️ Retains compiler policy only for an explicitly issued execution port and its exact child selection. */
export function inheritedCargoExecutionCustodyV1(request:CommandRequestV1,environment:Readonly<Record<string,string|undefined>>,invocation:ScriptInvocation):CargoTestPolicyV1|undefined {
  checkScriptInvocation(invocation);
  if(!("cargoExecutionBackend" in invocation.capabilities))return;
  const backend=readCargoExecutionContractV1("backend",invocation.capabilities.cargoExecutionBackend),selected=backend.request;
  const continuation=backend.continuation;
  if(selected.kind!=="tests"||!isAbsolute(backend.cwd)||!continuation||request.command!==continuation.program||request.cwd!==continuation.cwd||!isDeepStrictEqual(request.args,continuation.arguments)||request.cwd!==resolve(backend.cwd,selected.cwd)||request.manifests.length!==1||request.manifests[0]!==resolve(backend.cwd,selected.manifest))throw Error("Original delegated Cargo command selection differs");
  const policy=readCargoTestPolicyV1(environment);
  if(policy.manifestPath!==request.manifests[0]||policy.targetDirectory!==environment.CARGO_TARGET_DIR||policy.mode!==environment.SEMIO_BUILD_MODE)throw Error("Original delegated Cargo command policy differs");
  return policy;
}
