/** 🎯️ A spawned program whose instance was retired no longer is the export/sync target. */
import { test, expect } from "bun:test";
import { resolveSyncTargetSessionV1 } from "../🟦️.ts";

const app = { id: "table", controllerId: "c" } as never;
const primary = { pluginId: "home", instanceId: 1, app: { id: "home" }, viewState: {} } as never;
const panel = { activeSpawnedId: "s", spawnedApps: [{ id: "s", pluginId: "stdio-office", appId: "table", instanceId: 3 }] } as never;
const loadedPlugins = [{ handle: { pluginId: "stdio-office" }, manifest: { apps: [app] } }];
const resolve = (retired: boolean) => resolveSyncTargetSessionV1({ session: primary, panel, hostMode: true, loadedPlugins, retired: () => retired });

test("a live spawned instance is the target", () => expect(resolve(false)?.instanceId).toBe(3));
test("a retired spawned instance falls back to the primary session", () => expect(resolve(true)).toBe(primary));
test("no session has no target", () => expect(resolveSyncTargetSessionV1({ session: null, panel, hostMode: true, loadedPlugins, retired: () => false })).toBeNull());
