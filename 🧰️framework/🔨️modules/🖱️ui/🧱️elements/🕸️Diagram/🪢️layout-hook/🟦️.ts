import type { Edge, Node } from "@xyflow/react";
import { type DiagramLayoutPublicationResult, type DiagramLayoutOptions, diagramLayoutCredits, DIAGRAM_LAYOUT_CODEC_KIND } from "../📐️layout/📜️wire/🟦️.ts";
import { reactHostPort, interactiveJobPort } from "../../🔌️Ports/🟦️.tsx";
import { createDiagramLayoutPublication } from "../📐️layout/📢️publication/🟦️.ts";

let diagramLayoutGeneration = 0;

function nextDiagramLayoutGeneration(): number {
  diagramLayoutGeneration = diagramLayoutGeneration >= Number.MAX_SAFE_INTEGER ? 1 : diagramLayoutGeneration + 1;
  return diagramLayoutGeneration;
}

type DiagramLayoutHookResult = {
  readonly edges: Edge[];
  readonly layoutRejection?: "bytes" | "items";
  readonly layoutStatus: "complete" | "pending" | "rejected" | "source";
  readonly nodes: Node[];
};

type DiagramLayoutPublishedState = {
  readonly authority: DiagramLayoutPublicationResult;
  readonly edges: Edge[];
  readonly generation: number;
  readonly nodes: Node[];
  readonly sourceEdges: Edge[];
  readonly sourceNodes: Node[];
};

type DiagramLayoutOwnershipState = {
  readonly activeGeneration: number;
  readonly candidate?: DiagramLayoutPublishedState;
  readonly committed?: DiagramLayoutPublishedState;
};

function retireDiagramLayoutResult(result: DiagramLayoutPublicationResult): void {
  setTimeout(function closeResultStep() {
    const closed = result.closeStep();
    if (!closed) setTimeout(closeResultStep, 0);
  }, 0);
}

/**
 * 📡️ Submits directed layout only through the process-wide interactive worker port.
 **/
export function useDiagramLayout(initialNodes: Node[], initialEdges: Edge[], layoutOptions?: DiagramLayoutOptions): DiagramLayoutHookResult {
  const portSnapshot = reactHostPort.useSyncExternalStore(interactiveJobPort.subscribe, interactiveJobPort.getSnapshot, interactiveJobPort.getSnapshot);
  const credits = diagramLayoutCredits(initialNodes.length, initialEdges.length);
  const [ownership, setOwnership] = reactHostPort.useState<DiagramLayoutOwnershipState>({ activeGeneration: 0 });
  const activeGenerationRef = reactHostPort.useRef(0);
  const ownedResultsRef = reactHostPort.useRef(new Set<DiagramLayoutPublicationResult>());
  const selected = ownership.candidate ?? ownership.committed;
  const displayed = selected?.sourceNodes === initialNodes && selected.sourceEdges === initialEdges ? selected : undefined;
  reactHostPort.useEffect(() => {
    const generation = nextDiagramLayoutGeneration();
    activeGenerationRef.current = generation;
    let abandonsAuthority = false;
    for (const authority of ownedResultsRef.current) {
      if (authority !== displayed?.authority) {
        abandonsAuthority = true;
        break;
      }
    }
    if (abandonsAuthority) setOwnership({ activeGeneration: generation, committed: displayed });
    if (portSnapshot.status !== "ready" || !credits.admitted) return;
    const publication = createDiagramLayoutPublication(initialNodes, initialEdges, layoutOptions ?? {}, generation);
    let live = true;
    let lease = interactiveJobPort.submit(
      {
        generation,
        inputBytes: credits.inputBytes,
        inputItems: credits.inputItems,
        inputPageItems: 64,
        kind: DIAGRAM_LAYOUT_CODEC_KIND,
        operation: generation,
        outputBytes: credits.outputBytes,
        outputItems: credits.outputItems,
        outputPageItems: 128,
        pageBytes: 16 * 1024,
        payload: publication.descriptor,
      },
      {
        readInputPage: (cursor, maxItems) => publication.readInputPage(cursor, maxItems),
        onOutputPage: (page) => {
          if (!live || !publication.acceptOutputPage(page)) lease?.cancel();
        },
        onTerminal: (terminal) => {
          const validIdentity = terminal.generation === generation && terminal.operation === generation;
          const result = publication.acceptTerminal(
            validIdentity
              ? terminal.status === "fault"
                ? { generation, kind: "terminal", reason: terminal.detail ?? "Diagram layout fault", status: "fault" }
                : { generation, kind: "terminal", status: terminal.status }
              : { generation: -1, kind: "terminal", reason: "Diagram layout terminal identity mismatch", status: "fault" },
          );
          if (!live || !result) {
            if (result) retireDiagramLayoutResult(result);
            return;
          }
          const next = { authority: result, edges: result.edges, generation, nodes: result.nodes, sourceEdges: initialEdges, sourceNodes: initialNodes };
          ownedResultsRef.current.add(result);
          setOwnership((retained) => (activeGenerationRef.current === generation ? { ...retained, candidate: next } : retained));
        },
        closeStep: () => publication.closeStep(),
        terminalIsEmpty: () => publication.terminalIsEmpty(),
      },
    );
    if (!lease) {
      setTimeout(function closeRejectedPublication() {
        if (!publication.closeStep()) setTimeout(closeRejectedPublication, 0);
      }, 0);
    }
    return () => {
      live = false;
      lease?.cancel();
      lease = undefined;
    };
  }, [
    initialNodes,
    initialEdges,
    layoutOptions?.direction,
    layoutOptions?.nodeHeight,
    layoutOptions?.nodeSep,
    layoutOptions?.nodeWidth,
    layoutOptions?.rankSep,
    portSnapshot,
    credits.admitted,
    credits.admitted ? credits.inputBytes : credits.reason,
  ]);
  reactHostPort.useLayoutEffect(() => {
    const candidate = ownership.candidate;
    if (!candidate || displayed?.authority !== candidate.authority) return;
    setOwnership((retained) => (retained.candidate?.authority === candidate.authority ? { activeGeneration: retained.activeGeneration, committed: candidate } : retained));
  }, [displayed?.authority, ownership.candidate]);
  reactHostPort.useEffect(() => {
    const committed = displayed?.authority;
    for (const authority of ownedResultsRef.current) {
      if (authority === committed) continue;
      ownedResultsRef.current.delete(authority);
      retireDiagramLayoutResult(authority);
    }
  }, [displayed?.authority, ownership.activeGeneration]);
  reactHostPort.useEffect(
    () => () => {
      for (const authority of ownedResultsRef.current) retireDiagramLayoutResult(authority);
      ownedResultsRef.current.clear();
    },
    [],
  );
  if (displayed) return { edges: displayed.edges, layoutStatus: "complete", nodes: displayed.nodes };
  if (!credits.admitted) return { edges: initialEdges, layoutRejection: credits.reason, layoutStatus: "rejected", nodes: initialNodes };
  return { edges: initialEdges, layoutStatus: portSnapshot.status === "ready" ? "pending" : "source", nodes: initialNodes };
}
