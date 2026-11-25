import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import cases from "./rejection-cases.json";
import schema from "./rejection.schema.json";
import { createPluginUiIntakeRejectionLedgerV1, parsePluginUiIntakeFailureV1, PLUGIN_UI_INTAKE_REJECTED_CODE, PluginUiIntakeRejectedError, pluginUiIntakeRejectionV1 } from "./rejection.ts";
import { windowFaultFromError, windowFaultTextV1 } from "../../🏛️ShellHost/🩺️fault/🟦️.ts";

test("the fixture is a valid case table", () => {
  expect(new Ajv2020({ strict: true }).compile(schema)(cases)).toBe(true);
  expect(PLUGIN_UI_INTAKE_REJECTED_CODE).toBe(cases.code);
});

test("a refused patch names its contract rule, node and localized notice", () => {
  for (const row of cases.cases) {
    const rejection = pluginUiIntakeRejectionV1(row.surfaceId, row.phase, row.failure);
    expect({ rule: rejection.rule, node: rejection.node }).toEqual({ rule: row.rule, node: row.node });
    const error = new PluginUiIntakeRejectedError(rejection);
    expect(error.notice).toEqual(row.notice);
    expect(error.message.startsWith(`${cases.code}:${row.phase}:`)).toBe(true);
    const fault = windowFaultFromError(error);
    expect({ code: fault.code, origin: fault.origin, notice: fault.notice }).toEqual({ code: cases.code, origin: row.surfaceId, notice: row.notice });
    expect(windowFaultTextV1(fault, fault.message, "en")).toBe(row.notice.en);
    expect(windowFaultTextV1(fault, fault.message, "de")).toBe(row.notice.de);
    expect(windowFaultTextV1(fault, fault.message, "fr")).toBe(fault.message);
  }
});

test("the parser reads exactly the contract's own violation text", () => {
  expect(parsePluginUiIntakeFailureV1("Owned UI graph invariants violated (nodeQuota) count=9 max=8")).toEqual({ rule: "nodeQuota", node: null });
  expect(parsePluginUiIntakeFailureV1("Native input rejected")).toEqual({ rule: null, node: null });
});

test("a refresh that fails while the refused surface is unpublished fails with the rejection, once logged", () => {
  const [row] = cases.cases;
  const logged: PluginUiIntakeRejectedError[] = [];
  const ledger = createPluginUiIntakeRejectionLedgerV1((error) => logged.push(error));
  const thrown = ledger.record(1, pluginUiIntakeRejectionV1(row!.surfaceId, row!.phase, row!.failure));
  expect(logged).toEqual([thrown]);
  expect(ledger.explain(1, [row!.surfaceId], thrown)).toBe(thrown);
  const missing = new Error(`PluginRuntime: actor stdio-pdf#1 stopped without publishing requested UI surfaces (missing=["${row!.surfaceId}"], status=idle, faults=[])`);
  const explained = ledger.explain(1, ["1:pdf.page", row!.surfaceId], missing) as PluginUiIntakeRejectedError;
  expect(explained).toBeInstanceOf(PluginUiIntakeRejectedError);
  expect(explained.cause).toBe(missing);
  expect(windowFaultTextV1(windowFaultFromError(explained), explained.message, "de")).toBe(row!.notice.de);
  expect(ledger.explain(2, [row!.surfaceId], missing)).toBe(missing);
  expect(ledger.explain(1, ["1:pdf.page"], missing)).toBe(missing);
  ledger.clear(1, row!.surfaceId);
  expect(ledger.explain(1, [row!.surfaceId], missing)).toBe(missing);
  ledger.record(1, pluginUiIntakeRejectionV1(row!.surfaceId, row!.phase, row!.failure));
  ledger.forget(1);
  expect(ledger.explain(1, [row!.surfaceId], missing)).toBe(missing);
  expect(logged.length).toBe(2);
});
