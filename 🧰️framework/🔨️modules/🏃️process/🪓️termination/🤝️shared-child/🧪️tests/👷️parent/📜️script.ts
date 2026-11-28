import { createSharedChildRegistryV1, sharedChildNodeHostV1, spawnSharedChildV1 } from "../../🟦️.ts";

const registry = createSharedChildRegistryV1(sharedChildNodeHostV1("shared cargo"));
const lease = registry.acquire("cargo:target", "task-a", () => spawnSharedChildV1("sh", ["-c", "sleep 300 & wait"], { stdio: "ignore" }));
console.log(JSON.stringify({ parent: process.pid, child: lease.child.pid }));
setInterval(() => {}, 1 << 30);
