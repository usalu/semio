/** 🪢️ Contract: the repository Cargo profile policy renders `trim-paths` into every profile, and cargo accepts that key only when the workspace root declares `cargo-features = ["trim-paths"]`, so every tracked or unignored workspace root must declare it. Independent oracle: `git ls-files` lists the manifests and `@iarna/toml` parses them. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join,resolve}from"node:path";
import TOML from"@iarna/toml";
import{renderCargoProfileConfigurationV1}from"../../🏗️build/🎚️mode/🦀️profile/🟦️.ts";

const repoRoot=resolve(import.meta.dir,"../../../../../../.."),skipped=["/🎫️tickets/","/target/","/.cache/"];

test("the profile policy renders trim-paths for every mode, so each workspace root must carry the cargo feature",()=>{
 for(const mode of["prototype","release"]as const)expect(renderCargoProfileConfigurationV1({version:1,mode,target:"x86_64-unknown-linux-gnu",producer:"artifact",operation:"check"},"Cargo.toml","isolated").selectors).toContain(`profile.${mode}.trim-paths="object"`);
});

test("every workspace root manifest declares cargo-features trim-paths",()=>{
 const listed=Bun.spawnSync(["git","ls-files","-z","--cached","--others","--exclude-standard","--","*Cargo.toml"],{cwd:repoRoot}).stdout.toString().split("\0").filter(Boolean),roots:string[]=[],missing:string[]=[];
 for(const path of listed){if(skipped.some(part=>`/${path}`.includes(part)))continue;let text:string;try{text=readFileSync(join(repoRoot,path),"utf8");}catch{continue;}
  const manifest=TOML.parse(text)as{workspace?:unknown;"cargo-features"?:unknown};if(manifest.workspace===undefined)continue;roots.push(path);
  const features=manifest["cargo-features"];if(!Array.isArray(features)||!features.includes("trim-paths"))missing.push(path);}
 expect(roots.length).toBeGreaterThan(0);expect(missing).toEqual([]);
});
