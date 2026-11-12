/** 🧱️ The one binary buffer of a GLB: 4-byte aligned buffer views and typed accessors, written little-endian through `DataView` so the bytes never depend on the host.
 *
 * @see https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#accessors
 */
import { Pace } from "../../../⏱️operation/🟦️.ts";
import { ByteWriter } from "../../🧱️bytes/🟦️.ts";

/** 🔢️ The glTF component types this encoder writes. */
export const ComponentType = { UnsignedByte: 5121, UnsignedShort: 5123, UnsignedInt: 5125, Float: 5126 } as const;
/** 🎯️ The buffer view targets of vertex attributes and indices. */
export const BufferTarget = { Attribute: 34962, Index: 34963 } as const;

/** 📏️ The element types of an accessor and their component counts. */
export const ElementCount = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 } as const;
/** 🔤️ An accessor element type. */
export type ElementType = keyof typeof ElementCount;
/** 🔣️ A glTF component type code. */
export type ComponentCode = (typeof ComponentType)[keyof typeof ComponentType];

/** 🧾️ A glTF buffer view. */
export type BufferView = { buffer: 0; byteOffset: number; byteLength: number; target?: number };
/** 🪪️ A glTF accessor. */
export type Accessor = { bufferView: number; componentType: number; count: number; type: ElementType; min?: number[]; max?: number[] };

/** 🧰️ Accumulates buffer views and accessors; every view starts on a 4-byte boundary. Writing every value counts as work of the optional `pace`, so one huge accessor stays cancellable. */
export class BufferBuilder {
  readonly views: BufferView[] = [];
  readonly accessors: Accessor[] = [];
  private readonly writer = new ByteWriter();

  constructor(private readonly pace: Pace = new Pace()) {}

  /** 📐️ The byte length of the buffer so far, padding included. */
  get byteLength(): number {
    return this.writer.length;
  }

  /** 📥️ Adds one accessor holding `values` (`type` elements, `componentType` components); returns its index. `bounds` writes min/max per component. */
  add(values: ArrayLike<number>, componentType: ComponentCode, type: ElementType, options: { target?: number; bounds?: boolean } = {}): number {
    const components = ElementCount[type];
    if (values.length % components !== 0) throw new Error(`accessor of ${type} needs a multiple of ${components} values, got ${values.length}`);
    const byteOffset = this.writer.length;
    const write = componentType === ComponentType.Float ? (v: number) => this.writer.f32(v) : componentType === ComponentType.UnsignedInt ? (v: number) => this.writer.u32(v) : componentType === ComponentType.UnsignedShort ? (v: number) => this.writer.u16(v) : (v: number) => this.writer.u8(v);
    for (let i = 0; i < values.length; i += 1) {
      this.pace.tick();
      const value = values[i]!;
      if (!Number.isFinite(value)) throw new Error(`non-finite value at ${i} of an accessor of ${type}`);
      write(value);
    }
    const byteLength = this.writer.length - byteOffset;
    this.writer.pad(4);
    const viewIndex = this.views.length;
    this.views.push({ buffer: 0, byteOffset, byteLength, ...(options.target === undefined ? {} : { target: options.target }) });
    const accessor: Accessor = { bufferView: viewIndex, componentType, count: values.length / components, type };
    if (options.bounds === true && values.length > 0) {
      const min = new Array<number>(components).fill(Infinity);
      const max = new Array<number>(components).fill(-Infinity);
      for (let i = 0; i < values.length; i += 1) {
        this.pace.tick();
        const value = componentType === ComponentType.Float ? Math.fround(values[i]!) : values[i]!;
        const c = i % components;
        if (value < min[c]!) min[c] = value;
        if (value > max[c]!) max[c] = value;
      }
      accessor.min = min;
      accessor.max = max;
    }
    this.accessors.push(accessor);
    return this.accessors.length - 1;
  }

  /** 📦️ The buffer bytes (a multiple of four long). */
  bytes(): Uint8Array {
    return this.writer.finish();
  }
}

/** 🎁️ Wraps a JSON chunk and an optional binary chunk into GLB bytes (container version 2, chunks padded with spaces and zeros). */
export function packGlb(json: string, bin: Uint8Array): Uint8Array {
  const jsonBytes = new TextEncoder().encode(json);
  const jsonLength = jsonBytes.length + ((4 - (jsonBytes.length % 4)) % 4);
  const binLength = bin.length + ((4 - (bin.length % 4)) % 4);
  const out = new ByteWriter(12 + 8 + jsonLength + (bin.length > 0 ? 8 + binLength : 0));
  out.u32(0x46546c67);
  out.u32(2);
  out.u32(0);
  out.u32(jsonLength);
  out.u32(0x4e4f534a);
  out.append(jsonBytes);
  out.pad(4, 0x20);
  if (bin.length > 0) {
    out.u32(binLength);
    out.u32(0x004e4942);
    out.append(bin);
    out.pad(4);
  }
  out.patchU32(8, out.length);
  return out.finish();
}
