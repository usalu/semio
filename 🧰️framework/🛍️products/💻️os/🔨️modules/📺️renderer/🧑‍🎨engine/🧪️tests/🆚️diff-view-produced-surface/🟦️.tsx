// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createElement } from "react";
import Ajv2020 from "ajv/dist/2020.js";
import { diffArrays } from "diff";
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { DiffViewHost } from "../../🧱️elements/🔺️DiffViewHost/🟦️.tsx";

type ExpectedLine = { readonly kind: "equal" | "remove" | "add"; readonly text: string };
type Fixture = {
  readonly before: string;
  readonly after: string;
  readonly mode: "unified" | "split";
  readonly language: string;
  readonly expected: readonly ExpectedLine[];
};

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🆚️diff-view-produced-surface/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🆚️diff-view-produced-surface/🔣️.json"), "utf8"));

function thirdPartyLines(): ExpectedLine[] {
  return diffArrays(fixture.before.split("\n"), fixture.after.split("\n")).flatMap((change) => {
    const kind: ExpectedLine["kind"] = change.added ? "add" : change.removed ? "remove" : "equal";
    return change.value.map((text) => ({ kind, text }));
  });
}

function node(mode: "unified" | "split") {
  return {
    type: "componentScene",
    hostId: "diff-fixture-host",
    surfaceId: "diff-fixture",
    controllerId: "fixture",
    componentKind: "diffView",
    presence: {},
    diffView: { before: fixture.before, after: fixture.after, mode, language: fixture.language },
  };
}

afterEach(() => cleanup());

describe("authored DiffView surface parity", () => {
  it("validates the neutral corpus against the independent diff oracle", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(thirdPartyLines()).toEqual(fixture.expected);
  });

  it("renders the actual React host with the same unified operations and split panes", () => {
    const unified = render(createElement(DiffViewHost, { node: node("unified"), onAction: () => {} } as any));
    const rows = [...unified.container.querySelectorAll<HTMLElement>(".semio-diff-view-unified > div")];
    expect(rows.map((row) => row.querySelectorAll("span").item(3).textContent)).toEqual(fixture.expected.map((line) => line.text));
    expect(rows.map((row) => (row.className.includes("text-emerald-400") ? "add" : row.className.includes("text-destructive") ? "remove" : "equal"))).toEqual(fixture.expected.map((line) => line.kind));
    unified.unmount();

    const split = render(createElement(DiffViewHost, { node: node("split"), onAction: () => {} } as any));
    expect(split.container.querySelectorAll(".semio-diff-view-split-pane")).toHaveLength(2);
  });
});
