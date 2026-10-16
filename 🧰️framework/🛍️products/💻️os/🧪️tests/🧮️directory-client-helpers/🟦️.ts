/** 🧮️ Shared helpers of the directory client and backbone worker wire suites: the source locator of the package entry and the worker request and directory event types.
 *
 * @see ../../🟦️.ts — the package entry the suites read fixtures relative to
 */
import * as nodePath from "node:path";
import * as nodeUrl from "node:url";

type TestSource = { readonly directory: string; readonly url: string };

export const source: TestSource = { directory: nodePath.resolve(nodePath.dirname(nodeUrl.fileURLToPath(import.meta.url)), "../.."), url: new URL("../../🟦️.ts", import.meta.url).href };

export type BackboneWorkerRequest = import("@semio-tech/framework/backbone").BackboneWorkerRequest;

export type DirectoryEvent = import("@semio-tech/framework/directory/schema").DirectoryEvent;
