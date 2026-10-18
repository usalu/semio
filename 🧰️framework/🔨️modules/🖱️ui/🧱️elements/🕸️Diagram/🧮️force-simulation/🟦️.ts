import { type DiagramForceConfig, type DiagramForceDragBatch, type DiagramForceDragKind, type DiagramForceDragNode, type DiagramForceFrame, type DiagramForceHashCursor, type DiagramForceIdentity, type DiagramForceInitializationPhase, type DiagramForceLink, type DiagramForceLookupCursor, type DiagramForceMergeCursor, type DiagramForceNode, type DiagramForceNodeRuntime, type DiagramForcePairCursor, type DiagramForcePendingLink, type DiagramForcePort, type DiagramForceSimulation, type DiagramForceSource, type DiagramForceStepResult, type DiagramForceTickPhase, type DiagramForceWork, type DiagramForceWorkCursor, type ResolvedDiagramForceLink, diagramForceAlphaDecay, diagramForceAlphaMin, diagramForceBudget, diagramForceVelocityRetention } from "../📋️force-model/🟦️.ts";

function fallbackForcePosition(identity: DiagramForceIdentity): readonly [number, number] {
  const angle = (identity.hashA / 0x1_0000_0000) * Math.PI * 2;
  const radius = 10 + ((identity.hashB >>> 8) % 17);
  return [Math.cos(angle) * radius, Math.sin(angle) * radius];
}

function forceJiggle(left: DiagramForceIdentity, right: DiagramForceIdentity): readonly [number, number] {
  const [first, second] = left.index <= right.index ? [left, right] : [right, left];
  const hash = Math.imul(first.hashA ^ second.hashC ^ first.index, 16777619) ^ Math.imul(first.hashB ^ second.hashD ^ second.index, 668265263);
  const angle = (hash / 0x1_0000_0000) * Math.PI * 2;
  return [Math.cos(angle) * 1e-6, Math.sin(angle) * 1e-6];
}

function createForceMergeCursor<Value>(source: Value[]): DiagramForceMergeCursor<Value> {
  return { comparisonCursor: 0, left: 0, leftCursor: 0, middle: Math.min(1, source.length), right: Math.min(2, source.length), rightCursor: Math.min(1, source.length), source, target: [], width: 1 };
}

function takeForceFuel(work: DiagramForceWorkCursor): boolean {
  if (work.remaining <= 0) return false;
  work.remaining -= 1;
  return true;
}

function stepForceTextComparison(left: string, right: string, cursor: DiagramForceMergeCursor<any> | DiagramForceLookupCursor<any>, work: DiagramForceWorkCursor): number | undefined {
  if (!takeForceFuel(work)) return undefined;
  const length = Math.min(left.length, right.length);
  if (cursor.comparisonCursor >= length) return left.length - right.length;
  const result = left.charCodeAt(cursor.comparisonCursor) - right.charCodeAt(cursor.comparisonCursor);
  cursor.comparisonCursor += 1;
  return result || (cursor.comparisonCursor >= length ? left.length - right.length : undefined);
}

function stepForceMerge<Value>(cursor: DiagramForceMergeCursor<Value>, work: DiagramForceWorkCursor, text: (value: Value) => string, tie: (left: Value, right: Value) => number, duplicateFault: boolean): boolean {
  if (cursor.source.length < 2 || cursor.width >= cursor.source.length) return true;
  if (cursor.left >= cursor.source.length) {
    cursor.source = cursor.target;
    cursor.target = [];
    cursor.width *= 2;
    cursor.comparisonCursor = 0;
    cursor.left = 0;
    cursor.leftCursor = 0;
    cursor.middle = Math.min(cursor.width, cursor.source.length);
    cursor.rightCursor = cursor.middle;
    cursor.right = Math.min(cursor.width * 2, cursor.source.length);
    return cursor.width >= cursor.source.length;
  }
  if (cursor.leftCursor >= cursor.middle && cursor.rightCursor >= cursor.right) {
    cursor.left += cursor.width * 2;
    cursor.leftCursor = cursor.left;
    cursor.middle = Math.min(cursor.left + cursor.width, cursor.source.length);
    cursor.rightCursor = cursor.middle;
    cursor.right = Math.min(cursor.left + cursor.width * 2, cursor.source.length);
    cursor.comparisonCursor = 0;
    return false;
  }
  if (!takeForceFuel(work)) return false;
  if (cursor.rightCursor >= cursor.right || cursor.leftCursor >= cursor.middle) {
    cursor.target.push(cursor.source[cursor.rightCursor >= cursor.right ? cursor.leftCursor++ : cursor.rightCursor++]!);
    cursor.comparisonCursor = 0;
    return false;
  }
  work.remaining += 1;
  const left = cursor.source[cursor.leftCursor]!;
  const right = cursor.source[cursor.rightCursor]!;
  const textResult = stepForceTextComparison(text(left), text(right), cursor, work);
  if (textResult === undefined) return false;
  if (duplicateFault && textResult === 0) throw new Error("Duplicate Diagram force node id");
  const result = textResult || tie(left, right);
  const takeLeft = result <= 0;
  cursor.target.push(cursor.source[takeLeft ? cursor.leftCursor++ : cursor.rightCursor++]!);
  cursor.comparisonCursor = 0;
  return false;
}

export function finiteForceValue(value: number | null | undefined, fallback: number): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

/** 🧲️ Deterministic force subset with finite per-frame and per-tick work. */
export class OwnedDiagramForceSimulation<NodeType extends DiagramForceNode> implements DiagramForceSimulation<NodeType> {
  private orderedNodes: DiagramForceNodeRuntime<NodeType>[] = [];
  private resolvedLinks: ResolvedDiagramForceLink<NodeType>[] = [];
  private readonly chargeStrength: number;
  private readonly linkDistance: number;
  private readonly collideRadius: number;
  private readonly centerStrength: number;
  private readonly updateIntervalMs: number;
  private alpha = 1;
  private targetAlpha = 0;
  private listener?: (frame: DiagramForceFrame) => boolean | void;
  private running = false;
  private frameHandle?: number;
  private frameKind?: "animation" | "timeout";
  private lastNotification = Number.NEGATIVE_INFINITY;
  private notificationComplete = false;
  private notificationGeneration = 0;
  private notificationPending = false;
  private generation = 0;
  private chargeCursor: DiagramForcePairCursor = { left: 0, right: 1 };
  private collideCursor: DiagramForcePairCursor = { left: 0, right: 1 };
  private linkCursor = 0;
  private nodeCursor = 0;
  private tickPhase: DiagramForceTickPhase = "alpha";
  private tickPhaseProgress = 0;
  private initializationPhase: DiagramForceInitializationPhase = "nodes";
  private initializationCursor = 0;
  private initializationGeneration = 0;
  private pendingNode?: DiagramForceHashCursor<NodeType>;
  private pendingLink?: DiagramForcePendingLink<NodeType>;
  private nodeSort?: DiagramForceMergeCursor<DiagramForceNodeRuntime<NodeType>>;
  private linkSort?: DiagramForceMergeCursor<ResolvedDiagramForceLink<NodeType>>;
  private pendingDrag?: DiagramForceDragBatch<NodeType>;
  private releasePinned = false;
  private releasePinnedIterator?: IterableIterator<DiagramForceNodeRuntime<NodeType>>;
  private readonly pinnedNodes = new Set<DiagramForceNodeRuntime<NodeType>>();

  constructor(
    private readonly forceNodes: NodeType[],
    private readonly source: DiagramForceSource<NodeType>,
    private readonly ownsForceNodes: boolean,
    config: DiagramForceConfig,
  ) {
    this.chargeStrength = finiteForceValue(config.chargeStrength, -100);
    this.linkDistance = Math.max(0, finiteForceValue(config.linkDistance, 100));
    this.collideRadius = Math.max(0, finiteForceValue(config.collideRadius, 50));
    this.centerStrength = Math.max(0, finiteForceValue(config.centerStrength, 0.1));
    this.updateIntervalMs = Math.max(0, finiteForceValue(config.updateIntervalMs, 50));
  }

  alphaDecay(): number {
    return diagramForceAlphaDecay;
  }

  alphaMin(): number {
    return diagramForceAlphaMin;
  }

  alphaTarget(value: number): this {
    this.targetAlpha = finiteForceValue(value, 0);
    return this;
  }

  drag(kind: DiagramForceDragKind, node: DiagramForceDragNode, nodes: readonly DiagramForceDragNode[]): this {
    if (kind === "start" || kind === "stop") this.releasePinned = true;
    this.pendingDrag = kind === "stop" ? undefined : { cursor: 0, kind, length: node.selected && nodes.length > 0 ? nodes.length : 1, node, nodes, selected: Boolean(node.selected && nodes.length > 0) };
    this.generation += 1;
    if (this.initializationPhase !== "ready") this.initializationGeneration = this.generation;
    this.lastNotification = Number.NEGATIVE_INFINITY;
    this.notificationPending = false;
    if (this.running) {
      this.cancelFrame();
      if (!this.scheduleFrame()) this.running = false;
    }
    return this;
  }

  nodes(): NodeType[] {
    return this.forceNodes;
  }

  on(_event: "tick", listener: (frame: DiagramForceFrame) => boolean | void): this {
    this.listener = listener;
    return this;
  }

  restart(): this {
    this.generation += 1;
    if (this.initializationPhase !== "ready") this.initializationGeneration = this.generation;
    this.lastNotification = Number.NEGATIVE_INFINITY;
    this.notificationPending = false;
    if (this.running) this.cancelFrame();
    this.running = true;
    if (!this.scheduleFrame()) this.running = false;
    return this;
  }

  stop(): this {
    this.running = false;
    this.generation += 1;
    if (this.initializationPhase !== "ready") this.initializationGeneration = this.generation;
    this.notificationPending = false;
    this.cancelFrame();
    return this;
  }

  step(work: DiagramForceWork): DiagramForceStepResult {
    const cursor = { deadline: finiteForceValue(work.deadline, this.now()), remaining: Math.max(0, Math.floor(finiteForceValue(work.fuel, 0))) };
    const initialized = this.stepInitialization(cursor);
    const dragged = initialized && this.stepDrag(cursor);
    const tickComplete = dragged && this.stepTick(cursor);
    return { initialized, remainingFuel: cursor.remaining, tickComplete };
  }

  private recover(runtime: DiagramForceNodeRuntime<NodeType>): void {
    const node = runtime.node;
    node.x = finiteForceValue(node.x, runtime.fallbackX);
    node.y = finiteForceValue(node.y, runtime.fallbackY);
    node.vx = finiteForceValue(node.vx, 0);
    node.vy = finiteForceValue(node.vy, 0);
    if (node.fx !== undefined && node.fx !== null && !Number.isFinite(node.fx)) node.fx = null;
    if (node.fy !== undefined && node.fy !== null && !Number.isFinite(node.fy)) node.fy = null;
  }

  private advancePair(cursor: DiagramForcePairCursor): void {
    cursor.right += 1;
    if (cursor.right < this.orderedNodes.length) return;
    cursor.left += 1;
    if (cursor.left >= this.orderedNodes.length - 1) cursor.left = 0;
    cursor.right = cursor.left + 1;
  }

  private applyChargePair(): void {
    const left = this.orderedNodes[this.chargeCursor.left]!;
    const right = this.orderedNodes[this.chargeCursor.right]!;
    this.recover(left);
    this.recover(right);
    let deltaX = right.node.x! - left.node.x!;
    let deltaY = right.node.y! - left.node.y!;
    let distanceSquared = deltaX * deltaX + deltaY * deltaY;
    if (distanceSquared < 1e-12) {
      [deltaX, deltaY] = forceJiggle(left.identity, right.identity);
      distanceSquared = deltaX * deltaX + deltaY * deltaY;
    }
    const scale = (this.chargeStrength * this.alpha) / distanceSquared;
    left.node.vx! += deltaX * scale;
    left.node.vy! += deltaY * scale;
    right.node.vx! -= deltaX * scale;
    right.node.vy! -= deltaY * scale;
    this.advancePair(this.chargeCursor);
  }

  private applyLink(): void {
    const link = this.resolvedLinks[this.linkCursor]!;
    this.linkCursor = (this.linkCursor + 1) % this.resolvedLinks.length;
    this.recover(link.source);
    this.recover(link.target);
    let deltaX = link.target.node.x! + link.target.node.vx! - link.source.node.x! - link.source.node.vx!;
    let deltaY = link.target.node.y! + link.target.node.vy! - link.source.node.y! - link.source.node.vy!;
    let distance = Math.hypot(deltaX, deltaY);
    if (distance < 1e-6) {
      [deltaX, deltaY] = forceJiggle(link.source.identity, link.target.identity);
      distance = Math.hypot(deltaX, deltaY);
    }
    const scale = ((distance - this.linkDistance) / distance) * this.alpha * link.strength;
    deltaX *= scale;
    deltaY *= scale;
    link.target.node.vx! -= deltaX * link.bias;
    link.target.node.vy! -= deltaY * link.bias;
    link.source.node.vx! += deltaX * (1 - link.bias);
    link.source.node.vy! += deltaY * (1 - link.bias);
  }

  private applyCollisionPair(): void {
    const diameter = this.collideRadius * 2;
    const left = this.orderedNodes[this.collideCursor.left]!;
    const right = this.orderedNodes[this.collideCursor.right]!;
    this.recover(left);
    this.recover(right);
    let deltaX = right.node.x! + right.node.vx! - left.node.x! - left.node.vx!;
    let deltaY = right.node.y! + right.node.vy! - left.node.y! - left.node.vy!;
    let distance = Math.hypot(deltaX, deltaY);
    if (distance < 1e-6) {
      [deltaX, deltaY] = forceJiggle(left.identity, right.identity);
      distance = Math.hypot(deltaX, deltaY);
    }
    if (distance < diameter) {
      const scale = ((diameter - distance) / distance) * 0.5;
      left.node.vx! -= deltaX * scale;
      left.node.vy! -= deltaY * scale;
      right.node.vx! += deltaX * scale;
      right.node.vy! += deltaY * scale;
    }
    this.advancePair(this.collideCursor);
  }

  private applyNode(): void {
    const runtime = this.orderedNodes[this.nodeCursor]!;
    const node = runtime.node;
    this.nodeCursor = (this.nodeCursor + 1) % this.orderedNodes.length;
    this.recover(runtime);
    node.vx! += -node.x! * this.centerStrength * this.alpha;
    node.vy! += -node.y! * this.centerStrength * this.alpha;
    if (node.fx === undefined || node.fx === null) node.x! += node.vx! *= diagramForceVelocityRetention;
    else {
      node.x = node.fx;
      node.vx = 0;
    }
    if (node.fy === undefined || node.fy === null) node.y! += node.vy! *= diagramForceVelocityRetention;
    else {
      node.y = node.fy;
      node.vy = 0;
    }
  }

  private pairBudget(): number {
    return Math.min((this.orderedNodes.length * (this.orderedNodes.length - 1)) / 2, diagramForceBudget.maxPairsPerTick);
  }

  private hasWork(work: DiagramForceWorkCursor): boolean {
    return work.remaining > 0 && this.now() < work.deadline;
  }

  private createLookup(value: string): DiagramForceLookupCursor<NodeType> {
    return { comparisonCursor: 0, done: false, high: this.orderedNodes.length - 1, low: 0, middle: -1, value };
  }

  private stepLookup(lookup: DiagramForceLookupCursor<NodeType>, work: DiagramForceWorkCursor): boolean {
    while (this.hasWork(work) && !lookup.done) {
      if (lookup.low > lookup.high) {
        lookup.done = true;
        return true;
      }
      if (lookup.middle < 0) lookup.middle = Math.floor((lookup.low + lookup.high) / 2);
      const candidate = this.orderedNodes[lookup.middle]!;
      const comparison = stepForceTextComparison(lookup.value, candidate.identity.value, lookup, work);
      if (comparison === undefined) continue;
      if (comparison === 0) {
        lookup.done = true;
        lookup.result = candidate;
        return true;
      }
      if (comparison < 0) lookup.high = lookup.middle - 1;
      else lookup.low = lookup.middle + 1;
      lookup.comparisonCursor = 0;
      lookup.middle = -1;
    }
    return lookup.done;
  }

  private stepNodeInitialization(work: DiagramForceWorkCursor): void {
    if (!this.pendingNode) {
      if (this.initializationCursor >= this.source.nodeCount) return;
      if (!takeForceFuel(work)) return;
      const sourceIndex = this.initializationCursor++;
      const node = this.source.node(sourceIndex);
      const value = node.id;
      this.pendingNode = { cursor: 0, hashA: (2166136261 ^ value.length) >>> 0, hashB: (2246822507 ^ value.length) >>> 0, hashC: (3266489909 ^ value.length) >>> 0, hashD: (668265263 ^ value.length) >>> 0, node, sourceIndex, value };
    }
    const pending = this.pendingNode;
    if (pending.cursor < pending.value.length) {
      if (!takeForceFuel(work)) return;
      const code = pending.value.charCodeAt(pending.cursor++);
      pending.hashA = Math.imul(pending.hashA ^ code, 16777619) >>> 0;
      pending.hashB = Math.imul(pending.hashB ^ code, 3266489917) >>> 0;
      pending.hashC = Math.imul(pending.hashC ^ code, 668265263) >>> 0;
      pending.hashD = Math.imul(pending.hashD ^ code, 374761393) >>> 0;
      if (pending.cursor < pending.value.length) return;
    }
    const identity = { hashA: pending.hashA, hashB: pending.hashB, hashC: pending.hashC, hashD: pending.hashD, index: -1, length: pending.value.length, value: pending.value };
    const [fallbackX, fallbackY] = fallbackForcePosition(identity);
    const runtime = { degree: 0, fallbackX, fallbackY, identity, node: pending.node, sourceIndex: pending.sourceIndex };
    this.recover(runtime);
    this.orderedNodes.push(runtime);
    if (this.ownsForceNodes) this.forceNodes.push(pending.node);
    this.pendingNode = undefined;
  }

  private stepInitialization(work: DiagramForceWorkCursor): boolean {
    while (this.hasWork(work)) {
      if (this.initializationPhase === "nodes") {
        if (this.initializationCursor < this.source.nodeCount || this.pendingNode) this.stepNodeInitialization(work);
        else {
          this.nodeSort = createForceMergeCursor(this.orderedNodes);
          this.initializationPhase = "sort-nodes";
        }
      } else if (this.initializationPhase === "sort-nodes") {
        if (
          stepForceMerge(
            this.nodeSort!,
            work,
            (runtime) => runtime.identity.value,
            () => 0,
            true,
          )
        ) {
          this.orderedNodes = this.nodeSort!.source;
          this.initializationCursor = 0;
          this.initializationPhase = "index-nodes";
        }
      } else if (this.initializationPhase === "index-nodes") {
        if (this.initializationCursor < this.orderedNodes.length) {
          if (!takeForceFuel(work)) break;
          this.orderedNodes[this.initializationCursor]!.identity.index = this.initializationCursor++;
        } else {
          this.initializationCursor = 0;
          this.initializationPhase = "links";
        }
      } else if (this.initializationPhase === "links") {
        if (!this.pendingLink && this.initializationCursor < this.source.linkCount) {
          if (!takeForceFuel(work)) break;
          const link = this.source.link(this.initializationCursor++);
          this.pendingLink = { ...link, sourceLookup: this.createLookup(link.source) };
        }
        const pending = this.pendingLink;
        if (pending) {
          if (!pending.sourceLookup.done && !this.stepLookup(pending.sourceLookup, work)) break;
          pending.sourceRuntime = pending.sourceLookup.result;
          if (!pending.sourceRuntime) {
            this.pendingLink = undefined;
            continue;
          }
          pending.targetLookup ??= this.createLookup(pending.target);
          if (!pending.targetLookup.done && !this.stepLookup(pending.targetLookup, work)) break;
          if (pending.targetLookup.result) {
            this.resolvedLinks.push({ bias: 0, id: pending.id, source: pending.sourceRuntime, strength: 0, target: pending.targetLookup.result });
            this.source.linkResolved?.(pending, pending.sourceRuntime.sourceIndex, pending.targetLookup.result.sourceIndex);
          }
          this.pendingLink = undefined;
        } else if (this.initializationCursor >= this.source.linkCount) {
          this.linkSort = createForceMergeCursor(this.resolvedLinks);
          this.initializationPhase = "sort-links";
        }
      } else if (this.initializationPhase === "sort-links") {
        if (
          stepForceMerge(
            this.linkSort!,
            work,
            (link) => link.id,
            (left, right) => left.source.identity.index - right.source.identity.index || left.target.identity.index - right.target.identity.index,
            false,
          )
        ) {
          this.resolvedLinks = this.linkSort!.source;
          this.initializationCursor = 0;
          this.initializationPhase = "degrees";
        }
      } else if (this.initializationPhase === "degrees") {
        if (this.initializationCursor < this.resolvedLinks.length) {
          if (!takeForceFuel(work)) break;
          const link = this.resolvedLinks[this.initializationCursor++]!;
          link.source.degree += 1;
          link.target.degree += 1;
        } else {
          this.initializationCursor = 0;
          this.initializationPhase = "resolve-links";
        }
      } else if (this.initializationPhase === "resolve-links") {
        if (this.initializationCursor < this.resolvedLinks.length) {
          if (!takeForceFuel(work)) break;
          const link = this.resolvedLinks[this.initializationCursor++]!;
          const sourceDegree = Math.max(1, link.source.degree);
          const targetDegree = Math.max(1, link.target.degree);
          link.strength = 1 / Math.min(sourceDegree, targetDegree);
          link.bias = sourceDegree / (sourceDegree + targetDegree);
        } else this.initializationPhase = "ready";
      } else return true;
    }
    return this.initializationPhase === "ready";
  }

  private stepDrag(work: DiagramForceWorkCursor): boolean {
    while (this.hasWork(work)) {
      if (this.releasePinned) {
        this.releasePinnedIterator ??= this.pinnedNodes.values();
        if (!takeForceFuel(work)) break;
        const next = this.releasePinnedIterator.next();
        if (!next.done) {
          next.value.node.fx = null;
          next.value.node.fy = null;
          this.pinnedNodes.delete(next.value);
          continue;
        }
        this.releasePinned = false;
        this.releasePinnedIterator = undefined;
      }
      const batch = this.pendingDrag;
      if (!batch) return true;
      if (!batch.current) {
        if (batch.cursor >= batch.length) {
          this.pendingDrag = undefined;
          return true;
        }
        if (!takeForceFuel(work)) break;
        batch.current = batch.selected ? batch.nodes[batch.cursor] : batch.node;
        batch.cursor += 1;
        if (!batch.current) continue;
        batch.lookup = this.createLookup(batch.current.id);
      }
      if (!this.stepLookup(batch.lookup!, work)) break;
      const runtime = batch.lookup!.result;
      if (runtime) {
        runtime.node.x = batch.current.position.x;
        runtime.node.y = batch.current.position.y;
        runtime.node.fx = batch.current.position.x;
        runtime.node.fy = batch.current.position.y;
        this.pinnedNodes.add(runtime);
      }
      batch.current = undefined;
      batch.lookup = undefined;
    }
    return !this.releasePinned && !this.pendingDrag;
  }

  private stepTick(work: DiagramForceWorkCursor): boolean {
    while (this.hasWork(work)) {
      if (this.tickPhase === "alpha") {
        if (!takeForceFuel(work)) break;
        this.alpha += (this.targetAlpha - this.alpha) * diagramForceAlphaDecay;
        this.tickPhase = "charge";
        this.tickPhaseProgress = 0;
      } else if (this.tickPhase === "charge") {
        const budget = this.chargeStrength === 0 ? 0 : this.pairBudget();
        if (this.tickPhaseProgress < budget) {
          if (!takeForceFuel(work)) break;
          this.applyChargePair();
          this.tickPhaseProgress += 1;
        } else {
          this.tickPhase = "links";
          this.tickPhaseProgress = 0;
        }
      } else if (this.tickPhase === "links") {
        const budget = Math.min(this.resolvedLinks.length, diagramForceBudget.maxLinksPerTick);
        if (this.tickPhaseProgress < budget) {
          if (!takeForceFuel(work)) break;
          this.applyLink();
          this.tickPhaseProgress += 1;
        } else {
          this.tickPhase = "collision";
          this.tickPhaseProgress = 0;
        }
      } else if (this.tickPhase === "collision") {
        const budget = this.collideRadius === 0 ? 0 : this.pairBudget();
        if (this.tickPhaseProgress < budget) {
          if (!takeForceFuel(work)) break;
          this.applyCollisionPair();
          this.tickPhaseProgress += 1;
        } else {
          this.tickPhase = "nodes";
          this.tickPhaseProgress = 0;
        }
      } else {
        const budget = Math.min(this.orderedNodes.length, diagramForceBudget.maxNodesPerTick);
        if (this.tickPhaseProgress < budget) {
          if (!takeForceFuel(work)) break;
          this.applyNode();
          this.tickPhaseProgress += 1;
        } else {
          this.tickPhase = "alpha";
          this.tickPhaseProgress = 0;
          return true;
        }
      }
    }
    return false;
  }

  private now(): number {
    return typeof globalThis.performance?.now === "function" ? globalThis.performance.now() : Date.now();
  }

  private scheduleFrame(): boolean {
    if (typeof window === "undefined") return false;
    if (this.frameHandle !== undefined) return true;
    const generation = this.generation;
    if (typeof window.requestAnimationFrame === "function") {
      this.frameKind = "animation";
      this.frameHandle = window.requestAnimationFrame((time) => this.advanceFrame(time, generation));
    } else {
      this.frameKind = "timeout";
      this.frameHandle = window.setTimeout(() => this.advanceFrame(this.now(), generation), 16);
    }
    return true;
  }

  private cancelFrame(): void {
    if (this.frameHandle !== undefined && typeof window !== "undefined") {
      if (this.frameKind === "animation" && typeof window.cancelAnimationFrame === "function") window.cancelAnimationFrame(this.frameHandle);
      else window.clearTimeout(this.frameHandle);
    }
    this.frameHandle = undefined;
    this.frameKind = undefined;
  }

  private complete(): boolean {
    return this.initializationPhase === "ready" && !this.releasePinned && !this.pendingDrag && this.tickPhase === "alpha" && this.alpha < diagramForceAlphaMin && this.targetAlpha === 0;
  }

  private notify(work: DiagramForceWorkCursor, time: number): boolean {
    if (!this.notificationPending || !this.hasWork(work)) return false;
    const listener = this.listener;
    const finished =
      !listener ||
      listener({
        complete: this.notificationComplete,
        deadline: work.deadline,
        generation: this.notificationGeneration,
        hasTime: () => this.hasWork(work),
        take: () => this.hasWork(work) && takeForceFuel(work),
      }) !== false;
    if (!finished) return false;
    this.notificationPending = false;
    this.lastNotification = time;
    return true;
  }

  private advanceFrame(time: number, generation: number): void {
    if (!this.running || generation !== this.generation || (this.initializationPhase !== "ready" && this.initializationGeneration !== generation)) return;
    this.frameHandle = undefined;
    this.frameKind = undefined;
    const deadline = this.now() + diagramForceBudget.maxFrameMs - diagramForceBudget.publicationReserveMs;
    const work = { deadline, remaining: diagramForceBudget.maxUnitsPerFrame };
    if (!this.stepInitialization(work) || !this.stepDrag(work)) {
      if (!this.scheduleFrame()) this.running = false;
      return;
    }
    const notified = this.notify(work, time);
    if (this.notificationPending) {
      if (!this.scheduleFrame()) this.running = false;
      return;
    }
    let ticks = 0;
    while (ticks < diagramForceBudget.maxTicksPerFrame && !this.complete() && this.hasWork(work)) {
      if (this.stepTick(work)) ticks += 1;
    }
    const complete = this.complete();
    if (!notified && this.listener && (complete || time - this.lastNotification >= this.updateIntervalMs)) {
      this.notificationComplete = complete;
      this.notificationGeneration = this.generation;
      this.notificationPending = true;
      this.notify(work, time);
    }
    if (complete && !this.notificationPending) this.running = false;
    else if (!this.scheduleFrame()) this.running = false;
  }
}

const diagramForcePort: DiagramForcePort = {
  create(nodes, links, config) {
    const source: DiagramForceSource<(typeof nodes)[number]> = {
      link: (index) => {
        const link = links[index]!;
        return { id: link.id, index, source: typeof link.source === "string" ? link.source : link.source.id, target: typeof link.target === "string" ? link.target : link.target.id };
      },
      linkCount: links.length,
      node: (index) => nodes[index]!,
      nodeCount: nodes.length,
    };
    return new OwnedDiagramForceSimulation(nodes, source, false, config).stop();
  },
};

/** 🧲️ Creates the owned force-simulation handle used by the Diagram interaction loop. */
export function createDiagramForceSimulation<NodeType extends DiagramForceNode, LinkType extends DiagramForceLink<NodeType>>(nodes: NodeType[], links: LinkType[], config: DiagramForceConfig): DiagramForceSimulation<NodeType> {
  return diagramForcePort.create(nodes, links, config);
}
