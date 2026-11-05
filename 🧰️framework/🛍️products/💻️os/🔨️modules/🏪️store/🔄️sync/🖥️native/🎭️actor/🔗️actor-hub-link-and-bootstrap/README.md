
## ♻️ Hub lag forces a canonical pair refresh: clear the live projection tokens, keep unacked work queued,

drop the socket and ask the host for the pair ([`ArtifactEvent::RebootstrapRequired`]); the actor dials again only
once the host re-seeded it ([`Self::reseed_hub_document`]) — never into a Welcome it would have to refuse, and never
after an outage backoff (the rebuild's own close is not a shortage).
