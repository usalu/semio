import { isAbsolute, join, resolve } from "node:path";
import { readBuildMode } from "@semio-tech/process/build-mode";

/** 🔤️ Resolves one stable font publication per intent without issuing reader or writer custody. */
export function fontAssetPublicationRootV1(productsRoot: string, mode: string): string {
  if (!isAbsolute(productsRoot) || resolve(productsRoot) !== productsRoot) throw Error("Canonical font products root required");
  const directory = join(productsRoot, "infinite", readBuildMode(mode), "fonts");
  if (directory.length > 224) throw Error("Font artifact path capacity exceeded");
  return directory;
}
