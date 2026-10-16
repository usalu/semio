#!/usr/bin/env bun
import {createIssuedResourceControlV1} from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🏭️ Collects only this artifact's actual production inventory under Original compiler custody. */
import {join} from "node:path";
import {runRepositoryMutationBridgeScriptV1} from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏭️inventory/🟦️.ts";
import {runRepositoryCargoCommandV1} from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import {getWorkspaceRoot} from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import collector from "./🔣️collector.json" with {type:"json"};
const packageRoot=join(import.meta.dir,"📦️packages/🦀️rust"),binary="semio-space-space-mutation-bridge";
if(process.argv[2]==="check"){
 if(process.argv.length!==3)throw Error("check accepts no overrides");
 await receiveScriptProcessInvocation(process.env,original=>runRepositoryCargoCommandV1(original,createIssuedResourceControlV1(original,"cargo:bridge",0),getWorkspaceRoot(),["test","--manifest-path",join(packageRoot,"Cargo.toml"),"--package","semio-s-artifact-space-space-mutation-bridge","--offline","--locked","--bin",binary,"--","--nocapture"],import.meta.dir,process.env,collector.maximumMilliseconds));
}else await runRepositoryMutationBridgeScriptV1(packageRoot,binary,{...collector,version:1});
