/** 🧬️ Jack artifact schema with one composed graph-content identity. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface JackArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ name: string;
  /** @state artifact */ manifestId?: string;
  /** @state artifact */ manifest: Manifest;
  /** @state artifact */ camera: Camera;
  /** @state artifact @child kind=s.stdio.semio.graph */ content: ArtifactChild;
  /** @state artifact */ rootNodeId?: string;
}

export interface Camera { x: number; y: number; zoom: number }
export interface Manifest { nodeKinds: ManifestKind[]; edgeKinds: ManifestKind[]; portKinds: ManifestPortKind[] }
export interface ManifestKind { name: string }
export interface ManifestPortKind { name: string; direction: string }

const object = (value: unknown, at: string): Readonly<Record<string, unknown>> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: value must be an object`);
  return value as Record<string, unknown>;
};

const string = (value: unknown, at: string): string => {
  if (typeof value !== "string") throw new Error(`${at}: value must be a string`);
  return value;
};

const number = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${at}: value must be a finite number`);
  return value;
};

const exact = (row: Readonly<Record<string, unknown>>, required: readonly string[], optional: readonly string[], at: string): void => {
  const allowed = new Set([...required, ...optional]);
  if (required.some((key) => !Object.hasOwn(row, key)) || Object.keys(row).some((key) => !allowed.has(key))) throw new Error(`${at}: fields do not match the Jack schema`);
};

/** 🪪️ Parses the persisted Jack document boundary and refuses embedded graph payloads. */
export function parseJackArtifact(value: unknown, at = "$"): JackArtifact {
  const row = object(value, at);
  exact(row, ["schema", "name", "manifest", "camera", "content"], ["manifestId", "rootNodeId"], at);
  return {
    schema: string(row.schema, `${at}.schema`),
    name: string(row.name, `${at}.name`),
    manifestId: row.manifestId === undefined ? undefined : string(row.manifestId, `${at}.manifestId`),
    manifest: parseManifest(row.manifest, `${at}.manifest`),
    camera: parseCamera(row.camera, `${at}.camera`),
    content: parseArtifactChild(row.content),
    rootNodeId: row.rootNodeId === undefined ? undefined : string(row.rootNodeId, `${at}.rootNodeId`),
  };
}

export function parseCamera(value: unknown, at = "$"): Camera {
  const row = object(value, at);
  exact(row, ["x", "y", "zoom"], [], at);
  return { x: number(row.x, `${at}.x`), y: number(row.y, `${at}.y`), zoom: number(row.zoom, `${at}.zoom`) };
}

export function parseManifest(value: unknown, at = "$"): Manifest {
  const row = object(value, at);
  exact(row, ["nodeKinds", "edgeKinds", "portKinds"], [], at);
  const kinds = (entry: unknown, key: string): ManifestKind[] => {
    if (!Array.isArray(entry)) throw new Error(`${at}.${key}: value must be an array`);
    return entry.map((item, index) => parseManifestKind(item, `${at}.${key}[${index}]`));
  };
  if (!Array.isArray(row.portKinds)) throw new Error(`${at}.portKinds: value must be an array`);
  return {
    nodeKinds: kinds(row.nodeKinds, "nodeKinds"),
    edgeKinds: kinds(row.edgeKinds, "edgeKinds"),
    portKinds: row.portKinds.map((item, index) => parseManifestPortKind(item, `${at}.portKinds[${index}]`)),
  };
}

export function parseManifestKind(value: unknown, at = "$"): ManifestKind {
  const row = object(value, at);
  exact(row, ["name"], [], at);
  return { name: string(row.name, `${at}.name`) };
}

export function parseManifestPortKind(value: unknown, at = "$"): ManifestPortKind {
  const row = object(value, at);
  exact(row, ["name", "direction"], [], at);
  return { name: string(row.name, `${at}.name`), direction: string(row.direction, `${at}.direction`) };
}
