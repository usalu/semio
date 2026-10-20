import * as THREE from "three";
import { ephemeralBox } from "@semio-tech/framework-transient";

/** 🖼️ Reference plane media kind for infinite-world grid underlays. */
export type ReferenceMediaKind = "image" | "svg" | "pdf";

/** 🖼️ Source descriptor for {@link ReferenceMediaPort.loadReferenceTexture}. */
export interface ReferenceMediaSource {
  readonly url: string;
  readonly mediaKind: ReferenceMediaKind;
  readonly page?: number;
}

/** 🖼️ Loaded reference texture with intrinsic pixel dimensions. */
export interface ReferenceMediaLoadResult {
  readonly texture: THREE.Texture;
  readonly width: number;
  readonly height: number;
}

/** 🖼️ Port for rasterizing png/svg/pdf paths into three.js textures. */
export interface ReferenceMediaPort {
  loadReferenceTexture(source: ReferenceMediaSource): Promise<ReferenceMediaLoadResult>;
}

const REFERENCE_MEDIA_RASTER_MAX = 4096;

/** 🔎️ Infers reference media kind from a URL path extension. */
export function referenceMediaKindFromUrl(url: string): ReferenceMediaKind | null {
  const ext = url.split(/[?#]/, 1)[0]?.split(".").pop()?.toLowerCase() ?? "";
  if (["png", "jpg", "jpeg", "gif", "webp", "bmp", "avif", "tif", "tiff"].includes(ext)) {
    return "image";
  }
  if (ext === "svg") {
    return "svg";
  }
  if (ext === "pdf") {
    return "pdf";
  }
  return null;
}

async function rasterizeReferenceImage(url: string): Promise<{ readonly canvas: HTMLCanvasElement; readonly width: number; readonly height: number }> {
  const image = new Image();
  image.crossOrigin = "anonymous";
  await new Promise<void>((resolve, reject) => {
    image.onload = () => resolve();
    image.onerror = () => reject(new Error(`reference image load failed: ${url}`));
    image.src = url;
  });
  const width = Math.max(1, Math.min(REFERENCE_MEDIA_RASTER_MAX, image.naturalWidth || 512));
  const height = Math.max(1, Math.min(REFERENCE_MEDIA_RASTER_MAX, image.naturalHeight || 512));
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    throw new Error("reference canvas 2d unavailable");
  }
  ctx.drawImage(image, 0, 0, width, height);
  return { canvas, width, height };
}

const referencePdfWorkerReady = ephemeralBox<Promise<void> | null>("framework.modules.ui.packages.typescript.targets.react.index.tsx.referencePdfWorkerReady", null);

async function loadReferencePdfModule(): Promise<typeof import("pdfjs-dist")> {
  const pdfjs = await import("pdfjs-dist");
  if (!referencePdfWorkerReady.current) {
    referencePdfWorkerReady.current = Promise.resolve().then(() => {
      pdfjs.GlobalWorkerOptions.workerSrc = new URL("../pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url).toString();
    });
  }
  await referencePdfWorkerReady.current;
  return pdfjs;
}

async function rasterizeReferencePdf(url: string, pageNumber: number): Promise<{ readonly canvas: HTMLCanvasElement; readonly width: number; readonly height: number }> {
  const pdfjs = await loadReferencePdfModule();
  const doc = await pdfjs.getDocument(url).promise;
  const page = await doc.getPage(Math.max(1, Math.min(pageNumber, doc.numPages)));
  const viewport = page.getViewport({ scale: 1 });
  const scale = Math.min(1, REFERENCE_MEDIA_RASTER_MAX / Math.max(viewport.width, viewport.height, 1));
  const scaled = page.getViewport({ scale });
  const canvas = document.createElement("canvas");
  canvas.width = Math.ceil(scaled.width);
  canvas.height = Math.ceil(scaled.height);
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    throw new Error("reference pdf canvas 2d unavailable");
  }
  await page.render({ canvas, canvasContext: ctx, viewport: scaled }).promise;
  return { canvas, width: canvas.width, height: canvas.height };
}

function referenceCanvasTexture(canvas: HTMLCanvasElement): THREE.Texture {
  const texture = new THREE.CanvasTexture(canvas);
  texture.colorSpace = THREE.SRGBColorSpace;
  texture.needsUpdate = true;
  return texture;
}

async function loadReferenceImageTexture(url: string): Promise<ReferenceMediaLoadResult> {
  const loader = new THREE.TextureLoader();
  loader.setCrossOrigin("anonymous");
  const texture = await loader.loadAsync(url);
  const image = texture.image as { naturalWidth?: number; width?: number; naturalHeight?: number; height?: number };
  return {
    texture,
    width: Math.max(1, image.naturalWidth ?? image.width ?? 1),
    height: Math.max(1, image.naturalHeight ?? image.height ?? 1),
  };
}

/** 🖼️ Default {@link ReferenceMediaPort} wired through {@link sceneHostPort}. */
export let referenceMediaPort: ReferenceMediaPort = {
  async loadReferenceTexture(source) {
    if (source.mediaKind === "image") {
      return loadReferenceImageTexture(source.url);
    }
    if (source.mediaKind === "svg") {
      const raster = await rasterizeReferenceImage(source.url);
      return { texture: referenceCanvasTexture(raster.canvas), width: raster.width, height: raster.height };
    }
    const raster = await rasterizeReferencePdf(source.url, source.page ?? 1);
    return { texture: referenceCanvasTexture(raster.canvas), width: raster.width, height: raster.height };
  },
};
