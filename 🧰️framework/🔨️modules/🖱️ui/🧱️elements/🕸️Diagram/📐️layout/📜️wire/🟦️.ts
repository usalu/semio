import type { Edge, Node } from "@xyflow/react";

/**
 * 🖼️ Base pixel unit for diagram node sizing.
 **/
export const DIAGRAM_UNIT = 48;

/**
 * 📊️ Union type for diagram layout directions (TB/BT/LR/RL).
 **/
export type DiagramLayoutDirection = "TB" | "BT" | "LR" | "RL";

/**
 * 📈️ Configuration for the owned deterministic directed layout.
 **/
export interface DiagramLayoutOptions {
  direction?: DiagramLayoutDirection;
  nodeWidth?: number;
  nodeHeight?: number;
  rankSep?: number;
  nodeSep?: number;
}

export const DIAGRAM_LAYOUT_CODEC_KIND = "diagram-directed-layout-v1";
export const DIAGRAM_LAYOUT_INGRESS_ITEMS = 64;
export const DIAGRAM_LAYOUT_INGRESS_BYTES = 16 * 1024;
export const DIAGRAM_LAYOUT_OUTPUT_ITEMS = 128;
export const DIAGRAM_LAYOUT_MAX_INPUT_ITEMS = 65_536;
export const DIAGRAM_LAYOUT_MAX_ID_CHARACTERS = 512;
export const DIAGRAM_LAYOUT_MAX_NODE_BYTES = 64 + DIAGRAM_LAYOUT_MAX_ID_CHARACTERS * 4;
export const DIAGRAM_LAYOUT_MAX_EDGE_BYTES = 64 + DIAGRAM_LAYOUT_MAX_ID_CHARACTERS * 4 * 3;
export const DIAGRAM_LAYOUT_MAX_RESERVED_BYTES = 256 * 1024 * 1024;

export interface DiagramLayoutNodeWire {
  readonly height?: number;
  readonly id: string;
  readonly index: number;
  readonly measuredHeight?: number;
  readonly measuredWidth?: number;
  readonly styleHeight?: number;
  readonly styleWidth?: number;
  readonly width?: number;
}

export interface DiagramLayoutEdgeWire {
  readonly id: string;
  readonly index: number;
  readonly source: string;
  readonly target: string;
}

export type DiagramLayoutIngressPage =
  | { readonly bytes: number; readonly complete?: boolean; readonly generation: number; readonly kind: "nodes"; readonly offset: number; readonly values: readonly DiagramLayoutNodeWire[] }
  | { readonly bytes: number; readonly complete?: boolean; readonly generation: number; readonly kind: "edges"; readonly offset: number; readonly values: readonly DiagramLayoutEdgeWire[] }
  | { readonly generation: number; readonly kind: "seal" };

export interface DiagramLayoutPositionPage {
  readonly complete: boolean;
  readonly generation: number;
  readonly kind: "positions";
  readonly sequence: number;
  readonly values: readonly DiagramLayoutPosition[];
}

export type DiagramLayoutTerminal = { readonly generation: number; readonly kind: "terminal"; readonly status: "complete" | "cancelled" } | { readonly generation: number; readonly kind: "terminal"; readonly reason: string; readonly status: "fault" };

export interface DiagramLayoutDescriptor {
  readonly edgeCount: number;
  readonly generation: number;
  readonly kind: typeof DIAGRAM_LAYOUT_CODEC_KIND;
  readonly nodeCount: number;
  readonly options: DiagramLayoutOptions;
}

export interface DiagramLayoutHostPage {
  readonly byteLength: number;
  readonly complete: boolean;
  readonly itemCount: number;
  readonly payload: unknown;
}

export interface DiagramLayoutPublicationResult {
  readonly edges: Edge[];
  readonly nodes: Node[];
  closeStep(): boolean;
}

export function diagramLayoutUtf8Bytes(value: string): number {
  let bytes = 0;
  let characters = 0;
  for (let index = 0; index < value.length; index++) {
    characters += 1;
    if (characters > DIAGRAM_LAYOUT_MAX_ID_CHARACTERS) throw new Error("Diagram layout id exceeds 512 Unicode characters");
    const code = value.charCodeAt(index);
    if (code <= 0x7f) bytes += 1;
    else if (code <= 0x7ff) bytes += 2;
    else if (code >= 0xd800 && code <= 0xdbff && index + 1 < value.length && value.charCodeAt(index + 1) >= 0xdc00 && value.charCodeAt(index + 1) <= 0xdfff) {
      bytes += 4;
      index += 1;
    } else bytes += 3;
  }
  return bytes;
}

export function diagramLayoutNodeWireBytes(value: DiagramLayoutNodeWire): number {
  return 64 + diagramLayoutUtf8Bytes(value.id);
}

export function diagramLayoutEdgeWireBytes(value: DiagramLayoutEdgeWire): number {
  return 64 + diagramLayoutUtf8Bytes(value.id) + diagramLayoutUtf8Bytes(value.source) + diagramLayoutUtf8Bytes(value.target);
}

export function diagramLayoutIdentityAdmitted(value: unknown, allowEmpty = false): value is string {
  if (typeof value !== "string" || (!allowEmpty && value.length === 0)) return false;
  try {
    diagramLayoutUtf8Bytes(value);
    return true;
  } catch {
    return false;
  }
}

export type DiagramLayoutCredits =
  | { readonly admitted: true; readonly inputBytes: number; readonly inputItems: number; readonly outputBytes: number; readonly outputItems: number }
  | { readonly admitted: false; readonly reason: "bytes" | "items" };

/** 📏️ Declares exact worst-case credits from counts without reading either source array. */
export function diagramLayoutCredits(nodeCount: number, edgeCount: number): DiagramLayoutCredits {
  if (!Number.isSafeInteger(nodeCount) || !Number.isSafeInteger(edgeCount) || nodeCount < 0 || edgeCount < 0 || nodeCount + edgeCount > DIAGRAM_LAYOUT_MAX_INPUT_ITEMS) return { admitted: false, reason: "items" };
  const inputBytes = nodeCount * DIAGRAM_LAYOUT_MAX_NODE_BYTES + edgeCount * DIAGRAM_LAYOUT_MAX_EDGE_BYTES;
  const outputBytes = nodeCount * 32;
  if (!Number.isSafeInteger(inputBytes) || inputBytes + outputBytes > DIAGRAM_LAYOUT_MAX_RESERVED_BYTES) return { admitted: false, reason: "bytes" };
  return { admitted: true, inputBytes, inputItems: nodeCount + edgeCount, outputBytes, outputItems: nodeCount };
}

/** 📍️ One laid-out node position of a diagram layout. */
export interface DiagramLayoutPosition {
  readonly index: number;
  readonly x: number;
  readonly y: number;
}

/** 📄️ Items per page of the paged layout stores. */
export const diagramLayoutPageSize = 128;
