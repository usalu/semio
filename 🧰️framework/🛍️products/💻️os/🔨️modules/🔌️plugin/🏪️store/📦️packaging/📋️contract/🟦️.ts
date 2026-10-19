import {readComponentModelIntentV1,componentModelDirectoryKeyV1} from "@semio-tech/framework-plugin/model/contract";
import type { RepositoryArtifactRequestV1 } from "@semio-tech/repo-lib/artifacts";
import { artifactsStorageSchema as policy } from "@semio-tech/repo-lib/artifacts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {isAbsolute,join,resolve} from "node:path";
/** 📦️ Selects an extension package publication as finite DATA without reader or writer authority. */
export function extensionPackageRequestV1(productsRoot:string,value:unknown):RepositoryArtifactRequestV1{
 const intent=readComponentModelIntentV1(value),path=join(productsRoot,"browser",intent.mode,schema.definitions.Protocol.const.namespace,componentModelDirectoryKeyV1(intent.cratePath));if(!isAbsolute(productsRoot)||resolve(productsRoot)!==productsRoot||path.length>224)throw Error("Canonical bounded extension package path required");return Object.freeze({version:1,path,owner:intent.cratePath+"/Cargo.toml:extension-package",maximumBytes:policy.definitions.Protocol.const.maximumOutputBytes+policy.definitions.Protocol.const.maximumManifestBytes,maximumFiles:schema.definitions.Protocol.const.maximumFiles});
}
