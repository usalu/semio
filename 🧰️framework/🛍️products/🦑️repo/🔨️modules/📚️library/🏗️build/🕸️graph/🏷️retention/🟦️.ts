import {type BuildOwnerRegistryV1} from "../🚪️admission/🟦️.ts";
import {createHash} from "node:crypto";
import generationSchemaV1 from "../🕰️generations/🧬️schema/🔣️.json" with{type:"json"};


/** 🪤️ The tags of the owners the test and policy discovery produces. An owner with one of them is kept from generation to generation and holds the observations of the whole discovery, so it is discovered again only when one of them is stale, the environment moved or an input of the discovery was added, removed or changed; an owner with another tag (the tags before the observations were kept) is discovered again once. */
export const TEST_TAG_V1="tests-v2",POLICY_TAG_V1="policy-v2";


/** ❓️ Whether the test and policy owners of the previous generation are kept as they are: there is a previous generation whose owners carry the current tags, the environment did not move, no tagged owner is dirty (its root is among the roots being refreshed) and no input of the discovery (a `.feature` file or a `📜️script.ts`) was added, removed or changed since; an unknown change list counts as a change. */
export function taggedOwnersRetainedV1({cold,changed,roots,stubs,environment,previousEnvironment}:{cold:boolean;changed:readonly string[]|undefined;roots:ReadonlySet<string>;stubs:Record<string,{root:string;tag?:string}>;environment:string;previousEnvironment:string|null}):boolean{
  const tagged=Object.values(stubs).filter(stub=>stub.tag!==undefined);
  return !cold&&changed!==undefined&&tagged.length>0&&tagged.every(stub=>stub.tag===TEST_TAG_V1||stub.tag===POLICY_TAG_V1)&&environment===previousEnvironment&&!tagged.some(stub=>roots.has(stub.root))&&!changed.some(path=>path.endsWith(".feature")||path.endsWith("📜️script.ts"));
}


/** 🪪️ The identity of the generation a registry may read: the store format and the catalog providers whose output reaches every project. The owners of the registry are no identity: an added or removed project is a membership change the generation absorbs incrementally, and the one owner row an inference reads is a recorded `registry` fact. */
export function buildGraphGenerationIdentityV1(registry:BuildOwnerRegistryV1):string{
  return createHash("sha256").update(JSON.stringify([generationSchemaV1.definitions.Identity.const,registry.catalogProviders.map(({ownerRoot,sourcePath})=>[ownerRoot,sourcePath])])).digest("hex");
}


/** 🕓️ Whether a changed workspace file is a crate manifest that has no build project owner yet: a `Cargo.toml` that is no owner manifest, owner root or native root of the registry and declares no workspace. Such a crate is pending: it neither makes the workspace stale nor enters the graph until its owner row exists (a removed manifest is never pending, the environment rows of a removed workspace manifest stale it on their own). */
export async function pendingCrateManifestV1(registry:BuildOwnerRegistryV1,path:string,read:(path:string)=>Promise<string|undefined>):Promise<boolean>{
  if(path.slice(path.lastIndexOf("/")+1)!=="Cargo.toml")return false;
  const directory=path.includes("/")?path.slice(0,path.lastIndexOf("/")):".";
  if(registry.owners.some(owner=>owner.manifest===path||owner.root===directory||owner.nativeRoots?.includes(directory)))return false;
  const text=await read(path);
  return text!==undefined&&!text.includes("[workspace");
}
