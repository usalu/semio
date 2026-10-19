import {withDescriptorEmitterV1,withDescriptorEmitterTaskV1,type DescriptorEmitterV1} from "../🛠️emitter/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readArtifactBytesV1} from "../../🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import { checkResourceLeaseControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1, requestedCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { closeSync, lstatSync, mkdtempSync, openSync, readFileSync, readSync, realpathSync, rmSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";
import { devToolingEnv } from "@semio-tech/process/execution";
import { captureOwnedCommandV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { CATALOG_DESCRIPTOR_MAX_BYTES } from "@semio-tech/plugin-registry/catalog-verification";
import { verifySemanticDescriptorPairBytesV1 } from "@semio-tech/plugin-registry/catalog-verification/descriptor-pair";
import { ITERATE_COMPONENT_MAX_BYTES } from "../🏷️component-receipt/🟦️.ts";
import { CRATE_NAME, DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME, FRESH_IO_CHUNK_BYTES } from "../🏗️component-build/🟦️.ts";
import { freshPathIsWithin } from "../🧾️source-epoch/🟦️.ts";
export type DescriptorEmissionRequestV1 = Readonly<{
  component: string;
  artifactRoot: string;
}>;

export type SemanticDescriptorEmissionV1 = Readonly<{
  pluginId: string;
  packageId: string;
  role: "plugin" | "extension";
  version: string;
  componentSha256: string;
  jsonBytes: Uint8Array;
  packBytes: Uint8Array;
  semanticSha256: string;
}>;

export type DescriptorEmissionValidationControlV1 = Readonly<{
  compiler: CompilerAdmissionControl;
  environment?: Readonly<Record<string, string | undefined>>;
  cancelled?: () => boolean;
  deadlineMs?: number;
  checkpoint?: (stage: string) => void;
}>;

export type DescriptorEmissionControlV1=DescriptorEmissionValidationControlV1&Readonly<{invocation:ScriptInvocation;emitter:DescriptorEmitterV1}>;
export class DescribeScript extends BundleScript {
 async run(segments:string[]):Promise<void>{const control=scriptCompilerAdmissionControlV1(this.invocation,"descriptor:command",0);await withDescriptorEmitterV1(this.repoRoot,this.invocation,control,emitter=>withDescriptorEmitterTaskV1(emitter,this.invocation,async tool=>{await captureOwnedCommandV1(tool.path,["describe",...segments],this.repoRoot,{control:tool.control,env:devToolingEnv()});}));}
}

/** ⏱️ Carries the descriptor emission budget into each child process. */
export function remainingDescriptorEmissionBudgetMs(budgetMs: number, elapsedMs: number): number {
  return budgetMs === 0 ? 0 : Math.max(1, budgetMs - elapsedMs);
}

function emissionGuard(control: DescriptorEmissionValidationControlV1, startedAt: number, budgetMs: number, stage: string): void {
  checkResourceLeaseControl(control.compiler);
  if (control.cancelled?.()) throw new Error(`descriptor emission cancelled at ${stage}`);
  if (budgetMs !== 0 && Date.now() - startedAt > budgetMs) throw new Error(`descriptor emission deadline of ${budgetMs}ms exceeded at ${stage}`);
  control.checkpoint?.(stage);
}

function emissionDirectory(root: string, path: string, label: string): string {
  const exact = resolve(path);
  const info = lstatSync(exact);
  if (info.isSymbolicLink() || !info.isDirectory()) throw new Error(`${label} ${exact} must be a regular non-symlink directory`);
  const real = realpathSync(exact);
  if (!freshPathIsWithin(realpathSync(root), real)) throw new Error(`${label} ${exact} resolves outside ${root}`);
  return real;
}

function emissionArtifact(root: string, path: string, label: string, maximum: number): { readonly path: string; readonly sha256: string } {
  const exact = resolve(path);
  const info = lstatSync(exact);
  if (info.isSymbolicLink() || !info.isFile() || info.size === 0 || info.size > maximum) throw new Error(`${label} ${exact} must be a regular non-symlink file of 1..${maximum} bytes`);
  const real = realpathSync(exact);
  if (!freshPathIsWithin(realpathSync(root), real)) throw new Error(`${label} ${exact} resolves outside the declared artifact root ${root}`);
  const handle = openSync(real, "r");
  const chunk = Buffer.allocUnsafe(FRESH_IO_CHUNK_BYTES);
  const hash = createHash("sha256");
  try {
    let read = 0;
    while (read < info.size) {
      const count = readSync(handle, chunk, 0, Math.min(chunk.byteLength, info.size - read), read);
      if (count === 0) throw new Error(`${label} ${exact} shrank while being hashed`);
      hash.update(chunk.subarray(0, count));
      read += count;
    }
  } finally {
    chunk.fill(0);
    closeSync(handle);
  }
  if (statSync(real).size !== info.size) throw new Error(`${label} ${exact} changed while being hashed`);
  return { path: real, sha256: hash.digest("hex") };
}

/** 🛂️ The semantic descriptor emission contract: runs the published emitter once over ONE component inside `artifactRoot`
 * (a crate's deliverable directory, never the source tree) and returns the canonical semantic `🛂️.descriptor.semio` +
 * `🔣️.json` pair in memory, strict-decoded, compared form against form and refused if it carries any byte identity or a
 * placeholder identity. Nothing is published here: the describe stage decides whether the pair is verified against the
 * committed one, written over it, or kept local (ticket 26/09/23 wave A2). Every stage is cancellable and deadline bounded. */
/** 📏️ Validates bounded filesystem inputs and deadline DATA without selecting or executing any emitter. */
export async function validateDescriptorEmissionInputV1(repoRoot:string,request:DescriptorEmissionRequestV1,control:DescriptorEmissionValidationControlV1):Promise<Readonly<{artifactRoot:string;component:Readonly<{path:string;sha256:string}>;startedAt:number;budgetMs:number}>>{
 const startedAt=Date.now(),budgetMs=control.deadlineMs??buildBudgetMs();emissionGuard(control,startedAt,budgetMs,"validate");const artifactRoot=emissionDirectory(repoRoot,request.artifactRoot,"artifact root"),component=emissionArtifact(artifactRoot,request.component,"component",ITERATE_COMPONENT_MAX_BYTES);emissionGuard(control,startedAt,budgetMs,"emit");return Object.freeze({artifactRoot,component,startedAt,budgetMs});
}
export function emitSemanticDescriptorV1(repoRoot:string,request:DescriptorEmissionRequestV1,control:DescriptorEmissionControlV1):Promise<SemanticDescriptorEmissionV1>{
 requireIssuedResourceControlInvocationV1(control.compiler,control.invocation);return withDescriptorEmitterTaskV1(control.emitter,control.invocation,async tool=>{
  const {artifactRoot,component,startedAt,budgetMs}=await validateDescriptorEmissionInputV1(repoRoot,request,control),phase=requestedCompilerAdmissionControlV1(tool.control,compilerAdmissionBudgetV1(control.compiler)),staging=mkdtempSync(join(artifactRoot,".🛂️descriptor-staging-"));let primaryFailed=false,primaryFailure:unknown,result:SemanticDescriptorEmissionV1|undefined;
  try{emissionGuard(control,startedAt,budgetMs,"describe");await captureOwnedCommandV1(tool.path,["describe",component.path,"--out",staging],repoRoot,{env:control.environment??devToolingEnv(),control:phase});emissionGuard(control,startedAt,budgetMs,"verify");const packBytes=await readArtifactBytesV1(join(staging,DESCRIPTOR_PACK_FILENAME),CATALOG_DESCRIPTOR_MAX_BYTES,phase.signal),jsonBytes=await readArtifactBytesV1(join(staging,DESCRIPTOR_JSON_FILENAME),CATALOG_DESCRIPTOR_MAX_BYTES,phase.signal),pair=verifySemanticDescriptorPairBytesV1(jsonBytes,packBytes);await tool.recheck();result={pluginId:pair.pluginId,packageId:pair.packageId,role:pair.role,version:pair.version,componentSha256:component.sha256,jsonBytes,packBytes,semanticSha256:pair.semanticSha256};}
  catch(error){primaryFailed=true;primaryFailure=error;}finally{try{rmSync(staging,{recursive:true,force:true});}catch(error){if(!primaryFailed){primaryFailed=true;primaryFailure=error;}}}if(primaryFailed)throw primaryFailure;return result!;
 });
}
