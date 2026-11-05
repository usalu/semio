/** 💾️ The actual worker event handler retains typed storage failures for the close guard. */
import { describe, expect, it, vi } from "vitest";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import { handleDocumentEventMessage } from "../../🧱️elements/🏛️ShellHost/🧵️backbone-worker/📬️document-event-messages/🟦️.ts";
import { deriveSavedStateV1 } from "../../🧱️elements/🛠️ShellHelpers/💾️saved-state/🟦️.ts";

describe("document status event", () => {
  it("validates the language-neutral receipt cases independently", () => {
    expect(new Ajv().validate(fixture.schema, fixture.cases)).toBe(true);
  });
  for (const row of fixture.cases) it(row.name, () => {
    const dispatch = vi.fn();
    handleDocumentEventMessage({
      openDocumentSessionsRef: { current: new Map([["doc", { clientInstanceId: "live" }]]) },
      dispatch,
      message: { kind: "event", documentId: "doc", clientInstanceId: row.client, event: { kind: "status", ...row.status } },
    } as unknown as Parameters<typeof handleDocumentEventMessage>[0]);
    if (!row.accepted) { expect(dispatch).not.toHaveBeenCalled(); return; }
    expect(dispatch).toHaveBeenCalledTimes(1);
    const action = dispatch.mock.calls[0]![0];
    expect(action).toEqual({ type: "SET_SYNC_STATUS_FOR_DOCUMENT", documentId: "doc", status: row.status });
    expect(deriveSavedStateV1(action.status).kind).toBe(row.saved);
    expect(Object.hasOwn(action.status, "writeFailure")).toBe(Object.hasOwn(row.status, "writeFailure"));
  });
});
