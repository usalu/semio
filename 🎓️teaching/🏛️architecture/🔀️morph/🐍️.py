"""🔀 Morph facade: re-exports of the declarative transition API, the shared tokens and the full-morph helpers.

Domain-neutral; Manim only. Reference: https://docs.manim.community/en/stable/reference/manim.animation.transform.Transform.html
"""

from __future__ import annotations

import importlib

_full = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️")
_token = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🪙️token.🐍️")
_stage = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎭️stage.🐍️")
_spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")

INK, MUTED, DATA, GOOD, WARN, DARK, KINDS = _token.INK, _token.MUTED, _token.DATA, _token.GOOD, _token.WARN, _token.DARK, _token.KINDS
token, travel, tether = _token.token, _token.travel, _token.tether

Morph, PartMorph, Swarm, Flight, Dim, Trip, Flock, Move, Dissolve = _full.Morph, _full.PartMorph, _full.Swarm, _full.Flight, _full.Dim, _full.Trip, _full.Flock, _full.Move, _full.Dissolve
silhouette, proxy, capsule_pairs = _full.silhouette, _full.proxy, _full.capsule_pairs
morph, scatter, gather, split, unfold, retext, flight, execute = _full.morph, _full.scatter, _full.gather, _full.split, _full.unfold, _full.retext, _full.flight, _full.execute
CORRESPONDENCES, RECIPES = _full.CORRESPONDENCES, _full.RECIPES

stage, Stage = _stage.stage, _stage.Stage
load, parse, dump, validate = _spec.load, _spec.parse, _spec.dump, _spec.validate
Problem, SpecError = _spec.Problem, _spec.SpecError
