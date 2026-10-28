import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import lodash from "lodash";
import cases from "./body-cases.json";
import schema from "./body-cases.schema.json";
import { stagedBodyNode, type StagedBodySlot } from "./body.ts";
import { controlLabelTarget } from "./label-target.ts";
import { documentControlContext } from "./🟦️.ts";
import { STAGED_FORM_BODY_LIMITS, STAGED_FORM_PAGE_SIZES } from "@semio-tech/framework/manifest/staging";
import bodyLimitsSchema from "../../../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/body-limits.schema.json";

function node(row: typeof cases.cases[number]) {
  return { id: 0, component: row.kind === "form" || row.kind === "field" ? { type: "container", role: row.kind } : row.kind === "status" || row.kind === "extension" ? { type: "extension", extension: row.kind === "status" ? "framework.stagedFormStatus" : "unrelated" } : { type: row.kind, kind: "text", commit: null, items: [] }, children: row.children, bindings: Array.from({ length: row.bindings }, () => ({})) } as unknown as Parameters<typeof stagedBodyNode>[0];
}

test("closed language neutral body vectors", () => {
  const ajv = new Ajv({ strict: true });
  expect(ajv.validate(schema, cases)).toBe(true);
  expect(ajv.validate(schema, { ...cases, invented: true })).toBe(false);
  expect(ajv.validate(bodyLimitsSchema, STAGED_FORM_BODY_LIMITS)).toBe(true);
  expect(STAGED_FORM_BODY_LIMITS.content).toBe(STAGED_FORM_PAGE_SIZES.controls);
  expect(STAGED_FORM_BODY_LIMITS.controls).toBe(STAGED_FORM_BODY_LIMITS.content + STAGED_FORM_BODY_LIMITS.pages);
  expect(Object.isFrozen(STAGED_FORM_BODY_LIMITS)).toBe(true);
});

for (const row of cases.cases) test("body position: " + row.id, () => {
  const record = node(row);
  expect(stagedBodyNode(record, row.slot as StagedBodySlot)).toBe(row.admitted);
  if (row.slot === "form") {
    const oracle = row.kind === "form" && row.bindings === 0 && row.children.length >= 1 && row.children.length <= STAGED_FORM_BODY_LIMITS.controls + 1 && lodash.uniq(row.children).length === row.children.length && !row.children.includes(0);
    expect(stagedBodyNode(record, "form")).toBe(oracle);
  }
});

test("maximum form and control page boundary is exact", () => {
  const record = node(cases.cases[0]);
  record.children = Array.from({ length: 19 }, (_, index) => index + 1);
  expect(stagedBodyNode(record, "form")).toBe(true);
  record.children.push(20);
  expect(stagedBodyNode(record, "form")).toBe(false);
});

test("segmented labels target the selected actual radio, including an unselected or empty roster", () => {
  const record = { id: 7, key: "control", component: { type: "select", appearance: "segmented", value: "y", items: [{ value: "x", label: "X" }, { value: "y", label: "Y" }] } } as unknown as Parameters<typeof stagedBodyNode>[0];
  const context = documentControlContext({ store: { getState: () => ({ surface: "window", revision: 1 }), getNodeSnapshot: () => record, subscribeRevision: () => () => {}, buildIntent: () => { throw new Error("unused"); } }, onIntent: () => {}, domScope: "window/field" });
  expect(controlLabelTarget(context, record)).toBe("window/field/control.option.1");
  if (record.component.type !== "select") throw new Error("fixture");
  record.component.value = "unselected";
  expect(controlLabelTarget(context, record)).toBe("window/field/control.option.0");
  record.component.items = [];
  expect(controlLabelTarget(context, record)).toBeNull();
  record.component.appearance = "menu";
  expect(controlLabelTarget(context, record)).toBe("window/field/control");
});
