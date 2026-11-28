/** 🎯️ A spawned program whose instance was retired no longer is the export/sync target. */
import { test, expect } from "bun:test";
import { resolveSyncTargetSessionV1 } from "../🟦️.ts";

const app = { id: "table", controllerId: "c" } as never;
const primary = { pluginId: "home", instanceId: 1, app: { id: "home" }, viewState: {} } as never;
const panel = { activeSpawnedId: "s", spawnedApps: [{ id: "a", pluginId: "stdio-office", appId: "table", instanceId: 2 }, { id: "s", pluginId: "stdio-office", appId: "table", instanceId: 3 }] } as never;
const loadedPlugins = [{ handle: { pluginId: "stdio-office" }, manifest: { apps: [app] } }];
const resolve = (retired: (instanceId: number) => boolean) => resolveSyncTargetSessionV1({ session: primary, panel, hostMode: true, loadedPlugins, retired: (_pluginId, instanceId) => retired(instanceId) });

test("a live active spawned instance is the target", () => expect(resolve(() => false)?.instanceId).toBe(3));
test("a retired active instance moves the target to the live program A", () => expect(resolve((id) => id === 3)?.instanceId).toBe(2));
test("with every spawned instance retired the primary session is the target", () => expect(resolve(() => true)).toBe(primary));
test("without an active spawned program the primary session stays the target", () => expect(resolveSyncTargetSessionV1({ session: primary, panel: { ...(panel as object), activeSpawnedId: undefined } as never, hostMode: true, loadedPlugins, retired: () => false })).toBe(primary));
test("no session has no target", () => expect(resolveSyncTargetSessionV1({ session: null, panel, hostMode: true, loadedPlugins, retired: () => false })).toBeNull());
