import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { compare } from "fast-json-patch";
import cases from "./cases.json";
import pages from "./page-cases.json";
import schema from "./page-cases.schema.json";
import { StagedFormSession, type StagedControlSource, type StagedResidentControl, type StagedFormIntent } from "./🟦️.ts";
import { STAGED_FORM_PAGE_ACTION, STAGED_FORM_EVENT_ACTION, type StagedFormPageBinding, type StagedFormStatus } from "@semio-tech/framework/manifest/staging";

function fixture(token = cases.token) {
  const page = structuredClone(pages.page);
  const action = { scope: "ifc", name: STAGED_FORM_PAGE_ACTION, version: 1 };
  let status: StagedFormStatus = { token, ready: true, diagnostics: [], diagnosticOffset: "0", diagnosticTotal: "0" };
  let control: StagedResidentControl<string> = { node: "page-1", key: "next-page", trigger: "activate", disabled: false, editable: true, input: "none", action, binding: { token, page } as StagedFormPageBinding };
  const source: StagedControlSource<string, string> = { view: () => ({ surface: "form", revision: "publication" }), status: () => status, read: (node, trigger) => control.node === node && control.trigger === trigger ? control : undefined };
  const session = new StagedFormSession({ token, locale: "de", controllerId: "ifc" }, cases.args);
  const intent = (seq = 1n): StagedFormIntent<string, string> => ({ surface: "form", revision: "publication", node: "page-1", nodeKey: "next-page", trigger: "activate", action, args: { page: { fieldOffset: "0" }, token: { ...token, window: "other" } }, input: null, seq });
  return { session, source, intent, page, action, setStatus: (value: StagedFormStatus) => { status = value; }, setControl: (value: StagedResidentControl<string>) => { control = value; } };
}

test("closed language-neutral page vectors", () => {
  const ajv = new Ajv({ strict: true });
  expect(ajv.validate(schema, pages)).toBe(true);
  expect(ajv.validate(schema, { ...pages, args: {} })).toBe(false);
});

for (const row of pages.attacks) test("original page action refuses " + row.id, () => {
  const f = fixture();
  f.session.publish(f.source);
  const before = f.session.request();
  expect(f.session.handle({ ...f.intent(), [row.field]: row.value } as StagedFormIntent<string, string>).kind).toBe("refused");
  expect(f.session.request()).toEqual(before);
});

test("resident page destination wins while independent JSON Patch observes no command change", () => {
  const f = fixture();
  f.session.publish(f.source);
  const before = f.session.request()!;
  const intent = f.intent(9007199254740993n);
  Object.defineProperty(intent, "args", { get() { throw new Error("event args read"); } });
  expect(f.session.handle(intent).kind).toBe("staged");
  expect(f.session.request()?.args).toBe(before.args);
  expect(compare(before.args, f.session.request()!.args)).toEqual([]);
  expect(f.session.request()?.page).toEqual(pages.page);
  expect(f.session.request()?.token.generation).toBe("9007199254740994");
  expect(f.session.canExecute()).toBe(false);
  expect(f.session.handle(f.intent(9007199254740994n)).kind).toBe("refused");
  expect(f.session.executionArgs()).toBeUndefined();
  f.page.fieldOffset = "0";
  f.page.choice.path[0] = "other";
  expect(f.session.request()?.page).toEqual(pages.page);
  expect(Object.isFrozen(f.session.request()?.page)).toBe(true);
  expect(Object.isFrozen(f.session.request()?.page?.choice?.path)).toBe(true);
});

test("old preserved text proof cannot authorize paging; newest invalid status can", () => {
  const f = fixture();
  f.setControl({ node: "text-1", key: "count", trigger: "change", disabled: false, editable: true, input: "text", action: { ...f.action, name: STAGED_FORM_EVENT_ACTION }, binding: { token: cases.token, layout: "preserve", edits: [{ source: "text", path: ["count"] }] } });
  f.session.publish(f.source);
  const text = { ...f.intent(), node: "text-1", nodeKey: "count", trigger: "change", action: { ...f.action, name: STAGED_FORM_EVENT_ACTION }, input: "-" };
  expect(f.session.handle(text).kind).toBe("staged");
  const current = f.session.request()!;
  const pageControl = (token: typeof cases.token): StagedResidentControl<string> => ({ node: "page-1", key: "next-page", trigger: "activate", disabled: false, editable: true, input: "none", action: f.action, binding: { token, page: f.page } as StagedFormPageBinding });
  f.setControl(pageControl(cases.token));
  expect(f.session.handle(f.intent(2n)).kind).toBe("refused");
  expect(f.session.request()).toEqual(current);
  f.setControl(pageControl(current.token));
  f.setStatus({ token: current.token, ready: false, diagnostics: [], diagnosticOffset: "0", diagnosticTotal: "0" });
  expect(f.session.publish(f.source)).toBe(true);
  expect(f.session.handle(f.intent(2n)).kind).toBe("staged");
  expect(f.session.request()?.args.count).toBe("-");
});

test("page transitions are atomic at max generation and retire on reset or source revision", () => {
  const full = fixture({ ...cases.token, generation: "18446744073709551615" });
  full.session.publish(full.source);
  const before = full.session.request();
  expect(full.session.handle(full.intent()).kind).toBe("refused");
  expect(full.session.request()).toEqual(before);
  for (const transition of ["reset", "revise"] as const) {
    const f = fixture(); f.session.publish(f.source); f.session.handle(f.intent());
    expect(f.session.request()?.page).toEqual(pages.page);
    if (transition === "reset") f.session.reset(cases.args);
    else f.session.revise("new-source");
    expect(f.session.request()?.page).toBeUndefined();
  }
});

test("fresh text drafts preserve current page in both request and transition result", () => {
  const f = fixture(); f.session.publish(f.source); f.session.handle(f.intent());
  const request = f.session.request()!;
  const action = { ...f.action, name: STAGED_FORM_EVENT_ACTION };
  f.setStatus({ token: request.token, ready: true, diagnostics: [], diagnosticOffset: "0", diagnosticTotal: "0" });
  f.setControl({ node: "text-1", key: "count", trigger: "change", disabled: false, editable: true, input: "text", action, binding: { token: request.token, layout: "preserve", edits: [{ source: "text", path: ["count"] }] } });
  expect(f.session.publish(f.source)).toBe(true);
  const result = f.session.handle({ ...f.intent(2n), node: "text-1", nodeKey: "count", trigger: "change", action, input: "-" });
  expect(result.kind).toBe("staged");
  if (result.kind !== "staged") throw new Error(result.reason);
  expect(result.page).toBe(request.page);
  expect(f.session.request()?.page).toBe(request.page);
  expect(f.session.request()?.args.count).toBe("-");
});

test("a new page publication retains the original sequence watermark", () => {
  const f = fixture(); f.session.publish(f.source);
  expect(f.session.handle(f.intent(9007199254740993n)).kind).toBe("staged");
  const current = f.session.request()!;
  f.setStatus({ token: current.token, ready: true, diagnostics: [], diagnosticOffset: "0", diagnosticTotal: "0" });
  f.setControl({ node: "page-1", key: "next-page", trigger: "activate", disabled: false, editable: true, input: "none", action: f.action, binding: { token: current.token, page: { fieldOffset: "32", diagnosticOffset: "0" } } });
  expect(f.session.publish(f.source)).toBe(true);
  expect(f.session.canExecute()).toBe(true);
  expect(f.session.executionArgs()).toBe(cases.args);
  expect(f.session.handle(f.intent(9007199254740993n)).kind).toBe("refused");
  expect(f.session.request()).toEqual(current);
  expect(f.session.handle(f.intent(9007199254740994n)).kind).toBe("staged");
  expect(f.session.request()?.page?.fieldOffset).toBe("32");
});
