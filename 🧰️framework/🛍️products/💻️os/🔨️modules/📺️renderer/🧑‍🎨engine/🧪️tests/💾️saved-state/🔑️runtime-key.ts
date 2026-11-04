import { describe, expect, it } from "vitest";
import { resolveRuntimeKeyV1 } from "../../🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🔑️runtime-key/🟦️.ts";

const entry = (pluginId: string, instanceId: number, documentId: string) => ({ session: { pluginId, instanceId }, documentId });

describe("resolveRuntimeKeyV1", () => {
  const session = { pluginId: "stdio-image", instanceId: 4 };
  it("answers the single matching entry", () => {
    expect(resolveRuntimeKeyV1([["a", entry("stdio-image", 4, "d1")], ["b", entry("stdio-image", 5, "d2")]], session)).toBe("a");
  });
  it("fails closed when more than one entry matches, and disambiguates by document identity", () => {
    const entries = [["a", entry("stdio-image", 4, "d1")], ["b", entry("stdio-image", 4, "d2")]] as const;
    expect(resolveRuntimeKeyV1(entries, session)).toBeNull();
    expect(resolveRuntimeKeyV1(entries, session, "d2")).toBe("b");
    expect(resolveRuntimeKeyV1(entries, session, "d3")).toBeNull();
  });
  it("answers null when nothing matches", () => {
    expect(resolveRuntimeKeyV1([], session)).toBeNull();
  });
});

import { beginDocumentTransferV1 } from "../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";

describe("beginDocumentTransferV1 single flight", () => {
  it("admits one transfer at a time and re-admits after release", () => {
    const flight = { current: false };
    expect(beginDocumentTransferV1(flight)).toBe(true);
    expect(beginDocumentTransferV1(flight)).toBe(false);
    flight.current = false;
    expect(beginDocumentTransferV1(flight)).toBe(true);
  });
});
