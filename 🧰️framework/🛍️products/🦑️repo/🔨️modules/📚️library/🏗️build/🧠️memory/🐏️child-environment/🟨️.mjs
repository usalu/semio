import memory from "../🧬️schema/🔣️.json" with {type:"json"};
import {HOST_POLICY_V1} from "../../🏡️resident/📨️protocol/🎚️policy/🟨️.mjs";

/** 🐏️ The environment that caps a process reading the project graph: the collector target of a graph child (`childHeapMegabytes`, `BUN_JSC_forceRAMSize`) and the allocator purge of the host policy (`memoryPurge`), both declared in schema and free of heavy imports so the sealed front door may load it before it starts a command.
 * @see ../🧬️schema/🔣️.json
 * @see ../../🏡️resident/🧬️schema/🔣️.json */
export function graphChildEnvironmentV1(policy=memory.definitions.Policy.const,purge=HOST_POLICY_V1.memoryPurge){
 return Object.freeze({[policy.heapEnvironment]:String(policy.childHeapMegabytes*1048576),[purge.environmentVariable]:String(purge.delayMilliseconds)});
}
