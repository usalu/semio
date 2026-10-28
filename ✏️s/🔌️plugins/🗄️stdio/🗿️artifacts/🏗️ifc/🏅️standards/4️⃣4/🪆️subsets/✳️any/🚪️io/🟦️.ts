/** 🏗️ IFC4-specific STEP boundary, retaining the subset's owned snapshot types. */
import type {IfcSnapshot, IfcValue} from "../🧬️schema/📸️snapshot/🟦️.ts";
import {binary64, binary64Value} from "@semio-tech/framework/value/ieee754";
import {Part21Control, parsePart21, writePart21, type Part21Document, type Part21Options, type Part21Value} from "../../../../../../../📇️registry/🧬️contract/📐️part21/🟦️.ts";

export type Ifc4IoOptions = Part21Options;
const DOCUMENT_SCHEMA = "stdio.ifc";

function validate(document: Part21Document, control: Part21Control): void {
  const tick = (depth = 0) => control.checkDepth(depth);
  const text = (value: Part21Value | undefined): boolean => value?.kind === "string";
  const strings = (value: Part21Value | undefined): boolean => value?.kind === "aggregate" && value.value.length > 0 && value.value.every(text);
  const description = document.header.fileDescription;
  if (description.length !== 2 || !strings(description[0]) || !text(description[1])) throw new Error("ifc4: invalid FILE_DESCRIPTION attributes");
  const name = document.header.fileName;
  if (name.length !== 7 || ![0, 1, 4, 5, 6].every(index => text(name[index])) || !strings(name[2]) || !strings(name[3])) throw new Error("ifc4: invalid FILE_NAME attributes");
  const schema = document.header.fileSchema;
  if (schema.length !== 1 || schema[0]?.kind !== "aggregate" || schema[0].value.length !== 1 || schema[0].value[0]?.kind !== "string" || schema[0].value[0].value !== "IFC4") throw new Error("ifc4: FILE_SCHEMA must contain exactly IFC4");
  const ids = new Set<bigint>();
  for (const instance of document.instances) {
    tick();
    if (instance.id < 1n || instance.id > 18446744073709551615n || ids.has(instance.id)) throw new Error("ifc4: instance id must be positive unique u64");
    ids.add(instance.id);
  }
  for (const instance of document.instances) {
    const pending = instance.entities.flatMap(entity => entity.args.map(value => ({value, depth: 0})));
    while (pending.length) {
      const {value, depth} = pending.pop()!; tick(depth);
      if (value.kind === "reference" && !ids.has(value.value)) throw new Error("ifc4: instance #" + instance.id + " references missing #" + value.value);
      if (value.kind === "aggregate") for (const item of value.value) pending.push({value: item, depth: depth + 1});
      if (value.kind === "typedValue") for (const item of value.value.items) pending.push({value: item, depth: depth + 1});
    }
  }
}

function snapshotFromDocument(document: Part21Document, control: Part21Control): IfcSnapshot {
  const tick = (depth = 0) => control.checkDepth(depth);
  const value = (item: Part21Value, depth = 0): IfcValue => {
    tick(depth);
    switch (item.kind) {
      case "real": {
        const number = Number(item.value);
        if (!Number.isFinite(number)) throw new Error("ifc4: real outside finite binary64 range");
        return {kind: "real", value: binary64(number)};
      }
      case "aggregate": return {kind: "aggregate", value: item.value.map(item => value(item, depth + 1))};
      case "typedValue": return {kind: "typedValue", value: {name: item.value.name, items: item.value.items.map(item => value(item, depth + 1))}};
      default: return {...item};
    }
  };
  return {
    schema: DOCUMENT_SCHEMA,
    header: {fileDescription: document.header.fileDescription.map(item => value(item)), fileName: document.header.fileName.map(item => value(item)), fileSchema: document.header.fileSchema.map(item => value(item))},
    entities: document.instances.map(instance => {
      tick(); const [primary, ...complex] = instance.entities;
      if (!primary) throw new Error("ifc4: empty complex instance");
      return {id: instance.id, name: primary.name, args: primary.args.map(item => value(item)), complex: complex.map(record => ({name: record.name, args: record.args.map(item => value(item))}))};
    }),
  };
}

function documentFromSnapshot(snapshot: IfcSnapshot, control: Part21Control): Part21Document {
  if (snapshot.schema !== DOCUMENT_SCHEMA) throw new Error("ifc4: foreign snapshot schema");
  const tick = (depth = 0) => control.checkDepth(depth);
  const value = (item: IfcValue, depth = 0): Part21Value => {
    tick(depth);
    switch (item.kind) {
      case "real": {
        const number = binary64Value(item.value);
        if (!Number.isFinite(number)) throw new Error("ifc4: cannot export nonfinite STEP real");
        if (Object.is(number, -0)) return {kind: "real", value: "-0."};
        const [mantissa, exponent] = number.toString().toUpperCase().split("E");
        return {kind: "real", value: (mantissa!.includes(".") ? mantissa : mantissa + ".") + (exponent === undefined ? "" : "E" + exponent)};
      }
      case "aggregate": return {kind: "aggregate", value: item.value.map(item => value(item, depth + 1))};
      case "typedValue": return {kind: "typedValue", value: {name: item.value.name, items: item.value.items.map(item => value(item, depth + 1))}};
      default: return {...item};
    }
  };
  return {
    header: {fileDescription: snapshot.header.fileDescription.map(item => value(item)), fileName: snapshot.header.fileName.map(item => value(item)), fileSchema: snapshot.header.fileSchema.map(item => value(item))},
    instances: snapshot.entities.map(entity => {
      tick(); return {id: entity.id, entities: [{name: entity.name, args: entity.args.map(item => value(item))}, ...(entity.complex ?? []).map(record => ({name: record.name, args: record.args.map(item => value(item))}))]};
    }),
  };
}

/** 📥️ Decodes UTF-8 IFC4 SPF into the subset's snapshot after strict framing/header/reference validation. */
export function decodeIfc4(bytes: Uint8Array | string, options: Ifc4IoOptions = {}): IfcSnapshot {
  const control = new Part21Control(options);
  if (bytes.length > (options.maxInputLength ?? 16_000_000)) throw new Error("ifc4: input length limit");
  const text = typeof bytes === "string" ? bytes : new TextDecoder("utf-8", {fatal: true}).decode(bytes);
  const document = parsePart21(text, options, control); validate(document, control);
  const snapshot = snapshotFromDocument(document, control); control.finish(); return snapshot;
}

/** 📤️ Rebuilds IFC4 SPF bytes from the snapshot alone; malformed or unsupported values are refused. */
export function encodeIfc4(snapshot: IfcSnapshot, options: Ifc4IoOptions = {}): Uint8Array {
  const control = new Part21Control(options);
  const document = documentFromSnapshot(snapshot, control); validate(document, control);
  return new TextEncoder().encode(writePart21(document, options, control));
}
