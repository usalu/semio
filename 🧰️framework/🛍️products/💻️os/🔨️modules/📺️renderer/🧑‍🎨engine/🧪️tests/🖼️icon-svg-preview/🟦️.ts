import { afterEach, expect, it, vi } from "vitest";
import fixture from "../../🧱️elements/🖼️IconRenderHost/🧪️tests/🧫️fixtures/📤️svg-export/🔣️.json";
import { iconSvgBlob, rasterizeIconSvgPreview } from "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🖼️icon-svg-preview/🟦️.ts";
import { cancelPageImageDecode, requestPageIconSvgDecode, requestPageImageDecode, settlePageImageDecode } from "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🖼️page-image-decode/🟦️.ts";

const bridge = vi.hoisted(() => ({
  messages: [] as { kind: string; requestId: number; source?: Uint8Array }[],
  pending: undefined as { requestId: number } | undefined,
  sequence: 0,
}));

vi.mock("../../🎯️targets/🧊️wgpu/🎞️frame-worker/🧵️state/🟦️.ts", () => ({
  lifecycle: 1, closed: false, closing: false, failed: false,
  get pageImageDecode() { return bridge.pending; },
  get nextImageDecodeRequestId() { return bridge.sequence; },
  setPageImageDecode: (value: typeof bridge.pending) => { bridge.pending = value; },
  setNextImageDecodeRequestId: (id: number) => (bridge.sequence = id),
}));

vi.mock("../../🎯️targets/🧊️wgpu/🎞️frame-worker/⏱️steps-and-reporting/🟦️.ts", () => ({
  post: (message: (typeof bridge.messages)[number]) => bridge.messages.push(message),
  macrotask: () => Promise.resolve(),
  monitoredSuspension: (_name: string, operation: () => Promise<unknown>) => operation(),
}));

afterEach(() => {
  cancelPageImageDecode();
  bridge.messages.length = 0;
  vi.unstubAllGlobals();
});

const dimensions = { width: 64, height: 32, orientation: 1, mediaType: "image/svg+xml" } as const;
const pages = (source: string) => {
  const bytes = new TextEncoder().encode(source);
  const read = vi.fn((offset: number) => bytes.slice(offset, offset + 256 * 1024));
  return { length: bytes.length, read };
};

const svg = (width: number, height: number) => `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}">`;
const reply = (id: number, close = vi.fn()) => {
  settlePageImageDecode({ kind: "image-decode-result", lifecycle: 1, requestId: id, bitmap: { ...dimensions, close } as ImageBitmap });
};

it("pages the source and resizes intrinsic SVG extent without changing the exported view box", async () => {
  const source = `${svg(16384, 16384)}<!--${"x".repeat(560_000)}--></svg>`;
  const { length, read } = pages(source);
  const result = await iconSvgBlob(length, 512, 512, read, () => true);
  expect(read.mock.calls).toEqual([[0], [262144], [524288]]);
  expect(result.type).toBe("image/svg+xml");
  expect(await result.text()).toBe(source.replace('width="16384" height="16384"', 'width="512" height="512"'));
});

it("queues a preview behind a reference image and reads bounded straight RGBA through the fallback", async () => {
  vi.stubGlobal("createImageBitmap", vi.fn().mockRejectedValue(new Error("worker SVG decode unavailable")));
  vi.stubGlobal("OffscreenCanvas", class {
    getContext() {
      return {
        drawImage: () => {},
        getImageData: (_x: number, _y: number, w: number, h: number) => {
          const data = new Uint8ClampedArray(w * h * 4);
          for (let offset = 0; offset < data.length; offset += 4) data.set(fixture.previewRaster.rgba, offset);
          return { data };
        },
      };
    }
  });
  const ref = requestPageImageDecode(new Uint8Array(4), dimensions);
  await vi.waitFor(() => expect(bridge.messages).toHaveLength(1));
  const { length, read } = pages(`${svg(4096, 2048)}</svg>`);
  const result = rasterizeIconSvgPreview(length, dimensions.width, dimensions.height, read, new Promise<void>(() => {}));
  await vi.waitFor(() => expect(globalThis.createImageBitmap).toHaveBeenCalledOnce());
  expect(bridge.messages).toHaveLength(1);
  reply(bridge.messages[0]!.requestId);
  await ref;
  await vi.waitFor(() => expect(bridge.messages).toHaveLength(2));
  const source = new TextDecoder().decode(bridge.messages[1]!.source);
  expect(source).toContain('viewBox="0 0 4096 2048" width="64" height="32"');
  const close = vi.fn();
  reply(bridge.messages[1]!.requestId, close);
  const rgba = await result;
  expect(rgba.length).toBe(dimensions.width * dimensions.height * 4);
  expect([...rgba.subarray(0, 4)]).toEqual(fixture.previewRaster.rgba);
  expect(close).toHaveBeenCalledOnce();
});

it("cancels a specific waiting or active preview without disturbing another owner", async () => {
  const ref = requestPageImageDecode(new Uint8Array(4), dimensions);
  await vi.waitFor(() => expect(bridge.messages).toHaveLength(1));
  const ignored = vi.fn(async () => new Uint8Array(3));
  const waiting = new AbortController();
  const refused = requestPageIconSvgDecode(ignored, dimensions, waiting.signal);
  waiting.abort();
  await expect(refused).rejects.toMatchObject({ name: "AbortError" });
  expect(ignored).not.toHaveBeenCalled();

  const active = new AbortController();
  const requested = requestPageIconSvgDecode(async () => new Uint8Array(4), dimensions, active.signal);
  reply(bridge.messages[0]!.requestId);
  await ref;
  await vi.waitFor(() => expect(bridge.messages).toHaveLength(2));
  const id = bridge.messages[1]!.requestId;
  cancelPageImageDecode("reference");
  expect(bridge.messages).toHaveLength(2);
  active.abort();
  await expect(requested).rejects.toMatchObject({ name: "AbortError" });
  expect(bridge.messages[2]).toMatchObject({ kind: "image-decode-cancel", requestId: id });
  const late = vi.fn();
  reply(id, late);
  expect(late).toHaveBeenCalledOnce();
});
