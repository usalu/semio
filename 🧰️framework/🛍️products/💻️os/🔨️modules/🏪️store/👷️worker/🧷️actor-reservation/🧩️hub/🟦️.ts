import type { ArtifactState } from "../../🧾️document-state/🟦️.ts";
import type { ReservationSession } from "../🗃️state/🟦️.ts";

/** 🧩️ The collaborators of one reservation that reach each other only through the hub. Each collaborator declares its own member by augmenting this interface, so no collaborator imports another and the hub that wires them is the only place that names them all. */
export interface ReservationCollaborators {}

/** 🪢️ What a collaborator sees of its reservation: the other collaborators, the shell that owns them and the single private session they share. */
export type ReservationHub = ReservationCollaborators & Readonly<{ shell: NonNullable<ArtifactState["browserActorReservation"]>; session: ReservationSession }>;
