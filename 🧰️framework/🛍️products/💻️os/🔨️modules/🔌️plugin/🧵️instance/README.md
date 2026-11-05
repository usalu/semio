# Mounted Artifact Instance

VcsArtifactApp composes the stores, document ingress, child ownership, interaction, retained operations and rendering components of one mounted artifact. Its PluginApp implementation delegates each turn to the component that owns the relevant authority. The semantic UI contract remains independent of the renderer.

## Reads and Rendering

snapshot returns an owned document value; snapshot_ref borrows the committed read root. interaction_state combines persisted local interaction and ephemeral hover for the host presence heartbeat. The host combines that answer with the app declaration hover and selection specifications.

Render, measure, engagement and menu passes materialize interaction and peer-presence reads before borrowing mutable components. Framework bodies are served before app body-key matching. Every emitted interaction-domain tree uses the same read projection for that render. Context menus pass through the common idempotent organizer, preserving already organized groups.

pending_effects receives the same retained operation owner and live run that rendering observes. This lets it recognize an existing pending chain and finalize its completed run before another gesture starts work. Its document read remains the committed snapshot, while provisional run overlays belong to their operation owner.

## Document Loading and Retirement

Envelope ingress admits exact page and byte extents before constructing an input owner. A sealed decoder reports its own poll until it reaches Ready; the replacement job does not exist before that handoff. Decoding and initialization advance through maintenance rather than running inline.

Read the typed replacement refusal before acknowledging the operation: acknowledgement retires the original evidence. The occupied typed-operation count includes segmented output ownership; zero is the terminal witness that every admitted slot reached release. next_maintenance_stage identifies the unit whose budget the next call spends.

## Composition Ownership

An app whose snapshot carries child or link references explicitly synchronizes its composition graph after changing them. This keeps the ArtifactRefs requirement local to composed artifacts.

register_child adopts an already constructed member and seeds its ownership edge. Group-created children may have no earlier caller-held open_child reference, so adoption must establish the same graph membership. Group dispatch checks ownership against that graph before touching a child.

## Tool Runs

An action with no explicit run identity addresses the mounted instance live run at the dispatch generation. An explicitly supplied identity is validated unchanged. Each component advances only the retained work and finite grant admitted by its caller.
