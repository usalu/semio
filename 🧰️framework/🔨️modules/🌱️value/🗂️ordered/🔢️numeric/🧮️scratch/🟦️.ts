/** 🔢️ Mutable fixed-key AVL scratch storage retains spent slots through logical resets. */
export class NumericIndex<V> {
  private nodes: Array<{ key: number; value: V | undefined; generation: number; countGeneration: number; count: number; height: number; left: number | undefined; right: number | undefined; parent: number | undefined }> = [];
  private node(index: number) { return this.nodes[index]!; }
  private root: number | undefined;
  private generation = 1;
  private height(index: number | undefined): number { return index === undefined ? 0 : this.node(index).height; }
  private count(index: number | undefined): number { return index === undefined || this.node(index).countGeneration !== this.generation ? 0 : this.node(index).count; }
  private find(key: number): number | undefined { let current = this.root; while (current !== undefined) { const node = this.node(current); if (key === node.key) return current; current = key < node.key ? node.left : node.right; } return undefined; }
  private update(index: number): void { const node = this.node(index); node.height = 1 + Math.max(this.height(node.left), this.height(node.right)); node.count = this.count(node.left) + this.count(node.right) + Number(node.generation === this.generation && node.value !== undefined); node.countGeneration = this.generation; }
  private replace(parent: number | undefined, before: number, after: number): void { if (parent === undefined) this.root = after; else if (this.node(parent).left === before) this.node(parent).left = after; else this.node(parent).right = after; this.node(after).parent = parent; }
  private rotate(index: number, left: boolean): number { const node = this.node(index); const child = (left ? node.right : node.left)!; const middle = left ? this.node(child).left : this.node(child).right; this.replace(node.parent, index, child); if (left) { node.right = middle; this.node(child).left = index; } else { node.left = middle; this.node(child).right = index; } if (middle !== undefined) this.node(middle).parent = index; node.parent = child; this.update(index); this.update(child); return child; }
  private repair(current: number | undefined): void { while (current !== undefined) { this.update(current); const node = this.node(current); const balance = this.height(node.left) - this.height(node.right); let top = current; if (balance > 1) { const child = node.left!; if (this.height(this.node(child).left) < this.height(this.node(child).right)) this.rotate(child, true); top = this.rotate(current, false); } else if (balance < -1) { const child = node.right!; if (this.height(this.node(child).right) < this.height(this.node(child).left)) this.rotate(child, false); top = this.rotate(current, true); } current = this.node(top).parent; } }
  get size(): number { return this.count(this.root); }
  get storedKeys(): number { return this.nodes.length; }
  get depth(): number { return this.height(this.root); }
  get(key: number): V | undefined { const index = this.find(key); return index === undefined || this.node(index).generation !== this.generation ? undefined : this.node(index).value; }
  set(key: number, value: V): V | undefined {
    if (!Number.isSafeInteger(key) || value === undefined) throw new RangeError("Numeric index requires a fixed integer key and a present value");
    let parent: number | undefined; let current = this.root;
    while (current !== undefined) { const node = this.node(current); if (key === node.key) { const previous = node.generation === this.generation ? node.value : undefined; node.value = value; node.generation = this.generation; this.repair(current); return previous; } parent = current; current = key < node.key ? node.left : node.right; }
    const index = this.nodes.length; this.nodes.push({ key, value, generation: this.generation, countGeneration: this.generation, count: 1, height: 1, left: undefined, right: undefined, parent }); if (parent === undefined) this.root = index; else if (key < this.node(parent).key) this.node(parent).left = index; else this.node(parent).right = index; this.repair(parent); return undefined;
  }
  remove(key: number): V | undefined { const index = this.find(key); if (index === undefined || this.node(index).generation !== this.generation) return undefined; const value = this.node(index).value; this.node(index).value = undefined; this.repair(index); return value; }
  popFirst(): [number, V] | undefined { let current = this.root; if (this.size === 0 || current === undefined) return undefined; for (;;) { const node = this.node(current); if (this.count(node.left) > 0) { current = node.left!; continue; } if (node.generation === this.generation && node.value !== undefined) return [node.key, this.remove(node.key)!]; current = node.right!; } }
  reset(): void { if (this.generation === Number.MAX_SAFE_INTEGER) throw new RangeError("Numeric scratch generation exhausted"); this.generation++; }
}
