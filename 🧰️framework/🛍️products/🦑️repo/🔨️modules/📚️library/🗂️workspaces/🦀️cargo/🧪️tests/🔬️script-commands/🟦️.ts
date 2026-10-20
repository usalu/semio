import { join, isAbsolute } from "node:path";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { runtimeInputAdmissionV1 } from "../../../../🕸️dependencies/🧩️runtime/🟨️.mjs";
import { Script } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../🟦️.ts";
import { publishCargoWorkspaceMemberships, cargoRepositoryPackages } from "../../🟦️.ts";



/** 📣️ Checks or publishes current native owner membership through its authored regular-manifest recipe. */
export class MembersScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length !== 1 || !["--check", "--write"].includes(args[0]!)) throw new Error("members --check|--write");
    for (const { owner, changed } of publishCargoWorkspaceMemberships(getWorkspaceRoot(), args[0] === "--write" ? "write" : "check")) {
      console.log(`[cargo-members] ${owner.manifest} ${changed ? "published" : "current"}`);
    }
  }
}


export class RuntimeInputScript extends Script {
  async run(args: string[]): Promise<void> {
    if(args.length!==1)throw new Error("runtime-input-check requires one source request");
    const request=JSON.parse(Buffer.from(args[0]!,"base64").toString("utf8"));
    if(!request || typeof request!=="object" || Object.keys(request).some(k=>!["component","appScoped","sources"].includes(k)) || typeof request.component!=="string" || typeof request.appScoped!=="boolean" || !Array.isArray(request.sources) || request.sources.some((path:unknown)=>typeof path!=="string" || isAbsolute(path) || path.split("/").includes("..") || path.includes("\\")))throw new Error("Invalid selected runtime input request");
    const components=cargoRepositoryPackages(getWorkspaceRoot()).flatMap(pkg=>{const document=Bun.TOML.parse(readFileSync(join(getWorkspaceRoot(),pkg.manifest),"utf8")) as any,metadata=document.package?.metadata;if(!["plugin","extension"].includes(metadata?.semio?.["component-kind"]))return [];const row=metadata.semio;return [{...row,pluginId:metadata.component.package.slice(6),dependsOn:[...(row.extends?[row.extends]:[]),...(row["depends-on"]??[])]}];});
    const admission=runtimeInputAdmissionV1(components,[{id:request.component,appScoped:request.appScoped}],request.sources,(path:string)=>{let current=getWorkspaceRoot();for(const part of path.split("/").filter(Boolean)){current=join(current,part);if(!existsSync(current) || lstatSync(current).isSymbolicLink())return false;}return lstatSync(current).isFile();});
    if(admission.status!=="admitted")throw new Error(`Selected runtime input refused: ${JSON.stringify(admission.missing)}`);
    console.log(`[runtime-input] ${request.component} ${admission.selected.length} admitted components`);
  }
}
export class CapabilityContractScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("capability-contract-check accepts no overrides");
    const { runCargoCapabilityContributionChecks, runCargoCapabilityPhysicalChecks } = await import("../../🧩️capabilities/🧪️tests/🟦️.ts");
    console.log(`cargo capability contract: ${runCargoCapabilityContributionChecks() + runCargoCapabilityPhysicalChecks()} laws`);
  }
}
