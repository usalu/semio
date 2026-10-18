import type { OwnedBuildMiddleware, OwnedBuildPlugin } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { resolve, relative, isAbsolute, dirname } from "node:path";
import { assetPathFromRequest } from "../../../../../🖼️assets/📦️packages/🟦️typescript/🔍️resolver/🌐️delivery/🟦️.ts";
import { existsSync, statSync, readFileSync, mkdirSync, cpSync } from "node:fs";
import { type MeshDeliveryCatalog, parseMeshDeliveryCatalog } from "../../../../../🖼️assets/📦️packages/🟦️typescript/🥽️mesh/🟦️.ts";
import { serveFileWithValidatorsV1 } from "../📤️file-serving/🟦️.ts";
import type { MeshCollectionAssetSpecV1 } from "../🟦️.ts";

function contentTypeForUiAsset(filePath: string): string | undefined {
  if (filePath.endsWith(".woff2")) {
    return "font/woff2";
  }
  if (filePath.endsWith(".svg")) {
    return "image/svg+xml";
  }
  if (filePath.endsWith(".wasm")) {
    return "application/wasm";
  }
  return undefined;
}

export function createUiAssetsMiddleware(assetsRoot: string): OwnedBuildMiddleware {
  const assetsRootResolved = resolve(assetsRoot);
  return (req, res, next) => {
    const rel = req.url ? assetPathFromRequest(req.url) : null;
    if (rel === null) {
      next();
      return;
    }
    const filePath = resolve(assetsRootResolved, rel);
    const relToRoot = relative(assetsRootResolved, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    serveFileWithValidatorsV1(req, res, filePath, contentTypeForUiAsset(filePath));
  };
}

function readMeshDeliveryCatalog(repoRoot: string, spec: MeshCollectionAssetSpecV1): MeshDeliveryCatalog {
  if (spec.route !== "/mesh") throw new Error(`Unsupported mesh catalog route: ${spec.route}`);
  const read = (path: string): unknown => JSON.parse(readFileSync(resolve(repoRoot, path), "utf8"));
  return parseMeshDeliveryCatalog(read(spec.catalog), read);
}

/** 🌐️ Serves only explicit catalog transport paths, retaining public identity at the caller boundary. */
export function createMeshCollectionMiddleware(repoRoot: string, spec: MeshCollectionAssetSpecV1): OwnedBuildMiddleware {
  const route = `${spec.route}/`;
  const catalog = new Map(readMeshDeliveryCatalog(repoRoot, spec).map(entry => [`${route}${entry.path}`, entry]));
  return (req, res, next) => {
    if (!req.url?.startsWith(route)) {
      next();
      return;
    }
    let path: string;
    try {
      path = decodeURIComponent(req.url.split(/[?#]/, 1)[0] ?? "");
    } catch {
      res.statusCode = 400;
      res.end();
      return;
    }
    const entry = catalog.get(path);
    if (!entry) return next();
    const source = resolve(repoRoot, entry.source);
    if (!existsSync(source) || !statSync(source).isFile()) return next();
    serveFileWithValidatorsV1(req, res, source, "model/gltf-binary");
  };
}

/** 📦️ Copies only admitted source entries to their exact handpicked nested delivery paths. */
function copyMeshCollectionGlbs(repoRoot: string, catalog: MeshDeliveryCatalog, dest: string): void {
  mkdirSync(dest, { recursive: true });
  for (const entry of catalog) {
    const source = resolve(repoRoot, entry.source);
    if (!existsSync(source) || !statSync(source).isFile()) throw new Error(`Missing catalog mesh source: ${entry.source}`);
    const destination = resolve(dest, entry.path);
    mkdirSync(dirname(destination), { recursive: true });
    cpSync(source, destination);
  }
}

/** 🧊️ Dev and static delivery share one explicit public-ID/source/output authority. */
export function meshCollectionVitePlugin(repoRoot: string, spec: MeshCollectionAssetSpecV1): OwnedBuildPlugin[] {
  const serveMeshes = createMeshCollectionMiddleware(repoRoot, spec);
  const catalog = readMeshDeliveryCatalog(repoRoot, spec);
  const destName = spec.route.replace(/^\//, "");
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return [
    {
      name: `mesh-collection-serve${spec.route}`,
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveMeshes);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveMeshes);
      },
    },
    {
      name: `mesh-collection-build${spec.route}`,
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        const dest = resolve(outDir, destName);
        mkdirSync(outDir, { recursive: true });
        copyMeshCollectionGlbs(repoRoot, catalog, dest);
      },
    },
  ];
}
