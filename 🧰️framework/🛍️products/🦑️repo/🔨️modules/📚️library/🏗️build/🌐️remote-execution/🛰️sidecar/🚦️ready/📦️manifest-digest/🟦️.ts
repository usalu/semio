import{createHash}from"node:crypto";

const FIELDS=["name","version","dependencies","devDependencies","peerDependencies","peerDependenciesMeta","optionalDependencies","bundledDependencies","bundleDependencies","workspaces","overrides","resolutions","patchedDependencies","trustedDependencies","packageManager","engines","os","cpu","bin","catalog","catalogs"] as const,LIFECYCLE=["preinstall","install","postinstall","prepare","prepublish","preprepare","postprepare"] as const;

const canonical=(value:unknown):unknown=>Array.isArray(value)?value.map(canonical):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort().map(key=>[key,canonical((value as Record<string,unknown>)[key])])):value;

/** 📦️ The digest of a dependency manifest that a dependency sync can notice: the fields that decide what is installed and which install scripts run, canonically ordered, so an edit of its other fields (scripts that are not install hooks, description, exports, a reordered key) does not make the `dependencies` stage run again on every machine; a text that is not a JSON object digests as its raw bytes, so nothing unreadable is ever ignored.
 * @see ./🧪️tests/🟦️.ts */
export function readyManifestDigestV1(content:string|Uint8Array):string{
 const text=typeof content==="string"?content:Buffer.from(content).toString("utf8");
 try{
  const row=JSON.parse(text) as Record<string,unknown>;
  if(row===null||typeof row!=="object"||Array.isArray(row))throw Error("object");
  const kept:Record<string,unknown>={};
  for(const field of FIELDS)if(Object.hasOwn(row,field))kept[field]=row[field];
  const scripts=row.scripts;
  if(scripts!==null&&typeof scripts==="object"){const hooks:Record<string,unknown>={};for(const hook of LIFECYCLE)if(Object.hasOwn(scripts,hook))hooks[hook]=(scripts as Record<string,unknown>)[hook];if(Object.keys(hooks).length>0)kept.scripts=hooks;}
  return createHash("sha256").update(JSON.stringify(canonical(kept))).digest("hex");
 }catch{return createHash("sha256").update(typeof content==="string"?Buffer.from(content):content).digest("hex");}
}
