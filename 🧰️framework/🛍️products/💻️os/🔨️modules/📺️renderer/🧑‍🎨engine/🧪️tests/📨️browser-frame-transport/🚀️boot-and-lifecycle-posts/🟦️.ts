/** 🚀️ Worker boot and lifecycle posts: the transferable boot, the agent bridge offer, the hidden page and the window blur posts. */
import { describe, expect, it } from "vitest";
import { FakeWorker, transport } from "../🧰️transport-test-support/🟦️.ts";

describe("browser frame worker transport", () => {
  it("posts one transferable boot and remains fail-closed until the Worker acknowledges", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    expect(subject.status).toBe("booting");
    expect(worker.messages[0]?.kind).toBe("boot");
    expect(subject.flush()).toBe(false);
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.status).toBe("ready");
    expect(subject.flush(10)).toBe(true);
  });

  it("hands the page's agent-bridge offer to the Worker on the current lifecycle, and its withdrawal as null", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.setHostAgentBridge({ url: "ws://127.0.0.1:6300/bridge", admissionProof: "proof" });
    subject.setHostAgentBridge(null);
    const offers = worker.messages.filter((message) => message.kind === "host-agent-bridge");
    expect(offers).toEqual([
      { kind: "host-agent-bridge", lifecycle: 1, offer: { url: "ws://127.0.0.1:6300/bridge", admissionProof: "proof" } },
      { kind: "host-agent-bridge", lifecycle: 1, offer: null },
    ]);
  });

  it("posts the hidden page to the Worker at once, outside any batch, and nothing once closed", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.setHostPageHidden();
    expect(worker.messages.at(-1)).toEqual({ kind: "host-page-hidden", lifecycle: 1 });
    expect(worker.messages.filter((message) => message.kind === "batch")).toEqual([]);
    subject.close();
    const posted = worker.messages.length;
    subject.setHostPageHidden();
    expect(worker.messages.length).toBe(posted);
  });

  it("posts the page's window blur to the Worker at once, outside any batch, and nothing once closed", () => {
    const worker = new FakeWorker();
    const subject = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 });
    subject.setHostWindowBlur();
    expect(worker.messages.at(-1)).toEqual({ kind: "host-window-blur", lifecycle: 1 });
    expect(worker.messages.filter((message) => message.kind === "batch")).toEqual([]);
    subject.close();
    const posted = worker.messages.length;
    subject.setHostWindowBlur();
    expect(worker.messages.length).toBe(posted);
  });
});
