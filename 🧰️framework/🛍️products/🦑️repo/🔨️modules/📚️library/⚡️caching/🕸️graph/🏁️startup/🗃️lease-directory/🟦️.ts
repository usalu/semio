/** 📂️ Graph lease directory: the shared short workspace store graph coordination leases live in, held apart so publication and startup share it without importing each other. */
import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {findWorkspaceRoot} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🗃️ Namespace identity stays in the key while physical coordination uses the actual shared short workspace store. */
export function graphResourceLeaseDirectoryV1():string{return machinePathV1(graphCheckoutRootV1(),".cache/b/l/m/data");}

/** 🧭️ The checkout this module serves: the workspace root at or above the module (the module lives in the checkout of a worker), else the declared workspace root (`SEMIO_WORKSPACE_ROOT`, else the working directory) when the module is loaded from outside every checkout, as the Nx tooling is on a machine with a declared state directory (`<state>/b/t/m/nx/<digest>`). @see https://nodejs.org/api/url.html#urlfileurltopathurl */
export function graphCheckoutRootV1():string{
 try{return findWorkspaceRoot(dirname(fileURLToPath(import.meta.url)));}catch{return findWorkspaceRoot();}
}
