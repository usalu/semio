/** 🕸️ Surface of `graphlib` used by the independent graph oracles.
 *
 * `graphlib` ships without types and `@types/graphlib` is not part of the workspace, so the directed-graph
 * construction and the algorithms the oracles call are declared here. Delete this file the moment the
 * package gains types.
 *
 * @see https://github.com/dagrejs/graphlib/wiki/API-Reference
 */
declare module "graphlib" {
  namespace graphlib {
    interface GraphOptions {
      readonly directed?: boolean;
      readonly multigraph?: boolean;
      readonly compound?: boolean;
    }

    interface Edge {
      readonly v: string;
      readonly w: string;
      readonly name?: string;
    }

    class Graph {
      constructor(options?: GraphOptions);
      isDirected(): boolean;
      isMultigraph(): boolean;
      isCompound(): boolean;
      setGraph(label: unknown): this;
      graph(): unknown;
      nodeCount(): number;
      nodes(): string[];
      sources(): string[];
      sinks(): string[];
      setNode(name: string, label?: unknown): this;
      setNodes(names: readonly string[], label?: unknown): this;
      node(name: string): unknown;
      hasNode(name: string): boolean;
      removeNode(name: string): this;
      setParent(name: string, parent?: string): this;
      parent(name: string): string | undefined;
      children(name?: string): string[];
      predecessors(name: string): string[] | undefined;
      successors(name: string): string[] | undefined;
      neighbors(name: string): string[] | undefined;
      isLeaf(name: string): boolean;
      edgeCount(): number;
      edges(): Edge[];
      setEdge(from: string, to: string, label?: unknown, name?: string): this;
      setEdge(edge: Edge, label?: unknown): this;
      edge(from: string, to: string, name?: string): unknown;
      hasEdge(from: string, to: string, name?: string): boolean;
      removeEdge(from: string, to: string, name?: string): this;
      inEdges(name: string, from?: string): Edge[] | undefined;
      outEdges(name: string, to?: string): Edge[] | undefined;
      nodeEdges(name: string, other?: string): Edge[] | undefined;
    }

    const alg: {
      components(graph: Graph): string[][];
      findCycles(graph: Graph): string[][];
      isAcyclic(graph: Graph): boolean;
      postorder(graph: Graph, roots: string | readonly string[]): string[];
      preorder(graph: Graph, roots: string | readonly string[]): string[];
      tarjan(graph: Graph): string[][];
      topsort(graph: Graph): string[];
    };
  }

  export = graphlib;
}
