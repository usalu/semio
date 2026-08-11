/** 🧬️ SemioDocumentMutation — real TS mirror of the hand-rolled named-variant mutation enum (see
 * `🦀️component.rs`). Discriminated union on the `mutation` tag. */
import type { DocBlock, DocImage, DocStyle, RunStyle } from "../📸️snapshot/🟦️component";

export interface DocPathSegmentQuote { kind: "quote"; blockIndex: number; }
export interface DocPathSegmentListItem { kind: "listItem"; blockIndex: number; item: number; }
export interface DocPathSegmentTableCell { kind: "tableCell"; blockIndex: number; row: number; cell: number; }
export type DocPathSegment = DocPathSegmentQuote | DocPathSegmentListItem | DocPathSegmentTableCell;
export interface DocBlockPath { segments: DocPathSegment[]; index: number; }

export type SemioDocumentMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: unknown }
  | { mutation: "insertBlock"; path: DocBlockPath; block: DocBlock }
  | { mutation: "removeBlock"; path: DocBlockPath }
  | { mutation: "setBlockContent"; path: DocBlockPath; block: DocBlock }
  | { mutation: "setParagraphStyle"; path: DocBlockPath; styleId?: string }
  | { mutation: "setHeadingLevel"; path: DocBlockPath; level: number }
  | { mutation: "setListOrdered"; path: DocBlockPath; ordered: boolean }
  | { mutation: "setRunText"; path: DocBlockPath; runIndex: number; text: string }
  | { mutation: "setRunStyle"; path: DocBlockPath; runIndex: number; style: RunStyle }
  | { mutation: "setImageBlock"; path: DocBlockPath; imageId: string; alt: string; width?: number; height?: number }
  | { mutation: "insertStyle"; style: DocStyle }
  | { mutation: "removeStyle"; id: string }
  | { mutation: "setStyleName"; id: string; name: string }
  | { mutation: "setStyleBasedOn"; id: string; basedOn?: string }
  | { mutation: "insertImage"; image: DocImage }
  | { mutation: "removeImage"; id: string }
  | { mutation: "setImageBytes"; id: string; mime: string; bytes: number[] };
