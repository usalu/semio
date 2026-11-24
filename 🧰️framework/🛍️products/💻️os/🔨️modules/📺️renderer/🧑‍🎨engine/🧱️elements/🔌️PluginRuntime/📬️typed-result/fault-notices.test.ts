import { afterAll, describe, expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { SemioFaultError, type FaultNoticeDefinition } from "@semio-tech/framework";
import { syncShellLabelLocale } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { settledFaultNoticeV1 } from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import fixture from "../🧪️tests/🧫️fixtures/🔔️typed-operation-fault-notices.json";
import { consumeTypedOperationEffects, operationFaultFanouts, publishOperationFault, TYPED_OPERATION_UNDECODABLE_FAULT, TypedOperationRouter, typedOperationFaultError, withTypedOperationCall } from "./🟦️.ts";
import { OperationDiagnosticObservers } from "@semio-tech/framework/actor/wire-turn/diagnostic/delivery";
import Ajv2020 from "ajv";
import { diagnosticControlledSchema as faultSchema } from "@semio-tech/framework-diagnostic-ts/testing/fixtures";
import { valueRefusalSchema as valueSchema } from "@semio-tech/framework/value/testing/fixtures";
import { createInstanceTable, type PluginHandleScope } from "../🪢️handle-scope/🟦️.ts";

const app = { faultNotices: fixture.table as unknown as readonly FaultNoticeDefinition[] };
const token = (sequence: number) => ({ receiver: fixture.receiver, operation: BigInt(fixture.operation), generation: BigInt(fixture.generation), sequence, attempt: 1 });

function page(lane: number, text: string, sequence: number) {
  const body = Buffer.alloc(30);
  body.writeUInt32LE(fixture.receiver, 0);
  body.writeBigUInt64LE(BigInt(fixture.operation), 4);
  body.writeBigUInt64LE(BigInt(fixture.generation), 12);
  body.writeUInt32LE(sequence, 20);
  body[24] = 1;
  body[25] = lane;
  const payload = Buffer.from(text);
  body.writeUInt32LE(payload.length, 26);
  return { tag: "send-message", val: { target: { tag: "shell", val: String(fixture.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), body, payload])) } };
}

function surfaced(error: SemioFaultError, locale: "en" | "de") {
  syncShellLabelLocale(locale);
  return settledFaultNoticeV1(error.fault, app, fixture.terminology);
}

describe("typed-operation fault pages become SemioFaultErrors the shell tells in the person's language", () => {
  afterAll(() => syncShellLabelLocale("en"));

  for (const row of fixture.cases) {
    test(`${row.id}: the consumed page throws a SemioFaultError whose notice reads in en and de`, () => {
      const call = new TypedOperationRouter(() => {}, () => {}, () => {}).open("fixture");
      let thrown: unknown;
      try { consumeTypedOperationEffects([page(fixture.faultLane, JSON.stringify(row.page), 0)] as never, call); } catch (error) { thrown = error; }
      expect(thrown).toBeInstanceOf(SemioFaultError);
      const error = thrown as SemioFaultError;
      expect(error.fault.code).toBe(row.page.code);
      expect(error.fault.params).toEqual((row.page as { params?: Record<string, string> }).params);
      for (const locale of ["en", "de"] as const) expect(surfaced(error, locale)).toEqual(row.expected[locale]);
    });
  }

  test("a payload that is not a Fault object is still a SemioFaultError naming the code, never a plain Error", () => {
    const error = typedOperationFaultError(fixture.undecodable.text);
    expect(error).toBeInstanceOf(SemioFaultError);
    expect(error.fault.code).toBe(TYPED_OPERATION_UNDECODABLE_FAULT);
    expect(error.fault.code).toBe(fixture.undecodable.code);
    expect(error.message).toBe(fixture.undecodable.text);
    for (const locale of ["en", "de"] as const) expect(surfaced(error, locale)).toEqual(fixture.undecodable.expected[locale]);
  });

  test("a fault page parked for its own call rejects that call with the decoded SemioFaultError", async () => {
    const row = fixture.cases[0]!;
    const actorId = `fault-notice#${fixture.receiver}`;
    await expect(withTypedOperationCall(actorId, "owner", async (owner) => {
      await withTypedOperationCall(actorId, "observer", async (observer) => {
        owner.observe(token(0));
        expect(observer.fault(token(3), JSON.stringify(row.page))).toBe(false);
      });
    })).rejects.toMatchObject({ name: "SemioFaultError", fault: { code: row.page.code } });
  });

  test("a settled fault no awaiting call owns reaches the instance's subscribers, localized, exactly once", () => {
    const row = fixture.cases[0]!;
    const received: SemioFaultError[] = [];
    const listeners = new OperationDiagnosticObservers<SemioFaultError>(4, () => {});
    operationFaultFanouts.set(fixture.receiver, listeners);
    try {
      listeners.subscribe((error) => received.push(error));
      const router = new TypedOperationRouter(() => {}, () => {}, publishOperationFault);
      expect(router.fault(router.open("drain"), token(3), JSON.stringify(row.page))).toBe(false);
      expect(received.length).toBe(1);
      for (const locale of ["en", "de"] as const) expect(surfaced(received[0]!, locale)).toEqual(row.expected[locale]);
    } finally {
      operationFaultFanouts.delete(fixture.receiver);
    }
  });

  test("a successful operation raises and publishes nothing", () => {
    const published: SemioFaultError[] = [];
    const call = new TypedOperationRouter(() => {}, () => {}, (_receiver, error) => published.push(error)).open("fixture");
    const consumed = consumeTypedOperationEffects([page(fixture.terminalLane, "typed-operation-complete", 0)] as never, call);
    expect(consumed.map((effect) => effect.tag)).toEqual(["typed-operation-terminal-seen"]);
    expect(published).toEqual([]);
  });

  test("without subscribers a settled fault is logged with its code instead of dropped", () => {
    const logged: string[] = [];
    const original = console.error;
    console.error = (message: string) => { logged.push(message); };
    try {
      expect(publishOperationFault(fixture.receiver, typedOperationFaultError(JSON.stringify(fixture.cases[0]!.page)))).toBe(false);
    } finally {
      console.error = original;
    }
    expect(logged).toEqual([`${fixture.cases[0]!.page.code}: ${fixture.cases[0]!.page.message}`]);
  });

  test("every fixture fault page is admitted by the diagnostic Fault schema (Ajv, third party)", () => {
    const ajv = new Ajv2020({ strict: true }).addSchema(valueSchema).addSchema(faultSchema);
    const validate = ajv.getSchema(`${faultSchema.$id}#/$defs/Fault`)!;
    for (const row of fixture.cases) {
      const decoded = typedOperationFaultError(JSON.stringify(row.page)).fault;
      const admitted = { ...decoded, retainedProgress: { copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 } };
      expect(validate(admitted), `${row.id}: ${JSON.stringify(validate.errors)}`).toBe(true);
      expect(decoded.code).toBe(row.page.code);
    }
  });

  test("releasing an instance drops its fault fan-out so no subscription outlives it", () => {
    const listeners = new OperationDiagnosticObservers<SemioFaultError>(4, () => {});
    listeners.subscribe(() => {});
    operationFaultFanouts.set(fixture.receiver, listeners);
    const scope = {
      pluginId: "fixture", state: { disposing: false }, closingInstances: new Set<number>(), actorIdByInstance: new Map(), appIdByInstance: new Map(),
      documentBindings: new Map(), documentBindingGenerations: new Map(), lifecycleByInstance: new Map(), uiOwnerByInstance: new Map(), uiRetirementByInstance: new Map(),
      uiSurfaceByInstance: new Map(), uiIntakesByInstance: new Map(), uiReadsByInstance: new Map(),
    } as unknown as PluginHandleScope;
    createInstanceTable(scope).releaseInstanceMaps(fixture.receiver, `fault-notice-release#${fixture.receiver}`);
    expect(operationFaultFanouts.has(fixture.receiver)).toBe(false);
  });
});
