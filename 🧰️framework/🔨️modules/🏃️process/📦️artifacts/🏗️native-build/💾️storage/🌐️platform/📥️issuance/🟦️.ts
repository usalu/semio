import { lstatSync } from "node:fs";
import { openNativePlatformStorageV1, requireNativeStoragePathV1 } from "../🟦️.ts";
import type { NativeFixedPlatformOptionsV1 } from "../../🔌️port/🟦️.ts";
import type { NativeImageStorageV1 } from "../📋️contract/🟦️.ts";
import { requireIssuedResourceControlV1, withIssuedResourceOperationV1 } from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";

declare const physicalImageBrand: unique symbol;
/** 💾️ Names a live actual enforcing image loan without exposing its close or mutable provider object. */
export type IssuedNativeImageV1 = { readonly [physicalImageBrand]: true };
/** 📋️ Reports captured physical facts only after actual provider and Original liveness rechecks. */
export type IssuedNativeImageFactsV1 = Readonly<{ root: string; image: string; maximumPhysicalBytes: number; filesystemBytes: number }>;
type ImageScope = { live: boolean; accepting: boolean; pending: Set<Promise<unknown>>; failures: unknown[]; image: NativeImageStorageV1; control: NativeFixedPlatformOptionsV1["control"]; facts: IssuedNativeImageFactsV1; device: number; inode: number };
const images = new WeakMap<IssuedNativeImageV1, ImageScope>();

async function observe(scope: ImageScope): Promise<IssuedNativeImageFactsV1> {
  if (!scope.live) throw Error("Live Original physical image issuance required");
  requireIssuedResourceControlV1(scope.control); await scope.image.recheck(); requireIssuedResourceControlV1(scope.control);
  const current = lstatSync(scope.facts.root);
  if (!scope.live || !current.isDirectory() || current.isSymbolicLink() || current.dev !== scope.device || current.ino !== scope.inode) throw Error("Original physical image root changed");
  return scope.facts;
}

function operation<T>(scope: ImageScope | undefined, run: (original: ImageScope) => Promise<T>): Promise<T> {
  if (!scope?.live || !scope.accepting) return Promise.reject(Error("Live Original physical image issuance required"));
  const work = Promise.resolve().then(() => run(scope));
  scope.pending.add(work);
  void work.then(() => { scope.pending.delete(work); }, error => { scope.failures.push(error); scope.pending.delete(work); });
  return work;
}

/** 🪪️ Requires the exact privately issued control that owns this live actual image loan. */
export function requireIssuedNativeImageControlV1(loan:IssuedNativeImageV1,control:NativeFixedPlatformOptionsV1["control"]):void{
 const scope=images.get(loan);if(!scope?.live||!scope.accepting||scope.control!==control)throw Error("Exact issued physical image control required");requireIssuedResourceControlV1(control);
}

/** 🔎️ Drains accepted physical observations and refuses new operations after callback closure. */
export function readIssuedNativeImageV1(loan: IssuedNativeImageV1): Promise<IssuedNativeImageFactsV1> {
  return operation(images.get(loan), observe);
}

/** 📂️ Registers the exact returned path operation through ancestry validation and physical recheck. */
export function issuedNativeStoragePathV1(loan: IssuedNativeImageV1, path: string): Promise<string> {
  return operation(images.get(loan), async scope => { const facts = await observe(scope); return requireNativeStoragePathV1(facts.root, path); });
}

/** 🏪️ Issues custody only by opening a genuine enforcing backend under an issued Original control. */
export function withIssuedNativeImageV1<T>(options: NativeFixedPlatformOptionsV1, run: (loan: IssuedNativeImageV1) => Promise<T>): Promise<T> {
 return withIssuedResourceOperationV1(options.control, async () => {
  const image = await openNativePlatformStorageV1(options), loan = Object.freeze({}) as IssuedNativeImageV1;
  let scope: ImageScope | undefined;
  try {
    requireIssuedResourceControlV1(options.control); await image.recheck();
    const root = lstatSync(image.root);
    if (!root.isDirectory() || root.isSymbolicLink()) throw Error("Actual original physical image root required");
    scope = { live: true, accepting: true, pending: new Set(), failures: [], image, control: options.control, device: root.dev, inode: root.ino, facts: Object.freeze({ root: image.root, image: image.image, maximumPhysicalBytes: image.maximumPhysicalBytes, filesystemBytes: image.filesystemBytes }) };
    images.set(loan, scope);
    await observe(scope);
    let result: T | undefined, failed = false, failure: unknown;
    try { result = await run(loan); } catch (error) { failed = true; failure = error; }
    scope.accepting = false;
    while (scope.pending.size) await Promise.allSettled([...scope.pending]);
    if (failed) throw failure;
    if (scope.failures.length) throw scope.failures[0];
    await observe(scope); return result as T;
  } finally { if (scope) { scope.accepting = false; scope.live = false; } await image.close(); }
 });
}
