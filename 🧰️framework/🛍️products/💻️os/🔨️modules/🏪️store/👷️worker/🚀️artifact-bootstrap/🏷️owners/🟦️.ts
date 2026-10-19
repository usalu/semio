import type { ArtifactBootstrapAssembler } from "@semio-tech/framework-replication";
import type { FolderCanonicalBootstrapMirrorOwner } from "../../📁️folder/🏷️mirror-owner/🟦️.ts";

/** 🎟️ The private owner of one canonical artifact bootstrap: the assembler, folder mirror, deadline and socket that admitted it. Types only, so the document state and the session teardown hold it without importing the bootstrap module. */
export type DocumentArtifactBootstrapOwner = {
  assembler: ArtifactBootstrapAssembler | null;
  folderMirror: FolderCanonicalBootstrapMirrorOwner | null;
  deadlineMs: number | null;
  deadlineTimer: ReturnType<typeof setTimeout> | null;
  readonly rebootstrapOwner: DocumentArtifactRebootstrapOwner | null;
  readonly socket: WebSocket | null;
  assertCurrent(): void;
};

/** ⏳️ The private owner of one re-bootstrap deadline armed after a refused or lagging tail. */
export type DocumentArtifactRebootstrapOwner = {
  readonly deadlineMs: number;
  readonly timer: ReturnType<typeof setTimeout>;
  assertCurrent(): void;
};
