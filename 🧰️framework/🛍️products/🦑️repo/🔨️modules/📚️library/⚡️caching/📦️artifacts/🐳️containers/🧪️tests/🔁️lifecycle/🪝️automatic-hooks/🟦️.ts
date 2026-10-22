/** 🪝️ Verifies dashboard attach never installs automatic Git hooks through its actual recording host. */
import {test} from "bun:test";
import {testDevcontainerDashboardOwnership} from "../🟦️.ts";
test("devcontainer attach preserves dashboard control without automatic Git hooks",()=>testDevcontainerDashboardOwnership(process.cwd()));
