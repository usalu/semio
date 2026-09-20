// #region 🧲️Header
// 💻️ 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📖️stories/🎭️reference-media/🧪️.story.tsx
// Specs: Exercise `referenceMediaPort`/`referenceMediaKindFromUrl` (`framework/ui/js/react/index.tsx`) against the real files in `infinite/fixture/` — a sketch PNG, two floor-plan raster scans, and a PDF.
// Summary: `infinite/fixture/*` isn't served by any registered Storybook static-dir route for this scope, so each fixture is brought in via a Vite asset `import` (the PDF via the `?url` suffix, since PDFs aren't in Vite's default `assetsInclude`) — that gives `referenceMediaPort.loadReferenceTexture` a real fetchable URL with zero extra plumbing. The loaded `THREE.Texture`'s backing image/canvas is blitted onto a plain 2D `<canvas>` so the story needs no r3f `<Canvas>` at all.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { referenceMediaKindFromUrl, referenceMediaPort } from "@semio-tech/ui-react";
import { useEffect, useRef, useState, type ReactElement } from "react";
import { decodeReferenceImage, referenceImageSourceDimensions, streamReferenceImageBitmapRows } from "../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts";
import decodeFixture from "../../../📺️renderer/🧑‍🎨engine/🧫️fixtures/🖼️reference-image-decode/🔣️.json";

import sketchUrl from "../../🖼️assets/✏️sketch/🖼️.png";
import abbauAufbauUrl from "../../🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg";
import rathausAhlenUrl from "../../🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png";
import sitePdfUrl from "../../🖼️assets/🗺️site.pdf?url";

//#region StoryHost
type StoryReferenceMediaStatus = "loading" | "loaded" | "error";

/** @emoji 🖼️ Loads one `infinite/fixture/*` file through the real `referenceMediaPort`, then blits the resolved texture's backing image/canvas onto a plain 2D canvas for display. */
function ReferenceMediaPreview({ label, url, page }: { readonly label: string; readonly url: string; readonly page?: number }): ReactElement {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [status, setStatus] = useState<StoryReferenceMediaStatus>("loading");
  const [dims, setDims] = useState<{ readonly width: number; readonly height: number } | null>(null);
  const mediaKind = referenceMediaKindFromUrl(url);

  useEffect(() => {
    if (!mediaKind) {
      setStatus("error");
      return;
    }
    let cancelled = false;
    setStatus("loading");
    setDims(null);
    referenceMediaPort
      .loadReferenceTexture({ url, mediaKind, page })
      .then((loaded) => {
        if (cancelled) {
          loaded.texture.dispose();
          return;
        }
        setDims({ width: loaded.width, height: loaded.height });
        const canvas = canvasRef.current;
        const image = loaded.texture.image as CanvasImageSource | undefined;
        if (canvas && image) {
          canvas.width = loaded.width;
          canvas.height = loaded.height;
          canvas.getContext("2d")?.drawImage(image, 0, 0, loaded.width, loaded.height);
        }
        loaded.texture.dispose();
        setStatus("loaded");
      })
      .catch(() => {
        if (!cancelled) setStatus("error");
      });
    return () => {
      cancelled = true;
    };
  }, [url, mediaKind, page]);

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 4, minWidth: 0 }}>
      <div data-testid={`reference-media-status-${label}`} style={{ fontSize: 11 }}>
        {label} — kind: {mediaKind ?? "unknown"} — status: {status}
        {dims ? ` — ${dims.width}×${dims.height}` : ""}
      </div>
      <canvas ref={canvasRef} style={{ maxWidth: "100%", height: "auto", border: "1px solid #8888", background: "#0000" }} />
    </div>
  );
}

function ReferenceMediaStoryHost({ entries }: { readonly entries: readonly { readonly label: string; readonly url: string; readonly page?: number }[] }): ReactElement {
  return (
    <div className="semio-reference-media-story" style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))", gap: 12, padding: 12, width: "100%", height: "100%", overflow: "auto", boxSizing: "border-box" }}>
      {entries.map((entry) => (
        <ReferenceMediaPreview key={entry.label} label={entry.label} url={entry.url} page={entry.page} />
      ))}
    </div>
  );
}

function jpegWithExifOrientation(source: Uint8Array<ArrayBuffer>, orientation: 6 | 8): Uint8Array<ArrayBuffer> {
  if (source[0] !== 0xff || source[1] !== 0xd8) throw new Error("reference-oracle-jpeg");
  const exif = new Uint8Array([0xff, 0xe1, 0, 34, 69, 120, 105, 102, 0, 0, 73, 73, 42, 0, 8, 0, 0, 0, 1, 0, 18, 1, 3, 0, 1, 0, 0, 0, orientation, 0, 0, 0, 0, 0, 0, 0]);
  const result = new Uint8Array(source.length + exif.length);
  result.set(source.subarray(0, 2));
  result.set(exif, 2);
  result.set(source.subarray(2), 2 + exif.length);
  return result;
}

async function compareReferenceDecode(bytes: Uint8Array<ArrayBuffer>): Promise<string> {
  const url = URL.createObjectURL(new Blob([bytes], { type: "image/jpeg" }));
  try {
    const [decoded, react] = await Promise.all([
      decodeReferenceImage(new Blob([bytes]), referenceImageSourceDimensions(bytes)),
      referenceMediaPort.loadReferenceTexture({ url, mediaKind: "image" }),
    ]);
    try {
      if (react.width !== decoded.width || react.height !== decoded.height) return `failed:dims=${decoded.width}x${decoded.height}/${react.width}x${react.height}`;
      const canvas = document.createElement("canvas");
      canvas.width = decoded.width;
      canvas.height = decoded.height;
      const context = canvas.getContext("2d", { alpha: true, colorSpace: "srgb", willReadFrequently: true });
      if (!context) throw new Error("reference-oracle-context");
      context.imageSmoothingEnabled = true;
      context.imageSmoothingQuality = "medium";
      context.globalCompositeOperation = "copy";
      context.drawImage(react.texture.image as CanvasImageSource, 0, 0, decoded.width, decoded.height);
      const expected = context.getImageData(0, 0, decoded.width, decoded.height, { colorSpace: "srgb" }).data;
      const stride = Math.max(4, Math.floor(expected.length / decodeFixture.oracle.sampleCapacity / 4) * 4);
      let samples = 0;
      let sum = 0;
      let max = 0;
      await streamReferenceImageBitmapRows(decoded, decoded.width, decoded.height, (stripOffset, pixels) => {
        const stripEnd = stripOffset + pixels.byteLength;
        for (let offset = Math.ceil(stripOffset / stride) * stride; offset < stripEnd; offset += stride) {
          for (let channel = 0; channel < 4; channel++) {
            const delta = Math.abs(expected[offset + channel] - pixels[offset - stripOffset + channel]);
            sum += delta;
            max = Math.max(max, delta);
            samples += 1;
          }
        }
        return true;
      }, () => true, async () => Promise.resolve());
      const mean = sum / samples;
      return mean <= decodeFixture.oracle.meanChannelDeltaMax && max <= decodeFixture.oracle.maxChannelDeltaMax ? `passed:${decoded.width}x${decoded.height}:mean=${mean.toFixed(3)}:max=${max}` : `failed:mean=${mean.toFixed(3)}:max=${max}`;
    } finally {
      decoded.close();
      react.texture.dispose();
    }
  } finally {
    URL.revokeObjectURL(url);
  }
}

function ReferenceDecodeParityHost(): ReactElement {
  const [result, setResult] = useState("loading");
  useEffect(() => {
    let cancelled = false;
    void fetch(abbauAufbauUrl).then((response) => response.arrayBuffer()).then(async (buffer) => {
      const bytes = new Uint8Array(buffer);
      const variants = [["normal", bytes], ["exif-90", jpegWithExifOrientation(bytes, 6)], ["exif-270", jpegWithExifOrientation(bytes, 8)]] as const;
      const comparisons: string[] = [];
      for (const [name, variant] of variants) comparisons.push(`${name}=${await compareReferenceDecode(variant)}`);
      const oversized = new Uint8Array(24);
      oversized.set([137, 80, 78, 71, 13, 10, 26, 10]);
      new DataView(oversized.buffer).setUint32(16, 4097);
      new DataView(oversized.buffer).setUint32(20, 4097);
      let sourceRejected = false;
      try { referenceImageSourceDimensions(oversized); } catch { sourceRejected = true; }
      let live = true;
      const cancellation = decodeReferenceImage(new Blob([bytes]), referenceImageSourceDimensions(bytes), () => live);
      live = false;
      let cancelledAtResult = false;
      try { (await cancellation).close(); } catch (error) { cancelledAtResult = error instanceof DOMException && error.name === "AbortError"; }
      if (!cancelled) setResult(`${comparisons.join(";")};source-rejected=${sourceRejected};cancelled=${cancelledAtResult}`);
    }).catch((error: unknown) => {
      if (!cancelled) setResult(`error:${error instanceof Error ? error.message : String(error)}`);
    });
    return () => {
      cancelled = true;
    };
  }, []);
  return <output data-testid="reference-decode-parity" data-result={result}>{result}</output>;
}
//#endregion StoryHost

const meta = {
  title: "♾️infinite/ReferenceMedia",
  component: ReferenceMediaStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ReferenceMediaStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🖼️ All four real `infinite/fixture/*` files: a sketch PNG, two floor-plan scans (JPG/PNG), and page 1 of a PDF. */
export const AllFixtures: Story = {
  args: {
    entries: [
      { label: "sketch.png", url: sketchUrl },
      { label: "abbau-aufbau-masterarbeit-grundriss.jpg", url: abbauAufbauUrl },
      { label: "rathaus-ahlen-grundriss.png", url: rathausAhlenUrl },
      { label: "site.pdf", url: sitePdfUrl, page: 1 },
    ],
  },
};

/** 🖼️ Just the PDF fixture, rasterized via `referenceMediaPort`'s `pdfjs-dist` path. */
export const PdfOnly: Story = {
  args: {
    entries: [{ label: "site.pdf", url: sitePdfUrl, page: 1 }],
  },
};

/** 🧪️ Browser-native decode/resample output against React's real THREE.TextureLoader image. */
export const RasterDecodeParity: StoryObj<typeof ReferenceDecodeParityHost> = {
  render: () => <ReferenceDecodeParityHost />,
};
