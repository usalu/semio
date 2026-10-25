/** 📂️ Graph lease directory: the shared short workspace store graph coordination leases live in, held apart so publication and startup share it without importing each other. */
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {findWorkspaceRoot} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🗃️ Namespace identity stays in the key while physical coordination uses the actual shared short workspace store. */
export function graphResourceLeaseDirectoryV1():string{return join(findWorkspaceRoot(dirname(fileURLToPath(import.meta.url))),".cache/b/l/m/data");}
