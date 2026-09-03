//#region 🔌️Adapters
import { describe, expect, it } from "vitest";
import { ownedSchema as z } from "./🟦️✅️validation.ts";
//#endregion 🔌️Adapters

//#region 🧪️Fixtures
const RequestSchema = z.object({
  action: z.literal("open"),
  title: z.string().min(1),
  email: z.string().email(),
  role: z.enum(["developer", "admin", "owner"] as const).default("developer"),
  enabled: z.boolean().default(false),
  parent: z.string().nullable().default(null),
  files: z.array(z.string()).default([]),
  payload: z.unknown().default({}),
});
//#endregion 🧪️Fixtures

//#region 🧪️Contract
describe("owned coordinator request validation", () => {
  it("applies defaults, strips unknown keys, and preserves nullable values", () => {
    expect(RequestSchema.safeParse({ action: "open", title: "Ticket", email: "dev@example.com", ignored: true })).toEqual({
      success: true,
      data: { action: "open", title: "Ticket", email: "dev@example.com", role: "developer", enabled: false, parent: null, files: [], payload: {} },
    });
  });

  it.each([
    [{}, "action"],
    [{ action: "close", title: "Ticket", email: "dev@example.com" }, "action"],
    [{ action: "open", title: "", email: "dev@example.com" }, "title"],
    [{ action: "open", title: "Ticket", email: "invalid" }, "email"],
    [{ action: "open", title: "Ticket", email: "dev@example.com", role: "guest" }, "role"],
    [{ action: "open", title: "Ticket", email: "dev@example.com", files: [1] }, "files.0"],
  ])("rejects invalid request fields %#", (value, field) => {
    const parsed = RequestSchema.safeParse(value);
    expect(parsed.success).toBe(false);
    if (!parsed.success) expect(parsed.error.message).toContain(field);
  });
});
//#endregion 🧪️Contract
