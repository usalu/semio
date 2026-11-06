/** 📤️ Static app declarations bind a checked export to its output port and downloadable representation. */
import { EXPORT_MEDIA_FILE_ACTION_ID, EXPORT_SNAPSHOT_FILE_ACTION_ID, type AppDefinition, type MediaType } from "@semio-tech/framework";
import type { AppDocumentIdentity } from "@semio-tech/framework/wire-contracts";
import { decodePackValue } from "@semio-tech/framework/packs";
import { collectMediaExportBytes, type MediaTransportPort } from "../🚚️lifecycle/🟦️.ts";
import type { SegmentedDownloadSinkFactory } from "../../🧱️elements/📤️SegmentedDownload/🟦️.ts";

export type MediaFileExportV1 = {
  readonly outputPort: string;
  readonly filename: string;
  readonly mediaType: string;
  readonly formatKind: string;
  readonly portMediaType: MediaType;
};

export type MediaFileExportActionIdV1 = typeof EXPORT_MEDIA_FILE_ACTION_ID | typeof EXPORT_SNAPSHOT_FILE_ACTION_ID;

export type MediaFileExportOperationV1 = {
  readonly instanceId: number;
  readonly declaration: MediaFileExportV1;
  readonly port: MediaTransportPort & { readonly readAppDocumentIdentity: (instanceId: number) => Promise<AppDocumentIdentity> };
  readonly signal: AbortSignal;
  readonly current: () => boolean;
  readonly progress: (completed: number, total: number | null) => void;
  readonly sinkFactory: SegmentedDownloadSinkFactory;
  readonly yieldTurn?: () => Promise<void>;
};

/** 🔌️ Resolves one unambiguous app-owned export declaration without trusting invocation arguments. */
export function declaredMediaFileExportV1(app: AppDefinition, actionId: MediaFileExportActionIdV1): MediaFileExportV1 | null {
  const actions = [...app.actions, ...app.windowKinds.flatMap(kind => kind.actions)].filter(action => action.id === actionId);
  if (actions.length === 0) return null;
  const value = (id: string): string | null => {
    const values = actions.map(action => {
      const found = action.args.filter(arg => arg.id === id);
      return found.length === 1 && typeof found[0]!.default === "string" ? found[0]!.default : null;
    });
    return values.every(value => value !== null && value === values[0]) ? values[0]! : null;
  };
  const outputPort = value("outputPort"), filename = value("filename"), mediaType = value("mediaType"), formatKind = value("formatKind");
  if (outputPort === null || filename === null || mediaType === null || formatKind === null || !outputPort || !formatKind || !/^[^\x00-\x1f\x7f/\\:*?"<>|]{1,128}$/u.test(filename) || filename.length > 128 || /[. ]$/u.test(filename) || /^(con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/iu.test(filename) || !/^[a-z0-9!#$&^_.+-]+\/[a-z0-9!#$&^_.+-]+$/u.test(mediaType)) return null;
  const ports = app.io?.ports.filter(port => port.id === outputPort);
  if (ports?.length !== 1 || ports[0]!.direction !== "out") return null;
  return { outputPort, filename, mediaType, formatKind, portMediaType: ports[0]!.mediaType };
}

/** 🧾️ Checks the completed binary representation against its declared port and format. */
export function mediaFileDescriptorMatchesV1(value: unknown, declaration: MediaFileExportV1): boolean {
  if (value === null || typeof value !== "object") return false;
  const descriptor = value as { portId?: unknown; mediaType?: { class?: unknown; form?: unknown }; wire?: { kind?: unknown; format_kind?: unknown } };
  return descriptor.portId === declaration.outputPort
    && descriptor.mediaType?.class === declaration.portMediaType.class
    && descriptor.mediaType?.form === declaration.portMediaType.form
    && descriptor.wire?.kind === "binary"
    && descriptor.wire.format_kind === declaration.formatKind;
}

/** 💾️ Delivers fully drained bytes only while the original saved document revision still belongs to the caller. */
export async function downloadDeclaredMediaFileV1(operation: MediaFileExportOperationV1): Promise<boolean> {
  const { declaration, port, instanceId } = operation;
  const cancelled = (): boolean => operation.signal.aborted || !operation.current();
  if (cancelled()) return false;
  const identity = await port.readAppDocumentIdentity(instanceId);
  if (cancelled()) return false;
  if (identity.app_instance_id !== instanceId || !identity.parent_document_id) throw new Error("media-export.document-owner");
  const sourceCurrent = (latest: AppDocumentIdentity): boolean => latest.app_instance_id === identity.app_instance_id && latest.parent_document_id === identity.parent_document_id && latest.base_revision === identity.base_revision;
  operation.progress(0, null);
  const exported = await collectMediaExportBytes({
    instanceId, parentDocumentId: identity.parent_document_id, revision: identity.base_revision,
    outputPort: declaration.outputPort, mediaType: declaration.mediaType, port, cancelled,
    progress: status => operation.progress(Number(status.applied_progress > BigInt(Number.MAX_SAFE_INTEGER) ? BigInt(Number.MAX_SAFE_INTEGER) : status.applied_progress), null),
    ...(operation.yieldTurn === undefined ? {} : { yieldTurn: operation.yieldTurn }),
  });
  if (exported === null || cancelled()) return false;
  if (!mediaFileDescriptorMatchesV1(decodePackValue(Uint8Array.from(exported.descriptor)), declaration)) throw new Error("media-export.descriptor-mismatch");
  const latest = await port.readAppDocumentIdentity(instanceId);
  if (cancelled()) return false;
  if (!sourceCurrent(latest)) throw new Error("media-export.source-changed");
  const sink = await operation.sinkFactory(declaration.filename, declaration.mediaType);
  try {
    const total = exported.chunks.reduce((bytes, chunk) => bytes + chunk.byteLength, 0);
    let completed = 0;
    for (const chunk of exported.chunks) {
      if (cancelled()) { await sink.abort("media-export.cancelled"); return false; }
      await sink.write(chunk);
      completed += chunk.byteLength;
      operation.progress(completed, total);
      await (operation.yieldTurn?.() ?? new Promise<void>(resolve => globalThis.setTimeout(resolve, 0)));
    }
    const beforeDelivery = await port.readAppDocumentIdentity(instanceId);
    if (cancelled()) { await sink.abort("media-export.cancelled"); return false; }
    if (!sourceCurrent(beforeDelivery)) throw new Error("media-export.source-changed");
    await sink.close();
    return true;
  } catch (error) {
    await sink.abort(error);
    throw error;
  }
}
