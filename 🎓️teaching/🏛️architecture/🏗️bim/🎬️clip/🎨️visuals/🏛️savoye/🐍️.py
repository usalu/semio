"""🏛 Villa Savoye drawing widgets: axonometric, plan, section, elevation and detail drawings from one model-driven projection."""

from __future__ import annotations

import importlib

_PREFIX = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye."
_Axon = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️").Axon
_Plan = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📋️plan.🐍️").Plan
_Section = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.✂️section.🐍️").Section
_Detail = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🔩️detail.🐍️").Detail


class SavoyeOrtho(_Plan, _Section, _Detail):
    """📐️ Scene mixin with every orthographic Villa Savoye drawing: plans, sections, elevations and the piloti, bay, roof and slab details."""


class Savoye(_Axon, SavoyeOrtho):
    """🏛️ Scene mixin with every Villa Savoye drawing widget."""
