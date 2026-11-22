// @vitest-environment node
import { describe, expect, it } from "vitest";
import { createWgpuPageHostIo } from "../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts";
import { createWgpuPickedFiles, WGPU_PICKED_FILE_CAPACITY, WGPU_PICKED_FILE_RANGE_MAX_BYTES } from "../../🎯️targets/🧊️wgpu/🛬️picked-files/🟦️.ts";

const file = (name: string, text: string, type = ""): File => new File([text], name, { type });

describe("the page's table of picked OS files", () => {
  it("holds a dropped file under a fresh token and reads exact byte ranges", async () => {
    const files = createWgpuPickedFiles();
    const row = files.hold(file("deck.pdf", "0123456789", "application/pdf"));
    expect(row).toEqual({ token: 1, name: "deck.pdf", size: 10, mime: "application/pdf" });
    expect(Array.from(await files.read(row.token, 2, 6))).toEqual(Array.from(new TextEncoder().encode("2345")));
    expect(files.hold(file("b.pdf", "x")).token).toBe(2);
  });

  it("refuses unknown, released and oversized reads", async () => {
    const files = createWgpuPickedFiles();
    const { token } = files.hold(file("a.pdf", "abc"));
    await expect(files.read(99, 0, 1)).rejects.toThrow("unknown-token");
    await expect(files.read(token, 2, 1)).rejects.toThrow("range");
    await expect(files.read(token, 0, WGPU_PICKED_FILE_RANGE_MAX_BYTES + 1)).rejects.toThrow("range");
    files.release(token);
    await expect(files.read(token, 0, 1)).rejects.toThrow("unknown-token");
  });

  it("releases the oldest file past its capacity", async () => {
    const files = createWgpuPickedFiles();
    const first = files.hold(file("first.pdf", "a"));
    for (let index = 0; index < WGPU_PICKED_FILE_CAPACITY; index += 1) files.hold(file(`f${index}.pdf`, "a"));
    await expect(files.read(first.token, 0, 1)).rejects.toThrow("unknown-token");
  });

  it("answers the host-io read door in base64 and the release door with an empty answer", async () => {
    const files = createWgpuPickedFiles();
    const { token } = files.hold(file("a.pdf", "hello"));
    const io = createWgpuPageHostIo(files);
    expect(await io(JSON.stringify({ op: "read-picked-file", token, start: 1, end: 4 }), null)).toBe(btoa("ell"));
    expect(await io(JSON.stringify({ op: "release-picked-file", token }), null)).toBe("");
    await expect(io(JSON.stringify({ op: "read-picked-file", token, start: 0, end: 1 }), null)).rejects.toThrow("unknown-token");
  });
});

describe("the Open file catalog the Worker routes by", () => {
  it("flattens claims, owners, revision sniffs and package claims into one document", async () => {
    const claim = { kindId: "s.doc@1", shortId: "doc", aliases: [], mimes: ["application/x-doc"], extensions: [".doc"] };
    const catalog = {
      descriptors: () => [claim],
      artifactRefOf: (kindId: string) => (kindId === "s.doc@1" ? "s.doc@1/*" : kindId === "s.doc@2" ? "s.doc@2/*" : undefined),
      revisionsOf: () => [{ kindId: "s.doc@2", window: 64, contains: ["v2"] }],
      packageOf: (kindId: string) => (kindId === "s.doc@2" ? { mainPartContentTypes: ["x"], dialects: [] } : undefined),
    };
    const io = createWgpuPageHostIo(createWgpuPickedFiles(), catalog);
    const { formats } = JSON.parse(await io(JSON.stringify({ op: "format-claims" }), null));
    expect(formats).toEqual([{ ...claim, artifactRef: "s.doc@1/*", package: null, revisions: [{ kindId: "s.doc@2", artifactRef: "s.doc@2/*", package: { mainPartContentTypes: ["x"], dialects: [] }, window: 64, contains: ["v2"] }] }]);
    expect(JSON.parse(await createWgpuPageHostIo()(JSON.stringify({ op: "format-claims" }), null))).toEqual({ formats: [] });
    expect(await io(JSON.stringify({ op: "sha256-hex", text: "abc" }), null)).toBe("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
  });
});
