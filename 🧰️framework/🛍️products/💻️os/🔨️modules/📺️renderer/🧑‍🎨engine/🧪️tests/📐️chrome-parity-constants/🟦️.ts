import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";
import constants from "../../../../../../../🔨️modules/🖱️ui/🧬️schema/📐️chrome-parity-constants/🧮️values.json";
import schema from "../../../../../../../🔨️modules/🖱️ui/🧬️schema/📐️chrome-parity-constants/🔣️.json";
import { ARTIFACT_CREATION_PROGRESS_TEXT_V1 } from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🟦️.tsx";
import { SHELL_ROUTE_ADMISSION_LABELS } from "../../🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts";

const elements = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "🧱️elements");
const source = (...path: string[]): string => readFileSync(join(elements, ...path), "utf8");
const uiSource = (...path: string[]): string => readFileSync(join(elements, "..", "..", "..", "..", "..", "..", "🔨️modules", "🖱️ui", ...path), "utf8");

describe("📐️ chrome parity constants", () => {
  test("the values carry exactly the keys the schema requires", () => {
    for (const key of schema.required) expect(constants).toHaveProperty(key);
    for (const [section, definition] of Object.entries(schema.properties).filter(([key]) => key !== "$schema")) {
      expect(Object.keys((constants as Record<string, object>)[section]!).sort()).toEqual([...(definition as { required: string[] }).required].sort());
    }
  });

  test("the ShellHost stacks keep the widths and anchors the schema's rem and fractions describe", () => {
    const host = source("🏛️ShellHost", "🟦️.tsx");
    expect(host).toContain(`max-w-[${constants.statusStacks.artifactCreationMaxWidthRem}rem]`);
    expect(host.match(/max-w-\[90vw\]/g)?.length).toBe(2);
    expect(constants.statusStacks.routeAdmissionMaxWidthFraction).toBe(0.9);
    expect(constants.statusStacks.installMaxWidthFraction).toBe(0.9);
    expect(host).toContain('className="top-workbench left-1/2 -translate-x-1/2"');
    expect(host).toContain('className="top-workbench left-1/2 mt-double -translate-x-1/2"');
    expect(host).toContain('className="top-workbench left-double max-w-[28rem]"');
  });

  test("the transient notice has no width class, so left-1/2 leaves it the half the schema names", () => {
    const notice = source("🏛️ShellHost", "🍞️transient-notice", "🟦️.tsx");
    expect(notice).toContain("left-1/2");
    expect(notice).not.toMatch(/\b(?:max-)?w-(?:\[|\d)/);
    expect(constants.toast.maxWidthFraction).toBe(0.5);
  });

  test("the panel tab bar paints the insert bar and the nest fill the schema names", () => {
    const bar = uiSource("🧱️elements", "🧭️PanelTabBar", "🟦️.tsx");
    expect(bar).toContain(`w-${constants.panelDrop.insertBarWidthPx / 4} `);
    expect(bar).toContain(`bg-accent/${constants.panelDrop.nestFillAlpha * 100}`);
    expect(uiSource("🔨️modules", "🎨️chrome-class-names", "🟦️.ts")).toContain("--accent-secondary");
  });

  test("the Rust route-admission and artifact-creation copy is React's", () => {
    expect(SHELL_ROUTE_ADMISSION_LABELS["await-sign-in"].de.length).toBeGreaterThan(0);
    expect(ARTIFACT_CREATION_PROGRESS_TEXT_V1.en.phases.accepted).toBe("The creation request was accepted.");
    const vocab = readFileSync(join(elements, "..", "🎯️targets", "🧊️wgpu", "📖️shell-vocab", "🧵️task-manager-and-route-vocab", "🦀️.rs"), "utf8");
    for (const label of Object.values(SHELL_ROUTE_ADMISSION_LABELS)) {
      expect(vocab).toContain(label.en);
      expect(vocab).toContain(label.de);
    }
  });
});
