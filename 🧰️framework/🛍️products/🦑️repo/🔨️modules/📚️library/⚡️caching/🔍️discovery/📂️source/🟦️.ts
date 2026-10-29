import { readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join, sep } from "node:path";
import descriptorSchema from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json" with {type:"json"};
import retirementSchema from "../../🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json" with {type:"json"};

import authoredPolicy from "../../🔣️policy.json" with {type:"json"};
import {physicalSourceOriginV1,type PhysicalSourceModuleMetaV1} from "../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
export const CACHE_POLICY_PATH = fileURLToPath(new URL("../../🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)));
export const CACHE_POLICY = structuredClone(authoredPolicy);
export const SOURCE_OPACITY_PROTOCOL_V1=Object.freeze({prefixes:Object.freeze([descriptorSchema.definitions.RelativePreservationProtocol.const.capsulePrefix,retirementSchema.definitions.Policy.const.markerPrefix]),digestHex:descriptorSchema.definitions.RelativePreservationProtocol.const.capsuleDigestHex});

export interface CacheSourceEntry {
  readonly name: string;
  isDirectory(): boolean;
  isFile(): boolean;
  isSymbolicLink(): boolean;
}

export interface CacheSourceOperations {
  readDirectory(path: string): readonly CacheSourceEntry[];
}

export const slash = (path: string): string => path.split(sep).join("/");

/** 🫥️ Keeps schema-reserved preservation payloads and retirement journals outside authored Source. */
export function isOpaqueSourcePathV1(path:string):boolean {
  return path.split(/[\\/]/u).some(name=>SOURCE_OPACITY_PROTOCOL_V1.prefixes.some(prefix=>name.startsWith(prefix)&&name.length===prefix.length+SOURCE_OPACITY_PROTOCOL_V1.digestHex&&/^[a-f0-9]+$/u.test(name.slice(prefix.length))));
}

/** 🚫️ Emits equivalent glob exclusions for reserved files and opaque directory payloads. */
export function opaqueSourceInputExclusionsV1(root:string):string[] {
  return SOURCE_OPACITY_PROTOCOL_V1.prefixes.flatMap(prefix=>{
    const name=prefix+"[a-f0-9]".repeat(SOURCE_OPACITY_PROTOCOL_V1.digestHex);
    return [`!${root}/**/${name}`,`!${root}/**/${name}/**/*`];
  });
}

/** 📂️ Enumerates source entries without traversing generated stores, symlinks or opaque trees. */
export function sourceFiles(root: string, operations: CacheSourceOperations = { readDirectory: (path) => readdirSync(path, { withFileTypes: true }) }): string[] {
  const files: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of operations.readDirectory(join(root, directory))) {
      const path = slash(join(directory, entry.name));
      if (entry.isSymbolicLink() || isOpaqueSourcePathV1(path) || path === "compose" || path === "temp/compose" || entry.name === ".🧬semio" || CACHE_POLICY.generatedDirectories.includes(entry.name)) continue;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile()) files.push(path);
    }
  };
  walk("");
  return files.sort();
}
