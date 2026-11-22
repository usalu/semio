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

/** 🧰️ Accumulates buffer views and accessors; every view starts on a 4-byte boundary. Repeated payloads share a typed accessor after an exact encoded comparison. Large documents also reuse short payloads through a bounded index. Writing every value counts as work of the optional `pace`, so one huge accessor stays cancellable. */
export class BufferBuilder {
  readonly views: BufferView[] = [];
  readonly accessors: Accessor[] = [];
  private readonly writer = new ByteWriter();

  private readonly shared = new Map<string, number[]>();
  private readonly short = new Map<string, number[]>();
  private readonly float = new Float32Array(1);
  private readonly bits = new Uint32Array(this.float.buffer);

  constructor(private readonly pace: Pace = new Pace()) {}

  private key(values: ArrayLike<number>, component: ComponentCode, type: ElementType, target: number | undefined, bounds: boolean): string {
    let hash = 2166136261;
    for (let i = 0; i < values.length; i++) {
      this.pace.tick();
      const value = values[i]!;
      if (!Number.isFinite(value)) throw new Error(`non-finite value at ${i} of an accessor of ${type}`);
      if (component === ComponentType.Float) this.float[0] = value;
      hash = Math.imul(hash ^ (component === ComponentType.Float ? this.bits[0]! : value), 16777619);
    }
    return `${component}/${type}/${target}/${bounds}/${values.length}/${hash}`;
  }

  private matches(index: number, values: ArrayLike<number>, component: ComponentCode): boolean {
    const stored = this.views[this.accessors[index]!.bufferView]!;
    const bytes = new DataView(this.writer.bytes.buffer, stored.byteOffset, stored.byteLength);
    const read = component === ComponentType.Float ? (i: number) => bytes.getFloat32(i * 4, true)
      : component === ComponentType.UnsignedInt ? (i: number) => bytes.getUint32(i * 4, true)
      : component === ComponentType.UnsignedShort ? (i: number) => bytes.getUint16(i * 2, true)
      : (i: number) => bytes.getUint8(i);
    for (let i = 0; i < values.length; i++) {
      this.pace.tick();
      const value = component === ComponentType.Float ? Math.fround(values[i]!) : values[i]!;
      if (!Object.is(read(i), value)) return false;
    }
    return true;
  }

  /** 📐️ The byte length of the buffer so far, padding included. */
  get byteLength(): number {
    return this.writer.length;
  }

  /** 📥️ Adds one accessor holding `values` (`type` elements, `componentType` components); returns its index. `bounds` writes min/max per component. */
  add(values: ArrayLike<number>, componentType: ComponentCode, type: ElementType, options: { target?: number; bounds?: boolean } = {}): number {
    const components = ElementCount[type];
    if (values.length % components !== 0) throw new Error(`accessor of ${type} needs a multiple of ${components} values, got ${values.length}`);
    const width = componentType === ComponentType.UnsignedByte ? 1 : componentType === ComponentType.UnsignedShort ? 2 : 4;
    const indexForSize = values.length*width >= 1024 ? this.shared : this.writer.length >= 128*1024 ? this.short : undefined;
    const key = indexForSize && this.key(values, componentType, type, options.target, options.bounds === true);
    const candidates = key === undefined ? undefined : indexForSize?.get(key);
    const match = candidates?.find((index) => this.matches(index, values, componentType));
    if (match !== undefined) return match;
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
    const index = this.accessors.push(accessor) - 1;
    if (indexForSize && key !== undefined) {
      if (candidates !== undefined) candidates.push(index);
      else {
        if (indexForSize === this.short && this.short.size >= 4096) this.short.delete(this.short.keys().next().value!);
        indexForSize.set(key,[index]);
      }
    }
    return index;
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
