/** 🛠️ Shared helpers of the backbone wire suites: the source locator of the package entry, the document archive fixture and the app channel value types.
 *
 * @see ../../🟦️.ts — the package entry the suites read fixtures relative to
 */
import * as nodePath from "node:path";
import * as nodeUrl from "node:url";

type TestSource = { readonly directory: string; readonly url: string };

export const source: TestSource = { directory: nodePath.resolve(nodePath.dirname(nodeUrl.fileURLToPath(import.meta.url)), "../.."), url: new URL("../../🟦️.ts", import.meta.url).href };

export const documentArchive = {
  parent_pack: [1, 2],
  parent_spr: [3],
  members: [
    {
      ordinal: 0,
      reference: { artifact_id: "child-1", artifact_kind: "s.test.child", standard: "1", subset: "native" },
      owner: { parent: { artifact_id: "root-1", artifact_kind: "s.test.root", standard: "1", subset: "*" }, slot: "children", child_id: "child-1" },
      envelope_pack: [4, 5],
    },
    {
      ordinal: 1,
      reference: { artifact_id: "grandchild-1", artifact_kind: "s.test.child", standard: "1", subset: "native" },
      owner: { parent: { artifact_id: "child-1", artifact_kind: "s.test.child", standard: "1", subset: "native" }, slot: "nested", child_id: "grandchild-1" },
      envelope_pack: [6],
    },
  ],
} as const;

export type AppChannelHandle = import("@semio-tech/framework/app-channel").AppChannelHandle;

export type AppCommandValue = import("@semio-tech/framework/wire-contracts").AppCommandValue;

export type AppFrameValue = import("@semio-tech/framework/wire-contracts").AppFrameValue;

export type TurnOutcome = import("@semio-tech/framework-kernel/turn-outcome-broadcast").TurnOutcome;
