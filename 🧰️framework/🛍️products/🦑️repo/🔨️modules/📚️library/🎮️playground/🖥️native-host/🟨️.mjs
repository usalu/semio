import { Buffer } from "node:buffer";
import { requireStoreReceiptV1, storeReceiptOutputV1 } from "../../🏗️build/🧾️store-receipt/🟨️.mjs";
import schemaContract from "./🧬️schema/🔣️.json" with { type: "json" };

const fields=["cratePath","project","package","binary","target"];
const identifier=/^[a-z0-9][a-z0-9-]{0,127}$/u;
const projectIdentifier=/^(?:@[a-z0-9][a-z0-9-]*\/)?[a-z0-9][a-z0-9-]*$/u;
const messages=JSON.parse(JSON.stringify(schemaContract.$defs.NativeHostMessagesV1.const));
const requiredModes=["prototype","release"];

/** 🌐️ The languages the refusal is rendered in: the chosen one (`SEMIO_LANGUAGE`, `LC_ALL`, `LANG`), both declared languages when none is chosen. */
export function nativeHostLanguagesV1(environment=process.env) {
  const pick=value=>{const code=String(value??"").toLowerCase().split(/[-_.:@]/u)[0];return messages.languages.includes(code)?code:undefined;};
  const chosen=pick(environment.SEMIO_LANGUAGE)??pick(environment.LC_ALL)??pick(environment.LANG);
  return chosen?[chosen]:[...messages.languages];
}

/** 🔎️ Lists every identity the declaration disagrees with in the authored owner facts. */
export function nativeHostMismatchesV1(host,facts) {
  const rows=[],authored=value=>Array.isArray(value)?`[${value.join(", ")}]`:JSON.stringify(value);
  if(facts.package!==host.package) rows.push(["package",host.package,authored(facts.package)]);
  if(facts.project!==host.project) rows.push(["project",host.project,authored(facts.project)]);
  if(!facts.binaries.includes(host.binary)) rows.push(["binary",host.binary,authored(facts.binaries)]);
  if(!facts.targets.includes(host.target)) rows.push(["target",host.target,authored(facts.targets)]);
  else if(!requiredModes.every(mode=>facts.modes[host.target]?.includes(mode))) rows.push(["modes",requiredModes.join(", "),authored(facts.modes[host.target]??[])]);
  return rows;
}

/** 💬️ Renders the refusal in every requested language, naming the declared project, its crate and each disagreeing identity. */
export function nativeHostMismatchTextV1(host,facts,context="",languages=nativeHostLanguagesV1()) {
  const fill=(text,values)=>text.replace(/\{([a-zA-Z]+)\}/gu,(_,key)=>String(values[key]));
  return languages.map(language=>{
    const table=messages.texts[language],labels=messages.fields[language],rows=nativeHostMismatchesV1(host,facts);
    const differences=rows.length?rows.map(([field,declared,found])=>fill(table.difference,{field:labels[field],declared,authored:found})).join("; "):table.none;
    return fill(table.mismatch,{context:context?` (${context})`:"",project:host.project,cratePath:host.cratePath,differences});
  }).join("\n");
}

/** 🖥 Admits one explicit native executable producer against its actual authored owner. */
export function admitPlaygroundNativeHostV1(value,resolveOwner,context="") {
  if(value===undefined) return undefined;
  if(value===null || typeof value!=="object" || Array.isArray(value) || Object.keys(value).length!==fields.length || fields.some(key=>!Object.hasOwn(value,key) || typeof value[key]!=="string")) throw new Error("Invalid native host declaration");
  const {cratePath,project,package:packageName,binary,target}=value;
  if([...cratePath].length>1024 || !cratePath || cratePath.startsWith("/") || /[\\:\u0000]/u.test(cratePath) || cratePath.split("/").some(part=>!part || part==="." || part==="..")) throw new Error("Native host must name a canonical workspace owner");
  if(!identifier.test(packageName) || !identifier.test(binary) || !identifier.test(target) || !projectIdentifier.test(project) || project.length>128) throw new Error("Invalid native host identity");
  const facts=resolveOwner(cratePath);
  if(nativeHostMismatchesV1(value,facts).length) throw new Error(nativeHostMismatchTextV1(value,facts,context));
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

/** 📏 Reports exactly the decoded UTF-8 script admission predicate consumed by host ownership. */
export function nativeHostScriptAdmissionV1(source) {return {withinLimit:new TextEncoder().encode(source).byteLength<=1024*1024};}

/** 🧾 Reads Cargo and portable project identity through a bounded caller-owned source view. */
export function nativeHostSourceFactsV1(cratePath,view) {
  const admitPath=(path)=>{
    const parts=path.split("/");
    for(let index=1;index<=parts.length;index++) {
      const node=parts.slice(0,index).join("/"),kind=view.kind(node);
      if(kind!==(index===parts.length?"file":"directory")) throw new Error(`Native host source must be present and no-follow: ${node}`);
    }
  };
  const read=(path)=>{
    admitPath(path);
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
  const script=`${cratePath}/📜️script.ts`;admitPath(script);
  const admission=view.admitScript(script);
  if(!admission||typeof admission!=="object"||Array.isArray(admission)||Object.keys(admission).length!==1||!Object.hasOwn(admission,"withinLimit")||typeof admission.withinLimit!=="boolean")throw Error("Invalid native host script admission");
  if(!admission.withinLimit)throw Error("Native host script is oversized");
  const targets=Object.keys(project.targets??{}).filter(name=>{
    const target=project.targets[name];
    return ["prototype","release"].every(mode=>target.configurations?.[mode]?.env?.SEMIO_BUILD_MODE===mode) && target.executor==="repo:script" && target.options?.cwd===cratePath && /^repo (?:\.\/)?📜️script\.ts [a-z0-9-]+(?: |$)/u.test(target.options?.command??"");
  });
  return {package:packageName,project:project.name,binaries,targets,modes:Object.fromEntries(targets.map(target=>[target,["prototype","release"]])),outputs:Object.fromEntries(targets.map(name=>[name,Object.fromEntries(["prototype","release"].map(mode=>[mode,(project.targets[name].configurations[mode].outputs??project.targets[name].outputs)?.map(output=>typeof output==="string"?output.replaceAll("{options.env.SEMIO_BUILD_MODE}",mode):output)]))]))};
}

/** 📦 Resolves the published executable through the store receipt the selected producer target declares as its only output: the receipt names the one published directory of the profile, the executable lies inside it. */
export function nativeHostArtifactPathV1(host,profile,facts,platform,receipt) {
  if(!["prototype","release"].includes(profile)) throw new Error("Native host profile must be exact");
  const outputs=facts.outputs[host.target]?.[profile];
  if(!Array.isArray(outputs) || outputs.length!==1 || outputs[0]!==storeReceiptOutputV1(host.target)) throw new Error("Native host must declare exactly its store receipt as output");
  const checked=requireStoreReceiptV1(receipt);
  if(checked.project!==host.project || checked.target!==host.target) throw new Error("Native host receipt belongs to another producer");
  const [published]=checked.entries;
  if(checked.entries.length!==1 || checked.links.length!==0 || published.kind!=="directory" || published.path.split("/").at(-1)!==profile) throw new Error("Native host receipt must name exactly one published directory of its profile");
  return `${published.path}/${host.binary}${platform==="win32"?".exe":""}`;
}

/** 🧾 Captures every selected producer input while admitting both executable host declarations. */
export function declaredPlaygroundHostInputPathsV1(manifest,view) {
  const inputs=new Set(),hostView={kind:view.kind.bind(view),readText:(path)=>{inputs.add(path);return view.readText(path);},admitScript:(path)=>{inputs.add(path);return view.admitScript(path);}};
  for(const block of manifest.matchAll(/^\[\[package\.metadata\.semio\.playground\]\]\s*$([\s\S]*?)(?=^\[|$(?![\s\S]))/gmu)) {
    for(const field of ["nativeHost","mcpHost"]) {
      admitPlaygroundNativeHostV1(parsePlaygroundNativeHostV1(block[1],field),(owner)=>nativeHostSourceFactsV1(owner,hostView));
    }
  }
  return [...inputs].sort((left,right)=>Buffer.from(left).compare(Buffer.from(right)));
}
