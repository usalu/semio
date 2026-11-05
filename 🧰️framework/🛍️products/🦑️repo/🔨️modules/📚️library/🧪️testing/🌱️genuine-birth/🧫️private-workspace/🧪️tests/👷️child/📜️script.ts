import assert from "node:assert/strict";
import {relative,isAbsolute} from "node:path";
import {receivePrivateNativeTestProcessV1} from "../../🟦️.ts";
import {withRepositoryCacheStoreV1} from "../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";

await receivePrivateNativeTestProcessV1("native-birth-witness",async({original,control,workspace,codeRoot})=>{
 assert.notEqual(workspace,codeRoot);assert.equal(process.cwd(),workspace);
 await withRepositoryCacheStoreV1(workspace,"products",original,control,async store=>{const local=relative(workspace,store.root);assert.ok(local!==""&&!local.startsWith("..")&&!isAbsolute(local));});
});
