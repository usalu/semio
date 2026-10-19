import { type AddressInfo, connect, createServer, type Socket } from "node:net";
import type { Page } from "playwright";
import type { SeverableRelayV1 } from "../📋️contract/🟦️.ts";

export async function startSeverableRelay(hubOrigin: string): Promise<SeverableRelayV1> {
  const target = new URL(hubOrigin);
  const sockets = new Set<Socket>();
  let severed = false;
  const server = createServer((client) => {
    if (severed) {
      client.destroy();
      return;
    }
    const upstream = connect(Number(target.port || 80), target.hostname);
    for (const socket of [client, upstream]) {
      sockets.add(socket);
      socket.on("close", () => sockets.delete(socket));
      socket.on("error", () => {
        client.destroy();
        upstream.destroy();
      });
    }
    client.pipe(upstream);
    upstream.pipe(client);
  });
  await new Promise<void>((resolveListen) => server.listen(0, "127.0.0.1", resolveListen));
  const { port } = server.address() as AddressInfo;
  return {
    origin: `http://127.0.0.1:${port}`,
    sever: () => {
      severed = true;
      const cut = sockets.size;
      for (const socket of sockets) socket.destroy();
      return cut;
    },
    heal: () => {
      severed = false;
    },
    close: () =>
      new Promise<void>((resolveClose) => {
        for (const socket of sockets) socket.destroy();
        server.close(() => resolveClose());
      }),
  };
}

/** 🖼️ The RGBA pixels of one page region (a PNG screenshot decoded in the page, so no image library is needed). */
export async function regionPixels(page: Page, clip: Readonly<{ x: number; y: number; width: number; height: number }>, path?: string): Promise<number[]> {
  const png = await page.screenshot({ clip, type: "png", ...(path ? { path } : {}) });
  return page.evaluate(async (bytes) => {
    const bitmap = await createImageBitmap(new Blob([new Uint8Array(bytes)], { type: "image/png" }));
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const context = canvas.getContext("2d")!;
    context.drawImage(bitmap, 0, 0);
    return Array.from(context.getImageData(0, 0, bitmap.width, bitmap.height).data);
  }, [...png]);
}

/** 🔢️ How many pixels differ by more than a visible amount (sum of the RGB channel deltas > 60). */
export function changedPixelCount(left: readonly number[], right: readonly number[]): number {
  let changed = Math.abs(left.length - right.length) / 4;
  for (let index = 0; index < Math.min(left.length, right.length); index += 4) {
    if (Math.abs(left[index]! - right[index]!) + Math.abs(left[index + 1]! - right[index + 1]!) + Math.abs(left[index + 2]! - right[index + 2]!) > 60) changed += 1;
  }
  return changed;
}

/** 🎨️ How many distinct colours a sampled screenshot holds — a painted frame is not a single colour. */
export async function distinctColours(page: Page, path: string): Promise<number> {
  const png = await page.screenshot({ path, type: "png" });
  return page.evaluate(async (bytes) => {
    const bitmap = await createImageBitmap(new Blob([new Uint8Array(bytes)], { type: "image/png" }));
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const context = canvas.getContext("2d")!;
    context.drawImage(bitmap, 0, 0);
    const data = context.getImageData(0, 0, bitmap.width, bitmap.height).data;
    const colours = new Set<string>();
    for (let index = 0; index < data.length && colours.size < 64; index += 4 * 97) colours.add(`${data[index]},${data[index + 1]},${data[index + 2]}`);
    return colours.size;
  }, [...png]);
}
