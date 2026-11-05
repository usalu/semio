/** 🧭️ Runs the authored shooting route contract in the registered browser-source suite. */
import {test} from "bun:test";
import {testShootingRouteV1} from "../🟦️.ts";
import {findWorkspaceRoot} from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";

test("shooting route retains its Specific contribution after hub composition",()=>testShootingRouteV1(findWorkspaceRoot()));
