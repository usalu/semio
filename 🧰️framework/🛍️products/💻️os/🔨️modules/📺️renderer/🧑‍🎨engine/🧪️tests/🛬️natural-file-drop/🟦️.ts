// @vitest-environment node
import { describe, expect, it } from "vitest";
import { carriesOsFilesV1, installNaturalFileDropV1 } from "../../🧱️elements/🏛️ShellHost/🛬️natural-file-drop/🟦️.ts";

type Transfer = { types: string[]; files: File[]; dropEffect?: string };
const fire = (target: EventTarget, type: "dragover" | "drop", dataTransfer: Transfer | null): Event => {
  const event = new Event(type, { cancelable: true });
  Object.assign(event, { dataTransfer });
  target.dispatchEvent(event);
  return event;
};
const files = (...names: string[]): File[] => names.map((name) => new File(["x"], name));

describe("OS file drop onto the shell", () => {
  it("accepts a drag that carries files and announces a copy", () => {
    const target = new EventTarget();
    installNaturalFileDropV1(target, async () => undefined);
    const transfer: Transfer = { types: ["Files"], files: [] };
    expect(fire(target, "dragover", transfer).defaultPrevented).toBe(true);
    expect(transfer.dropEffect).toBe("copy");
  });

  it("leaves text, links and semio payload drags to their own handlers", () => {
    const target = new EventTarget();
    const opened: string[] = [];
    installNaturalFileDropV1(target, async (file) => { opened.push(file.name); });
    for (const types of [["text/plain"], ["text/uri-list"], ["application/x-semio-node"], []]) {
      expect(fire(target, "dragover", { types, files: files("a.xlsx") }).defaultPrevented).toBe(false);
      expect(fire(target, "drop", { types, files: files("a.xlsx") }).defaultPrevented).toBe(false);
    }
    expect(fire(target, "drop", null).defaultPrevented).toBe(false);
    expect(opened).toEqual([]);
  });

  it("opens every dropped file in order, one after the other", async () => {
    const target = new EventTarget();
    const log: string[] = [];
    let release: () => void = () => undefined;
    installNaturalFileDropV1(target, async (file) => {
      log.push(`start:${file.name}`);
      await new Promise<void>((resolve) => { release = resolve; });
      log.push(`end:${file.name}`);
    });
    expect(fire(target, "drop", { types: ["Files"], files: files("a.xlsx", "b.xlsx") }).defaultPrevented).toBe(true);
    await Promise.resolve();
    expect(log).toEqual(["start:a.xlsx"]);
    release();
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(log).toEqual(["start:a.xlsx", "end:a.xlsx", "start:b.xlsx"]);
    release();
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(log.at(-1)).toBe("end:b.xlsx");
  });

  it("ignores events another handler already took", () => {
    const target = new EventTarget();
    const opened: string[] = [];
    target.addEventListener("drop", (event) => event.preventDefault());
    installNaturalFileDropV1(target, async (file) => { opened.push(file.name); });
    fire(target, "drop", { types: ["Files"], files: files("taken.xlsx") });
    expect(opened).toEqual([]);
  });

  it("stops accepting drops after uninstall", () => {
    const target = new EventTarget();
    const opened: string[] = [];
    const uninstall = installNaturalFileDropV1(target, async (file) => { opened.push(file.name); });
    uninstall();
    expect(fire(target, "drop", { types: ["Files"], files: files("late.xlsx") }).defaultPrevented).toBe(false);
    expect(opened).toEqual([]);
  });

  it("recognises OS files only by the Files type", () => {
    expect(carriesOsFilesV1({ types: ["Files"] })).toBe(true);
    expect(carriesOsFilesV1({ types: ["text/plain", "Files"] })).toBe(true);
    expect(carriesOsFilesV1({ types: ["text/plain"] })).toBe(false);
    expect(carriesOsFilesV1(null)).toBe(false);
    expect(carriesOsFilesV1(undefined)).toBe(false);
  });
});
