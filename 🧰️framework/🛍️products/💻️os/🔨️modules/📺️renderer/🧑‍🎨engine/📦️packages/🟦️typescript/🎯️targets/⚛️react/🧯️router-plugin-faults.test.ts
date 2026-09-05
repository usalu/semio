/** 🧯️ The shell's catalog probe must expose the `AppRouter` fault that excluded a plugin, and every
 * other plugin must keep routing — driven by the SAME language-neutral vectors the kernel `AppRouter`
 * and its Rust twin consume (`🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🔣️.json`), with strict
 * Ajv as the independent oracle for the fixture's own shape. */

import Ajv from "ajv";
import { AppRouter, type AppRouterManifest } from "@semio-tech/framework";
import { describe, expect, it } from "vitest";
import { shellCatalogProbePlugins } from "../../../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import routerFaultSchema from "../../../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🧬️.schema.json";
import routerFaultFixture from "../../../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🔣️.json";

describe("shell catalog probe router faults", () => {
  it("accepts the shared fixture under a strict independent schema oracle", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(routerFaultSchema);
    expect(validate(routerFaultFixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("reports every excluded plugin on its own probe row and leaves the others healthy", () => {
    const router = AppRouter.build(routerFaultFixture.manifests as readonly AppRouterManifest[]);
    const registry = routerFaultFixture.manifests.map((manifest) => ({ pluginId: manifest.pluginId }));
    const rows = shellCatalogProbePlugins(registry, Object.fromEntries(registry.map((entry) => [entry.pluginId, "loaded"])), router);
    expect(rows.map((row) => row.pluginId)).toEqual(registry.map((entry) => entry.pluginId));
    expect(rows.every((row) => row.status === "loaded")).toBe(true);
    expect(rows.filter((row) => row.routerFault).map((row) => ({ pluginId: row.pluginId, code: row.routerFault!.code }))).toEqual(
      routerFaultFixture.expectedFaults.map((fault) => ({ pluginId: fault.pluginId, code: fault.code })),
    );
    for (const row of rows.filter((candidate) => candidate.routerFault)) expect(row.routerFault!.message, row.pluginId).toContain(row.pluginId);
  });
});
