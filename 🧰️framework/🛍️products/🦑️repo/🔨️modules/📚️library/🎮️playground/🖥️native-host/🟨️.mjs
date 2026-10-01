import { Buffer } from "node:buffer";

const fields=["cratePath","project","package","binary","target"];
const identifier=/^[a-z0-9][a-z0-9-]{0,127}$/u;
const projectIdentifier=/^(?:@[a-z0-9][a-z0-9-]*\/)?[a-z0-9][a-z0-9-]*$/u;

/** 🖥 Admits one explicit native executable producer against its actual authored owner. */
export function admitPlaygroundNativeHostV1(value,resolveOwner) {
  if(value===undefined) return undefined;
  if(value===null || typeof value!=="object" || Array.isArray(value) || Object.keys(value).length!==fields.length || fields.some(key=>!Object.hasOwn(value,key) || typeof value[key]!=="string")) throw new Error("Invalid native host declaration");
  const {cratePath,project,package:packageName,binary,target}=value;
  if([...cratePath].length>1024 || !cratePath || cratePath.startsWith("/") || /[\\:\u0000]/u.test(cratePath) || cratePath.split("/").some(part=>!part || part==="." || part==="..")) throw new Error("Native host must name a canonical workspace owner");
  if(!identifier.test(packageName) || !identifier.test(binary) || !identifier.test(target) || !projectIdentifier.test(project) || project.length>128) throw new Error("Invalid native host identity");
  const facts=resolveOwner(cratePath);
  if(facts.package!==packageName || facts.project!==project || !facts.binaries.includes(binary) || ![target,`${target}-release`].every(name=>facts.targets.includes(name))) throw new Error("Native host does not match its authored producer");
  return Object.freeze({cratePath,project,package:packageName,binary,target});
}

/** 📜 Reads the canonical inline declaration without a domain or executable-name fallback. */
export function parsePlaygroundNativeHostV1(block,fieldName="nativeHost") {
  if(!["nativeHost","mcpHost"].includes(fieldName))throw new Error("Undeclared executable host field");
  const sourceBlock=fieldName==="nativeHost"?block:block.split("\n").filter(line=>!/^\s*nativeHost\s*=/u.test(line)).join("\n").replace(/^\s*mcpHost\s*=/gmu,"nativeHost =");
  const declarations=sourceBlock.split("\n").filter(line=>/^\s*nativeHost\s*=/u.test(line));
  if(declarations.length===0) return undefined;
  const match=declarations.length===1 && /^\s*nativeHost\s*=\s*\{(.*)\}\s*$/u.exec(declarations[0]);
  if(!match) throw new Error("Malformed or duplicate native host declaration");
  const value={},source=match[1],member=/\s*([a-zA-Z][a-zA-Z0-9]*)\s*=\s*("(?:[^"\\]|\\.)*")\s*(,|$)/uy;
  let index=0;
  while(index<source.length) {
    member.lastIndex=index;
    const field=member.exec(source);
    if(!field || !fields.includes(field[1]) || Object.hasOwn(value,field[1])) throw new Error("Malformed native host field");
    value[field[1]]=JSON.parse(field[2]);
    index=member.lastIndex;
    if(field[3]==="," && !source.slice(index).trim()) throw new Error("Native host trailing comma is not canonical");
  }
  if(Object.keys(value).length!==fields.length) throw new Error("Native host declaration is incomplete");
  return value;
}

/** 🧾 Reads actual Cargo and Nx identity through a bounded caller-owned source view. */
export function nativeHostSourceFactsV1(cratePath,view) {
  const read=(path)=>{
    const parts=path.split("/");
    for(let index=1;index<=parts.length;index++) {
      const node=parts.slice(0,index).join("/"),kind=view.kind(node);
      if(kind!==(index===parts.length?"file":"directory")) throw new Error(`Native host source must be present and no-follow: ${node}`);
    }
    const source=view.readText(path);
    if(new TextEncoder().encode(source).byteLength>1024*1024) throw new Error("Native host metadata is oversized");
    return source;
  };
  const cargo=read(`${cratePath}/Cargo.toml`),project=JSON.parse(read(`${cratePath}/📋️project.json`));
  const packageBlock=/^\[package\]\s*$([\s\S]*?)(?=^\[|$(?![\s\S]))/mu.exec(cargo)?.[1];
  const names=packageBlock?.match(/^name\s*=\s*"([^"\r\n]+)"\s*$/gmu);
  if(names?.length!==1 || project.root!==cratePath || typeof project.name!=="string") throw new Error("Native host has invalid source ownership");
  const packageName=/"([^"]+)"/u.exec(names[0])[1];
  const binaries=[...cargo.matchAll(/^\[\[bin\]\]\s*$([\s\S]*?)(?=^\[|$(?![\s\S]))/gmu)].map(match=>{
    const rows=match[1].match(/^name\s*=\s*"([^"\r\n]+)"\s*$/gmu);
    if(rows?.length!==1) throw new Error("Native host binary identity must be exact");
    return /"([^"]+)"/u.exec(rows[0])[1];
  });
  read(`${cratePath}/📜️script.ts`);
  const targets=Object.keys(project.targets??{}).filter(name=>{
    const target=project.targets[name];
    return target.executor==="nx:run-commands" && target.options?.cwd===cratePath && /^bun (?:\.\/)?📜️script\.ts [a-z0-9-]+(?: |$)/u.test(target.options?.command??"");
  });
  return {package:packageName,project:project.name,binaries,targets,outputs:Object.fromEntries(targets.map(target=>[target,project.targets[target].outputs]))};
}

/** 📦 Resolves one exact published artifact directory from the selected producer target. */
export function nativeHostArtifactPathV1(host,profile,facts,platform) {
  if(!["dev","release"].includes(profile)) throw new Error("Native host profile must be exact");
  const target=host.target+(profile==="release"?"-release":""), outputs=facts.outputs[target];
  if(!Array.isArray(outputs) || outputs.length!==1 || typeof outputs[0]!=="string" || !outputs[0].startsWith("{projectRoot}/")) throw new Error("Native host must publish exactly one owned directory");
  const output=outputs[0].slice("{projectRoot}/".length);
  if(!output || /[{}\\:\u0000]/u.test(output) || output.split("/").some(part=>!part || part==="." || part==="..")) throw new Error("Native host output is outside its producer");
  return `${host.cratePath}/${output}/${host.binary}${platform==="win32"?".exe":""}`;
}

/** 🧾 Captures every selected producer input while admitting both executable host declarations. */
export function declaredPlaygroundHostInputPathsV1(manifest,view) {
  const inputs=new Set(),hostView={kind:view.kind.bind(view),readText:(path)=>{inputs.add(path);return view.readText(path);}};
  for(const block of manifest.matchAll(/^\[\[package\.metadata\.semio\.playground\]\]\s*$([\s\S]*?)(?=^\[|$(?![\s\S]))/gmu)) {
    for(const field of ["nativeHost","mcpHost"]) {
      admitPlaygroundNativeHostV1(parsePlaygroundNativeHostV1(block[1],field),(owner)=>nativeHostSourceFactsV1(owner,hostView));
    }
  }
  return [...inputs].sort((left,right)=>Buffer.from(left).compare(Buffer.from(right)));
}
