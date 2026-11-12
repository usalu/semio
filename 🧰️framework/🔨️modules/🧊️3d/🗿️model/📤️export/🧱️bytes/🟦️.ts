/** 🧱️ The one little-endian byte writer of both encoders: a growing buffer written through `DataView`, so the bytes never depend on the host byte order.
 *
 * @see https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView */
export class ByteWriter {
  bytes: Uint8Array;
  length = 0;
  private view: DataView;

  constructor(capacity = 1 << 16) {
    this.bytes = new Uint8Array(Math.max(16, capacity));
    this.view = new DataView(this.bytes.buffer);
  }

  private ensure(extra: number): void {
    if (this.length + extra <= this.bytes.length) return;
    let size = this.bytes.length;
    while (size < this.length + extra) size *= 2;
    const grown = new Uint8Array(size);
    grown.set(this.bytes.subarray(0, this.length));
    this.bytes = grown;
    this.view = new DataView(grown.buffer);
  }

  u8(value: number): void {
    this.ensure(1);
    this.bytes[this.length++] = value;
  }
  u16(value: number): void {
    this.ensure(2);
    this.view.setUint16(this.length, value, true);
    this.length += 2;
  }
  u32(value: number): void {
    this.ensure(4);
    this.view.setUint32(this.length, value, true);
    this.length += 4;
  }
  i32(value: number): void {
    this.ensure(4);
    this.view.setInt32(this.length, value, true);
    this.length += 4;
  }
  i64(value: number): void {
    this.ensure(8);
    this.view.setBigInt64(this.length, BigInt(value), true);
    this.length += 8;
  }
  f32(value: number): void {
    this.ensure(4);
    this.view.setFloat32(this.length, value, true);
    this.length += 4;
  }
  f64(value: number): void {
    this.ensure(8);
    this.view.setFloat64(this.length, value, true);
    this.length += 8;
  }
  append(bytes: Uint8Array): void {
    this.ensure(bytes.length);
    this.bytes.set(bytes, this.length);
    this.length += bytes.length;
  }
  pad(multiple: number, fill = 0): void {
    const extra = (multiple - (this.length % multiple)) % multiple;
    this.ensure(extra);
    this.bytes.fill(fill, this.length, this.length + extra);
    this.length += extra;
  }
  patchU32(at: number, value: number): void {
    this.view.setUint32(at, value, true);
  }
  finish(): Uint8Array {
    return this.bytes.slice(0, this.length);
  }
  written(): Uint8Array {
    return this.bytes.subarray(0, this.length);
  }
}
