type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { BROWSER_ACTOR_CHILD_REJECTION_LIMITS, BROWSER_ACTOR_CHILD_REJECTION_PHASES, boundChildText, childRejectionFrame, childRejectionReason, childRejectionText, isChildRejectionReason } = dependencies;

  const { expect, it } = vitest;
  it("browser actor child rejection carries a bounded typed reason", async () => {
    const encoder = new TextEncoder();
    const bytes = (value: string) => encoder.encode(value).byteLength;
    expect(source.url.length > 0).toBe(true);
    expect(BROWSER_ACTOR_CHILD_REJECTION_PHASES).toEqual(["load", "invoke"]);

    const raw = new TypeError("guest refused the activation turn");
    raw.stack = "TypeError: guest refused the activation turn\n    at poll (blob:http://127.0.0.1:6191/8f1c-0001:412:19)\n    at invoke (blob:http://127.0.0.1:6191/8f1c-0001:88:3)";
    const reason = childRejectionReason("invoke", ["reactor", "poll"], raw);
    expect(reason).toEqual({ phase: "invoke", path: "reactor/poll", errorClass: "TypeError", message: "guest refused the activation turn", frame: "poll@412:19<invoke@88:3" });
    expect(isChildRejectionReason(reason)).toBe(true);
    expect(childRejectionText(reason)).toBe("invoke reactor/poll: TypeError: guest refused the activation turn at poll@412:19<invoke@88:3");
    expect(Object.isFrozen(reason)).toBe(true);
    expect(JSON.stringify(reason)).not.toContain("blob:");
    expect(JSON.stringify(reason)).not.toContain("127.0.0.1");

    expect(childRejectionFrame(null)).toBe("");
    expect(childRejectionFrame("Error: x\n    at blob:http://h/id:9:1")).toBe("<anonymous>@9:1");
    expect(childRejectionFrame("Error: x\n    at async Module.step (/a/b.js:3:4)")).toBe("Module.step@3:4");
    expect(childRejectionFrame("Error: x\n   no frames here")).toBe("");

    expect(childRejectionReason("load", ["activate"], new Error("")).message).toBe("");
    expect(childRejectionReason("load", ["activate"], "plain string").errorClass).toBe("String");
    expect(childRejectionReason("load", ["activate"], Object.create(null)).errorClass).toBe("object");
    expect(childRejectionReason("load", ["activate"], "plain string").message).toBe("plain string");
    expect(childRejectionReason("invoke", [1, "poll", null], new Error("x")).path).toBe("poll");
    expect(childRejectionText(childRejectionReason("invoke", [], new Error("x")))).toContain("invoke -: ");

    const hostile = {
      get name() {
        throw new Error("hostile name");
      },
      get message() {
        throw new Error("hostile message");
      },
    };
    const named = childRejectionReason("invoke", ["reactor"], hostile);
    expect(isChildRejectionReason(named)).toBe(true);
    expect(named.errorClass).toBe("Object");

    const long = childRejectionReason("invoke", ["a".repeat(400), "b".repeat(400)], new Error("m".repeat(4000)));
    expect(bytes(long.path)).toBeLessThanOrEqual(BROWSER_ACTOR_CHILD_REJECTION_LIMITS.pathBytes);
    expect(bytes(long.message)).toBeLessThanOrEqual(BROWSER_ACTOR_CHILD_REJECTION_LIMITS.messageBytes);
    expect(long.message.endsWith("...")).toBe(true);
    expect(isChildRejectionReason(long)).toBe(true);
    expect(bytes(boundChildText("é".repeat(600), 32))).toBeLessThanOrEqual(32);
    expect(boundChildText("short", 512)).toBe("short");

    for (const hostileRow of [
      null,
      "reason",
      { ...reason, phase: "boot" },
      { ...reason, extra: true },
      { phase: "invoke", path: "reactor", errorClass: "Error", message: "m" },
      { ...reason, frame: "f".repeat(BROWSER_ACTOR_CHILD_REJECTION_LIMITS.frameBytes + 1) },
      { ...reason, frame: 3 },
      { ...reason, message: "m".repeat(BROWSER_ACTOR_CHILD_REJECTION_LIMITS.messageBytes + 1) },
      { ...reason, path: "p".repeat(BROWSER_ACTOR_CHILD_REJECTION_LIMITS.pathBytes + 1) },
      { ...reason, errorClass: "c".repeat(BROWSER_ACTOR_CHILD_REJECTION_LIMITS.classBytes + 1) },
      { ...reason, message: 7 },
      Object.defineProperty({ ...reason }, "message", { get: () => "getter" }),
    ])
      expect(isChildRejectionReason(hostileRow)).toBe(false);
  });
}
