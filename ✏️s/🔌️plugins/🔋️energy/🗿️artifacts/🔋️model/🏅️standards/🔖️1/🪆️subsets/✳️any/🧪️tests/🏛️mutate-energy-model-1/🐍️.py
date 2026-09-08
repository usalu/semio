"""🐍️ `s.energy.model`'s second, independent implementation of its own mutation vocabulary.

No third-party library reads or writes `.dsl.semio` — the recorded survey named and DECLINED
EnergyPlus and OpenStudio, and the `energyplus` weather reader already registered under
`✏️s/🔌️plugins/🗄️stdio`'s `🌦️epw` subset reads a different format for a different purpose, so it is
deliberately not reused here. The reference is therefore a second IMPLEMENTATION, written from this
subset's own committed `../../🧬️schema/📸️snapshot/🔣️.json`, each kind's own
`../../🧬️schema/🧬️mutations/<dir>/🧬️schema/🔣️.json`, and
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/`'s `📓️taxonomy.md`
verb table and `📓️derivation-rules.md` shape rules. It imports nothing from the Rust it judges and
transliterates none of it.

The honest boundary this file used to carry is gone with `replace-model`: a whole-document swap is
not a mutation at all (rule 6), so there is no longer a kind whose only vector is a no-op. Every kind
below is exercised by one vector that moves the document and one that is refused.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome
# endregion 🔖️Imports


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "rename-model-renames-the-model": "asset://🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/✅️renames-the-model",
    "rename-model-refuses-a-blank-name": "asset://🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/⛔️refuses-a-blank-name",
    "change-model-version-bumps-the-version": "asset://🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/✅️bumps-the-version",
    "change-model-version-refuses-a-blank-version": "asset://🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/⛔️refuses-a-blank-version",
    "update-site-relocates-to-denver": "asset://🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/✅️relocates-to-denver",
    "update-site-refuses-a-bad-latitude": "asset://🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/⛔️refuses-a-bad-latitude",
    "update-ground-temperature-sets-denver-ground": "asset://🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/✅️sets-denver-ground",
    "update-ground-temperature-refuses-a-short-year": "asset://🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/⛔️refuses-a-short-year",
    "update-run-period-shortens-to-january": "asset://🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/✅️shortens-to-january",
    "update-run-period-refuses-month-13": "asset://🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/⛔️refuses-month-13",
    "replace-airflow-network-attaches-a-network": "asset://🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/✅️attaches-a-network",
    "replace-airflow-network-refuses-unpaired-nodes": "asset://🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/⛔️refuses-unpaired-nodes",
    "add-output-variable-adds-zone-air-temp": "asset://🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/✅️adds-zone-air-temp",
    "add-output-variable-refuses-a-duplicate": "asset://🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/⛔️refuses-a-duplicate",
    "remove-output-variable-drops-zone-air-temp": "asset://🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/✅️drops-zone-air-temp",
    "remove-output-variable-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/⛔️refuses-an-absent-one",
    "bind-weather-file-binds-hannover-epw": "asset://🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/✅️binds-hannover-epw",
    "bind-weather-file-refuses-a-bad-uri": "asset://🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/⛔️refuses-a-bad-uri",
    "unbind-weather-file-unbinds-the-weather": "asset://🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/✅️unbinds-the-weather",
    "unbind-weather-file-refuses-when-unbound": "asset://🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/⛔️refuses-when-unbound",
    "connect-referenced-model-connects-the-geometry": "asset://🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/✅️connects-the-geometry",
    "connect-referenced-model-refuses-a-bad-uri": "asset://🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/⛔️refuses-a-bad-uri",
    "disconnect-referenced-model-disconnects-the-geometry": "asset://🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/✅️disconnects-the-geometry",
    "disconnect-referenced-model-refuses-when-absent": "asset://🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/⛔️refuses-when-absent",
    "rename-zone-renames-zone-one": "asset://🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/✅️renames-zone-one",
    "rename-zone-refuses-a-missing-zone": "asset://🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/⛔️refuses-a-missing-zone",
    "change-zone-volume-resizes-zone-one": "asset://🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/✅️resizes-zone-one",
    "change-zone-volume-refuses-zero-volume": "asset://🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/⛔️refuses-zero-volume",
    "change-zone-multiplier-stacks-four-storeys": "asset://🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/✅️stacks-four-storeys",
    "change-zone-multiplier-refuses-zero-instances": "asset://🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/⛔️refuses-zero-instances",
    "change-zone-conditioned-frees-the-zone": "asset://🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/✅️frees-the-zone",
    "change-zone-conditioned-refuses-a-missing-zone": "asset://🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/⛔️refuses-a-missing-zone",
    "change-zone-floor-area-participation-excludes-the-zone": "asset://🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/✅️excludes-the-zone",
    "change-zone-floor-area-participation-refuses-a-missing-zone": "asset://🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/⛔️refuses-a-missing-zone",
    "create-zone-adds-a-second-zone": "asset://🧬️schema/🧬️mutations/🏘️create-zone/🧪️tests/✅️adds-a-second-zone",
    "create-zone-refuses-a-taken-id": "asset://🧬️schema/🧬️mutations/🏘️create-zone/🧪️tests/⛔️refuses-a-taken-id",
    "delete-zone-deletes-a-free-zone": "asset://🧬️schema/🧬️mutations/🏚️delete-zone/🧪️tests/✅️deletes-a-free-zone",
    "delete-zone-refuses-a-used-zone": "asset://🧬️schema/🧬️mutations/🏚️delete-zone/🧪️tests/⛔️refuses-a-used-zone",
    "create-space-adds-a-space": "asset://🧬️schema/🧬️mutations/🪑️create-space/🧪️tests/✅️adds-a-space",
    "create-space-refuses-a-missing-zone": "asset://🧬️schema/🧬️mutations/🪑️create-space/🧪️tests/⛔️refuses-a-missing-zone",
    "delete-space-deletes-a-free-space": "asset://🧬️schema/🧬️mutations/🧹️delete-space/🧪️tests/✅️deletes-a-free-space",
    "delete-space-refuses-a-missing-space": "asset://🧬️schema/🧬️mutations/🧹️delete-space/🧪️tests/⛔️refuses-a-missing-space",
    "rename-space-renames-a-space": "asset://🧬️schema/🧬️mutations/🔤️rename-space/🧪️tests/✅️renames-a-space",
    "rename-space-refuses-a-blank-name": "asset://🧬️schema/🧬️mutations/🔤️rename-space/🧪️tests/⛔️refuses-a-blank-name",
    "change-space-floor-area-resizes-a-space": "asset://🧬️schema/🧬️mutations/🧮️change-space-floor-area/🧪️tests/✅️resizes-a-space",
    "change-space-floor-area-refuses-negative-area": "asset://🧬️schema/🧬️mutations/🧮️change-space-floor-area/🧪️tests/⛔️refuses-negative-area",
    "change-space-zone-moves-a-space": "asset://🧬️schema/🧬️mutations/🚚️change-space-zone/🧪️tests/✅️moves-a-space",
    "change-space-zone-refuses-a-missing-zone": "asset://🧬️schema/🧬️mutations/🚚️change-space-zone/🧪️tests/⛔️refuses-a-missing-zone",
    "create-surface-adds-a-south-wall": "asset://🧬️schema/🧬️mutations/🟫️create-surface/🧪️tests/✅️adds-a-south-wall",
    "create-surface-refuses-two-vertices": "asset://🧬️schema/🧬️mutations/🟫️create-surface/🧪️tests/⛔️refuses-two-vertices",
    "delete-surface-cascades-a-window": "asset://🧬️schema/🧬️mutations/🪚️delete-surface/🧪️tests/✅️cascades-a-window",
    "delete-surface-refuses-a-partner": "asset://🧬️schema/🧬️mutations/🪚️delete-surface/🧪️tests/⛔️refuses-a-partner",
    "rename-surface-renames-a-wall": "asset://🧬️schema/🧬️mutations/🏳️rename-surface/🧪️tests/✅️renames-a-wall",
    "rename-surface-refuses-a-taken-name": "asset://🧬️schema/🧬️mutations/🏳️rename-surface/🧪️tests/⛔️refuses-a-taken-name",
    "change-surface-zone-moves-a-wall": "asset://🧬️schema/🧬️mutations/🗜️change-surface-zone/🧪️tests/✅️moves-a-wall",
    "change-surface-zone-refuses-a-missing-zone": "asset://🧬️schema/🧬️mutations/🗜️change-surface-zone/🧪️tests/⛔️refuses-a-missing-zone",
    "change-surface-class-turns-a-wall-to-roof": "asset://🧬️schema/🧬️mutations/🧩️change-surface-class/🧪️tests/✅️turns-a-wall-to-roof",
    "change-surface-class-refuses-a-missing-one": "asset://🧬️schema/🧬️mutations/🧩️change-surface-class/🧪️tests/⛔️refuses-a-missing-one",
    "replace-surface-vertices-narrows-a-wall": "asset://🧬️schema/🧬️mutations/🔺️replace-surface-vertices/🧪️tests/✅️narrows-a-wall",
    "replace-surface-vertices-refuses-a-line": "asset://🧬️schema/🧬️mutations/🔺️replace-surface-vertices/🧪️tests/⛔️refuses-a-line",
    "change-surface-construction-swaps-the-wall-stack": "asset://🧬️schema/🧬️mutations/🧰️change-surface-construction/🧪️tests/✅️swaps-the-wall-stack",
    "change-surface-construction-refuses-a-missing-one": "asset://🧬️schema/🧬️mutations/🧰️change-surface-construction/🧪️tests/⛔️refuses-a-missing-one",
    "change-surface-boundary-condition-grounds-a-floor": "asset://🧬️schema/🧬️mutations/🚧️change-surface-boundary-condition/🧪️tests/✅️grounds-a-floor",
    "change-surface-boundary-condition-refuses-half-a-union": "asset://🧬️schema/🧬️mutations/🚧️change-surface-boundary-condition/🧪️tests/⛔️refuses-half-a-union",
    "change-surface-sun-exposed-shades-a-wall": "asset://🧬️schema/🧬️mutations/🌅️change-surface-sun-exposed/🧪️tests/✅️shades-a-wall",
    "change-surface-sun-exposed-refuses-a-missing-one": "asset://🧬️schema/🧬️mutations/🌅️change-surface-sun-exposed/🧪️tests/⛔️refuses-a-missing-one",
    "change-surface-wind-exposed-shelters-a-wall": "asset://🧬️schema/🧬️mutations/🍃️change-surface-wind-exposed/🧪️tests/✅️shelters-a-wall",
    "change-surface-wind-exposed-refuses-a-missing-one": "asset://🧬️schema/🧬️mutations/🍃️change-surface-wind-exposed/🧪️tests/⛔️refuses-a-missing-one",
    "change-surface-multiplier-repeats-a-wall": "asset://🧬️schema/🧬️mutations/🔁️change-surface-multiplier/🧪️tests/✅️repeats-a-wall",
    "change-surface-multiplier-refuses-zero": "asset://🧬️schema/🧬️mutations/🔁️change-surface-multiplier/🧪️tests/⛔️refuses-zero",
    "create-fenestration-adds-a-south-window": "asset://🧬️schema/🧬️mutations/🪟️create-fenestration/🧪️tests/✅️adds-a-south-window",
    "create-fenestration-refuses-a-missing-host": "asset://🧬️schema/🧬️mutations/🪟️create-fenestration/🧪️tests/⛔️refuses-a-missing-host",
    "delete-fenestration-removes-a-window": "asset://🧬️schema/🧬️mutations/🚪️delete-fenestration/🧪️tests/✅️removes-a-window",
    "delete-fenestration-refuses-a-missing-one": "asset://🧬️schema/🧬️mutations/🚪️delete-fenestration/🧪️tests/⛔️refuses-a-missing-one",
    "rename-fenestration-renames-a-window": "asset://🧬️schema/🧬️mutations/🏁️rename-fenestration/🧪️tests/✅️renames-a-window",
    "rename-fenestration-refuses-a-blank-name": "asset://🧬️schema/🧬️mutations/🏁️rename-fenestration/🧪️tests/⛔️refuses-a-blank-name",
    "change-fenestration-surface-rehosts-a-window": "asset://🧬️schema/🧬️mutations/🧲️change-fenestration-surface/🧪️tests/✅️rehosts-a-window",
    "change-fenestration-surface-refuses-a-missing-host": "asset://🧬️schema/🧬️mutations/🧲️change-fenestration-surface/🧪️tests/⛔️refuses-a-missing-host",
    "change-fenestration-u-value-swaps-the-glazing": "asset://🧬️schema/🧬️mutations/🌐️change-fenestration-u-value/🧪️tests/✅️swaps-the-glazing",
    "change-fenestration-u-value-refuses-zero-u": "asset://🧬️schema/🧬️mutations/🌐️change-fenestration-u-value/🧪️tests/⛔️refuses-zero-u",
    "change-fenestration-shgc-dims-the-solar-gain": "asset://🧬️schema/🧬️mutations/🌇️change-fenestration-shgc/🧪️tests/✅️dims-the-solar-gain",
    "change-fenestration-shgc-refuses-shgc-above-one": "asset://🧬️schema/🧬️mutations/🌇️change-fenestration-shgc/🧪️tests/⛔️refuses-shgc-above-one",
    "change-fenestration-vlt-dims-the-daylight": "asset://🧬️schema/🧬️mutations/🌈️change-fenestration-vlt/🧪️tests/✅️dims-the-daylight",
    "change-fenestration-vlt-refuses-negative-vlt": "asset://🧬️schema/🧬️mutations/🌈️change-fenestration-vlt/🧪️tests/⛔️refuses-negative-vlt",
    "change-fenestration-area-doubles-the-glazing": "asset://🧬️schema/🧬️mutations/🟥️change-fenestration-area/🧪️tests/✅️doubles-the-glazing",
    "change-fenestration-area-refuses-zero-area": "asset://🧬️schema/🧬️mutations/🟥️change-fenestration-area/🧪️tests/⛔️refuses-zero-area",
    "change-fenestration-frame-conductance-adds-a-frame": "asset://🧬️schema/🧬️mutations/🖼️change-fenestration-frame-conductance/🧪️tests/✅️adds-a-frame",
    "change-fenestration-frame-conductance-refuses-negative-frame": "asset://🧬️schema/🧬️mutations/🖼️change-fenestration-frame-conductance/🧪️tests/⛔️refuses-negative-frame",
    "change-fenestration-divider-conductance-adds-dividers": "asset://🧬️schema/🧬️mutations/🧷️change-fenestration-divider-conductance/🧪️tests/✅️adds-dividers",
    "change-fenestration-divider-conductance-refuses-a-negative": "asset://🧬️schema/🧬️mutations/🧷️change-fenestration-divider-conductance/🧪️tests/⛔️refuses-a-negative",
    "create-shading-surface-adds-an-awning": "asset://🧬️schema/🧬️mutations/🌳️create-shading-surface/🧪️tests/✅️adds-an-awning",
    "create-shading-surface-refuses-a-line": "asset://🧬️schema/🧬️mutations/🌳️create-shading-surface/🧪️tests/⛔️refuses-a-line",
    "delete-shading-surface-removes-an-awning": "asset://🧬️schema/🧬️mutations/🪵️delete-shading-surface/🧪️tests/✅️removes-an-awning",
    "delete-shading-surface-refuses-a-missing-one": "asset://🧬️schema/🧬️mutations/🪵️delete-shading-surface/🧪️tests/⛔️refuses-a-missing-one",
    "rename-shading-surface-renames-an-awning": "asset://🧬️schema/🧬️mutations/🏕️rename-shading-surface/🧪️tests/✅️renames-an-awning",
    "rename-shading-surface-refuses-a-blank-name": "asset://🧬️schema/🧬️mutations/🏕️rename-shading-surface/🧪️tests/⛔️refuses-a-blank-name",
    "replace-shading-surface-vertices-deepens-an-awning": "asset://🧬️schema/🧬️mutations/🗺️replace-shading-surface-vertices/🧪️tests/✅️deepens-an-awning",
    "replace-shading-surface-vertices-refuses-a-line": "asset://🧬️schema/🧬️mutations/🗺️replace-shading-surface-vertices/🧪️tests/⛔️refuses-a-line",
    "change-shading-surface-transmittance-schedule-lets-light-in": "asset://🧬️schema/🧬️mutations/⛱️change-shading-surface-transmittance-schedule/🧪️tests/✅️lets-light-in",
    "change-shading-surface-transmittance-schedule-refuses-a-ghost": "asset://🧬️schema/🧬️mutations/⛱️change-shading-surface-transmittance-schedule/🧪️tests/⛔️refuses-a-ghost",
    "connect-surfaces-joins-two-walls": "asset://🧬️schema/🧬️mutations/🤝️connect-surfaces/🧪️tests/✅️joins-two-walls",
    "connect-surfaces-refuses-a-self-pair": "asset://🧬️schema/🧬️mutations/🤝️connect-surfaces/🧪️tests/⛔️refuses-a-self-pair",
    "disconnect-surfaces-parts-two-walls": "asset://🧬️schema/🧬️mutations/💔️disconnect-surfaces/🧪️tests/✅️parts-two-walls",
    "disconnect-surfaces-refuses-a-missing-pair": "asset://🧬️schema/🧬️mutations/💔️disconnect-surfaces/🧪️tests/⛔️refuses-a-missing-pair",
    "bind-fenestration-glazing-construction-glazes-a-window": "asset://🧬️schema/🧬️mutations/🧊️bind-fenestration-glazing-construction/🧪️tests/✅️glazes-a-window",
    "bind-fenestration-glazing-construction-refuses-no-stack": "asset://🧬️schema/🧬️mutations/🧊️bind-fenestration-glazing-construction/🧪️tests/⛔️refuses-no-stack",
    "clear-fenestration-glazing-construction-ungazes-a-window": "asset://🧬️schema/🧬️mutations/🫗️clear-fenestration-glazing-construction/🧪️tests/✅️ungazes-a-window",
    "clear-fenestration-glazing-construction-refuses-empty": "asset://🧬️schema/🧬️mutations/🫗️clear-fenestration-glazing-construction/🧪️tests/⛔️refuses-empty",
    "change-fenestration-height-raises-the-head": "asset://🧬️schema/🧬️mutations/⬆️change-fenestration-height/🧪️tests/✅️raises-the-head",
    "change-fenestration-height-refuses-zero-height": "asset://🧬️schema/🧬️mutations/⬆️change-fenestration-height/🧪️tests/⛔️refuses-zero-height",
    "change-fenestration-sill-height-raises-the-sill": "asset://🧬️schema/🧬️mutations/⬇️change-fenestration-sill-height/🧪️tests/✅️raises-the-sill",
    "change-fenestration-sill-height-refuses-negative-sill": "asset://🧬️schema/🧬️mutations/⬇️change-fenestration-sill-height/🧪️tests/⛔️refuses-negative-sill",
    "change-fenestration-overhang-depth-adds-case-610-shade": "asset://🧬️schema/🧬️mutations/🧢️change-fenestration-overhang-depth/🧪️tests/✅️adds-case-610-shade",
    "change-fenestration-overhang-depth-refuses-negative-depth": "asset://🧬️schema/🧬️mutations/🧢️change-fenestration-overhang-depth/🧪️tests/⛔️refuses-negative-depth",
    "change-fenestration-overhang-offset-lifts-the-overhang": "asset://🧬️schema/🧬️mutations/🎩️change-fenestration-overhang-offset/🧪️tests/✅️lifts-the-overhang",
    "change-fenestration-overhang-offset-refuses-negative-offset": "asset://🧬️schema/🧬️mutations/🎩️change-fenestration-overhang-offset/🧪️tests/⛔️refuses-negative-offset",
    "change-fenestration-fin-depth-adds-case-630-fins": "asset://🧬️schema/🧬️mutations/🐬️change-fenestration-fin-depth/🧪️tests/✅️adds-case-630-fins",
    "change-fenestration-fin-depth-refuses-negative-fin": "asset://🧬️schema/🧬️mutations/🐬️change-fenestration-fin-depth/🧪️tests/⛔️refuses-negative-fin",
    "change-fenestration-fin-offset-spreads-the-fins": "asset://🧬️schema/🧬️mutations/🐋️change-fenestration-fin-offset/🧪️tests/✅️spreads-the-fins",
    "change-fenestration-fin-offset-refuses-negative-offset": "asset://🧬️schema/🧬️mutations/🐋️change-fenestration-fin-offset/🧪️tests/⛔️refuses-negative-offset",
    "create-material-applies": "asset://🧬️schema/🧬️mutations/🧱️create-material/🧪️tests/✅️applies",
    "create-material-refuses": "asset://🧬️schema/🧬️mutations/🧱️create-material/🧪️tests/⛔️refuses",
    "delete-material-applies": "asset://🧬️schema/🧬️mutations/🪨️delete-material/🧪️tests/✅️applies",
    "delete-material-refuses": "asset://🧬️schema/🧬️mutations/🪨️delete-material/🧪️tests/⛔️refuses",
    "rename-material-applies": "asset://🧬️schema/🧬️mutations/🪧️rename-material/🧪️tests/✅️applies",
    "rename-material-refuses": "asset://🧬️schema/🧬️mutations/🪧️rename-material/🧪️tests/⛔️refuses",
    "change-material-thickness-applies": "asset://🧬️schema/🧬️mutations/📏️change-material-thickness/🧪️tests/✅️applies",
    "change-material-thickness-refuses": "asset://🧬️schema/🧬️mutations/📏️change-material-thickness/🧪️tests/⛔️refuses",
    "change-material-conductivity-applies": "asset://🧬️schema/🧬️mutations/🔥️change-material-conductivity/🧪️tests/✅️applies",
    "change-material-conductivity-refuses": "asset://🧬️schema/🧬️mutations/🔥️change-material-conductivity/🧪️tests/⛔️refuses",
    "change-material-density-applies": "asset://🧬️schema/🧬️mutations/⚖️change-material-density/🧪️tests/✅️applies",
    "change-material-density-refuses": "asset://🧬️schema/🧬️mutations/⚖️change-material-density/🧪️tests/⛔️refuses",
    "change-material-specific-heat-applies": "asset://🧬️schema/🧬️mutations/♨️change-material-specific-heat/🧪️tests/✅️applies",
    "change-material-specific-heat-refuses": "asset://🧬️schema/🧬️mutations/♨️change-material-specific-heat/🧪️tests/⛔️refuses",
    "change-material-thermal-absorptance-applies": "asset://🧬️schema/🧬️mutations/🔆️change-material-thermal-absorptance/🧪️tests/✅️applies",
    "change-material-thermal-absorptance-refuses": "asset://🧬️schema/🧬️mutations/🔆️change-material-thermal-absorptance/🧪️tests/⛔️refuses",
    "change-material-solar-absorptance-applies": "asset://🧬️schema/🧬️mutations/☀️change-material-solar-absorptance/🧪️tests/✅️applies",
    "change-material-solar-absorptance-refuses": "asset://🧬️schema/🧬️mutations/☀️change-material-solar-absorptance/🧪️tests/⛔️refuses",
    "change-material-visible-absorptance-applies": "asset://🧬️schema/🧬️mutations/👁️change-material-visible-absorptance/🧪️tests/✅️applies",
    "change-material-visible-absorptance-refuses": "asset://🧬️schema/🧬️mutations/👁️change-material-visible-absorptance/🧪️tests/⛔️refuses",
    "create-construction-applies": "asset://🧬️schema/🧬️mutations/🏗️create-construction/🧪️tests/✅️applies",
    "create-construction-refuses": "asset://🧬️schema/🧬️mutations/🏗️create-construction/🧪️tests/⛔️refuses",
    "delete-construction-applies": "asset://🧬️schema/🧬️mutations/🧨️delete-construction/🧪️tests/✅️applies",
    "delete-construction-refuses": "asset://🧬️schema/🧬️mutations/🧨️delete-construction/🧪️tests/⛔️refuses",
    "rename-construction-applies": "asset://🧬️schema/🧬️mutations/🪪️rename-construction/🧪️tests/✅️applies",
    "rename-construction-refuses": "asset://🧬️schema/🧬️mutations/🪪️rename-construction/🧪️tests/⛔️refuses",
    "add-construction-layer-applies": "asset://🧬️schema/🧬️mutations/➕️add-construction-layer/🧪️tests/✅️applies",
    "add-construction-layer-refuses": "asset://🧬️schema/🧬️mutations/➕️add-construction-layer/🧪️tests/⛔️refuses",
    "remove-construction-layer-applies": "asset://🧬️schema/🧬️mutations/➖️remove-construction-layer/🧪️tests/✅️applies",
    "remove-construction-layer-refuses": "asset://🧬️schema/🧬️mutations/➖️remove-construction-layer/🧪️tests/⛔️refuses",
    "reorder-construction-layers-applies": "asset://🧬️schema/🧬️mutations/🔀️reorder-construction-layers/🧪️tests/✅️applies",
    "reorder-construction-layers-refuses": "asset://🧬️schema/🧬️mutations/🔀️reorder-construction-layers/🧪️tests/⛔️refuses",
    "create-people-gain-applies": "asset://🧬️schema/🧬️mutations/👤️create-people-gain/🧪️tests/✅️applies",
    "create-people-gain-refuses": "asset://🧬️schema/🧬️mutations/👤️create-people-gain/🧪️tests/⛔️refuses",
    "delete-people-gain-applies": "asset://🧬️schema/🧬️mutations/🚷️delete-people-gain/🧪️tests/✅️applies",
    "delete-people-gain-refuses": "asset://🧬️schema/🧬️mutations/🚷️delete-people-gain/🧪️tests/⛔️refuses",
    "change-people-gain-zone-applies": "asset://🧬️schema/🧬️mutations/🚶️change-people-gain-zone/🧪️tests/✅️applies",
    "change-people-gain-zone-refuses": "asset://🧬️schema/🧬️mutations/🚶️change-people-gain-zone/🧪️tests/⛔️refuses",
    "change-people-gain-schedule-applies": "asset://🧬️schema/🧬️mutations/⏰️change-people-gain-schedule/🧪️tests/✅️applies",
    "change-people-gain-schedule-refuses": "asset://🧬️schema/🧬️mutations/⏰️change-people-gain-schedule/🧪️tests/⛔️refuses",
    "change-people-gain-activity-schedule-applies": "asset://🧬️schema/🧬️mutations/🏃️change-people-gain-activity-schedule/🧪️tests/✅️applies",
    "change-people-gain-activity-schedule-refuses": "asset://🧬️schema/🧬️mutations/🏃️change-people-gain-activity-schedule/🧪️tests/⛔️refuses",
    "change-people-gain-people-per-area-applies": "asset://🧬️schema/🧬️mutations/👥️change-people-gain-people-per-area/🧪️tests/✅️applies",
    "change-people-gain-people-per-area-refuses": "asset://🧬️schema/🧬️mutations/👥️change-people-gain-people-per-area/🧪️tests/⛔️refuses",
    "change-people-gain-sensible-fraction-applies": "asset://🧬️schema/🧬️mutations/🌞️change-people-gain-sensible-fraction/🧪️tests/✅️applies",
    "change-people-gain-sensible-fraction-refuses": "asset://🧬️schema/🧬️mutations/🌞️change-people-gain-sensible-fraction/🧪️tests/⛔️refuses",
    "change-people-gain-latent-fraction-applies": "asset://🧬️schema/🧬️mutations/💧️change-people-gain-latent-fraction/🧪️tests/✅️applies",
    "change-people-gain-latent-fraction-refuses": "asset://🧬️schema/🧬️mutations/💧️change-people-gain-latent-fraction/🧪️tests/⛔️refuses",
    "change-people-gain-radiant-fraction-applies": "asset://🧬️schema/🧬️mutations/📡️change-people-gain-radiant-fraction/🧪️tests/✅️applies",
    "change-people-gain-radiant-fraction-refuses": "asset://🧬️schema/🧬️mutations/📡️change-people-gain-radiant-fraction/🧪️tests/⛔️refuses",
    "create-lighting-gain-applies": "asset://🧬️schema/🧬️mutations/💡️create-lighting-gain/🧪️tests/✅️applies",
    "create-lighting-gain-refuses": "asset://🧬️schema/🧬️mutations/💡️create-lighting-gain/🧪️tests/⛔️refuses",
    "delete-lighting-gain-applies": "asset://🧬️schema/🧬️mutations/🕯️delete-lighting-gain/🧪️tests/✅️applies",
    "delete-lighting-gain-refuses": "asset://🧬️schema/🧬️mutations/🕯️delete-lighting-gain/🧪️tests/⛔️refuses",
    "change-lighting-gain-zone-applies": "asset://🧬️schema/🧬️mutations/🔦️change-lighting-gain-zone/🧪️tests/✅️applies",
    "change-lighting-gain-zone-refuses": "asset://🧬️schema/🧬️mutations/🔦️change-lighting-gain-zone/🧪️tests/⛔️refuses",
    "change-lighting-gain-schedule-applies": "asset://🧬️schema/🧬️mutations/⏱️change-lighting-gain-schedule/🧪️tests/✅️applies",
    "change-lighting-gain-schedule-refuses": "asset://🧬️schema/🧬️mutations/⏱️change-lighting-gain-schedule/🧪️tests/⛔️refuses",
    "change-lighting-gain-watts-per-area-applies": "asset://🧬️schema/🧬️mutations/🔌️change-lighting-gain-watts-per-area/🧪️tests/✅️applies",
    "change-lighting-gain-watts-per-area-refuses": "asset://🧬️schema/🧬️mutations/🔌️change-lighting-gain-watts-per-area/🧪️tests/⛔️refuses",
    "change-lighting-gain-radiant-fraction-applies": "asset://🧬️schema/🧬️mutations/🌟️change-lighting-gain-radiant-fraction/🧪️tests/✅️applies",
    "change-lighting-gain-radiant-fraction-refuses": "asset://🧬️schema/🧬️mutations/🌟️change-lighting-gain-radiant-fraction/🧪️tests/⛔️refuses",
    "change-lighting-gain-visible-fraction-applies": "asset://🧬️schema/🧬️mutations/🔅️change-lighting-gain-visible-fraction/🧪️tests/✅️applies",
    "change-lighting-gain-visible-fraction-refuses": "asset://🧬️schema/🧬️mutations/🔅️change-lighting-gain-visible-fraction/🧪️tests/⛔️refuses",
    "change-lighting-gain-return-air-fraction-applies": "asset://🧬️schema/🧬️mutations/🎐️change-lighting-gain-return-air-fraction/🧪️tests/✅️applies",
    "change-lighting-gain-return-air-fraction-refuses": "asset://🧬️schema/🧬️mutations/🎐️change-lighting-gain-return-air-fraction/🧪️tests/⛔️refuses",
    "create-equipment-gain-applies": "asset://🧬️schema/🧬️mutations/🖥️create-equipment-gain/🧪️tests/✅️applies",
    "create-equipment-gain-refuses": "asset://🧬️schema/🧬️mutations/🖥️create-equipment-gain/🧪️tests/⛔️refuses",
    "delete-equipment-gain-applies": "asset://🧬️schema/🧬️mutations/🧯️delete-equipment-gain/🧪️tests/✅️applies",
    "delete-equipment-gain-refuses": "asset://🧬️schema/🧬️mutations/🧯️delete-equipment-gain/🧪️tests/⛔️refuses",
    "change-equipment-gain-zone-applies": "asset://🧬️schema/🧬️mutations/🖨️change-equipment-gain-zone/🧪️tests/✅️applies",
    "change-equipment-gain-zone-refuses": "asset://🧬️schema/🧬️mutations/🖨️change-equipment-gain-zone/🧪️tests/⛔️refuses",
    "change-equipment-gain-schedule-applies": "asset://🧬️schema/🧬️mutations/⌛️change-equipment-gain-schedule/🧪️tests/✅️applies",
    "change-equipment-gain-schedule-refuses": "asset://🧬️schema/🧬️mutations/⌛️change-equipment-gain-schedule/🧪️tests/⛔️refuses",
    "change-equipment-gain-watts-per-area-applies": "asset://🧬️schema/🧬️mutations/⚡️change-equipment-gain-watts-per-area/🧪️tests/✅️applies",
    "change-equipment-gain-watts-per-area-refuses": "asset://🧬️schema/🧬️mutations/⚡️change-equipment-gain-watts-per-area/🧪️tests/⛔️refuses",
    "change-equipment-gain-radiant-fraction-applies": "asset://🧬️schema/🧬️mutations/🌠️change-equipment-gain-radiant-fraction/🧪️tests/✅️applies",
    "change-equipment-gain-radiant-fraction-refuses": "asset://🧬️schema/🧬️mutations/🌠️change-equipment-gain-radiant-fraction/🧪️tests/⛔️refuses",
    "change-equipment-gain-latent-fraction-applies": "asset://🧬️schema/🧬️mutations/💦️change-equipment-gain-latent-fraction/🧪️tests/✅️applies",
    "change-equipment-gain-latent-fraction-refuses": "asset://🧬️schema/🧬️mutations/💦️change-equipment-gain-latent-fraction/🧪️tests/⛔️refuses",
    "create-infiltration-applies": "asset://🧬️schema/🧬️mutations/💨️create-infiltration/🧪️tests/✅️applies",
    "create-infiltration-refuses": "asset://🧬️schema/🧬️mutations/💨️create-infiltration/🧪️tests/⛔️refuses",
    "delete-infiltration-applies": "asset://🧬️schema/🧬️mutations/🧽️delete-infiltration/🧪️tests/✅️applies",
    "delete-infiltration-refuses": "asset://🧬️schema/🧬️mutations/🧽️delete-infiltration/🧪️tests/⛔️refuses",
    "change-infiltration-zone-applies": "asset://🧬️schema/🧬️mutations/🌀️change-infiltration-zone/🧪️tests/✅️applies",
    "change-infiltration-zone-refuses": "asset://🧬️schema/🧬️mutations/🌀️change-infiltration-zone/🧪️tests/⛔️refuses",
    "change-infiltration-schedule-applies": "asset://🧬️schema/🧬️mutations/⏳️change-infiltration-schedule/🧪️tests/✅️applies",
    "change-infiltration-schedule-refuses": "asset://🧬️schema/🧬️mutations/⏳️change-infiltration-schedule/🧪️tests/⛔️refuses",
    "change-infiltration-flow-per-exterior-area-applies": "asset://🧬️schema/🧬️mutations/🌫️change-infiltration-flow-per-exterior-area/🧪️tests/✅️applies",
    "change-infiltration-flow-per-exterior-area-refuses": "asset://🧬️schema/🧬️mutations/🌫️change-infiltration-flow-per-exterior-area/🧪️tests/⛔️refuses",
    "change-infiltration-constant-term-coefficient-applies": "asset://🧬️schema/🧬️mutations/🅰️change-infiltration-constant-term-coefficient/🧪️tests/✅️applies",
    "change-infiltration-constant-term-coefficient-refuses": "asset://🧬️schema/🧬️mutations/🅰️change-infiltration-constant-term-coefficient/🧪️tests/⛔️refuses",
    "change-infiltration-temperature-term-coefficient-applies": "asset://🧬️schema/🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/🧪️tests/✅️applies",
    "change-infiltration-temperature-term-coefficient-refuses": "asset://🧬️schema/🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/🧪️tests/⛔️refuses",
    "change-infiltration-velocity-term-coefficient-applies": "asset://🧬️schema/🧬️mutations/🆎️change-infiltration-velocity-term-coefficient/🧪️tests/✅️applies",
    "change-infiltration-velocity-term-coefficient-refuses": "asset://🧬️schema/🧬️mutations/🆎️change-infiltration-velocity-term-coefficient/🧪️tests/⛔️refuses",
    "change-infiltration-velocity-squared-term-coefficient-applies": "asset://🧬️schema/🧬️mutations/🆑️change-infiltration-velocity-squared-term-coefficient/🧪️tests/✅️applies",
    "change-infiltration-velocity-squared-term-coefficient-refuses": "asset://🧬️schema/🧬️mutations/🆑️change-infiltration-velocity-squared-term-coefficient/🧪️tests/⛔️refuses",
    "create-mechanical-ventilation-applies": "asset://🧬️schema/🧬️mutations/🌪️create-mechanical-ventilation/🧪️tests/✅️applies",
    "create-mechanical-ventilation-refuses": "asset://🧬️schema/🧬️mutations/🌪️create-mechanical-ventilation/🧪️tests/⛔️refuses",
    "delete-mechanical-ventilation-applies": "asset://🧬️schema/🧬️mutations/🚫️delete-mechanical-ventilation/🧪️tests/✅️applies",
    "delete-mechanical-ventilation-refuses": "asset://🧬️schema/🧬️mutations/🚫️delete-mechanical-ventilation/🧪️tests/⛔️refuses",
    "change-mechanical-ventilation-zone-applies": "asset://🧬️schema/🧬️mutations/🧭️change-mechanical-ventilation-zone/🧪️tests/✅️applies",
    "change-mechanical-ventilation-zone-refuses": "asset://🧬️schema/🧬️mutations/🧭️change-mechanical-ventilation-zone/🧪️tests/⛔️refuses",
    "change-mechanical-ventilation-schedule-applies": "asset://🧬️schema/🧬️mutations/📆️change-mechanical-ventilation-schedule/🧪️tests/✅️applies",
    "change-mechanical-ventilation-schedule-refuses": "asset://🧬️schema/🧬️mutations/📆️change-mechanical-ventilation-schedule/🧪️tests/⛔️refuses",
    "change-mechanical-ventilation-design-flow-applies": "asset://🧬️schema/🧬️mutations/🚿️change-mechanical-ventilation-design-flow/🧪️tests/✅️applies",
    "change-mechanical-ventilation-design-flow-refuses": "asset://🧬️schema/🧬️mutations/🚿️change-mechanical-ventilation-design-flow/🧪️tests/⛔️refuses",
    "change-mechanical-ventilation-fan-total-efficiency-applies": "asset://🧬️schema/🧬️mutations/💠️change-mechanical-ventilation-fan-total-efficiency/🧪️tests/✅️applies",
    "change-mechanical-ventilation-fan-total-efficiency-refuses": "asset://🧬️schema/🧬️mutations/💠️change-mechanical-ventilation-fan-total-efficiency/🧪️tests/⛔️refuses",
    "change-mechanical-ventilation-fan-delta-pressure-applies": "asset://🧬️schema/🧬️mutations/🎈️change-mechanical-ventilation-fan-delta-pressure/🧪️tests/✅️applies",
    "change-mechanical-ventilation-fan-delta-pressure-refuses": "asset://🧬️schema/🧬️mutations/🎈️change-mechanical-ventilation-fan-delta-pressure/🧪️tests/⛔️refuses",
    "change-infiltration-method-applies": "asset://🧬️schema/🧬️mutations/🔬️change-infiltration-method/🧪️tests/✅️applies",
    "change-infiltration-method-refuses": "asset://🧬️schema/🧬️mutations/🔬️change-infiltration-method/🧪️tests/⛔️refuses",
    "change-infiltration-design-flow-ach-applies": "asset://🧬️schema/🧬️mutations/🔄️change-infiltration-design-flow-ach/🧪️tests/✅️applies",
    "change-infiltration-design-flow-ach-refuses": "asset://🧬️schema/🧬️mutations/🔄️change-infiltration-design-flow-ach/🧪️tests/⛔️refuses",
    "change-infiltration-effective-leakage-area-applies": "asset://🧬️schema/🧬️mutations/🕳️change-infiltration-effective-leakage-area/🧪️tests/✅️applies",
    "change-infiltration-effective-leakage-area-refuses": "asset://🧬️schema/🧬️mutations/🕳️change-infiltration-effective-leakage-area/🧪️tests/⛔️refuses",
    "change-infiltration-discharge-coefficient-applies": "asset://🧬️schema/🧬️mutations/🚰️change-infiltration-discharge-coefficient/🧪️tests/✅️applies",
    "change-infiltration-discharge-coefficient-refuses": "asset://🧬️schema/🧬️mutations/🚰️change-infiltration-discharge-coefficient/🧪️tests/⛔️refuses",
    "change-infiltration-stack-height-applies": "asset://🧬️schema/🧬️mutations/🏭️change-infiltration-stack-height/🧪️tests/✅️applies",
    "change-infiltration-stack-height-refuses": "asset://🧬️schema/🧬️mutations/🏭️change-infiltration-stack-height/🧪️tests/⛔️refuses",
    "create-thermostat-controls-zone-one": "asset://🧬️schema/🧬️mutations/🩺️create-thermostat/🧪️tests/✅️controls-zone-one",
    "create-thermostat-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/🩺️create-thermostat/🧪️tests/⛔️refuses-an-absent-zone",
    "delete-thermostat-frees-zone-one": "asset://🧬️schema/🧬️mutations/🛑️delete-thermostat/🧪️tests/✅️frees-zone-one",
    "delete-thermostat-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🛑️delete-thermostat/🧪️tests/⛔️refuses-an-absent-one",
    "change-thermostat-zone-moves-to-zone-two": "asset://🧬️schema/🧬️mutations/🛖️change-thermostat-zone/🧪️tests/✅️moves-to-zone-two",
    "change-thermostat-zone-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/🛖️change-thermostat-zone/🧪️tests/⛔️refuses-an-absent-zone",
    "change-thermostat-heating-setpoint-schedule-repoints-heating": "asset://🧬️schema/🧬️mutations/🥵️change-thermostat-heating-setpoint-schedule/🧪️tests/✅️repoints-heating",
    "change-thermostat-heating-setpoint-schedule-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🥵️change-thermostat-heating-setpoint-schedule/🧪️tests/⛔️refuses-an-absent-one",
    "change-thermostat-cooling-setpoint-schedule-repoints-cooling": "asset://🧬️schema/🧬️mutations/🐧️change-thermostat-cooling-setpoint-schedule/🧪️tests/✅️repoints-cooling",
    "change-thermostat-cooling-setpoint-schedule-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🐧️change-thermostat-cooling-setpoint-schedule/🧪️tests/⛔️refuses-an-absent-one",
    "change-thermostat-heating-throttle-range-widens-heating-band": "asset://🧬️schema/🧬️mutations/🎚️change-thermostat-heating-throttle-range/🧪️tests/✅️widens-heating-band",
    "change-thermostat-heating-throttle-range-refuses-a-zero-band": "asset://🧬️schema/🧬️mutations/🎚️change-thermostat-heating-throttle-range/🧪️tests/⛔️refuses-a-zero-band",
    "change-thermostat-cooling-throttle-range-widens-cooling-band": "asset://🧬️schema/🧬️mutations/🎛️change-thermostat-cooling-throttle-range/🧪️tests/✅️widens-cooling-band",
    "change-thermostat-cooling-throttle-range-refuses-a-negative-band": "asset://🧬️schema/🧬️mutations/🎛️change-thermostat-cooling-throttle-range/🧪️tests/⛔️refuses-a-negative-band",
    "create-humidistat-controls-zone-one": "asset://🧬️schema/🧬️mutations/🌂️create-humidistat/🧪️tests/✅️controls-zone-one",
    "create-humidistat-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🌂️create-humidistat/🧪️tests/⛔️refuses-an-absent-one",
    "delete-humidistat-drops-the-control": "asset://🧬️schema/🧬️mutations/🏜️delete-humidistat/🧪️tests/✅️drops-the-control",
    "delete-humidistat-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🏜️delete-humidistat/🧪️tests/⛔️refuses-an-absent-one",
    "change-humidistat-zone-moves-to-zone-two": "asset://🧬️schema/🧬️mutations/🏙️change-humidistat-zone/🧪️tests/✅️moves-to-zone-two",
    "change-humidistat-zone-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/🏙️change-humidistat-zone/🧪️tests/⛔️refuses-an-absent-zone",
    "change-humidistat-humidifying-setpoint-schedule-repoints-humidifying": "asset://🧬️schema/🧬️mutations/☔️change-humidistat-humidifying-setpoint-schedule/🧪️tests/✅️repoints-humidifying",
    "change-humidistat-humidifying-setpoint-schedule-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/☔️change-humidistat-humidifying-setpoint-schedule/🧪️tests/⛔️refuses-an-absent-one",
    "change-humidistat-dehumidifying-setpoint-schedule-repoints-drying": "asset://🧬️schema/🧬️mutations/🏝️change-humidistat-dehumidifying-setpoint-schedule/🧪️tests/✅️repoints-drying",
    "change-humidistat-dehumidifying-setpoint-schedule-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🏝️change-humidistat-dehumidifying-setpoint-schedule/🧪️tests/⛔️refuses-an-absent-one",
    "change-humidistat-humidifying-throttle-range-widens-the-band": "asset://🧬️schema/🧬️mutations/🌧️change-humidistat-humidifying-throttle-range/🧪️tests/✅️widens-the-band",
    "change-humidistat-humidifying-throttle-range-refuses-a-zero-band": "asset://🧬️schema/🧬️mutations/🌧️change-humidistat-humidifying-throttle-range/🧪️tests/⛔️refuses-a-zero-band",
    "change-humidistat-dehumidifying-throttle-range-widens-the-band": "asset://🧬️schema/🧬️mutations/🧻️change-humidistat-dehumidifying-throttle-range/🧪️tests/✅️widens-the-band",
    "change-humidistat-dehumidifying-throttle-range-refuses-a-negative-band": "asset://🧬️schema/🧬️mutations/🧻️change-humidistat-dehumidifying-throttle-range/🧪️tests/⛔️refuses-a-negative-band",
    "create-ideal-loads-system-serves-zone-one": "asset://🧬️schema/🧬️mutations/🫁️create-ideal-loads-system/🧪️tests/✅️serves-zone-one",
    "create-ideal-loads-system-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/🫁️create-ideal-loads-system/🧪️tests/⛔️refuses-an-absent-zone",
    "delete-ideal-loads-system-drops-the-system": "asset://🧬️schema/🧬️mutations/🫥️delete-ideal-loads-system/🧪️tests/✅️drops-the-system",
    "delete-ideal-loads-system-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🫥️delete-ideal-loads-system/🧪️tests/⛔️refuses-an-absent-one",
    "change-ideal-loads-system-zone-moves-to-zone-two": "asset://🧬️schema/🧬️mutations/🏢️change-ideal-loads-system-zone/🧪️tests/✅️moves-to-zone-two",
    "change-ideal-loads-system-zone-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/🏢️change-ideal-loads-system-zone/🧪️tests/⛔️refuses-an-absent-zone",
    "change-ideal-loads-system-max-heating-supply-air-temp-cools-the-supply": "asset://🧬️schema/🧬️mutations/🔴️change-ideal-loads-system-max-heating-supply-air-temp/🧪️tests/✅️cools-the-supply",
    "change-ideal-loads-system-max-heating-supply-air-temp-refuses-a-hot-supply": "asset://🧬️schema/🧬️mutations/🔴️change-ideal-loads-system-max-heating-supply-air-temp/🧪️tests/⛔️refuses-a-hot-supply",
    "change-ideal-loads-system-min-cooling-supply-air-temp-lowers-the-supply": "asset://🧬️schema/🧬️mutations/🔵️change-ideal-loads-system-min-cooling-supply-air-temp/🧪️tests/✅️lowers-the-supply",
    "change-ideal-loads-system-min-cooling-supply-air-temp-refuses-a-cold-supply": "asset://🧬️schema/🧬️mutations/🔵️change-ideal-loads-system-min-cooling-supply-air-temp/🧪️tests/⛔️refuses-a-cold-supply",
    "change-ideal-loads-system-max-heating-capacity-caps-the-heating": "asset://🧬️schema/🧬️mutations/⛽️change-ideal-loads-system-max-heating-capacity/🧪️tests/✅️caps-the-heating",
    "change-ideal-loads-system-max-heating-capacity-refuses-a-stray-value": "asset://🧬️schema/🧬️mutations/⛽️change-ideal-loads-system-max-heating-capacity/🧪️tests/⛔️refuses-a-stray-value",
    "change-ideal-loads-system-max-cooling-capacity-caps-the-cooling": "asset://🧬️schema/🧬️mutations/🟧️change-ideal-loads-system-max-cooling-capacity/🧪️tests/✅️caps-the-cooling",
    "change-ideal-loads-system-max-cooling-capacity-refuses-a-stray-value": "asset://🧬️schema/🧬️mutations/🟧️change-ideal-loads-system-max-cooling-capacity/🧪️tests/⛔️refuses-a-stray-value",
    "change-ideal-loads-system-outdoor-air-per-person-ventilates-per-head": "asset://🧬️schema/🧬️mutations/🧍️change-ideal-loads-system-outdoor-air-per-person/🧪️tests/✅️ventilates-per-head",
    "change-ideal-loads-system-outdoor-air-per-person-refuses-a-negative": "asset://🧬️schema/🧬️mutations/🧍️change-ideal-loads-system-outdoor-air-per-person/🧪️tests/⛔️refuses-a-negative",
    "change-ideal-loads-system-outdoor-air-per-area-ventilates-per-area": "asset://🧬️schema/🧬️mutations/🔳️change-ideal-loads-system-outdoor-air-per-area/🧪️tests/✅️ventilates-per-area",
    "change-ideal-loads-system-outdoor-air-per-area-refuses-a-negative": "asset://🧬️schema/🧬️mutations/🔳️change-ideal-loads-system-outdoor-air-per-area/🧪️tests/⛔️refuses-a-negative",
    "create-zone-equipment-adds-a-baseboard": "asset://🧬️schema/🧬️mutations/🛠️create-zone-equipment/🧪️tests/✅️adds-a-baseboard",
    "create-zone-equipment-refuses-rank-zero": "asset://🧬️schema/🧬️mutations/🛠️create-zone-equipment/🧪️tests/⛔️refuses-rank-zero",
    "delete-zone-equipment-drops-the-baseboard": "asset://🧬️schema/🧬️mutations/🗑️delete-zone-equipment/🧪️tests/✅️drops-the-baseboard",
    "delete-zone-equipment-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🗑️delete-zone-equipment/🧪️tests/⛔️refuses-an-absent-one",
    "change-zone-equipment-zone-moves-to-zone-two": "asset://🧬️schema/🧬️mutations/🏬️change-zone-equipment-zone/🧪️tests/✅️moves-to-zone-two",
    "change-zone-equipment-zone-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/🏬️change-zone-equipment-zone/🧪️tests/⛔️refuses-an-absent-zone",
    "change-zone-equipment-type-swaps-to-a-fan-coil": "asset://🧬️schema/🧬️mutations/🔧️change-zone-equipment-type/🧪️tests/✅️swaps-to-a-fan-coil",
    "change-zone-equipment-type-refuses-an-absent-row": "asset://🧬️schema/🧬️mutations/🔧️change-zone-equipment-type/🧪️tests/⛔️refuses-an-absent-row",
    "change-zone-equipment-priority-demotes-it": "asset://🧬️schema/🧬️mutations/🎗️change-zone-equipment-priority/🧪️tests/✅️demotes-it",
    "change-zone-equipment-priority-refuses-rank-zero": "asset://🧬️schema/🧬️mutations/🎗️change-zone-equipment-priority/🧪️tests/⛔️refuses-rank-zero",
    "change-zone-equipment-heating-capacity-uprates-heating": "asset://🧬️schema/🧬️mutations/🧇️change-zone-equipment-heating-capacity/🧪️tests/✅️uprates-heating",
    "change-zone-equipment-heating-capacity-refuses-a-negative": "asset://🧬️schema/🧬️mutations/🧇️change-zone-equipment-heating-capacity/🧪️tests/⛔️refuses-a-negative",
    "change-zone-equipment-cooling-capacity-uprates-cooling": "asset://🧬️schema/🧬️mutations/🍧️change-zone-equipment-cooling-capacity/🧪️tests/✅️uprates-cooling",
    "change-zone-equipment-cooling-capacity-refuses-a-negative": "asset://🧬️schema/🧬️mutations/🍧️change-zone-equipment-cooling-capacity/🧪️tests/⛔️refuses-a-negative",
    "create-daylight-zone-lights-zone-one": "asset://🧬️schema/🧬️mutations/🔭️create-daylight-zone/🧪️tests/✅️lights-zone-one",
    "create-daylight-zone-refuses-a-bad-tau": "asset://🧬️schema/🧬️mutations/🔭️create-daylight-zone/🧪️tests/⛔️refuses-a-bad-tau",
    "delete-daylight-zone-darkens-the-zone": "asset://🧬️schema/🧬️mutations/🌗️delete-daylight-zone/🧪️tests/✅️darkens-the-zone",
    "delete-daylight-zone-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🌗️delete-daylight-zone/🧪️tests/⛔️refuses-an-absent-one",
    "change-daylight-zone-zone-moves-to-zone-two": "asset://🧬️schema/🧬️mutations/🏫️change-daylight-zone-zone/🧪️tests/✅️moves-to-zone-two",
    "change-daylight-zone-zone-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/🏫️change-daylight-zone-zone/🧪️tests/⛔️refuses-an-absent-zone",
    "change-daylight-zone-illuminance-target-dims-the-target": "asset://🧬️schema/🧬️mutations/🪔️change-daylight-zone-illuminance-target/🧪️tests/✅️dims-the-target",
    "change-daylight-zone-illuminance-target-refuses-a-dark-target": "asset://🧬️schema/🧬️mutations/🪔️change-daylight-zone-illuminance-target/🧪️tests/⛔️refuses-a-dark-target",
    "change-daylight-zone-glare-limit-tightens-glare": "asset://🧬️schema/🧬️mutations/🕶️change-daylight-zone-glare-limit/🧪️tests/✅️tightens-glare",
    "change-daylight-zone-glare-limit-refuses-a-negative": "asset://🧬️schema/🧬️mutations/🕶️change-daylight-zone-glare-limit/🧪️tests/⛔️refuses-a-negative",
    "change-daylight-zone-window-transmittance-darkens-the-glass": "asset://🧬️schema/🧬️mutations/🥃️change-daylight-zone-window-transmittance/🧪️tests/✅️darkens-the-glass",
    "change-daylight-zone-window-transmittance-refuses-a-bad-tau": "asset://🧬️schema/🧬️mutations/🥃️change-daylight-zone-window-transmittance/🧪️tests/⛔️refuses-a-bad-tau",
    "create-sizing-object-sizes-zone-one": "asset://🧬️schema/🧬️mutations/📶️create-sizing-object/🧪️tests/✅️sizes-zone-one",
    "create-sizing-object-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/📶️create-sizing-object/🧪️tests/⛔️refuses-an-absent-zone",
    "delete-sizing-object-drops-the-sizing": "asset://🧬️schema/🧬️mutations/🪒️delete-sizing-object/🧪️tests/✅️drops-the-sizing",
    "delete-sizing-object-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🪒️delete-sizing-object/🧪️tests/⛔️refuses-an-absent-one",
    "change-sizing-object-zone-moves-to-zone-two": "asset://🧬️schema/🧬️mutations/🏨️change-sizing-object-zone/🧪️tests/✅️moves-to-zone-two",
    "change-sizing-object-zone-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/🏨️change-sizing-object-zone/🧪️tests/⛔️refuses-an-absent-zone",
    "change-sizing-object-sizing-type-sizes-for-cooling": "asset://🧬️schema/🧬️mutations/🧾️change-sizing-object-sizing-type/🧪️tests/✅️sizes-for-cooling",
    "change-sizing-object-sizing-type-refuses-an-absent-row": "asset://🧬️schema/🧬️mutations/🧾️change-sizing-object-sizing-type/🧪️tests/⛔️refuses-an-absent-row",
    "change-sizing-object-design-day-type-reads-a-hot-day": "asset://🧬️schema/🧬️mutations/🌥️change-sizing-object-design-day-type/🧪️tests/✅️reads-a-hot-day",
    "change-sizing-object-design-day-type-refuses-an-absent-row": "asset://🧬️schema/🧬️mutations/🌥️change-sizing-object-design-day-type/🧪️tests/⛔️refuses-an-absent-row",
    "create-room-air-model-assignment-stratifies-zone-two": "asset://🧬️schema/🧬️mutations/🛏️create-room-air-model-assignment/🧪️tests/✅️stratifies-zone-two",
    "create-room-air-model-assignment-refuses-a-second-one": "asset://🧬️schema/🧬️mutations/🛏️create-room-air-model-assignment/🧪️tests/⛔️refuses-a-second-one",
    "delete-room-air-model-assignment-falls-back": "asset://🧬️schema/🧬️mutations/🧺️delete-room-air-model-assignment/🧪️tests/✅️falls-back",
    "delete-room-air-model-assignment-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🧺️delete-room-air-model-assignment/🧪️tests/⛔️refuses-an-absent-one",
    "change-room-air-model-stratifies-the-air": "asset://🧬️schema/🧬️mutations/🪭️change-room-air-model/🧪️tests/✅️stratifies-the-air",
    "change-room-air-model-refuses-an-absent-row": "asset://🧬️schema/🧬️mutations/🪭️change-room-air-model/🧪️tests/⛔️refuses-an-absent-row",
    "create-setpoint-manager-adds-a-scheduled-spm": "asset://🧬️schema/🧬️mutations/📌️create-setpoint-manager/🧪️tests/✅️adds-a-scheduled-spm",
    "create-setpoint-manager-refuses-a-bad-kind": "asset://🧬️schema/🧬️mutations/📌️create-setpoint-manager/🧪️tests/⛔️refuses-a-bad-kind",
    "delete-setpoint-manager-drops-the-spm": "asset://🧬️schema/🧬️mutations/🍄️delete-setpoint-manager/🧪️tests/✅️drops-the-spm",
    "delete-setpoint-manager-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🍄️delete-setpoint-manager/🧪️tests/⛔️refuses-an-absent-one",
    "rename-setpoint-manager-renames-the-spm": "asset://🧬️schema/🧬️mutations/🖇️rename-setpoint-manager/🧪️tests/✅️renames-the-spm",
    "rename-setpoint-manager-refuses-a-blank-name": "asset://🧬️schema/🧬️mutations/🖇️rename-setpoint-manager/🧪️tests/⛔️refuses-a-blank-name",
    "replace-setpoint-manager-kind-resets-on-outdoor-air": "asset://🧬️schema/🧬️mutations/🔃️replace-setpoint-manager-kind/🧪️tests/✅️resets-on-outdoor-air",
    "replace-setpoint-manager-kind-refuses-stray-limits": "asset://🧬️schema/🧬️mutations/🔃️replace-setpoint-manager-kind/🧪️tests/⛔️refuses-stray-limits",
    "change-setpoint-manager-schedule-repoints-the-spm": "asset://🧬️schema/🧬️mutations/🎼️change-setpoint-manager-schedule/🧪️tests/✅️repoints-the-spm",
    "change-setpoint-manager-schedule-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🎼️change-setpoint-manager-schedule/🧪️tests/⛔️refuses-an-absent-one",
    "create-air-loop-adds-a-main-loop": "asset://🧬️schema/🧬️mutations/🛞️create-air-loop/🧪️tests/✅️adds-a-main-loop",
    "create-air-loop-refuses-a-jumbled-list": "asset://🧬️schema/🧬️mutations/🛞️create-air-loop/🧪️tests/⛔️refuses-a-jumbled-list",
    "delete-air-loop-drops-the-loop": "asset://🧬️schema/🧬️mutations/🥀️delete-air-loop/🧪️tests/✅️drops-the-loop",
    "delete-air-loop-refuses-a-served-loop": "asset://🧬️schema/🧬️mutations/🥀️delete-air-loop/🧪️tests/⛔️refuses-a-served-loop",
    "rename-air-loop-renames-the-loop": "asset://🧬️schema/🧬️mutations/📇️rename-air-loop/🧪️tests/✅️renames-the-loop",
    "rename-air-loop-refuses-a-blank-name": "asset://🧬️schema/🧬️mutations/📇️rename-air-loop/🧪️tests/⛔️refuses-a-blank-name",
    "change-air-loop-supply-node-repoints-supply": "asset://🧬️schema/🧬️mutations/↗️change-air-loop-supply-node/🧪️tests/✅️repoints-supply",
    "change-air-loop-supply-node-refuses-node-zero": "asset://🧬️schema/🧬️mutations/↗️change-air-loop-supply-node/🧪️tests/⛔️refuses-node-zero",
    "change-air-loop-return-node-repoints-return": "asset://🧬️schema/🧬️mutations/↘️change-air-loop-return-node/🧪️tests/✅️repoints-return",
    "change-air-loop-return-node-refuses-node-zero": "asset://🧬️schema/🧬️mutations/↘️change-air-loop-return-node/🧪️tests/⛔️refuses-node-zero",
    "change-air-loop-design-supply-air-flow-uprates-the-flow": "asset://🧬️schema/🧬️mutations/🍥️change-air-loop-design-supply-air-flow/🧪️tests/✅️uprates-the-flow",
    "change-air-loop-design-supply-air-flow-refuses-no-flow": "asset://🧬️schema/🧬️mutations/🍥️change-air-loop-design-supply-air-flow/🧪️tests/⛔️refuses-no-flow",
    "add-air-loop-terminal-zone-serves-zone-two": "asset://🧬️schema/🧬️mutations/🪺️add-air-loop-terminal-zone/🧪️tests/✅️serves-zone-two",
    "add-air-loop-terminal-zone-refuses-an-absent-zone": "asset://🧬️schema/🧬️mutations/🪺️add-air-loop-terminal-zone/🧪️tests/⛔️refuses-an-absent-zone",
    "remove-air-loop-terminal-zone-stops-serving-one": "asset://🧬️schema/🧬️mutations/🪹️remove-air-loop-terminal-zone/🧪️tests/✅️stops-serving-one",
    "remove-air-loop-terminal-zone-refuses-an-unserved": "asset://🧬️schema/🧬️mutations/🪹️remove-air-loop-terminal-zone/🧪️tests/⛔️refuses-an-unserved",
    "create-plant-loop-adds-a-hot-loop": "asset://🧬️schema/🧬️mutations/⚗️create-plant-loop/🧪️tests/✅️adds-a-hot-loop",
    "create-plant-loop-refuses-a-jumbled-list": "asset://🧬️schema/🧬️mutations/⚗️create-plant-loop/🧪️tests/⛔️refuses-a-jumbled-list",
    "delete-plant-loop-drops-the-loop": "asset://🧬️schema/🧬️mutations/💣️delete-plant-loop/🧪️tests/✅️drops-the-loop",
    "delete-plant-loop-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/💣️delete-plant-loop/🧪️tests/⛔️refuses-an-absent-one",
    "rename-plant-loop-renames-the-loop": "asset://🧬️schema/🧬️mutations/📛️rename-plant-loop/🧪️tests/✅️renames-the-loop",
    "rename-plant-loop-refuses-a-blank-name": "asset://🧬️schema/🧬️mutations/📛️rename-plant-loop/🧪️tests/⛔️refuses-a-blank-name",
    "change-plant-loop-type-turns-it-chilled": "asset://🧬️schema/🧬️mutations/♻️change-plant-loop-type/🧪️tests/✅️turns-it-chilled",
    "change-plant-loop-type-refuses-an-absent-row": "asset://🧬️schema/🧬️mutations/♻️change-plant-loop-type/🧪️tests/⛔️refuses-an-absent-row",
    "change-plant-loop-supply-temperature-cools-the-supply": "asset://🧬️schema/🧬️mutations/☕️change-plant-loop-supply-temperature/🧪️tests/✅️cools-the-supply",
    "change-plant-loop-supply-temperature-refuses-a-hot-supply": "asset://🧬️schema/🧬️mutations/☕️change-plant-loop-supply-temperature/🧪️tests/⛔️refuses-a-hot-supply",
    "change-plant-loop-return-temperature-cools-the-return": "asset://🧬️schema/🧬️mutations/🫖️change-plant-loop-return-temperature/🧪️tests/✅️cools-the-return",
    "change-plant-loop-return-temperature-refuses-a-cold-return": "asset://🧬️schema/🧬️mutations/🫖️change-plant-loop-return-temperature/🧪️tests/⛔️refuses-a-cold-return",
    "change-plant-loop-design-flow-uprates-the-flow": "asset://🧬️schema/🧬️mutations/🚤️change-plant-loop-design-flow/🧪️tests/✅️uprates-the-flow",
    "change-plant-loop-design-flow-refuses-no-flow": "asset://🧬️schema/🧬️mutations/🚤️change-plant-loop-design-flow/🧪️tests/⛔️refuses-no-flow",
    "add-plant-loop-equipment-names-equipment": "asset://🧬️schema/🧬️mutations/🔩️add-plant-loop-equipment/🧪️tests/✅️names-equipment",
    "add-plant-loop-equipment-refuses-the-unset-id": "asset://🧬️schema/🧬️mutations/🔩️add-plant-loop-equipment/🧪️tests/⛔️refuses-the-unset-id",
    "remove-plant-loop-equipment-drops-equipment": "asset://🧬️schema/🧬️mutations/⚙️remove-plant-loop-equipment/🧪️tests/✅️drops-equipment",
    "remove-plant-loop-equipment-refuses-an-unlisted": "asset://🧬️schema/🧬️mutations/⚙️remove-plant-loop-equipment/🧪️tests/⛔️refuses-an-unlisted",
    "create-outdoor-air-system-ventilates-the-loop": "asset://🧬️schema/🧬️mutations/🌲️create-outdoor-air-system/🧪️tests/✅️ventilates-the-loop",
    "create-outdoor-air-system-refuses-an-absent-loop": "asset://🧬️schema/🧬️mutations/🌲️create-outdoor-air-system/🧪️tests/⛔️refuses-an-absent-loop",
    "delete-outdoor-air-system-drops-the-system": "asset://🧬️schema/🧬️mutations/🍂️delete-outdoor-air-system/🧪️tests/✅️drops-the-system",
    "delete-outdoor-air-system-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/🍂️delete-outdoor-air-system/🧪️tests/⛔️refuses-an-absent-one",
    "change-outdoor-air-system-air-loop-moves-to-the-spare": "asset://🧬️schema/🧬️mutations/⛓️change-outdoor-air-system-air-loop/🧪️tests/✅️moves-to-the-spare",
    "change-outdoor-air-system-air-loop-refuses-an-absent-loop": "asset://🧬️schema/🧬️mutations/⛓️change-outdoor-air-system-air-loop/🧪️tests/⛔️refuses-an-absent-loop",
    "change-outdoor-air-system-min-oa-flow-raises-the-minimum": "asset://🧬️schema/🧬️mutations/🦋️change-outdoor-air-system-min-oa-flow/🧪️tests/✅️raises-the-minimum",
    "change-outdoor-air-system-min-oa-flow-refuses-a-negative": "asset://🧬️schema/🧬️mutations/🦋️change-outdoor-air-system-min-oa-flow/🧪️tests/⛔️refuses-a-negative",
    "change-outdoor-air-system-economizer-enabled-frees-the-cooling": "asset://🧬️schema/🧬️mutations/💰️change-outdoor-air-system-economizer-enabled/🧪️tests/✅️frees-the-cooling",
    "change-outdoor-air-system-economizer-enabled-refuses-an-absent-row": "asset://🧬️schema/🧬️mutations/💰️change-outdoor-air-system-economizer-enabled/🧪️tests/⛔️refuses-an-absent-row",
    "create-electrical-load-center-applies": "asset://🧬️schema/🧬️mutations/🏦️create-electrical-load-center/🧪️tests/✅️applies",
    "create-electrical-load-center-refuses": "asset://🧬️schema/🧬️mutations/🏦️create-electrical-load-center/🧪️tests/⛔️refuses",
    "delete-electrical-load-center-applies": "asset://🧬️schema/🧬️mutations/🔻️delete-electrical-load-center/🧪️tests/✅️applies",
    "delete-electrical-load-center-refuses": "asset://🧬️schema/🧬️mutations/🔻️delete-electrical-load-center/🧪️tests/⛔️refuses",
    "rename-electrical-load-center-applies": "asset://🧬️schema/🧬️mutations/🖊️rename-electrical-load-center/🧪️tests/✅️applies",
    "rename-electrical-load-center-refuses": "asset://🧬️schema/🧬️mutations/🖊️rename-electrical-load-center/🧪️tests/⛔️refuses",
    "add-electrical-load-center-pv-applies": "asset://🧬️schema/🧬️mutations/☄️add-electrical-load-center-pv/🧪️tests/✅️applies",
    "add-electrical-load-center-pv-refuses": "asset://🧬️schema/🧬️mutations/☄️add-electrical-load-center-pv/🧪️tests/⛔️refuses",
    "remove-electrical-load-center-pv-applies": "asset://🧬️schema/🧬️mutations/🌘️remove-electrical-load-center-pv/🧪️tests/✅️applies",
    "remove-electrical-load-center-pv-refuses": "asset://🧬️schema/🧬️mutations/🌘️remove-electrical-load-center-pv/🧪️tests/⛔️refuses",
    "add-electrical-load-center-battery-applies": "asset://🧬️schema/🧬️mutations/🔋️add-electrical-load-center-battery/🧪️tests/✅️applies",
    "add-electrical-load-center-battery-refuses": "asset://🧬️schema/🧬️mutations/🔋️add-electrical-load-center-battery/🧪️tests/⛔️refuses",
    "remove-electrical-load-center-battery-applies": "asset://🧬️schema/🧬️mutations/🪝️remove-electrical-load-center-battery/🧪️tests/✅️applies",
    "remove-electrical-load-center-battery-refuses": "asset://🧬️schema/🧬️mutations/🪝️remove-electrical-load-center-battery/🧪️tests/⛔️refuses",
    "create-pv-system-applies": "asset://🧬️schema/🧬️mutations/✨️create-pv-system/🧪️tests/✅️applies",
    "create-pv-system-refuses": "asset://🧬️schema/🧬️mutations/✨️create-pv-system/🧪️tests/⛔️refuses",
    "delete-pv-system-applies": "asset://🧬️schema/🧬️mutations/🌒️delete-pv-system/🧪️tests/✅️applies",
    "delete-pv-system-refuses": "asset://🧬️schema/🧬️mutations/🌒️delete-pv-system/🧪️tests/⛔️refuses",
    "change-pv-system-dc-capacity-applies": "asset://🧬️schema/🧬️mutations/⚛️change-pv-system-dc-capacity/🧪️tests/✅️applies",
    "change-pv-system-dc-capacity-refuses": "asset://🧬️schema/🧬️mutations/⚛️change-pv-system-dc-capacity/🧪️tests/⛔️refuses",
    "change-pv-system-area-applies": "asset://🧬️schema/🧬️mutations/🟨️change-pv-system-area/🧪️tests/✅️applies",
    "change-pv-system-area-refuses": "asset://🧬️schema/🧬️mutations/🟨️change-pv-system-area/🧪️tests/⛔️refuses",
    "change-pv-system-tilt-applies": "asset://🧬️schema/🧬️mutations/📈️change-pv-system-tilt/🧪️tests/✅️applies",
    "change-pv-system-tilt-refuses": "asset://🧬️schema/🧬️mutations/📈️change-pv-system-tilt/🧪️tests/⛔️refuses",
    "change-pv-system-azimuth-applies": "asset://🧬️schema/🧬️mutations/🧿️change-pv-system-azimuth/🧪️tests/✅️applies",
    "change-pv-system-azimuth-refuses": "asset://🧬️schema/🧬️mutations/🧿️change-pv-system-azimuth/🧪️tests/⛔️refuses",
    "change-pv-system-module-efficiency-applies": "asset://🧬️schema/🧬️mutations/🎖️change-pv-system-module-efficiency/🧪️tests/✅️applies",
    "change-pv-system-module-efficiency-refuses": "asset://🧬️schema/🧬️mutations/🎖️change-pv-system-module-efficiency/🧪️tests/⛔️refuses",
    "change-pv-system-inverter-efficiency-applies": "asset://🧬️schema/🧬️mutations/♌️change-pv-system-inverter-efficiency/🧪️tests/✅️applies",
    "change-pv-system-inverter-efficiency-refuses": "asset://🧬️schema/🧬️mutations/♌️change-pv-system-inverter-efficiency/🧪️tests/⛔️refuses",
    "create-battery-applies": "asset://🧬️schema/🧬️mutations/🪙️create-battery/🧪️tests/✅️applies",
    "create-battery-refuses": "asset://🧬️schema/🧬️mutations/🪙️create-battery/🧪️tests/⛔️refuses",
    "delete-battery-applies": "asset://🧬️schema/🧬️mutations/♒️delete-battery/🧪️tests/✅️applies",
    "delete-battery-refuses": "asset://🧬️schema/🧬️mutations/♒️delete-battery/🧪️tests/⛔️refuses",
    "change-battery-capacity-applies": "asset://🧬️schema/🧬️mutations/🥫️change-battery-capacity/🧪️tests/✅️applies",
    "change-battery-capacity-refuses": "asset://🧬️schema/🧬️mutations/🥫️change-battery-capacity/🧪️tests/⛔️refuses",
    "change-battery-max-charge-applies": "asset://🧬️schema/🧬️mutations/⏫️change-battery-max-charge/🧪️tests/✅️applies",
    "change-battery-max-charge-refuses": "asset://🧬️schema/🧬️mutations/⏫️change-battery-max-charge/🧪️tests/⛔️refuses",
    "change-battery-max-discharge-applies": "asset://🧬️schema/🧬️mutations/⏬️change-battery-max-discharge/🧪️tests/✅️applies",
    "change-battery-max-discharge-refuses": "asset://🧬️schema/🧬️mutations/⏬️change-battery-max-discharge/🧪️tests/⛔️refuses",
    "change-battery-round-trip-efficiency-applies": "asset://🧬️schema/🧬️mutations/🥉️change-battery-round-trip-efficiency/🧪️tests/✅️applies",
    "change-battery-round-trip-efficiency-refuses": "asset://🧬️schema/🧬️mutations/🥉️change-battery-round-trip-efficiency/🧪️tests/⛔️refuses",
    "create-shw-system-applies": "asset://🧬️schema/🧬️mutations/🛀️create-shw-system/🧪️tests/✅️applies",
    "create-shw-system-refuses": "asset://🧬️schema/🧬️mutations/🛀️create-shw-system/🧪️tests/⛔️refuses",
    "delete-shw-system-applies": "asset://🧬️schema/🧬️mutations/🚱️delete-shw-system/🧪️tests/✅️applies",
    "delete-shw-system-refuses": "asset://🧬️schema/🧬️mutations/🚱️delete-shw-system/🧪️tests/⛔️refuses",
    "change-shw-system-heater-capacity-applies": "asset://🧬️schema/🧬️mutations/🍵️change-shw-system-heater-capacity/🧪️tests/✅️applies",
    "change-shw-system-heater-capacity-refuses": "asset://🧬️schema/🧬️mutations/🍵️change-shw-system-heater-capacity/🧪️tests/⛔️refuses",
    "change-shw-system-storage-volume-applies": "asset://🧬️schema/🧬️mutations/🛢️change-shw-system-storage-volume/🧪️tests/✅️applies",
    "change-shw-system-storage-volume-refuses": "asset://🧬️schema/🧬️mutations/🛢️change-shw-system-storage-volume/🧪️tests/⛔️refuses",
    "change-shw-system-setpoint-applies": "asset://🧬️schema/🧬️mutations/🏹️change-shw-system-setpoint/🧪️tests/✅️applies",
    "change-shw-system-setpoint-refuses": "asset://🧬️schema/🧬️mutations/🏹️change-shw-system-setpoint/🧪️tests/⛔️refuses",
    "change-shw-system-schedule-applies": "asset://🧬️schema/🧬️mutations/🕐️change-shw-system-schedule/🧪️tests/✅️applies",
    "change-shw-system-schedule-refuses": "asset://🧬️schema/🧬️mutations/🕐️change-shw-system-schedule/🧪️tests/⛔️refuses",
    "create-solar-thermal-system-applies": "asset://🧬️schema/🧬️mutations/🌄️create-solar-thermal-system/🧪️tests/✅️applies",
    "create-solar-thermal-system-refuses": "asset://🧬️schema/🧬️mutations/🌄️create-solar-thermal-system/🧪️tests/⛔️refuses",
    "delete-solar-thermal-system-applies": "asset://🧬️schema/🧬️mutations/🌆️delete-solar-thermal-system/🧪️tests/✅️applies",
    "delete-solar-thermal-system-refuses": "asset://🧬️schema/🧬️mutations/🌆️delete-solar-thermal-system/🧪️tests/⛔️refuses",
    "change-solar-thermal-system-collector-area-applies": "asset://🧬️schema/🧬️mutations/🟩️change-solar-thermal-system-collector-area/🧪️tests/✅️applies",
    "change-solar-thermal-system-collector-area-refuses": "asset://🧬️schema/🧬️mutations/🟩️change-solar-thermal-system-collector-area/🧪️tests/⛔️refuses",
    "change-solar-thermal-system-efficiency-applies": "asset://🧬️schema/🧬️mutations/🏅️change-solar-thermal-system-efficiency/🧪️tests/✅️applies",
    "change-solar-thermal-system-efficiency-refuses": "asset://🧬️schema/🧬️mutations/🏅️change-solar-thermal-system-efficiency/🧪️tests/⛔️refuses",
    "change-solar-thermal-system-storage-volume-applies": "asset://🧬️schema/🧬️mutations/🧃️change-solar-thermal-system-storage-volume/🧪️tests/✅️applies",
    "change-solar-thermal-system-storage-volume-refuses": "asset://🧬️schema/🧬️mutations/🧃️change-solar-thermal-system-storage-volume/🧪️tests/⛔️refuses",
    "change-solar-thermal-system-tilt-applies": "asset://🧬️schema/🧬️mutations/🔼️change-solar-thermal-system-tilt/🧪️tests/✅️applies",
    "change-solar-thermal-system-tilt-refuses": "asset://🧬️schema/🧬️mutations/🔼️change-solar-thermal-system-tilt/🧪️tests/⛔️refuses",
    "change-solar-thermal-system-azimuth-applies": "asset://🧬️schema/🧬️mutations/⛵️change-solar-thermal-system-azimuth/🧪️tests/✅️applies",
    "change-solar-thermal-system-azimuth-refuses": "asset://🧬️schema/🧬️mutations/⛵️change-solar-thermal-system-azimuth/🧪️tests/⛔️refuses",
    "create-refrigeration-system-applies": "asset://🧬️schema/🧬️mutations/❄️create-refrigeration-system/🧪️tests/✅️applies",
    "create-refrigeration-system-refuses": "asset://🧬️schema/🧬️mutations/❄️create-refrigeration-system/🧪️tests/⛔️refuses",
    "delete-refrigeration-system-applies": "asset://🧬️schema/🧬️mutations/🫠️delete-refrigeration-system/🧪️tests/✅️applies",
    "delete-refrigeration-system-refuses": "asset://🧬️schema/🧬️mutations/🫠️delete-refrigeration-system/🧪️tests/⛔️refuses",
    "change-refrigeration-system-case-count-applies": "asset://🧬️schema/🧬️mutations/🗄️change-refrigeration-system-case-count/🧪️tests/✅️applies",
    "change-refrigeration-system-case-count-refuses": "asset://🧬️schema/🧬️mutations/🗄️change-refrigeration-system-case-count/🧪️tests/⛔️refuses",
    "change-refrigeration-system-design-load-applies": "asset://🧬️schema/🧬️mutations/🏋️change-refrigeration-system-design-load/🧪️tests/✅️applies",
    "change-refrigeration-system-design-load-refuses": "asset://🧬️schema/🧬️mutations/🏋️change-refrigeration-system-design-load/🧪️tests/⛔️refuses",
    "change-refrigeration-system-defrost-schedule-applies": "asset://🧬️schema/🧬️mutations/🕑️change-refrigeration-system-defrost-schedule/🧪️tests/✅️applies",
    "change-refrigeration-system-defrost-schedule-refuses": "asset://🧬️schema/🧬️mutations/🕑️change-refrigeration-system-defrost-schedule/🧪️tests/⛔️refuses",
    "create-water-system-applies": "asset://🧬️schema/🧬️mutations/🚽️create-water-system/🧪️tests/✅️applies",
    "create-water-system-refuses": "asset://🧬️schema/🧬️mutations/🚽️create-water-system/🧪️tests/⛔️refuses",
    "delete-water-system-applies": "asset://🧬️schema/🧬️mutations/🧼️delete-water-system/🧪️tests/✅️applies",
    "delete-water-system-refuses": "asset://🧬️schema/🧬️mutations/🧼️delete-water-system/🧪️tests/⛔️refuses",
    "change-water-system-fixture-count-applies": "asset://🧬️schema/🧬️mutations/🪣️change-water-system-fixture-count/🧪️tests/✅️applies",
    "change-water-system-fixture-count-refuses": "asset://🧬️schema/🧬️mutations/🪣️change-water-system-fixture-count/🧪️tests/⛔️refuses",
    "change-water-system-peak-flow-applies": "asset://🧬️schema/🧬️mutations/🚾️change-water-system-peak-flow/🧪️tests/✅️applies",
    "change-water-system-peak-flow-refuses": "asset://🧬️schema/🧬️mutations/🚾️change-water-system-peak-flow/🧪️tests/⛔️refuses",
    "change-water-system-schedule-applies": "asset://🧬️schema/🧬️mutations/🕒️change-water-system-schedule/🧪️tests/✅️applies",
    "change-water-system-schedule-refuses": "asset://🧬️schema/🧬️mutations/🕒️change-water-system-schedule/🧪️tests/⛔️refuses",
    "create-fault-applies": "asset://🧬️schema/🧬️mutations/⚠️create-fault/🧪️tests/✅️applies",
    "create-fault-refuses": "asset://🧬️schema/🧬️mutations/⚠️create-fault/🧪️tests/⛔️refuses",
    "delete-fault-applies": "asset://🧬️schema/🧬️mutations/🩹️delete-fault/🧪️tests/✅️applies",
    "delete-fault-refuses": "asset://🧬️schema/🧬️mutations/🩹️delete-fault/🧪️tests/⛔️refuses",
    "change-fault-target-equipment-applies": "asset://🧬️schema/🧬️mutations/🎣️change-fault-target-equipment/🧪️tests/✅️applies",
    "change-fault-target-equipment-refuses": "asset://🧬️schema/🧬️mutations/🎣️change-fault-target-equipment/🧪️tests/⛔️refuses",
    "change-fault-type-applies": "asset://🧬️schema/🧬️mutations/🐛️change-fault-type/🧪️tests/✅️applies",
    "change-fault-type-refuses": "asset://🧬️schema/🧬️mutations/🐛️change-fault-type/🧪️tests/⛔️refuses",
    "change-fault-severity-applies": "asset://🧬️schema/🧬️mutations/🌶️change-fault-severity/🧪️tests/✅️applies",
    "change-fault-severity-refuses": "asset://🧬️schema/🧬️mutations/🌶️change-fault-severity/🧪️tests/⛔️refuses",
    "change-fault-start-schedule-applies": "asset://🧬️schema/🧬️mutations/🕓️change-fault-start-schedule/🧪️tests/✅️applies",
    "change-fault-start-schedule-refuses": "asset://🧬️schema/🧬️mutations/🕓️change-fault-start-schedule/🧪️tests/⛔️refuses",
    "create-space-list-applies": "asset://🧬️schema/🧬️mutations/📋️create-space-list/🧪️tests/✅️applies",
    "create-space-list-refuses": "asset://🧬️schema/🧬️mutations/📋️create-space-list/🧪️tests/⛔️refuses",
    "delete-space-list-applies": "asset://🧬️schema/🧬️mutations/🗒️delete-space-list/🧪️tests/✅️applies",
    "delete-space-list-refuses": "asset://🧬️schema/🧬️mutations/🗒️delete-space-list/🧪️tests/⛔️refuses",
    "rename-space-list-applies": "asset://🧬️schema/🧬️mutations/🪶️rename-space-list/🧪️tests/✅️applies",
    "rename-space-list-refuses": "asset://🧬️schema/🧬️mutations/🪶️rename-space-list/🧪️tests/⛔️refuses",
    "add-space-list-member-applies": "asset://🧬️schema/🧬️mutations/➡️add-space-list-member/🧪️tests/✅️applies",
    "add-space-list-member-refuses": "asset://🧬️schema/🧬️mutations/➡️add-space-list-member/🧪️tests/⛔️refuses",
    "remove-space-list-member-applies": "asset://🧬️schema/🧬️mutations/🪤️remove-space-list-member/🧪️tests/✅️applies",
    "remove-space-list-member-refuses": "asset://🧬️schema/🧬️mutations/🪤️remove-space-list-member/🧪️tests/⛔️refuses",
    "create-thermal-enclosure-applies": "asset://🧬️schema/🧬️mutations/🏟️create-thermal-enclosure/🧪️tests/✅️applies",
    "create-thermal-enclosure-refuses": "asset://🧬️schema/🧬️mutations/🏟️create-thermal-enclosure/🧪️tests/⛔️refuses",
    "delete-thermal-enclosure-applies": "asset://🧬️schema/🧬️mutations/🏯️delete-thermal-enclosure/🧪️tests/✅️applies",
    "delete-thermal-enclosure-refuses": "asset://🧬️schema/🧬️mutations/🏯️delete-thermal-enclosure/🧪️tests/⛔️refuses",
    "rename-thermal-enclosure-applies": "asset://🧬️schema/🧬️mutations/🖋️rename-thermal-enclosure/🧪️tests/✅️applies",
    "rename-thermal-enclosure-refuses": "asset://🧬️schema/🧬️mutations/🖋️rename-thermal-enclosure/🧪️tests/⛔️refuses",
    "add-thermal-enclosure-zone-applies": "asset://🧬️schema/🧬️mutations/🔒️add-thermal-enclosure-zone/🧪️tests/✅️applies",
    "add-thermal-enclosure-zone-refuses": "asset://🧬️schema/🧬️mutations/🔒️add-thermal-enclosure-zone/🧪️tests/⛔️refuses",
    "remove-thermal-enclosure-zone-applies": "asset://🧬️schema/🧬️mutations/🔓️remove-thermal-enclosure-zone/🧪️tests/✅️applies",
    "remove-thermal-enclosure-zone-refuses": "asset://🧬️schema/🧬️mutations/🔓️remove-thermal-enclosure-zone/🧪️tests/⛔️refuses",
    "create-constant-schedule-applies": "asset://🧬️schema/🧬️mutations/🕜️create-constant-schedule/🧪️tests/✅️applies",
    "create-constant-schedule-refuses": "asset://🧬️schema/🧬️mutations/🕜️create-constant-schedule/🧪️tests/⛔️refuses",
    "delete-constant-schedule-applies": "asset://🧬️schema/🧬️mutations/📍️delete-constant-schedule/🧪️tests/✅️applies",
    "delete-constant-schedule-refuses": "asset://🧬️schema/🧬️mutations/📍️delete-constant-schedule/🧪️tests/⛔️refuses",
    "change-constant-schedule-value-applies": "asset://🧬️schema/🧬️mutations/🕝️change-constant-schedule-value/🧪️tests/✅️applies",
    "change-constant-schedule-value-refuses": "asset://🧬️schema/🧬️mutations/🕝️change-constant-schedule-value/🧪️tests/⛔️refuses",
    "create-daily-schedule-applies": "asset://🧬️schema/🧬️mutations/🕞️create-daily-schedule/🧪️tests/✅️applies",
    "create-daily-schedule-refuses": "asset://🧬️schema/🧬️mutations/🕞️create-daily-schedule/🧪️tests/⛔️refuses",
    "delete-daily-schedule-applies": "asset://🧬️schema/🧬️mutations/🌓️delete-daily-schedule/🧪️tests/✅️applies",
    "delete-daily-schedule-refuses": "asset://🧬️schema/🧬️mutations/🌓️delete-daily-schedule/🧪️tests/⛔️refuses",
    "replace-daily-schedule-hourly-values-applies": "asset://🧬️schema/🧬️mutations/🕔️replace-daily-schedule-hourly-values/🧪️tests/✅️applies",
    "replace-daily-schedule-hourly-values-refuses": "asset://🧬️schema/🧬️mutations/🕔️replace-daily-schedule-hourly-values/🧪️tests/⛔️refuses",
    "change-daily-schedule-interpolation-applies": "asset://🧬️schema/🧬️mutations/🕕️change-daily-schedule-interpolation/🧪️tests/✅️applies",
    "change-daily-schedule-interpolation-refuses": "asset://🧬️schema/🧬️mutations/🕕️change-daily-schedule-interpolation/🧪️tests/⛔️refuses",
    "change-daily-schedule-limits-applies": "asset://🧬️schema/🧬️mutations/🕟️change-daily-schedule-limits/🧪️tests/✅️applies",
    "change-daily-schedule-limits-refuses": "asset://🧬️schema/🧬️mutations/🕟️change-daily-schedule-limits/🧪️tests/⛔️refuses",
    "create-weekly-schedule-applies": "asset://🧬️schema/🧬️mutations/🗓️create-weekly-schedule/🧪️tests/✅️applies",
    "create-weekly-schedule-refuses": "asset://🧬️schema/🧬️mutations/🗓️create-weekly-schedule/🧪️tests/⛔️refuses",
    "delete-weekly-schedule-applies": "asset://🧬️schema/🧬️mutations/🕖️delete-weekly-schedule/🧪️tests/✅️applies",
    "delete-weekly-schedule-refuses": "asset://🧬️schema/🧬️mutations/🕖️delete-weekly-schedule/🧪️tests/⛔️refuses",
    "change-weekly-schedule-day-applies": "asset://🧬️schema/🧬️mutations/🕗️change-weekly-schedule-day/🧪️tests/✅️applies",
    "change-weekly-schedule-day-refuses": "asset://🧬️schema/🧬️mutations/🕗️change-weekly-schedule-day/🧪️tests/⛔️refuses",
    "create-annual-schedule-applies": "asset://🧬️schema/🧬️mutations/📚️create-annual-schedule/🧪️tests/✅️applies",
    "create-annual-schedule-refuses": "asset://🧬️schema/🧬️mutations/📚️create-annual-schedule/🧪️tests/⛔️refuses",
    "delete-annual-schedule-applies": "asset://🧬️schema/🧬️mutations/📕️delete-annual-schedule/🧪️tests/✅️applies",
    "delete-annual-schedule-refuses": "asset://🧬️schema/🧬️mutations/📕️delete-annual-schedule/🧪️tests/⛔️refuses",
    "insert-annual-schedule-rule-applies": "asset://🧬️schema/🧬️mutations/📗️insert-annual-schedule-rule/🧪️tests/✅️applies",
    "insert-annual-schedule-rule-refuses": "asset://🧬️schema/🧬️mutations/📗️insert-annual-schedule-rule/🧪️tests/⛔️refuses",
    "remove-annual-schedule-rule-applies": "asset://🧬️schema/🧬️mutations/📙️remove-annual-schedule-rule/🧪️tests/✅️applies",
    "remove-annual-schedule-rule-refuses": "asset://🧬️schema/🧬️mutations/📙️remove-annual-schedule-rule/🧪️tests/⛔️refuses",
    "reorder-annual-schedule-rules-applies": "asset://🧬️schema/🧬️mutations/🗂️reorder-annual-schedule-rules/🧪️tests/✅️applies",
    "reorder-annual-schedule-rules-refuses": "asset://🧬️schema/🧬️mutations/🗂️reorder-annual-schedule-rules/🧪️tests/⛔️refuses",
    "change-annual-schedule-default-daily-schedule-applies": "asset://🧬️schema/🧬️mutations/🎌️change-annual-schedule-default-daily-schedule/🧪️tests/✅️applies",
    "change-annual-schedule-default-daily-schedule-refuses": "asset://🧬️schema/🧬️mutations/🎌️change-annual-schedule-default-daily-schedule/🧪️tests/⛔️refuses",
    "change-annual-schedule-holiday-daily-schedule-applies": "asset://🧬️schema/🧬️mutations/🎄️change-annual-schedule-holiday-daily-schedule/🧪️tests/✅️applies",
    "change-annual-schedule-holiday-daily-schedule-refuses": "asset://🧬️schema/🧬️mutations/🎄️change-annual-schedule-holiday-daily-schedule/🧪️tests/⛔️refuses",
    "add-annual-schedule-holiday-applies": "asset://🧬️schema/🧬️mutations/🎉️add-annual-schedule-holiday/🧪️tests/✅️applies",
    "add-annual-schedule-holiday-refuses": "asset://🧬️schema/🧬️mutations/🎉️add-annual-schedule-holiday/🧪️tests/⛔️refuses",
    "remove-annual-schedule-holiday-applies": "asset://🧬️schema/🧬️mutations/🎊️remove-annual-schedule-holiday/🧪️tests/✅️applies",
    "remove-annual-schedule-holiday-refuses": "asset://🧬️schema/🧬️mutations/🎊️remove-annual-schedule-holiday/🧪️tests/⛔️refuses",
    "create-time-series-schedule-applies": "asset://🧬️schema/🧬️mutations/🪗️create-time-series-schedule/🧪️tests/✅️applies",
    "create-time-series-schedule-refuses": "asset://🧬️schema/🧬️mutations/🪗️create-time-series-schedule/🧪️tests/⛔️refuses",
    "delete-time-series-schedule-applies": "asset://🧬️schema/🧬️mutations/🎞️delete-time-series-schedule/🧪️tests/✅️applies",
    "delete-time-series-schedule-refuses": "asset://🧬️schema/🧬️mutations/🎞️delete-time-series-schedule/🧪️tests/⛔️refuses",
    "replace-time-series-schedule-values-applies": "asset://🧬️schema/🧬️mutations/🕘️replace-time-series-schedule-values/🧪️tests/✅️applies",
    "replace-time-series-schedule-values-refuses": "asset://🧬️schema/🧬️mutations/🕘️replace-time-series-schedule-values/🧪️tests/⛔️refuses",
    "change-time-series-schedule-timestep-applies": "asset://🧬️schema/🧬️mutations/🕙️change-time-series-schedule-timestep/🧪️tests/✅️applies",
    "change-time-series-schedule-timestep-refuses": "asset://🧬️schema/🧬️mutations/🕙️change-time-series-schedule-timestep/🧪️tests/⛔️refuses",
}


def _read_json(ctx: Context, uri: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(uri))


def _vector(ctx: Context, scenario: str):
    root = VECTOR_ROOTS[scenario]
    return (
        _read_json(ctx, f"{root}/📸️snapshot/⬅️before/🔣️.json"),
        _read_json(ctx, f"{root}/🦠️mutation/🔣️.json"),
        _read_json(ctx, f"{root}/📸️snapshot/➡️after/🔣️.json"),
        _read_json(ctx, f"{root}/🎯️outcome/🔣️.json"),
    )
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 Splits the committed mutation document into its kind tag and its argument object. The wire
    tag is the lowerCamel spelling of the Rust variant (`renameModel`), not the kebab catalog id."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


def wire_tag(kind: str) -> str:
    """🔤 The catalog id (kebab) a wire tag maps to, read off the committed manifest's
    `productionDispatch`, never transliterated from a Rust variant name."""
    head, *rest = kind.split("-")
    return head + "".join(part.capitalize() for part in rest)
# endregion 🔖️Wire


# region 🔖️Outcomes
def applied(*messages):
    """🎯️ An applied outcome and its ordered diagnostics."""
    return {"status": "applied", "messages": [{"level": level, "code": code} for level, code in messages]}


def rejected(code, path):
    """⛔️ A refusal: one fault code and the offending address."""
    return {"status": "rejected", "code": code, "path": list(path)}


def unchanged(before):
    """🪞 A refused or no-op step leaves the document exactly where it was."""
    return copy.deepcopy(before)
# endregion 🔖️Outcomes


# region 🔖️Links
def parse_uri(uri):
    """🔗️ `<artifactId>!<artifactKind>@<standard>/<subset>` — the flattened `ArtifactRef` form the
    snapshot schema's link slots carry. Anything else is not a reference."""
    if not isinstance(uri, str) or "!" not in uri:
        return None
    artifact_id, rest = uri.split("!", 1)
    if "@" not in rest or "/" not in rest:
        return None
    artifact_kind, rest = rest.split("@", 1)
    standard, subset = rest.split("/", 1)
    if not artifact_id or not artifact_kind or not standard or not subset or "/" in subset:
        return None
    return {"artifactId": artifact_id, "dialect": {"artifactKind": artifact_kind, "standard": standard, "subset": subset}}


def head_link(target, role):
    """🔗️ A head-pinned link filling one named slot."""
    return {"target": target, "pin": {"kind": "head"}, "role": role}
# endregion 🔖️Links


# region 🔖️Vocabulary
def rename_model(before, payload):
    """🏷️ `rename-model{newName}` — taxonomy.md's `rename` verb on the document identity field."""
    name = payload["newName"]
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [name])
    if before["model"]["name"] == name:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["name"] = name
    return after, applied()


def change_model_version(before, payload):
    """🔢️ `change-model-version{newVersion}` — one scalar field."""
    version = payload["newVersion"]
    if not version.strip():
        return unchanged(before), rejected("mutation.invariant", [version])
    if before["model"]["version"] == version:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["version"] = version
    return after, applied()


def update_site(before, payload):
    """🌍️ `update-site` — one inseparable five-field facet, all fields required every time."""
    site = {
        "latitude_deg": payload["latitudeDeg"],
        "longitude_deg": payload["longitudeDeg"],
        "elevation_m": payload["elevationM"],
        "time_zone_hours": payload["timeZoneHours"],
        "north_axis_deg": payload["northAxisDeg"],
    }
    if not -90.0 <= site["latitude_deg"] <= 90.0 or not -180.0 <= site["longitude_deg"] <= 180.0 or not -12.0 <= site["time_zone_hours"] <= 14.0:
        return unchanged(before), rejected("mutation.invariant", [])
    if before["model"]["site"] == site:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["site"] = site
    return after, applied()


def update_ground_temperature(before, payload):
    """🌡️ `update-ground-temperature` — twelve monthly values per series, read together."""
    building = payload["buildingSurfaceC"]
    shallow = payload["shallowC"]
    if len(building) != 12 or len(shallow) != 12:
        return unchanged(before), rejected("mutation.invariant", [])
    ground = {"building_surface_c": building, "shallow_c": shallow, "deep_c": payload["deepC"]}
    if before["model"]["ground_temperature"] == ground:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["ground_temperature"] = ground
    return after, applied()


def update_run_period(before, payload):
    """📅️ `update-run-period` — start and end are one calendar interval."""
    run_period = {
        "start_month": payload["startMonth"],
        "start_day": payload["startDay"],
        "end_month": payload["endMonth"],
        "end_day": payload["endDay"],
        "year": payload["year"],
    }
    months_ok = 1 <= run_period["start_month"] <= 12 and 1 <= run_period["end_month"] <= 12
    days_ok = 1 <= run_period["start_day"] <= 31 and 1 <= run_period["end_day"] <= 31
    if not months_ok or not days_ok:
        return unchanged(before), rejected("mutation.invariant", [])
    if before["model"]["run_period"] == run_period:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["run_period"] = run_period
    return after, applied()


def replace_airflow_network(before, payload):
    """🫧️ `replace-airflow-network` — a whole-value swap of the document-root singleton; `present`
    false detaches it, so one kind covers attach and detach (taxonomy.md's `replace` verb)."""
    zone_ids = payload["zoneIds"]
    node_ids = payload["nodeIds"]
    link_ids = payload["linkIds"]
    if len(zone_ids) != len(node_ids):
        return unchanged(before), rejected("mutation.invariant", [])
    if not payload["present"] and (zone_ids or link_ids):
        return unchanged(before), rejected("mutation.invariant", [])
    network = None
    if payload["present"]:
        network = {"zone_node_ids": [[zone, node] for zone, node in zip(zone_ids, node_ids)], "outdoor_node_id": payload["outdoorNodeId"], "link_ids": link_ids}
    if before["model"]["airflow_network"] == network:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["airflow_network"] = network
    return after, applied()


def add_output_variable(before, payload):
    """📊️ `add-output-variable` — set-like membership keyed by the natural `(name, key)` pair."""
    name, key = payload["name"], payload["key"]
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [key])
    if any(spec["name"] == name and spec["key"] == key for spec in before["model"]["output_variables"]):
        return unchanged(before), rejected("mutation.duplicate-id", [name, key])
    after = copy.deepcopy(before)
    after["model"]["output_variables"].append({"name": name, "key": key, "reporting_frequency": payload["reportingFrequency"]})
    return after, applied()


def remove_output_variable(before, payload):
    """📉️ `remove-output-variable` — the `add` verb's inverse partner."""
    name, key = payload["name"], payload["key"]
    if not any(spec["name"] == name and spec["key"] == key for spec in before["model"]["output_variables"]):
        return unchanged(before), rejected("mutation.target-missing", [name, key])
    after = copy.deepcopy(before)
    after["model"]["output_variables"] = [spec for spec in after["model"]["output_variables"] if not (spec["name"] == name and spec["key"] == key)]
    return after, applied()


def bind_weather_file(before, payload):
    """🌦️ `bind-weather-file{targetUri}` — taxonomy.md's `bind` verb attaching a parameterization."""
    target = parse_uri(payload["targetUri"])
    if target is None:
        return unchanged(before), rejected("mutation.invariant", [payload["targetUri"]])
    link = head_link(target, "weather")
    if before.get("weatherLink") == link:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["weatherLink"] = link
    return after, applied()


def unbind_weather_file(before, payload):
    """🌤️ `unbind-weather-file` — `bind`'s inverse partner; refused when nothing is bound."""
    del payload
    if not before.get("weatherLink"):
        return unchanged(before), rejected("mutation.target-missing", [])
    after = copy.deepcopy(before)
    after["weatherLink"] = None
    return after, applied()


def connect_referenced_model(before, payload):
    """🪢️ `connect-referenced-model{targetUri}` — taxonomy.md's `connect` verb on a relationship."""
    target = parse_uri(payload["targetUri"])
    if target is None:
        return unchanged(before), rejected("mutation.invariant", [payload["targetUri"]])
    link = head_link(target, "model")
    if before.get("referencedModel") == link:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["referencedModel"] = link
    return after, applied()


def disconnect_referenced_model(before, payload):
    """✂️ `disconnect-referenced-model` — `connect`'s inverse partner."""
    del payload
    if not before.get("referencedModel"):
        return unchanged(before), rejected("mutation.target-missing", [])
    after = copy.deepcopy(before)
    after["referencedModel"] = None
    return after, applied()


def _zone(before, entity_id):
    for zone in before["model"]["zones"]:
        if zone["id"] == entity_id:
            return zone
    return None


def _with_zone(before, entity_id, field, value):
    after = copy.deepcopy(before)
    for zone in after["model"]["zones"]:
        if zone["id"] == entity_id:
            zone[field] = value
    return after


def rename_zone(before, payload):
    """🏠️ `rename-zone{id,newName}` — id-keyed identity field; a duplicate name is refused because
    every report keys on it."""
    entity_id, name = payload["id"], payload["newName"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(other["id"] != entity_id and other["name"] == name for other in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if zone["name"] == name:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "name", name), applied()


def change_zone_volume(before, payload):
    """📦️ `change-zone-volume{id,newVolumeM3}` — the zone air capacitance."""
    entity_id, volume = payload["id"], payload["newVolumeM3"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if volume != volume or volume in (float("inf"), float("-inf")) or volume <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if zone["volume_m3"] == volume:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "volume_m3", volume), applied()


def change_zone_multiplier(before, payload):
    """✖️ `change-zone-multiplier{id,newMultiplier}` — identical zone instances."""
    entity_id, multiplier = payload["id"], payload["newMultiplier"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if multiplier == 0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if zone["multiplier"] == multiplier:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "multiplier", multiplier), applied()


def change_zone_conditioned(before, payload):
    """🌬️ `change-zone-conditioned{id,newConditioned}` — whether equipment serves the zone."""
    entity_id, conditioned = payload["id"], payload["newConditioned"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if zone["conditioned"] == conditioned:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "conditioned", conditioned), applied()


def change_zone_floor_area_participation(before, payload):
    """📐️ `change-zone-floor-area-participation{id,newPartOfTotalFloorArea}` — whether the zone's
    floor area counts toward the building total the normalized reports divide by."""
    entity_id, participates = payload["id"], payload["newPartOfTotalFloorArea"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if zone["part_of_total_floor_area"] == participates:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "part_of_total_floor_area", participates), applied()


def create_zone(before, payload):
    """🏘️ `create-zone` — Adds one thermal zone the caller has already chosen an id for — no mutation mints an id, so an undo can name the same zone again."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["name"] == payload["name"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not (payload["volumeM3"] == payload["volumeM3"] and abs(payload["volumeM3"]) != float("inf") and payload["volumeM3"] > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if payload["multiplier"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "volume_m3": payload["volumeM3"], "multiplier": payload["multiplier"], "conditioned": payload["conditioned"], "part_of_total_floor_area": payload["partOfTotalFloorArea"]}
    rows = after["model"]["zones"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    rows.insert(position, created)
    return after, applied()


def _invert_create_zone(before, payload):
    return [("delete-zone", {"id": payload["id"]})]


def delete_zone(before, payload):
    """🏚️ `delete-zone{id}` — Removes one thermal zone. It RESTRICTS rather than cascades: while any space, surface, gain, HVAC object, grouping or airflow node still names the zone it is refused with `mutation.invariant`, because a cascade here would have to delete surfaces, which cascade again to fenestrations and adjacency pairs."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["zones"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if any(row["zone_id"] == entity_id for name in ("spaces", "surfaces", "people", "lighting", "equipment", "thermostats", "humidistats", "ideal_loads", "zone_equipment", "infiltrations", "mechanical_ventilations", "sizing_objects", "daylight_zones", "room_air_models") for row in before["model"][name]) or any(entity_id in row["zone_ids"] for row in before["model"]["thermal_enclosures"]) or any(entity_id in row["terminal_zone_ids"] for row in before["model"]["air_loops"]) or (before["model"]["airflow_network"] is not None and any(pair[0] == entity_id for pair in before["model"]["airflow_network"]["zone_node_ids"])):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    after = copy.deepcopy(before)
    after["model"]["zones"] = [row for row in after["model"]["zones"] if row["id"] != entity_id]
    return after, applied()


def _invert_delete_zone(before, payload):
    item = next(row for row in before["model"]["zones"] if row["id"] == payload["id"])
    return [("create-zone", {"id": item["id"], "name": item["name"], "volumeM3": item["volume_m3"], "multiplier": item["multiplier"], "conditioned": item["conditioned"], "partOfTotalFloorArea": item["part_of_total_floor_area"]})]


def create_space(before, payload):
    """🪑️ `create-space` — Adds one space inside an existing zone. The owning zone must already exist, so the document never carries a space whose zone the reports cannot resolve."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["spaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (payload["floorAreaM2"] == payload["floorAreaM2"] and abs(payload["floorAreaM2"]) != float("inf") and payload["floorAreaM2"] >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "zone_id": payload["zoneId"], "floor_area_m2": payload["floorAreaM2"]}
    rows = after["model"]["spaces"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    rows.insert(position, created)
    return after, applied()


def _invert_create_space(before, payload):
    return [("delete-space", {"id": payload["id"]})]


def delete_space(before, payload):
    """🧹️ `delete-space{id}` — Removes one space. Refused while a space list still names it, so no grouping is left pointing at nothing."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["spaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if any(payload["id"] in row["space_ids"] for row in before["model"]["space_lists"]):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    after = copy.deepcopy(before)
    after["model"]["spaces"] = [row for row in after["model"]["spaces"] if row["id"] != entity_id]
    return after, applied()


def _invert_delete_space(before, payload):
    item = next(row for row in before["model"]["spaces"] if row["id"] == payload["id"])
    return [("create-space", {"id": item["id"], "name": item["name"], "zoneId": item["zone_id"], "floorAreaM2": item["floor_area_m2"]})]


def rename_space(before, payload):
    """🔤️ `rename-space{id,newName}` — Sets one space's identity field; a name another space already holds is refused."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["spaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["id"] != entity_id and row["name"] == value for row in before["model"]["spaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["spaces"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_space(before, payload):
    item = next(row for row in before["model"]["spaces"] if row["id"] == payload["id"])
    return [("rename-space", {"id": payload["id"], "newName": item["name"]})]


def change_space_floor_area(before, payload):
    """🧮️ `change-space-floor-area{id,newFloorAreaM2}` — Sets one space's floor area in square metres — the denominator every per-area gain in that space is expanded against."""
    entity_id, value = payload["id"], payload["newFloorAreaM2"]
    item = next((row for row in before["model"]["spaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["floor_area_m2"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["spaces"]:
        if row["id"] == entity_id:
            row["floor_area_m2"] = value
    return after, applied()


def _invert_change_space_floor_area(before, payload):
    item = next(row for row in before["model"]["spaces"] if row["id"] == payload["id"])
    return [("change-space-floor-area", {"id": payload["id"], "newFloorAreaM2": item["floor_area_m2"]})]


def change_space_zone(before, payload):
    """🚚️ `change-space-zone{id,newZoneId}` — Reassigns one space to another existing zone. `Space::zone_id` is a foreign key between two flat collections, not a recursive parent field, so this is the `change` verb rather than a hierarchy move."""
    entity_id, value = payload["id"], payload["newZoneId"]
    item = next((row for row in before["model"]["spaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["zone_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["spaces"]:
        if row["id"] == entity_id:
            row["zone_id"] = value
    return after, applied()


def _invert_change_space_zone(before, payload):
    item = next(row for row in before["model"]["spaces"] if row["id"] == payload["id"])
    return [("change-space-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def create_surface(before, payload):
    """🟫️ `create-surface` — Adds one planar polygon surface to an existing zone with an existing construction. The exterior boundary arrives as its two halves — a `boundary` discriminator and the `interzoneSurfaceId` only the `Interzone` arm carries — because `dsl::DslScalar` binds unit variants only."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == payload["constructionId"] for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if len(payload["verticesM"]) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if (payload["interzoneSurfaceId"] is not None and (payload["interzoneSurfaceId"] == entity_id or not any(row["id"] == payload["interzoneSurfaceId"] for row in before["model"]["surfaces"]))):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if payload["multiplier"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if (payload["boundary"] == "Interzone") != (payload["interzoneSurfaceId"] is not None):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    boundary = {"Interzone": payload["interzoneSurfaceId"]} if payload["interzoneSurfaceId"] is not None else payload["boundary"]
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "zone_id": payload["zoneId"], "class": payload["class"], "vertices_m": payload["verticesM"], "construction_id": payload["constructionId"], "outside_boundary_condition": boundary, "sun_exposed": payload["sunExposed"], "wind_exposed": payload["windExposed"], "multiplier": payload["multiplier"]}
    rows = after["model"]["surfaces"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    rows.insert(position, created)
    return after, applied()


def _invert_create_surface(before, payload):
    return [("delete-surface", {"id": payload["id"]})]


def delete_surface(before, payload):
    """🪚️ `delete-surface{id}` — the one cascading delete in this vocabulary: fenestrations and
    adjacency pairs go with their host surface, an interzone partner blocks it."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if any(row["outside_boundary_condition"] == {"Interzone": entity_id} for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    windows = [row for row in before["model"]["fenestrations"] if row["surface_id"] == entity_id]
    pairs = [row for row in before["model"]["adjacency_pairs"] if entity_id in (row["surface_a_id"], row["surface_b_id"])]
    after = copy.deepcopy(before)
    after["model"]["surfaces"] = [row for row in after["model"]["surfaces"] if row["id"] != entity_id]
    after["model"]["fenestrations"] = [row for row in after["model"]["fenestrations"] if row["surface_id"] != entity_id]
    after["model"]["adjacency_pairs"] = [row for row in after["model"]["adjacency_pairs"] if entity_id not in (row["surface_a_id"], row["surface_b_id"])]
    if not windows and not pairs:
        return after, applied()
    return after, applied(("info", "mutation.cascade"))


def _invert_delete_surface(before, payload):
    entity_id = payload["id"]
    item = next(row for row in before["model"]["surfaces"] if row["id"] == entity_id)
    boundary = item["outside_boundary_condition"]
    partner = boundary["Interzone"] if isinstance(boundary, dict) else None
    steps = [("create-surface", {"id": item["id"], "name": item["name"], "zoneId": item["zone_id"], "class": item["class"], "verticesM": item["vertices_m"], "constructionId": item["construction_id"], "boundary": "Interzone" if partner is not None else boundary, "interzoneSurfaceId": partner, "sunExposed": item["sun_exposed"], "windExposed": item["wind_exposed"], "multiplier": item["multiplier"]})]
    for window in before["model"]["fenestrations"]:
        if window["surface_id"] == entity_id:
            steps.append(("create-fenestration", {"id": window["id"], "name": window["name"], "surfaceId": window["surface_id"], "uValueWM2k": window["u_value_w_m2k"], "shgc": window["shgc"], "vlt": window["vlt"], "areaM2": window["area_m2"], "heightM": window["height_m"], "sillHeightM": window["sill_height_m"], "frameConductanceWK": window["frame_conductance_w_k"], "dividerConductanceWK": window["divider_conductance_w_k"], "overhangDepthM": window["overhang_depth_m"], "overhangOffsetM": window["overhang_offset_m"], "finDepthM": window["fin_depth_m"], "finOffsetM": window["fin_offset_m"], "glazingConstructionId": window["glazing_construction_id"]}))
    for pair in before["model"]["adjacency_pairs"]:
        if entity_id in (pair["surface_a_id"], pair["surface_b_id"]):
            steps.append(("connect-surfaces", {"surfaceAId": pair["surface_a_id"], "surfaceBId": pair["surface_b_id"]}))
    return steps


def rename_surface(before, payload):
    """🏳️ `rename-surface{id,newName}` — Sets one surface's identity field; a name another surface already holds is refused because every surface-level report keys on it."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["id"] != entity_id and row["name"] == value for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_surface(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("rename-surface", {"id": payload["id"], "newName": item["name"]})]


def change_surface_zone(before, payload):
    """🗜️ `change-surface-zone{id,newZoneId}` — Reassigns one surface to another existing zone — the zone whose air heat balance the surface's inside face exchanges with."""
    entity_id, value = payload["id"], payload["newZoneId"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["zone_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["zone_id"] = value
    return after, applied()


def _invert_change_surface_zone(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_surface_class(before, payload):
    """🧩️ `change-surface-class{id,newClass}` — Sets which of the eight boundary roles the surface plays — the role the kernel reads to decide whether it sees sky, ground or another zone."""
    entity_id, value = payload["id"], payload["newClass"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["class"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["class"] = value
    return after, applied()


def _invert_change_surface_class(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-class", {"id": payload["id"], "newClass": item["class"]})]


def replace_surface_vertices(before, payload):
    """🔺️ `replace-surface-vertices{id,newVerticesM}` — Swaps the surface's whole polygon. Geometry is one value — moving a single corner would leave the other vertices describing a different plane — so the taxonomy's `replace` verb carries the full ring, minimum three vertices."""
    entity_id, value = payload["id"], payload["newVerticesM"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if len(value) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["vertices_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["vertices_m"] = value
    return after, applied()


def _invert_replace_surface_vertices(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("replace-surface-vertices", {"id": payload["id"], "newVerticesM": item["vertices_m"]})]


def change_surface_construction(before, payload):
    """🧰️ `change-surface-construction{id,newConstructionId}` — Points the surface at another existing layered construction — the layer stack the conduction transfer functions are derived from."""
    entity_id, value = payload["id"], payload["newConstructionId"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == value for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["construction_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["construction_id"] = value
    return after, applied()


def _invert_change_surface_construction(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-construction", {"id": payload["id"], "newConstructionId": item["construction_id"]})]


def change_surface_boundary_condition(before, payload):
    """🚧️ `change-surface-boundary-condition{id,newBoundary,newInterzoneSurfaceId}` — the tagged
    union's two halves, admitted only when they agree."""
    entity_id, tag, partner = payload["id"], payload["newBoundary"], payload["newInterzoneSurfaceId"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if (tag == "Interzone") != (partner is not None):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if partner is not None and (partner == entity_id or not any(row["id"] == partner for row in before["model"]["surfaces"])):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    boundary = {"Interzone": partner} if partner is not None else tag
    if item["outside_boundary_condition"] == boundary:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["outside_boundary_condition"] = boundary
    return after, applied()


def _invert_change_surface_boundary_condition(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    boundary = item["outside_boundary_condition"]
    partner = boundary["Interzone"] if isinstance(boundary, dict) else None
    return [("change-surface-boundary-condition", {"id": payload["id"], "newBoundary": "Interzone" if partner is not None else boundary, "newInterzoneSurfaceId": partner})]


def change_surface_sun_exposed(before, payload):
    """🌅️ `change-surface-sun-exposed{id,newSunExposed}` — Sets whether the outside face receives beam and diffuse solar at all."""
    entity_id, value = payload["id"], payload["newSunExposed"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["sun_exposed"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["sun_exposed"] = value
    return after, applied()


def _invert_change_surface_sun_exposed(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-sun-exposed", {"id": payload["id"], "newSunExposed": item["sun_exposed"]})]


def change_surface_wind_exposed(before, payload):
    """🍃️ `change-surface-wind-exposed{id,newWindExposed}` — Sets whether the outside face uses the wind-driven exterior convection correlation or the sheltered one."""
    entity_id, value = payload["id"], payload["newWindExposed"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["wind_exposed"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["wind_exposed"] = value
    return after, applied()


def _invert_change_surface_wind_exposed(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-wind-exposed", {"id": payload["id"], "newWindExposed": item["wind_exposed"]})]


def change_surface_multiplier(before, payload):
    """🔁️ `change-surface-multiplier{id,newMultiplier}` — Sets how many identical instances of the surface the heat balance is scaled by."""
    entity_id, value = payload["id"], payload["newMultiplier"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["multiplier"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["multiplier"] = value
    return after, applied()


def _invert_change_surface_multiplier(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-multiplier", {"id": payload["id"], "newMultiplier": item["multiplier"]})]


def create_fenestration(before, payload):
    """🪟️ `create-fenestration` — Adds one window, skylight or door to an existing host surface, with the full optics, geometry and attached-shading payload the entity carries — including the optional `glazingConstructionId` that supersedes the three scalar optics fields when it is set."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["fenestrations"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not any(row["id"] == payload["surfaceId"] for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if (payload["glazingConstructionId"] is not None and not any(row["id"] == payload["glazingConstructionId"] for row in before["model"]["constructions"])):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (payload["uValueWM2k"] == payload["uValueWM2k"] and abs(payload["uValueWM2k"]) != float("inf") and payload["uValueWM2k"] > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["shgc"] == payload["shgc"] and 0.0 <= payload["shgc"] <= 1.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["vlt"] == payload["vlt"] and 0.0 <= payload["vlt"] <= 1.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["areaM2"] == payload["areaM2"] and abs(payload["areaM2"]) != float("inf") and payload["areaM2"] > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["heightM"] == payload["heightM"] and abs(payload["heightM"]) != float("inf") and payload["heightM"] > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["sillHeightM"] == payload["sillHeightM"] and abs(payload["sillHeightM"]) != float("inf") and payload["sillHeightM"] >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "surface_id": payload["surfaceId"], "u_value_w_m2k": payload["uValueWM2k"], "shgc": payload["shgc"], "vlt": payload["vlt"], "area_m2": payload["areaM2"], "height_m": payload["heightM"], "sill_height_m": payload["sillHeightM"], "frame_conductance_w_k": payload["frameConductanceWK"], "divider_conductance_w_k": payload["dividerConductanceWK"], "overhang_depth_m": payload["overhangDepthM"], "overhang_offset_m": payload["overhangOffsetM"], "fin_depth_m": payload["finDepthM"], "fin_offset_m": payload["finOffsetM"], "glazing_construction_id": payload["glazingConstructionId"]}
    rows = after["model"]["fenestrations"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    rows.insert(position, created)
    return after, applied()


def _invert_create_fenestration(before, payload):
    return [("delete-fenestration", {"id": payload["id"]})]


def delete_fenestration(before, payload):
    """🚪️ `delete-fenestration{id}` — Removes one fenestration. Nothing in the document references a fenestration, so it neither cascades nor restricts."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    after = copy.deepcopy(before)
    after["model"]["fenestrations"] = [row for row in after["model"]["fenestrations"] if row["id"] != entity_id]
    return after, applied()


def _invert_delete_fenestration(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("create-fenestration", {"id": item["id"], "name": item["name"], "surfaceId": item["surface_id"], "uValueWM2k": item["u_value_w_m2k"], "shgc": item["shgc"], "vlt": item["vlt"], "areaM2": item["area_m2"], "heightM": item["height_m"], "sillHeightM": item["sill_height_m"], "frameConductanceWK": item["frame_conductance_w_k"], "dividerConductanceWK": item["divider_conductance_w_k"], "overhangDepthM": item["overhang_depth_m"], "overhangOffsetM": item["overhang_offset_m"], "finDepthM": item["fin_depth_m"], "finOffsetM": item["fin_offset_m"], "glazingConstructionId": item["glazing_construction_id"]})]


def rename_fenestration(before, payload):
    """🏁️ `rename-fenestration{id,newName}` — Sets one fenestration's identity field; a name another fenestration already holds is refused."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["id"] != entity_id and row["name"] == value for row in before["model"]["fenestrations"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_fenestration(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("rename-fenestration", {"id": payload["id"], "newName": item["name"]})]


def change_fenestration_surface(before, payload):
    """🧲️ `change-fenestration-surface{id,newSurfaceId}` — Rehosts the fenestration on another existing surface — the surface whose orientation, tilt and zone the window then inherits."""
    entity_id, value = payload["id"], payload["newSurfaceId"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == value for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["surface_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["surface_id"] = value
    return after, applied()


def _invert_change_fenestration_surface(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-surface", {"id": payload["id"], "newSurfaceId": item["surface_id"]})]


def change_fenestration_u_value(before, payload):
    """🌐️ `change-fenestration-u-value{id,newUValueWM2k}` — Sets the glazing's air-to-air thermal transmittance in W/(m²·K) — the conductance the film-free window term multiplies by area and the outdoor-to-zone temperature difference."""
    entity_id, value = payload["id"], payload["newUValueWM2k"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["u_value_w_m2k"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["u_value_w_m2k"] = value
    return after, applied()


def _invert_change_fenestration_u_value(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-u-value", {"id": payload["id"], "newUValueWM2k": item["u_value_w_m2k"]})]


def change_fenestration_shgc(before, payload):
    """🌇️ `change-fenestration-shgc{id,newShgc}` — Sets the solar heat gain coefficient — the fraction of incident solar the glazing passes to the zone, directly and by re-radiation."""
    entity_id, value = payload["id"], payload["newShgc"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and 0.0 <= value <= 1.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["shgc"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["shgc"] = value
    return after, applied()


def _invert_change_fenestration_shgc(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-shgc", {"id": payload["id"], "newShgc": item["shgc"]})]


def change_fenestration_vlt(before, payload):
    """🌈️ `change-fenestration-vlt{id,newVlt}` — Sets the visible light transmittance — the daylight fraction the illuminance calculation reads, independent of the solar gain the SHGC governs."""
    entity_id, value = payload["id"], payload["newVlt"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and 0.0 <= value <= 1.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["vlt"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["vlt"] = value
    return after, applied()


def _invert_change_fenestration_vlt(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-vlt", {"id": payload["id"], "newVlt": item["vlt"]})]


def change_fenestration_area(before, payload):
    """🟥️ `change-fenestration-area{id,newAreaM2}` — Sets the glazed area in square metres — the area every window heat-balance and solar term is proportional to."""
    entity_id, value = payload["id"], payload["newAreaM2"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["area_m2"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["area_m2"] = value
    return after, applied()


def _invert_change_fenestration_area(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-area", {"id": payload["id"], "newAreaM2": item["area_m2"]})]


def change_fenestration_frame_conductance(before, payload):
    """🖼️ `change-fenestration-frame-conductance{id,newFrameConductanceWK}` — Sets the frame's whole-assembly thermal conductance in W/K, added to the glazing term rather than distributed over the area."""
    entity_id, value = payload["id"], payload["newFrameConductanceWK"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["frame_conductance_w_k"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["frame_conductance_w_k"] = value
    return after, applied()


def _invert_change_fenestration_frame_conductance(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-frame-conductance", {"id": payload["id"], "newFrameConductanceWK": item["frame_conductance_w_k"]})]


def change_fenestration_divider_conductance(before, payload):
    """🧷️ `change-fenestration-divider-conductance{id,newDividerConductanceWK}` — Sets the divider bars' whole-assembly thermal conductance in W/K, the frame term's sibling for the muntins inside the glazed area."""
    entity_id, value = payload["id"], payload["newDividerConductanceWK"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["divider_conductance_w_k"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["divider_conductance_w_k"] = value
    return after, applied()


def _invert_change_fenestration_divider_conductance(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-divider-conductance", {"id": payload["id"], "newDividerConductanceWK": item["divider_conductance_w_k"]})]


def create_shading_surface(before, payload):
    """🌳️ `create-shading-surface` — Adds one free-standing solar obstruction — the site-level sibling of a window's own attached overhang and fins, which stay on the fenestration."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["shading_surfaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if len(payload["verticesM"]) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if (payload["transmittanceScheduleId"] is not None and not any(row["id"] == payload["transmittanceScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][family])):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "vertices_m": payload["verticesM"], "transmittance_schedule_id": payload["transmittanceScheduleId"]}
    rows = after["model"]["shading_surfaces"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    rows.insert(position, created)
    return after, applied()


def _invert_create_shading_surface(before, payload):
    return [("delete-shading-surface", {"id": payload["id"]})]


def delete_shading_surface(before, payload):
    """🪵️ `delete-shading-surface{id}` — Removes one free-standing solar obstruction. Nothing in the document references a shading surface, so it neither cascades nor restricts."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["shading_surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    after = copy.deepcopy(before)
    after["model"]["shading_surfaces"] = [row for row in after["model"]["shading_surfaces"] if row["id"] != entity_id]
    return after, applied()


def _invert_delete_shading_surface(before, payload):
    item = next(row for row in before["model"]["shading_surfaces"] if row["id"] == payload["id"])
    return [("create-shading-surface", {"id": item["id"], "name": item["name"], "verticesM": item["vertices_m"], "transmittanceScheduleId": item["transmittance_schedule_id"]})]


def rename_shading_surface(before, payload):
    """🏕️ `rename-shading-surface{id,newName}` — Sets one shading surface's identity field; a name another shading surface already holds is refused."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["shading_surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["id"] != entity_id and row["name"] == value for row in before["model"]["shading_surfaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["shading_surfaces"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_shading_surface(before, payload):
    item = next(row for row in before["model"]["shading_surfaces"] if row["id"] == payload["id"])
    return [("rename-shading-surface", {"id": payload["id"], "newName": item["name"]})]


def replace_shading_surface_vertices(before, payload):
    """🗺️ `replace-shading-surface-vertices{id,newVerticesM}` — Swaps the shading surface's whole polygon — geometry is one value, so the taxonomy's `replace` verb carries the full ring, minimum three vertices."""
    entity_id, value = payload["id"], payload["newVerticesM"]
    item = next((row for row in before["model"]["shading_surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if len(value) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["vertices_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["shading_surfaces"]:
        if row["id"] == entity_id:
            row["vertices_m"] = value
    return after, applied()


def _invert_replace_shading_surface_vertices(before, payload):
    item = next(row for row in before["model"]["shading_surfaces"] if row["id"] == payload["id"])
    return [("replace-shading-surface-vertices", {"id": payload["id"], "newVerticesM": item["vertices_m"]})]


def change_shading_surface_transmittance_schedule(before, payload):
    """⛱️ `change-shading-surface-transmittance-schedule{id,newTransmittanceScheduleId}` — Points the shading surface at one of the model's own schedules for its time-varying solar transmittance, or at none for a fully opaque obstruction."""
    entity_id, value = payload["id"], payload["newTransmittanceScheduleId"]
    item = next((row for row in before["model"]["shading_surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if (value is not None and not any(row["id"] == value for family in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][family])):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["transmittance_schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["shading_surfaces"]:
        if row["id"] == entity_id:
            row["transmittance_schedule_id"] = value
    return after, applied()


def _invert_change_shading_surface_transmittance_schedule(before, payload):
    item = next(row for row in before["model"]["shading_surfaces"] if row["id"] == payload["id"])
    return [("change-shading-surface-transmittance-schedule", {"id": payload["id"], "newTransmittanceScheduleId": item["transmittance_schedule_id"]})]


def connect_surfaces(before, payload):
    """🤝️ `connect-surfaces{surfaceAId,surfaceBId}` — taxonomy.md's `connect` verb on an edge
    collection with no id of its own, so the unordered pair is the address."""
    first, second = payload["surfaceAId"], payload["surfaceBId"]
    address = [str(first), str(second)]
    if first == second:
        return unchanged(before), rejected("mutation.invariant", address)
    known = [row["id"] for row in before["model"]["surfaces"]]
    if first not in known or second not in known:
        return unchanged(before), rejected("mutation.target-missing", address)
    if any({row["surface_a_id"], row["surface_b_id"]} == {first, second} for row in before["model"]["adjacency_pairs"]):
        return unchanged(before), rejected("mutation.duplicate-id", address)
    after = copy.deepcopy(before)
    rows = after["model"]["adjacency_pairs"]
    position = next((index for index, row in enumerate(rows) if (row["surface_a_id"], row["surface_b_id"]) > (first, second)), len(rows))
    rows.insert(position, {"surface_a_id": first, "surface_b_id": second})
    return after, applied()


def _invert_connect_surfaces(before, payload):
    return [("disconnect-surfaces", {"surfaceAId": payload["surfaceAId"], "surfaceBId": payload["surfaceBId"]})]


def disconnect_surfaces(before, payload):
    """💔️ `disconnect-surfaces{surfaceAId,surfaceBId}` — `connect`'s inverse partner."""
    first, second = payload["surfaceAId"], payload["surfaceBId"]
    if not any({row["surface_a_id"], row["surface_b_id"]} == {first, second} for row in before["model"]["adjacency_pairs"]):
        return unchanged(before), rejected("mutation.target-missing", [str(first), str(second)])
    after = copy.deepcopy(before)
    after["model"]["adjacency_pairs"] = [row for row in after["model"]["adjacency_pairs"] if {row["surface_a_id"], row["surface_b_id"]} != {first, second}]
    return after, applied()


def _invert_disconnect_surfaces(before, payload):
    item = next(row for row in before["model"]["adjacency_pairs"] if {row["surface_a_id"], row["surface_b_id"]} == {payload["surfaceAId"], payload["surfaceBId"]})
    return [("connect-surfaces", {"surfaceAId": item["surface_a_id"], "surfaceBId": item["surface_b_id"]})]


def bind_fenestration_glazing_construction(before, payload):
    """🧊️ `bind-fenestration-glazing-construction{id,constructionId}` — taxonomy.md's `bind` verb
    filling the optional glazing slot with a real layer stack."""
    entity_id, construction_id = payload["id"], payload["constructionId"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == construction_id for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["glazing_construction_id"] == construction_id:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["glazing_construction_id"] = construction_id
    return after, applied()


def _invert_bind_fenestration_glazing_construction(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    previous = item["glazing_construction_id"]
    if previous is None:
        return [("clear-fenestration-glazing-construction", {"id": payload["id"]})]
    return [("bind-fenestration-glazing-construction", {"id": payload["id"], "constructionId": previous})]


def clear_fenestration_glazing_construction(before, payload):
    """🫗️ `clear-fenestration-glazing-construction{id}` — `bind`'s inverse partner."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None or item["glazing_construction_id"] is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["glazing_construction_id"] = None
    return after, applied()


def _invert_clear_fenestration_glazing_construction(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("bind-fenestration-glazing-construction", {"id": payload["id"], "constructionId": item["glazing_construction_id"]})]


def change_fenestration_height(before, payload):
    """⬆️ `change-fenestration-height{id,newHeightM}` — Sets the glazing's head-to-sill height in metres — the length the overhang and fin shadow geometry is projected against."""
    entity_id, value = payload["id"], payload["newHeightM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["height_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["height_m"] = value
    return after, applied()


def _invert_change_fenestration_height(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-height", {"id": payload["id"], "newHeightM": item["height_m"]})]


def change_fenestration_sill_height(before, payload):
    """⬇️ `change-fenestration-sill-height{id,newSillHeightM}` — Sets the height of the glazing's sill above the host surface's base in metres."""
    entity_id, value = payload["id"], payload["newSillHeightM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["sill_height_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["sill_height_m"] = value
    return after, applied()


def _invert_change_fenestration_sill_height(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-sill-height", {"id": payload["id"], "newSillHeightM": item["sill_height_m"]})]


def change_fenestration_overhang_depth(before, payload):
    """🧢️ `change-fenestration-overhang-depth{id,newOverhangDepthM}` — Sets how far the horizontal projection above the window head reaches out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 610/910 are exactly this field."""
    entity_id, value = payload["id"], payload["newOverhangDepthM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["overhang_depth_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["overhang_depth_m"] = value
    return after, applied()


def _invert_change_fenestration_overhang_depth(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-overhang-depth", {"id": payload["id"], "newOverhangDepthM": item["overhang_depth_m"]})]


def change_fenestration_overhang_offset(before, payload):
    """🎩️ `change-fenestration-overhang-offset{id,newOverhangOffsetM}` — Sets how far above the window head the horizontal projection sits, in metres."""
    entity_id, value = payload["id"], payload["newOverhangOffsetM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["overhang_offset_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["overhang_offset_m"] = value
    return after, applied()


def _invert_change_fenestration_overhang_offset(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-overhang-offset", {"id": payload["id"], "newOverhangOffsetM": item["overhang_offset_m"]})]


def change_fenestration_fin_depth(before, payload):
    """🐬️ `change-fenestration-fin-depth{id,newFinDepthM}` — Sets how far the two vertical projections beside the window jambs reach out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 630/930 are exactly this field."""
    entity_id, value = payload["id"], payload["newFinDepthM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["fin_depth_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["fin_depth_m"] = value
    return after, applied()


def _invert_change_fenestration_fin_depth(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-fin-depth", {"id": payload["id"], "newFinDepthM": item["fin_depth_m"]})]


def change_fenestration_fin_offset(before, payload):
    """🐋️ `change-fenestration-fin-offset{id,newFinOffsetM}` — Sets how far beside the window jambs the two vertical projections stand, in metres."""
    entity_id, value = payload["id"], payload["newFinOffsetM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["fin_offset_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["fin_offset_m"] = value
    return after, applied()


def _invert_change_fenestration_fin_offset(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-fin-offset", {"id": payload["id"], "newFinOffsetM": item["fin_offset_m"]})]


def create_material(before, payload):
    """🧱️ `create-material{index,…}` — Adds one opaque material layer definition at a stated position in the model's material list. Thickness, conductivity, density and specific heat are the four the conduction transfer functions integrate; the three absorptances close the surface radiation balance."""
    rows = before["model"]["materials"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["materials"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "thickness_m": payload["thicknessM"], "conductivity_w_m_k": payload["conductivityWMK"], "density_kg_m3": payload["densityKgM3"], "specific_heat_j_kg_k": payload["specificHeatJKgK"], "thermal_absorptance": payload["thermalAbsorptance"], "solar_absorptance": payload["solarAbsorptance"], "visible_absorptance": payload["visibleAbsorptance"]})
    return after, applied()


def _invert_create_material(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-material", {"id": payload["id"]})]


def delete_material(before, payload):
    """🪨️ `delete-material{id}` — Removes one material definition. Refused while any construction still names it as a layer — the alternative would be to cascade into constructions, which cascade into surfaces."""
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(payload["id"] in row["layer_material_ids"] for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["materials"] = [row for row in after["model"]["materials"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_material(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["materials"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-material", {"index": index, "id": item["id"], "name": item["name"], "thicknessM": item["thickness_m"], "conductivityWMK": item["conductivity_w_m_k"], "densityKgM3": item["density_kg_m3"], "specificHeatJKgK": item["specific_heat_j_kg_k"], "thermalAbsorptance": item["thermal_absorptance"], "solarAbsorptance": item["solar_absorptance"], "visibleAbsorptance": item["visible_absorptance"]})]


def rename_material(before, payload):
    """🪧️ `rename-material{id,newName}` — Sets one material's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["materials"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_material(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("rename-material", {"id": payload["id"], "newName": item["name"]})]


def change_material_thickness(before, payload):
    """📏️ `change-material-thickness{id,newThicknessM}` — Sets thickness (m) on one material, addressed by id."""
    value = payload["newThicknessM"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["thickness_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["thickness_m"] = value
    return after, applied()


def _invert_change_material_thickness(before, payload):
    """↩️ The thickness_m the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-thickness", {"id": payload["id"], "newThicknessM": item["thickness_m"]})]


def change_material_conductivity(before, payload):
    """🔥️ `change-material-conductivity{id,newConductivityWMK}` — Sets conductivity (W/m·K) on one material, addressed by id."""
    value = payload["newConductivityWMK"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["conductivity_w_m_k"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["conductivity_w_m_k"] = value
    return after, applied()


def _invert_change_material_conductivity(before, payload):
    """↩️ The conductivity_w_m_k the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-conductivity", {"id": payload["id"], "newConductivityWMK": item["conductivity_w_m_k"]})]


def change_material_density(before, payload):
    """⚖️ `change-material-density{id,newDensityKgM3}` — Sets density (kg/m³) on one material, addressed by id."""
    value = payload["newDensityKgM3"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["density_kg_m3"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["density_kg_m3"] = value
    return after, applied()


def _invert_change_material_density(before, payload):
    """↩️ The density_kg_m3 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-density", {"id": payload["id"], "newDensityKgM3": item["density_kg_m3"]})]


def change_material_specific_heat(before, payload):
    """♨️ `change-material-specific-heat{id,newSpecificHeatJKgK}` — Sets specific heat (J/kg·K) on one material, addressed by id."""
    value = payload["newSpecificHeatJKgK"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["specific_heat_j_kg_k"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["specific_heat_j_kg_k"] = value
    return after, applied()


def _invert_change_material_specific_heat(before, payload):
    """↩️ The specific_heat_j_kg_k the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-specific-heat", {"id": payload["id"], "newSpecificHeatJKgK": item["specific_heat_j_kg_k"]})]


def change_material_thermal_absorptance(before, payload):
    """🔆️ `change-material-thermal-absorptance{id,newThermalAbsorptance}` — Sets thermal absorptance on one material, addressed by id."""
    value = payload["newThermalAbsorptance"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["thermal_absorptance"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["thermal_absorptance"] = value
    return after, applied()


def _invert_change_material_thermal_absorptance(before, payload):
    """↩️ The thermal_absorptance the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-thermal-absorptance", {"id": payload["id"], "newThermalAbsorptance": item["thermal_absorptance"]})]


def change_material_solar_absorptance(before, payload):
    """☀️ `change-material-solar-absorptance{id,newSolarAbsorptance}` — Sets solar absorptance on one material, addressed by id."""
    value = payload["newSolarAbsorptance"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["solar_absorptance"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["solar_absorptance"] = value
    return after, applied()


def _invert_change_material_solar_absorptance(before, payload):
    """↩️ The solar_absorptance the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-solar-absorptance", {"id": payload["id"], "newSolarAbsorptance": item["solar_absorptance"]})]


def change_material_visible_absorptance(before, payload):
    """👁️ `change-material-visible-absorptance{id,newVisibleAbsorptance}` — Sets visible absorptance on one material, addressed by id."""
    value = payload["newVisibleAbsorptance"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["visible_absorptance"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["visible_absorptance"] = value
    return after, applied()


def _invert_change_material_visible_absorptance(before, payload):
    """↩️ The visible_absorptance the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-visible-absorptance", {"id": payload["id"], "newVisibleAbsorptance": item["visible_absorptance"]})]


def create_construction(before, payload):
    """🏗️ `create-construction{index,…}` — Adds one layered construction, outside-to-inside layer order. Every layer must already name a material the document defines, so a construction is never born dangling."""
    rows = before["model"]["constructions"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    missing = next((identifier for identifier in payload["layerMaterialIds"] if not any(row["id"] == identifier for row in before["model"]["materials"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["constructions"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "layer_material_ids": payload["layerMaterialIds"]})
    return after, applied()


def _invert_create_construction(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-construction", {"id": payload["id"]})]


def delete_construction(before, payload):
    """🧨️ `delete-construction{id}` — Removes one construction. Refused while any surface still names it, because a surface without a construction has no heat path at all."""
    item = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(row["construction_id"] == payload["id"] for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["constructions"] = [row for row in after["model"]["constructions"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_construction(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["constructions"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-construction", {"index": index, "id": item["id"], "name": item["name"], "layerMaterialIds": item["layer_material_ids"]})]


def rename_construction(before, payload):
    """🪪️ `rename-construction{id,newName}` — Sets one construction's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_construction(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["constructions"] if row["id"] == payload["id"])
    return [("rename-construction", {"id": payload["id"], "newName": item["name"]})]


def add_construction_layer(before, payload):
    """➕️ `add-construction-layer{id,index,materialId}` — one layer inserted at a stated position,
    outside-to-inside; layer order is physically load-bearing."""
    construction = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if construction is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["materialId"] for row in before["model"]["materials"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["materialId"])])
    if payload["index"] > len(construction["layer_material_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            row["layer_material_ids"].insert(payload["index"], payload["materialId"])
    return after, applied()


def _invert_add_construction_layer(before, payload):
    """↩️ Undone by removing the layer at exactly the index it was inserted at."""
    return [("remove-construction-layer", {"id": payload["id"], "index": payload["index"]})]


def remove_construction_layer(before, payload):
    """➖️ `remove-construction-layer{id,index}` — addressed by position, because one material may
    legitimately appear in a construction more than once."""
    construction = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if construction is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["index"] >= len(construction["layer_material_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            del row["layer_material_ids"][payload["index"]]
    return after, applied()


def _invert_remove_construction_layer(before, payload):
    """↩️ Re-inserts exactly the material that stood at that index, read off BASE."""
    construction = next(row for row in before["model"]["constructions"] if row["id"] == payload["id"])
    return [("add-construction-layer", {"id": payload["id"], "index": payload["index"], "materialId": construction["layer_material_ids"][payload["index"]]})]


def reorder_construction_layers(before, payload):
    """🔀️ `reorder-construction-layers{id,newLayerMaterialIds}` — a permutation of the layers the
    construction already holds; exchanging a layer is a different kind's job."""
    construction = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if construction is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    wanted = payload["newLayerMaterialIds"]
    if sorted(wanted) != sorted(construction["layer_material_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if construction["layer_material_ids"] == wanted:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            row["layer_material_ids"] = list(wanted)
    return after, applied()


def _invert_reorder_construction_layers(before, payload):
    """↩️ The order BASE held, restated."""
    construction = next(row for row in before["model"]["constructions"] if row["id"] == payload["id"])
    return [("reorder-construction-layers", {"id": payload["id"], "newLayerMaterialIds": list(construction["layer_material_ids"])})]


def create_people_gain(before, payload):
    """👤️ `create-people-gain{index,…}` — Adds one occupancy gain to a zone: the occupant density, the occupancy schedule, the metabolic activity schedule and the sensible/latent/radiant split the zone heat balance needs."""
    rows = before["model"]["people"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    if not any(row["id"] == payload["activityScheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["activityScheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["people"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "activity_schedule_id": payload["activityScheduleId"], "people_per_area": payload["peoplePerArea"], "sensible_fraction": payload["sensibleFraction"], "latent_fraction": payload["latentFraction"], "radiant_fraction": payload["radiantFraction"]})
    return after, applied()


def _invert_create_people_gain(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-people-gain", {"id": payload["id"]})]


def delete_people_gain(before, payload):
    """🚷️ `delete-people-gain{id}` — Removes one occupancy gain. Nothing references a gain, so this never has to refuse for use."""
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["people"] = [row for row in after["model"]["people"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_people_gain(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["people"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-people-gain", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "activityScheduleId": item["activity_schedule_id"], "peoplePerArea": item["people_per_area"], "sensibleFraction": item["sensible_fraction"], "latentFraction": item["latent_fraction"], "radiantFraction": item["radiant_fraction"]})]


def change_people_gain_zone(before, payload):
    """🚶️ `change-people-gain-zone{id,newZoneId}` — Re-points one people gain's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_people_gain_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_people_gain_schedule(before, payload):
    """⏰️ `change-people-gain-schedule{id,newScheduleId}` — Re-points one people gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_people_gain_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_people_gain_activity_schedule(before, payload):
    """🏃️ `change-people-gain-activity-schedule{id,newActivityScheduleId}` — Re-points one people gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newActivityScheduleId"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["activity_schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["activity_schedule_id"] = value
    return after, applied()


def _invert_change_people_gain_activity_schedule(before, payload):
    """↩️ The activity_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-activity-schedule", {"id": payload["id"], "newActivityScheduleId": item["activity_schedule_id"]})]


def change_people_gain_people_per_area(before, payload):
    """👥️ `change-people-gain-people-per-area{id,newPeoplePerArea}` — Sets occupant density (people/m²) on one people gain, addressed by id."""
    value = payload["newPeoplePerArea"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["people_per_area"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["people_per_area"] = value
    return after, applied()


def _invert_change_people_gain_people_per_area(before, payload):
    """↩️ The people_per_area the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-people-per-area", {"id": payload["id"], "newPeoplePerArea": item["people_per_area"]})]


def change_people_gain_sensible_fraction(before, payload):
    """🌞️ `change-people-gain-sensible-fraction{id,newSensibleFraction}` — Sets sensible fraction on one people gain, addressed by id."""
    value = payload["newSensibleFraction"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["sensible_fraction"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["sensible_fraction"] = value
    return after, applied()


def _invert_change_people_gain_sensible_fraction(before, payload):
    """↩️ The sensible_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-sensible-fraction", {"id": payload["id"], "newSensibleFraction": item["sensible_fraction"]})]


def change_people_gain_latent_fraction(before, payload):
    """💧️ `change-people-gain-latent-fraction{id,newLatentFraction}` — Sets latent fraction on one people gain, addressed by id."""
    value = payload["newLatentFraction"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["latent_fraction"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["latent_fraction"] = value
    return after, applied()


def _invert_change_people_gain_latent_fraction(before, payload):
    """↩️ The latent_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-latent-fraction", {"id": payload["id"], "newLatentFraction": item["latent_fraction"]})]


def change_people_gain_radiant_fraction(before, payload):
    """📡️ `change-people-gain-radiant-fraction{id,newRadiantFraction}` — Sets radiant fraction on one people gain, addressed by id."""
    value = payload["newRadiantFraction"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["radiant_fraction"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["radiant_fraction"] = value
    return after, applied()


def _invert_change_people_gain_radiant_fraction(before, payload):
    """↩️ The radiant_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-radiant-fraction", {"id": payload["id"], "newRadiantFraction": item["radiant_fraction"]})]


def create_lighting_gain(before, payload):
    """💡️ `create-lighting-gain{index,…}` — Adds one lighting gain to a zone: the installed power density, its schedule and the radiant/visible/return-air split that decides how much of it reaches the zone air directly."""
    rows = before["model"]["lighting"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["lighting"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "watts_per_area": payload["wattsPerArea"], "radiant_fraction": payload["radiantFraction"], "visible_fraction": payload["visibleFraction"], "return_air_fraction": payload["returnAirFraction"]})
    return after, applied()


def _invert_create_lighting_gain(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-lighting-gain", {"id": payload["id"]})]


def delete_lighting_gain(before, payload):
    """🕯️ `delete-lighting-gain{id}` — Removes one lighting gain."""
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["lighting"] = [row for row in after["model"]["lighting"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_lighting_gain(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["lighting"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-lighting-gain", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "wattsPerArea": item["watts_per_area"], "radiantFraction": item["radiant_fraction"], "visibleFraction": item["visible_fraction"], "returnAirFraction": item["return_air_fraction"]})]


def change_lighting_gain_zone(before, payload):
    """🔦️ `change-lighting-gain-zone{id,newZoneId}` — Re-points one lighting gain's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_lighting_gain_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_lighting_gain_schedule(before, payload):
    """⏱️ `change-lighting-gain-schedule{id,newScheduleId}` — Re-points one lighting gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_lighting_gain_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_lighting_gain_watts_per_area(before, payload):
    """🔌️ `change-lighting-gain-watts-per-area{id,newWattsPerArea}` — Sets installed power density (W/m²) on one lighting gain, addressed by id."""
    value = payload["newWattsPerArea"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["watts_per_area"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["watts_per_area"] = value
    return after, applied()


def _invert_change_lighting_gain_watts_per_area(before, payload):
    """↩️ The watts_per_area the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-watts-per-area", {"id": payload["id"], "newWattsPerArea": item["watts_per_area"]})]


def change_lighting_gain_radiant_fraction(before, payload):
    """🌟️ `change-lighting-gain-radiant-fraction{id,newRadiantFraction}` — Sets radiant fraction on one lighting gain, addressed by id."""
    value = payload["newRadiantFraction"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["radiant_fraction"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["radiant_fraction"] = value
    return after, applied()


def _invert_change_lighting_gain_radiant_fraction(before, payload):
    """↩️ The radiant_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-radiant-fraction", {"id": payload["id"], "newRadiantFraction": item["radiant_fraction"]})]


def change_lighting_gain_visible_fraction(before, payload):
    """🔅️ `change-lighting-gain-visible-fraction{id,newVisibleFraction}` — Sets visible fraction on one lighting gain, addressed by id."""
    value = payload["newVisibleFraction"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["visible_fraction"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["visible_fraction"] = value
    return after, applied()


def _invert_change_lighting_gain_visible_fraction(before, payload):
    """↩️ The visible_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-visible-fraction", {"id": payload["id"], "newVisibleFraction": item["visible_fraction"]})]


def change_lighting_gain_return_air_fraction(before, payload):
    """🎐️ `change-lighting-gain-return-air-fraction{id,newReturnAirFraction}` — Sets return-air fraction on one lighting gain, addressed by id."""
    value = payload["newReturnAirFraction"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["return_air_fraction"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["return_air_fraction"] = value
    return after, applied()


def _invert_change_lighting_gain_return_air_fraction(before, payload):
    """↩️ The return_air_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-return-air-fraction", {"id": payload["id"], "newReturnAirFraction": item["return_air_fraction"]})]


def create_equipment_gain(before, payload):
    """🖥️ `create-equipment-gain{index,…}` — Adds one electric equipment gain to a zone — ANSI/ASHRAE 140 §5.2's 200 W internal load is exactly this entity at 60 % radiative."""
    rows = before["model"]["equipment"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["equipment"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "watts_per_area": payload["wattsPerArea"], "radiant_fraction": payload["radiantFraction"], "latent_fraction": payload["latentFraction"]})
    return after, applied()


def _invert_create_equipment_gain(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-equipment-gain", {"id": payload["id"]})]


def delete_equipment_gain(before, payload):
    """🧯️ `delete-equipment-gain{id}` — Removes one electric equipment gain."""
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["equipment"] = [row for row in after["model"]["equipment"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_equipment_gain(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["equipment"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-equipment-gain", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "wattsPerArea": item["watts_per_area"], "radiantFraction": item["radiant_fraction"], "latentFraction": item["latent_fraction"]})]


def change_equipment_gain_zone(before, payload):
    """🖨️ `change-equipment-gain-zone{id,newZoneId}` — Re-points one equipment gain's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_equipment_gain_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_equipment_gain_schedule(before, payload):
    """⌛️ `change-equipment-gain-schedule{id,newScheduleId}` — Re-points one equipment gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_equipment_gain_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_equipment_gain_watts_per_area(before, payload):
    """⚡️ `change-equipment-gain-watts-per-area{id,newWattsPerArea}` — Sets equipment power density (W/m²) on one equipment gain, addressed by id."""
    value = payload["newWattsPerArea"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["watts_per_area"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["watts_per_area"] = value
    return after, applied()


def _invert_change_equipment_gain_watts_per_area(before, payload):
    """↩️ The watts_per_area the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-watts-per-area", {"id": payload["id"], "newWattsPerArea": item["watts_per_area"]})]


def change_equipment_gain_radiant_fraction(before, payload):
    """🌠️ `change-equipment-gain-radiant-fraction{id,newRadiantFraction}` — Sets radiant fraction on one equipment gain, addressed by id."""
    value = payload["newRadiantFraction"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["radiant_fraction"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["radiant_fraction"] = value
    return after, applied()


def _invert_change_equipment_gain_radiant_fraction(before, payload):
    """↩️ The radiant_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-radiant-fraction", {"id": payload["id"], "newRadiantFraction": item["radiant_fraction"]})]


def change_equipment_gain_latent_fraction(before, payload):
    """💦️ `change-equipment-gain-latent-fraction{id,newLatentFraction}` — Sets latent fraction on one equipment gain, addressed by id."""
    value = payload["newLatentFraction"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["latent_fraction"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["latent_fraction"] = value
    return after, applied()


def _invert_change_equipment_gain_latent_fraction(before, payload):
    """↩️ The latent_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-latent-fraction", {"id": payload["id"], "newLatentFraction": item["latent_fraction"]})]


def create_infiltration(before, payload):
    """💨️ `create-infiltration{index,…}` — Adds one zone infiltration specification: the method plus every parameter each method needs, so the kernel maps the entity onto one `air_exchange::InfiltrationSpec` without pinning a method or smuggling the flow through a coefficient."""
    rows = before["model"]["infiltrations"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["infiltrations"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "method": payload["method"], "design_flow_ach": payload["designFlowAch"], "flow_per_exterior_area_m3_s_m2": payload["flowPerExteriorAreaM3SM2"], "effective_leakage_area_m2": payload["effectiveLeakageAreaM2"], "discharge_coefficient": payload["dischargeCoefficient"], "stack_height_m": payload["stackHeightM"], "constant_term_coefficient": payload["constantTermCoefficient"], "temperature_term_coefficient": payload["temperatureTermCoefficient"], "velocity_term_coefficient": payload["velocityTermCoefficient"], "velocity_squared_term_coefficient": payload["velocitySquaredTermCoefficient"]})
    return after, applied()


def _invert_create_infiltration(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-infiltration", {"id": payload["id"]})]


def delete_infiltration(before, payload):
    """🧽️ `delete-infiltration{id}` — Removes one zone infiltration specification, leaving the zone airtight."""
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["infiltrations"] = [row for row in after["model"]["infiltrations"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_infiltration(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["infiltrations"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-infiltration", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "method": item["method"], "designFlowAch": item["design_flow_ach"], "flowPerExteriorAreaM3SM2": item["flow_per_exterior_area_m3_s_m2"], "effectiveLeakageAreaM2": item["effective_leakage_area_m2"], "dischargeCoefficient": item["discharge_coefficient"], "stackHeightM": item["stack_height_m"], "constantTermCoefficient": item["constant_term_coefficient"], "temperatureTermCoefficient": item["temperature_term_coefficient"], "velocityTermCoefficient": item["velocity_term_coefficient"], "velocitySquaredTermCoefficient": item["velocity_squared_term_coefficient"]})]


def change_infiltration_zone(before, payload):
    """🌀️ `change-infiltration-zone{id,newZoneId}` — Re-points one infiltration's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_infiltration_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_infiltration_schedule(before, payload):
    """⏳️ `change-infiltration-schedule{id,newScheduleId}` — Re-points one infiltration's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_infiltration_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_infiltration_flow_per_exterior_area(before, payload):
    """🌫️ `change-infiltration-flow-per-exterior-area{id,newFlowPerExteriorAreaM3SM2}` — Sets flow per exterior area (m³/s·m²) on one infiltration, addressed by id."""
    value = payload["newFlowPerExteriorAreaM3SM2"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["flow_per_exterior_area_m3_s_m2"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["flow_per_exterior_area_m3_s_m2"] = value
    return after, applied()


def _invert_change_infiltration_flow_per_exterior_area(before, payload):
    """↩️ The flow_per_exterior_area_m3_s_m2 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-flow-per-exterior-area", {"id": payload["id"], "newFlowPerExteriorAreaM3SM2": item["flow_per_exterior_area_m3_s_m2"]})]


def change_infiltration_constant_term_coefficient(before, payload):
    """🅰️ `change-infiltration-constant-term-coefficient{id,newConstantTermCoefficient}` — Sets EnergyPlus's `ZoneInfiltration:DesignFlowRate` constant term coefficient A. The four coefficients are non-negative by that object's own convention (BLAST 0.606/0.03636/0.1177/0, DOE-2 0/0/0.224/0)."""
    value = payload["newConstantTermCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["constant_term_coefficient"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["constant_term_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_constant_term_coefficient(before, payload):
    """↩️ The constant_term_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-constant-term-coefficient", {"id": payload["id"], "newConstantTermCoefficient": item["constant_term_coefficient"]})]


def change_infiltration_temperature_term_coefficient(before, payload):
    """🅱️ `change-infiltration-temperature-term-coefficient{id,newTemperatureTermCoefficient}` — Sets the temperature term coefficient B on one infiltration, addressed by id."""
    value = payload["newTemperatureTermCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["temperature_term_coefficient"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["temperature_term_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_temperature_term_coefficient(before, payload):
    """↩️ The temperature_term_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-temperature-term-coefficient", {"id": payload["id"], "newTemperatureTermCoefficient": item["temperature_term_coefficient"]})]


def change_infiltration_velocity_term_coefficient(before, payload):
    """🆎️ `change-infiltration-velocity-term-coefficient{id,newVelocityTermCoefficient}` — Sets the wind velocity term coefficient C on one infiltration, addressed by id."""
    value = payload["newVelocityTermCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["velocity_term_coefficient"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["velocity_term_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_velocity_term_coefficient(before, payload):
    """↩️ The velocity_term_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-velocity-term-coefficient", {"id": payload["id"], "newVelocityTermCoefficient": item["velocity_term_coefficient"]})]


def change_infiltration_velocity_squared_term_coefficient(before, payload):
    """🆑️ `change-infiltration-velocity-squared-term-coefficient{id,newVelocitySquaredTermCoefficient}` — Sets the squared wind velocity term coefficient D on one infiltration, addressed by id."""
    value = payload["newVelocitySquaredTermCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["velocity_squared_term_coefficient"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["velocity_squared_term_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_velocity_squared_term_coefficient(before, payload):
    """↩️ The velocity_squared_term_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-velocity-squared-term-coefficient", {"id": payload["id"], "newVelocitySquaredTermCoefficient": item["velocity_squared_term_coefficient"]})]


def create_mechanical_ventilation(before, payload):
    """🌪️ `create-mechanical-ventilation{index,…}` — Adds one mechanical ventilation specification to a zone: the design supply flow, its schedule and the fan work the supply air arrives with."""
    rows = before["model"]["mechanical_ventilations"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["mechanical_ventilations"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "design_flow_m3_s": payload["designFlowM3S"], "fan_total_efficiency": payload["fanTotalEfficiency"], "fan_delta_pressure_pa": payload["fanDeltaPressurePa"]})
    return after, applied()


def _invert_create_mechanical_ventilation(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-mechanical-ventilation", {"id": payload["id"]})]


def delete_mechanical_ventilation(before, payload):
    """🚫️ `delete-mechanical-ventilation{id}` — Removes one mechanical ventilation specification."""
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["mechanical_ventilations"] = [row for row in after["model"]["mechanical_ventilations"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_mechanical_ventilation(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["mechanical_ventilations"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-mechanical-ventilation", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "designFlowM3S": item["design_flow_m3_s"], "fanTotalEfficiency": item["fan_total_efficiency"], "fanDeltaPressurePa": item["fan_delta_pressure_pa"]})]


def change_mechanical_ventilation_zone(before, payload):
    """🧭️ `change-mechanical-ventilation-zone{id,newZoneId}` — Re-points one mechanical ventilation's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_mechanical_ventilation_schedule(before, payload):
    """📆️ `change-mechanical-ventilation-schedule{id,newScheduleId}` — Re-points one mechanical ventilation's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_mechanical_ventilation_design_flow(before, payload):
    """🚿️ `change-mechanical-ventilation-design-flow{id,newDesignFlowM3S}` — Sets design supply flow (m³/s) on one mechanical ventilation, addressed by id."""
    value = payload["newDesignFlowM3S"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_flow_m3_s"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["design_flow_m3_s"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_design_flow(before, payload):
    """↩️ The design_flow_m3_s the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-design-flow", {"id": payload["id"], "newDesignFlowM3S": item["design_flow_m3_s"]})]


def change_mechanical_ventilation_fan_total_efficiency(before, payload):
    """💠️ `change-mechanical-ventilation-fan-total-efficiency{id,newFanTotalEfficiency}` — Sets fan total efficiency on one mechanical ventilation, addressed by id."""
    value = payload["newFanTotalEfficiency"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["fan_total_efficiency"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["fan_total_efficiency"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_fan_total_efficiency(before, payload):
    """↩️ The fan_total_efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-fan-total-efficiency", {"id": payload["id"], "newFanTotalEfficiency": item["fan_total_efficiency"]})]


def change_mechanical_ventilation_fan_delta_pressure(before, payload):
    """🎈️ `change-mechanical-ventilation-fan-delta-pressure{id,newFanDeltaPressurePa}` — Sets fan pressure rise (Pa) on one mechanical ventilation, addressed by id."""
    value = payload["newFanDeltaPressurePa"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["fan_delta_pressure_pa"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["fan_delta_pressure_pa"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_fan_delta_pressure(before, payload):
    """↩️ The fan_delta_pressure_pa the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-fan-delta-pressure", {"id": payload["id"], "newFanDeltaPressurePa": item["fan_delta_pressure_pa"]})]


def change_infiltration_method(before, payload):
    """🔬️ `change-infiltration-method{id,newMethod}` — Selects which of `air_exchange::InfiltrationMethod`'s four flow calculations the kernel runs for one infiltration object. The parameters every method needs are already carried side by side, so switching the method never has to move data."""
    value = payload["newMethod"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if item["method"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["method"] = value
    return after, applied()


def _invert_change_infiltration_method(before, payload):
    """↩️ The method the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-method", {"id": payload["id"], "newMethod": item["method"]})]


def change_infiltration_design_flow_ach(before, payload):
    """🔄️ `change-infiltration-design-flow-ach{id,newDesignFlowAch}` — Sets design flow (air changes per hour) on one infiltration, addressed by id."""
    value = payload["newDesignFlowAch"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_flow_ach"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["design_flow_ach"] = value
    return after, applied()


def _invert_change_infiltration_design_flow_ach(before, payload):
    """↩️ The design_flow_ach the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-design-flow-ach", {"id": payload["id"], "newDesignFlowAch": item["design_flow_ach"]})]


def change_infiltration_effective_leakage_area(before, payload):
    """🕳️ `change-infiltration-effective-leakage-area{id,newEffectiveLeakageAreaM2}` — Sets effective leakage area (m²) on one infiltration, addressed by id."""
    value = payload["newEffectiveLeakageAreaM2"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["effective_leakage_area_m2"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["effective_leakage_area_m2"] = value
    return after, applied()


def _invert_change_infiltration_effective_leakage_area(before, payload):
    """↩️ The effective_leakage_area_m2 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-effective-leakage-area", {"id": payload["id"], "newEffectiveLeakageAreaM2": item["effective_leakage_area_m2"]})]


def change_infiltration_discharge_coefficient(before, payload):
    """🚰️ `change-infiltration-discharge-coefficient{id,newDischargeCoefficient}` — Sets the orifice discharge coefficient on one infiltration, addressed by id."""
    value = payload["newDischargeCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["discharge_coefficient"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["discharge_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_discharge_coefficient(before, payload):
    """↩️ The discharge_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-discharge-coefficient", {"id": payload["id"], "newDischargeCoefficient": item["discharge_coefficient"]})]


def change_infiltration_stack_height(before, payload):
    """🏭️ `change-infiltration-stack-height{id,newStackHeightM}` — Sets stack height (m) on one infiltration, addressed by id."""
    value = payload["newStackHeightM"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["stack_height_m"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["stack_height_m"] = value
    return after, applied()


def _invert_change_infiltration_stack_height(before, payload):
    """↩️ The stack_height_m the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-stack-height", {"id": payload["id"], "newStackHeightM": item["stack_height_m"]})]


def create_thermostat(before, payload):
    """🩺️ `create-thermostat` — one new thermostat row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["thermostats"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(entry["id"] == payload["heatingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["heatingSetpointScheduleId"])])
    if not any(entry["id"] == payload["coolingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["coolingSetpointScheduleId"])])
    if payload["heatingThrottleRangeK"] != payload["heatingThrottleRangeK"] or payload["heatingThrottleRangeK"] in (float("inf"), float("-inf")) or payload["heatingThrottleRangeK"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["coolingThrottleRangeK"] != payload["coolingThrottleRangeK"] or payload["coolingThrottleRangeK"] in (float("inf"), float("-inf")) or payload["coolingThrottleRangeK"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["thermostats"].append({"id": payload["id"], "zone_id": payload["zoneId"], "heating_setpoint_schedule_id": payload["heatingSetpointScheduleId"], "cooling_setpoint_schedule_id": payload["coolingSetpointScheduleId"], "heating_throttle_range_k": payload["heatingThrottleRangeK"], "cooling_throttle_range_k": payload["coolingThrottleRangeK"]})
    return after, applied()


def _invert_create_thermostat(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-thermostat", {"id": payload["id"]})]


def delete_thermostat(before, payload):
    """🛑️ `delete-thermostat` — drops the addressed thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["thermostats"] = [entry for entry in after["model"]["thermostats"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_thermostat(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("create-thermostat", {"id": item["id"], "zoneId": item["zone_id"], "heatingSetpointScheduleId": item["heating_setpoint_schedule_id"], "coolingSetpointScheduleId": item["cooling_setpoint_schedule_id"], "heatingThrottleRangeK": item["heating_throttle_range_k"], "coolingThrottleRangeK": item["cooling_throttle_range_k"]})]


def change_thermostat_zone(before, payload):
    """🛖️ `change-thermostat-zone` — the zone of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_thermostat_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_thermostat_heating_setpoint_schedule(before, payload):
    """🥵️ `change-thermostat-heating-setpoint-schedule` — the heating setpoint schedule of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newHeatingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newHeatingSetpointScheduleId"])])
    if item["heating_setpoint_schedule_id"] == payload["newHeatingSetpointScheduleId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["heating_setpoint_schedule_id"] = payload["newHeatingSetpointScheduleId"]
    return after, applied()


def _invert_change_thermostat_heating_setpoint_schedule(before, payload):
    """↩️ Its own inverse, with the old heating setpoint schedule read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-heating-setpoint-schedule", {"id": payload["id"], "newHeatingSetpointScheduleId": item["heating_setpoint_schedule_id"]})]


def change_thermostat_cooling_setpoint_schedule(before, payload):
    """🐧️ `change-thermostat-cooling-setpoint-schedule` — the cooling setpoint schedule of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newCoolingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newCoolingSetpointScheduleId"])])
    if item["cooling_setpoint_schedule_id"] == payload["newCoolingSetpointScheduleId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["cooling_setpoint_schedule_id"] = payload["newCoolingSetpointScheduleId"]
    return after, applied()


def _invert_change_thermostat_cooling_setpoint_schedule(before, payload):
    """↩️ Its own inverse, with the old cooling setpoint schedule read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-cooling-setpoint-schedule", {"id": payload["id"], "newCoolingSetpointScheduleId": item["cooling_setpoint_schedule_id"]})]


def change_thermostat_heating_throttle_range(before, payload):
    """🎚️ `change-thermostat-heating-throttle-range` — the heating throttle range of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newHeatingThrottleRangeK"] != payload["newHeatingThrottleRangeK"] or payload["newHeatingThrottleRangeK"] in (float("inf"), float("-inf")) or payload["newHeatingThrottleRangeK"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["heating_throttle_range_k"] == payload["newHeatingThrottleRangeK"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["heating_throttle_range_k"] = payload["newHeatingThrottleRangeK"]
    return after, applied()


def _invert_change_thermostat_heating_throttle_range(before, payload):
    """↩️ Its own inverse, with the old heating throttle range read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-heating-throttle-range", {"id": payload["id"], "newHeatingThrottleRangeK": item["heating_throttle_range_k"]})]


def change_thermostat_cooling_throttle_range(before, payload):
    """🎛️ `change-thermostat-cooling-throttle-range` — the cooling throttle range of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newCoolingThrottleRangeK"] != payload["newCoolingThrottleRangeK"] or payload["newCoolingThrottleRangeK"] in (float("inf"), float("-inf")) or payload["newCoolingThrottleRangeK"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["cooling_throttle_range_k"] == payload["newCoolingThrottleRangeK"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["cooling_throttle_range_k"] = payload["newCoolingThrottleRangeK"]
    return after, applied()


def _invert_change_thermostat_cooling_throttle_range(before, payload):
    """↩️ Its own inverse, with the old cooling throttle range read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-cooling-throttle-range", {"id": payload["id"], "newCoolingThrottleRangeK": item["cooling_throttle_range_k"]})]


def create_humidistat(before, payload):
    """🌂️ `create-humidistat` — one new humidistat row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["humidistats"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(entry["id"] == payload["humidifyingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["humidifyingSetpointScheduleId"])])
    if not any(entry["id"] == payload["dehumidifyingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["dehumidifyingSetpointScheduleId"])])
    if payload["humidifyingThrottleRange"] != payload["humidifyingThrottleRange"] or payload["humidifyingThrottleRange"] in (float("inf"), float("-inf")) or payload["humidifyingThrottleRange"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["dehumidifyingThrottleRange"] != payload["dehumidifyingThrottleRange"] or payload["dehumidifyingThrottleRange"] in (float("inf"), float("-inf")) or payload["dehumidifyingThrottleRange"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["humidistats"].append({"id": payload["id"], "zone_id": payload["zoneId"], "humidifying_setpoint_schedule_id": payload["humidifyingSetpointScheduleId"], "dehumidifying_setpoint_schedule_id": payload["dehumidifyingSetpointScheduleId"], "humidifying_throttle_range": payload["humidifyingThrottleRange"], "dehumidifying_throttle_range": payload["dehumidifyingThrottleRange"]})
    return after, applied()


def _invert_create_humidistat(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-humidistat", {"id": payload["id"]})]


def delete_humidistat(before, payload):
    """🏜️ `delete-humidistat` — drops the addressed humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["humidistats"] = [entry for entry in after["model"]["humidistats"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_humidistat(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("create-humidistat", {"id": item["id"], "zoneId": item["zone_id"], "humidifyingSetpointScheduleId": item["humidifying_setpoint_schedule_id"], "dehumidifyingSetpointScheduleId": item["dehumidifying_setpoint_schedule_id"], "humidifyingThrottleRange": item["humidifying_throttle_range"], "dehumidifyingThrottleRange": item["dehumidifying_throttle_range"]})]


def change_humidistat_zone(before, payload):
    """🏙️ `change-humidistat-zone` — the zone of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_humidistat_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_humidistat_humidifying_setpoint_schedule(before, payload):
    """☔️ `change-humidistat-humidifying-setpoint-schedule` — the humidifying setpoint schedule of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newHumidifyingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newHumidifyingSetpointScheduleId"])])
    if item["humidifying_setpoint_schedule_id"] == payload["newHumidifyingSetpointScheduleId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["humidifying_setpoint_schedule_id"] = payload["newHumidifyingSetpointScheduleId"]
    return after, applied()


def _invert_change_humidistat_humidifying_setpoint_schedule(before, payload):
    """↩️ Its own inverse, with the old humidifying setpoint schedule read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-humidifying-setpoint-schedule", {"id": payload["id"], "newHumidifyingSetpointScheduleId": item["humidifying_setpoint_schedule_id"]})]


def change_humidistat_dehumidifying_setpoint_schedule(before, payload):
    """🏝️ `change-humidistat-dehumidifying-setpoint-schedule` — the dehumidifying setpoint schedule of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newDehumidifyingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newDehumidifyingSetpointScheduleId"])])
    if item["dehumidifying_setpoint_schedule_id"] == payload["newDehumidifyingSetpointScheduleId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["dehumidifying_setpoint_schedule_id"] = payload["newDehumidifyingSetpointScheduleId"]
    return after, applied()


def _invert_change_humidistat_dehumidifying_setpoint_schedule(before, payload):
    """↩️ Its own inverse, with the old dehumidifying setpoint schedule read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-dehumidifying-setpoint-schedule", {"id": payload["id"], "newDehumidifyingSetpointScheduleId": item["dehumidifying_setpoint_schedule_id"]})]


def change_humidistat_humidifying_throttle_range(before, payload):
    """🌧️ `change-humidistat-humidifying-throttle-range` — the humidifying throttle range of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newHumidifyingThrottleRange"] != payload["newHumidifyingThrottleRange"] or payload["newHumidifyingThrottleRange"] in (float("inf"), float("-inf")) or payload["newHumidifyingThrottleRange"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["humidifying_throttle_range"] == payload["newHumidifyingThrottleRange"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["humidifying_throttle_range"] = payload["newHumidifyingThrottleRange"]
    return after, applied()


def _invert_change_humidistat_humidifying_throttle_range(before, payload):
    """↩️ Its own inverse, with the old humidifying throttle range read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-humidifying-throttle-range", {"id": payload["id"], "newHumidifyingThrottleRange": item["humidifying_throttle_range"]})]


def change_humidistat_dehumidifying_throttle_range(before, payload):
    """🧻️ `change-humidistat-dehumidifying-throttle-range` — the dehumidifying throttle range of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newDehumidifyingThrottleRange"] != payload["newDehumidifyingThrottleRange"] or payload["newDehumidifyingThrottleRange"] in (float("inf"), float("-inf")) or payload["newDehumidifyingThrottleRange"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["dehumidifying_throttle_range"] == payload["newDehumidifyingThrottleRange"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["dehumidifying_throttle_range"] = payload["newDehumidifyingThrottleRange"]
    return after, applied()


def _invert_change_humidistat_dehumidifying_throttle_range(before, payload):
    """↩️ Its own inverse, with the old dehumidifying throttle range read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-dehumidifying-throttle-range", {"id": payload["id"], "newDehumidifyingThrottleRange": item["dehumidifying_throttle_range"]})]


def create_ideal_loads_system(before, payload):
    """🫁️ `create-ideal-loads-system` — one new ideal loads system row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["ideal_loads"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["maxHeatingSupplyAirTempC"] != payload["maxHeatingSupplyAirTempC"] or payload["maxHeatingSupplyAirTempC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["maxHeatingSupplyAirTempC"] <= 200.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["minCoolingSupplyAirTempC"] != payload["minCoolingSupplyAirTempC"] or payload["minCoolingSupplyAirTempC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["minCoolingSupplyAirTempC"] <= 200.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["outdoorAirPerPersonM3S"] != payload["outdoorAirPerPersonM3S"] or payload["outdoorAirPerPersonM3S"] in (float("inf"), float("-inf")) or payload["outdoorAirPerPersonM3S"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["outdoorAirPerAreaM3SM2"] != payload["outdoorAirPerAreaM3SM2"] or payload["outdoorAirPerAreaM3SM2"] in (float("inf"), float("-inf")) or payload["outdoorAirPerAreaM3SM2"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not payload["maxHeatingCapacityPresent"] and payload["maxHeatingCapacityW"] != 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not payload["maxCoolingCapacityPresent"] and payload["maxCoolingCapacityW"] != 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["maxHeatingCapacityPresent"] and (payload["maxHeatingCapacityW"] != payload["maxHeatingCapacityW"] or payload["maxHeatingCapacityW"] in (float("inf"), float("-inf")) or payload["maxHeatingCapacityW"] <= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["maxCoolingCapacityPresent"] and (payload["maxCoolingCapacityW"] != payload["maxCoolingCapacityW"] or payload["maxCoolingCapacityW"] in (float("inf"), float("-inf")) or payload["maxCoolingCapacityW"] <= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["ideal_loads"].append({"id": payload["id"], "zone_id": payload["zoneId"], "max_heating_supply_air_temp_c": payload["maxHeatingSupplyAirTempC"], "min_cooling_supply_air_temp_c": payload["minCoolingSupplyAirTempC"], "max_heating_capacity_w": payload["maxHeatingCapacityW"] if payload["maxHeatingCapacityPresent"] else None, "max_cooling_capacity_w": payload["maxCoolingCapacityW"] if payload["maxCoolingCapacityPresent"] else None, "outdoor_air_per_person_m3_s": payload["outdoorAirPerPersonM3S"], "outdoor_air_per_area_m3_s_m2": payload["outdoorAirPerAreaM3SM2"]})
    return after, applied()


def _invert_create_ideal_loads_system(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-ideal-loads-system", {"id": payload["id"]})]


def delete_ideal_loads_system(before, payload):
    """🫥️ `delete-ideal-loads-system` — drops the addressed ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["ideal_loads"] = [entry for entry in after["model"]["ideal_loads"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_ideal_loads_system(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("create-ideal-loads-system", {"id": item["id"], "zoneId": item["zone_id"], "maxHeatingSupplyAirTempC": item["max_heating_supply_air_temp_c"], "minCoolingSupplyAirTempC": item["min_cooling_supply_air_temp_c"], "maxHeatingCapacityPresent": item["max_heating_capacity_w"] is not None, "maxHeatingCapacityW": item["max_heating_capacity_w"] if item["max_heating_capacity_w"] is not None else 0.0, "maxCoolingCapacityPresent": item["max_cooling_capacity_w"] is not None, "maxCoolingCapacityW": item["max_cooling_capacity_w"] if item["max_cooling_capacity_w"] is not None else 0.0, "outdoorAirPerPersonM3S": item["outdoor_air_per_person_m3_s"], "outdoorAirPerAreaM3SM2": item["outdoor_air_per_area_m3_s_m2"]})]


def change_ideal_loads_system_zone(before, payload):
    """🏢️ `change-ideal-loads-system-zone` — the zone of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_ideal_loads_system_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_ideal_loads_system_max_heating_supply_air_temp(before, payload):
    """🔴️ `change-ideal-loads-system-max-heating-supply-air-temp` — the maximum heating supply air temperature of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newMaxHeatingSupplyAirTempC"] != payload["newMaxHeatingSupplyAirTempC"] or payload["newMaxHeatingSupplyAirTempC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["newMaxHeatingSupplyAirTempC"] <= 200.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["max_heating_supply_air_temp_c"] == payload["newMaxHeatingSupplyAirTempC"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["max_heating_supply_air_temp_c"] = payload["newMaxHeatingSupplyAirTempC"]
    return after, applied()


def _invert_change_ideal_loads_system_max_heating_supply_air_temp(before, payload):
    """↩️ Its own inverse, with the old maximum heating supply air temperature read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-max-heating-supply-air-temp", {"id": payload["id"], "newMaxHeatingSupplyAirTempC": item["max_heating_supply_air_temp_c"]})]


def change_ideal_loads_system_min_cooling_supply_air_temp(before, payload):
    """🔵️ `change-ideal-loads-system-min-cooling-supply-air-temp` — the minimum cooling supply air temperature of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newMinCoolingSupplyAirTempC"] != payload["newMinCoolingSupplyAirTempC"] or payload["newMinCoolingSupplyAirTempC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["newMinCoolingSupplyAirTempC"] <= 200.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["min_cooling_supply_air_temp_c"] == payload["newMinCoolingSupplyAirTempC"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["min_cooling_supply_air_temp_c"] = payload["newMinCoolingSupplyAirTempC"]
    return after, applied()


def _invert_change_ideal_loads_system_min_cooling_supply_air_temp(before, payload):
    """↩️ Its own inverse, with the old minimum cooling supply air temperature read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-min-cooling-supply-air-temp", {"id": payload["id"], "newMinCoolingSupplyAirTempC": item["min_cooling_supply_air_temp_c"]})]


def change_ideal_loads_system_max_heating_capacity(before, payload):
    """⛽️ `change-ideal-loads-system-max-heating-capacity` — the optional maximum heating capacity of one ideal loads system row, carried as a present
    flag beside its value because `Option<T>` has no `dsl::DslField`."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newCapacityPresent"] and payload["newMaxHeatingCapacityW"] != 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newCapacityPresent"] and (payload["newMaxHeatingCapacityW"] != payload["newMaxHeatingCapacityW"] or payload["newMaxHeatingCapacityW"] in (float("inf"), float("-inf")) or payload["newMaxHeatingCapacityW"] <= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    value = payload["newMaxHeatingCapacityW"] if payload["newCapacityPresent"] else None
    if item["max_heating_capacity_w"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["max_heating_capacity_w"] = value
    return after, applied()


def _invert_change_ideal_loads_system_max_heating_capacity(before, payload):
    """↩️ Its own inverse, with the old maximum heating capacity read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    old = item["max_heating_capacity_w"]
    return [("change-ideal-loads-system-max-heating-capacity", {"id": payload["id"], "newCapacityPresent": old is not None, "newMaxHeatingCapacityW": old if old is not None else 0.0})]


def change_ideal_loads_system_max_cooling_capacity(before, payload):
    """🟧️ `change-ideal-loads-system-max-cooling-capacity` — the optional maximum cooling capacity of one ideal loads system row, carried as a present
    flag beside its value because `Option<T>` has no `dsl::DslField`."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newCapacityPresent"] and payload["newMaxCoolingCapacityW"] != 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newCapacityPresent"] and (payload["newMaxCoolingCapacityW"] != payload["newMaxCoolingCapacityW"] or payload["newMaxCoolingCapacityW"] in (float("inf"), float("-inf")) or payload["newMaxCoolingCapacityW"] <= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    value = payload["newMaxCoolingCapacityW"] if payload["newCapacityPresent"] else None
    if item["max_cooling_capacity_w"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["max_cooling_capacity_w"] = value
    return after, applied()


def _invert_change_ideal_loads_system_max_cooling_capacity(before, payload):
    """↩️ Its own inverse, with the old maximum cooling capacity read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    old = item["max_cooling_capacity_w"]
    return [("change-ideal-loads-system-max-cooling-capacity", {"id": payload["id"], "newCapacityPresent": old is not None, "newMaxCoolingCapacityW": old if old is not None else 0.0})]


def change_ideal_loads_system_outdoor_air_per_person(before, payload):
    """🧍️ `change-ideal-loads-system-outdoor-air-per-person` — the outdoor air rate per person of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newOutdoorAirPerPersonM3S"] != payload["newOutdoorAirPerPersonM3S"] or payload["newOutdoorAirPerPersonM3S"] in (float("inf"), float("-inf")) or payload["newOutdoorAirPerPersonM3S"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["outdoor_air_per_person_m3_s"] == payload["newOutdoorAirPerPersonM3S"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["outdoor_air_per_person_m3_s"] = payload["newOutdoorAirPerPersonM3S"]
    return after, applied()


def _invert_change_ideal_loads_system_outdoor_air_per_person(before, payload):
    """↩️ Its own inverse, with the old outdoor air rate per person read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-outdoor-air-per-person", {"id": payload["id"], "newOutdoorAirPerPersonM3S": item["outdoor_air_per_person_m3_s"]})]


def change_ideal_loads_system_outdoor_air_per_area(before, payload):
    """🔳️ `change-ideal-loads-system-outdoor-air-per-area` — the outdoor air rate per floor area of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newOutdoorAirPerAreaM3SM2"] != payload["newOutdoorAirPerAreaM3SM2"] or payload["newOutdoorAirPerAreaM3SM2"] in (float("inf"), float("-inf")) or payload["newOutdoorAirPerAreaM3SM2"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["outdoor_air_per_area_m3_s_m2"] == payload["newOutdoorAirPerAreaM3SM2"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["outdoor_air_per_area_m3_s_m2"] = payload["newOutdoorAirPerAreaM3SM2"]
    return after, applied()


def _invert_change_ideal_loads_system_outdoor_air_per_area(before, payload):
    """↩️ Its own inverse, with the old outdoor air rate per floor area read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-outdoor-air-per-area", {"id": payload["id"], "newOutdoorAirPerAreaM3SM2": item["outdoor_air_per_area_m3_s_m2"]})]


def create_zone_equipment(before, payload):
    """🛠️ `create-zone-equipment` — one new zone equipment row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["zone_equipment"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["priority"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["heatingCapacityW"] != payload["heatingCapacityW"] or payload["heatingCapacityW"] in (float("inf"), float("-inf")) or payload["heatingCapacityW"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["coolingCapacityW"] != payload["coolingCapacityW"] or payload["coolingCapacityW"] in (float("inf"), float("-inf")) or payload["coolingCapacityW"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["zone_equipment"].append({"id": payload["id"], "zone_id": payload["zoneId"], "equipment_type": payload["equipmentType"], "priority": payload["priority"], "heating_capacity_w": payload["heatingCapacityW"], "cooling_capacity_w": payload["coolingCapacityW"]})
    return after, applied()


def _invert_create_zone_equipment(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-zone-equipment", {"id": payload["id"]})]


def delete_zone_equipment(before, payload):
    """🗑️ `delete-zone-equipment` — drops the addressed zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["zone_equipment"] = [entry for entry in after["model"]["zone_equipment"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_zone_equipment(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("create-zone-equipment", {"id": item["id"], "zoneId": item["zone_id"], "equipmentType": item["equipment_type"], "priority": item["priority"], "heatingCapacityW": item["heating_capacity_w"], "coolingCapacityW": item["cooling_capacity_w"]})]


def change_zone_equipment_zone(before, payload):
    """🏬️ `change-zone-equipment-zone` — the zone of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_zone_equipment_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_zone_equipment_type(before, payload):
    """🔧️ `change-zone-equipment-type` — the equipment type of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["equipment_type"] == payload["newEquipmentType"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["equipment_type"] = payload["newEquipmentType"]
    return after, applied()


def _invert_change_zone_equipment_type(before, payload):
    """↩️ Its own inverse, with the old equipment type read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-type", {"id": payload["id"], "newEquipmentType": item["equipment_type"]})]


def change_zone_equipment_priority(before, payload):
    """🎗️ `change-zone-equipment-priority` — the priority of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newPriority"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["priority"] == payload["newPriority"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["priority"] = payload["newPriority"]
    return after, applied()


def _invert_change_zone_equipment_priority(before, payload):
    """↩️ Its own inverse, with the old priority read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-priority", {"id": payload["id"], "newPriority": item["priority"]})]


def change_zone_equipment_heating_capacity(before, payload):
    """🧇️ `change-zone-equipment-heating-capacity` — the heating capacity of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newHeatingCapacityW"] != payload["newHeatingCapacityW"] or payload["newHeatingCapacityW"] in (float("inf"), float("-inf")) or payload["newHeatingCapacityW"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["heating_capacity_w"] == payload["newHeatingCapacityW"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["heating_capacity_w"] = payload["newHeatingCapacityW"]
    return after, applied()


def _invert_change_zone_equipment_heating_capacity(before, payload):
    """↩️ Its own inverse, with the old heating capacity read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-heating-capacity", {"id": payload["id"], "newHeatingCapacityW": item["heating_capacity_w"]})]


def change_zone_equipment_cooling_capacity(before, payload):
    """🍧️ `change-zone-equipment-cooling-capacity` — the cooling capacity of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newCoolingCapacityW"] != payload["newCoolingCapacityW"] or payload["newCoolingCapacityW"] in (float("inf"), float("-inf")) or payload["newCoolingCapacityW"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["cooling_capacity_w"] == payload["newCoolingCapacityW"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["cooling_capacity_w"] = payload["newCoolingCapacityW"]
    return after, applied()


def _invert_change_zone_equipment_cooling_capacity(before, payload):
    """↩️ Its own inverse, with the old cooling capacity read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-cooling-capacity", {"id": payload["id"], "newCoolingCapacityW": item["cooling_capacity_w"]})]


def create_daylight_zone(before, payload):
    """🔭️ `create-daylight-zone` — one new daylight zone row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["daylight_zones"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["illuminanceTargetLux"] != payload["illuminanceTargetLux"] or payload["illuminanceTargetLux"] in (float("inf"), float("-inf")) or payload["illuminanceTargetLux"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["glareLimit"] != payload["glareLimit"] or payload["glareLimit"] in (float("inf"), float("-inf")) or payload["glareLimit"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["windowTransmittance"] != payload["windowTransmittance"] or payload["windowTransmittance"] in (float("inf"), float("-inf")) or not 0.0 <= payload["windowTransmittance"] <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["daylight_zones"].append({"id": payload["id"], "zone_id": payload["zoneId"], "illuminance_target_lux": payload["illuminanceTargetLux"], "glare_limit": payload["glareLimit"], "window_transmittance": payload["windowTransmittance"]})
    return after, applied()


def _invert_create_daylight_zone(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-daylight-zone", {"id": payload["id"]})]


def delete_daylight_zone(before, payload):
    """🌗️ `delete-daylight-zone` — drops the addressed daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["daylight_zones"] = [entry for entry in after["model"]["daylight_zones"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_daylight_zone(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"])
    return [("create-daylight-zone", {"id": item["id"], "zoneId": item["zone_id"], "illuminanceTargetLux": item["illuminance_target_lux"], "glareLimit": item["glare_limit"], "windowTransmittance": item["window_transmittance"]})]


def change_daylight_zone_zone(before, payload):
    """🏫️ `change-daylight-zone-zone` — the zone of one daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["daylight_zones"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_daylight_zone_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"])
    return [("change-daylight-zone-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_daylight_zone_illuminance_target(before, payload):
    """🪔️ `change-daylight-zone-illuminance-target` — the illuminance target of one daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newIlluminanceTargetLux"] != payload["newIlluminanceTargetLux"] or payload["newIlluminanceTargetLux"] in (float("inf"), float("-inf")) or payload["newIlluminanceTargetLux"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["illuminance_target_lux"] == payload["newIlluminanceTargetLux"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["daylight_zones"]:
        if entry["id"] == payload["id"]:
            entry["illuminance_target_lux"] = payload["newIlluminanceTargetLux"]
    return after, applied()


def _invert_change_daylight_zone_illuminance_target(before, payload):
    """↩️ Its own inverse, with the old illuminance target read off BASE."""
    item = next(entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"])
    return [("change-daylight-zone-illuminance-target", {"id": payload["id"], "newIlluminanceTargetLux": item["illuminance_target_lux"]})]


def change_daylight_zone_glare_limit(before, payload):
    """🕶️ `change-daylight-zone-glare-limit` — the glare limit of one daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newGlareLimit"] != payload["newGlareLimit"] or payload["newGlareLimit"] in (float("inf"), float("-inf")) or payload["newGlareLimit"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["glare_limit"] == payload["newGlareLimit"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["daylight_zones"]:
        if entry["id"] == payload["id"]:
            entry["glare_limit"] = payload["newGlareLimit"]
    return after, applied()


def _invert_change_daylight_zone_glare_limit(before, payload):
    """↩️ Its own inverse, with the old glare limit read off BASE."""
    item = next(entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"])
    return [("change-daylight-zone-glare-limit", {"id": payload["id"], "newGlareLimit": item["glare_limit"]})]


def change_daylight_zone_window_transmittance(before, payload):
    """🥃️ `change-daylight-zone-window-transmittance` — the window transmittance of one daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newWindowTransmittance"] != payload["newWindowTransmittance"] or payload["newWindowTransmittance"] in (float("inf"), float("-inf")) or not 0.0 <= payload["newWindowTransmittance"] <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["window_transmittance"] == payload["newWindowTransmittance"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["daylight_zones"]:
        if entry["id"] == payload["id"]:
            entry["window_transmittance"] = payload["newWindowTransmittance"]
    return after, applied()


def _invert_change_daylight_zone_window_transmittance(before, payload):
    """↩️ Its own inverse, with the old window transmittance read off BASE."""
    item = next(entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"])
    return [("change-daylight-zone-window-transmittance", {"id": payload["id"], "newWindowTransmittance": item["window_transmittance"]})]


def create_sizing_object(before, payload):
    """📶️ `create-sizing-object` — one new sizing object row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["sizing_objects"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    after = copy.deepcopy(before)
    after["model"]["sizing_objects"].append({"id": payload["id"], "zone_id": payload["zoneId"], "sizing_type": payload["sizingType"], "design_day_type": payload["designDayType"]})
    return after, applied()


def _invert_create_sizing_object(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-sizing-object", {"id": payload["id"]})]


def delete_sizing_object(before, payload):
    """🪒️ `delete-sizing-object` — drops the addressed sizing object row."""
    item = next((entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["sizing_objects"] = [entry for entry in after["model"]["sizing_objects"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_sizing_object(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"])
    return [("create-sizing-object", {"id": item["id"], "zoneId": item["zone_id"], "sizingType": item["sizing_type"], "designDayType": item["design_day_type"]})]


def change_sizing_object_zone(before, payload):
    """🏨️ `change-sizing-object-zone` — the zone of one sizing object row."""
    item = next((entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["sizing_objects"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_sizing_object_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"])
    return [("change-sizing-object-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_sizing_object_sizing_type(before, payload):
    """🧾️ `change-sizing-object-sizing-type` — the sizing type of one sizing object row."""
    item = next((entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["sizing_type"] == payload["newSizingType"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["sizing_objects"]:
        if entry["id"] == payload["id"]:
            entry["sizing_type"] = payload["newSizingType"]
    return after, applied()


def _invert_change_sizing_object_sizing_type(before, payload):
    """↩️ Its own inverse, with the old sizing type read off BASE."""
    item = next(entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"])
    return [("change-sizing-object-sizing-type", {"id": payload["id"], "newSizingType": item["sizing_type"]})]


def change_sizing_object_design_day_type(before, payload):
    """🌥️ `change-sizing-object-design-day-type` — the design day type of one sizing object row."""
    item = next((entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["design_day_type"] == payload["newDesignDayType"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["sizing_objects"]:
        if entry["id"] == payload["id"]:
            entry["design_day_type"] = payload["newDesignDayType"]
    return after, applied()


def _invert_change_sizing_object_design_day_type(before, payload):
    """↩️ Its own inverse, with the old design day type read off BASE."""
    item = next(entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"])
    return [("change-sizing-object-design-day-type", {"id": payload["id"], "newDesignDayType": item["design_day_type"]})]


def create_room_air_model_assignment(before, payload):
    """🛏️ `create-room-air-model-assignment` — one new room air model assignment row under a caller-minted id."""
    if any(entry["zone_id"] == payload["zoneId"] for entry in before["model"]["room_air_models"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["zoneId"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    after = copy.deepcopy(before)
    after["model"]["room_air_models"].append({"zone_id": payload["zoneId"], "model": payload["model"]})
    return after, applied()


def _invert_create_room_air_model_assignment(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-room-air-model-assignment", {"zoneId": payload["zoneId"]})]


def delete_room_air_model_assignment(before, payload):
    """🧺️ `delete-room-air-model-assignment` — drops the addressed room air model assignment row."""
    item = next((entry for entry in before["model"]["room_air_models"] if entry["zone_id"] == payload["zoneId"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])

    after = copy.deepcopy(before)
    after["model"]["room_air_models"] = [entry for entry in after["model"]["room_air_models"] if entry["zone_id"] != payload["zoneId"]]
    return after, applied()


def _invert_delete_room_air_model_assignment(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["room_air_models"] if entry["zone_id"] == payload["zoneId"])
    return [("create-room-air-model-assignment", {"zoneId": item["zone_id"], "model": item["model"]})]


def change_room_air_model(before, payload):
    """🪭️ `change-room-air-model` — the room air model of one room air model assignment row."""
    item = next((entry for entry in before["model"]["room_air_models"] if entry["zone_id"] == payload["zoneId"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])

    if item["model"] == payload["newModel"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["room_air_models"]:
        if entry["zone_id"] == payload["zoneId"]:
            entry["model"] = payload["newModel"]
    return after, applied()


def _invert_change_room_air_model(before, payload):
    """↩️ Its own inverse, with the old room air model read off BASE."""
    item = next(entry for entry in before["model"]["room_air_models"] if entry["zone_id"] == payload["zoneId"])
    return [("change-room-air-model", {"zoneId": payload["zoneId"], "newModel": item["model"]})]


def create_setpoint_manager(before, payload):
    """📌️ `create-setpoint-manager` — one new setpoint manager row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["setpoint_managers"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["name"] == payload["name"] for entry in before["model"]["setpoint_managers"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["kind"] not in ("Scheduled", "OutdoorAirReset", "WarmestZone", "ColdestZone"):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["kind"] != "OutdoorAirReset" and not (payload["lowOutdoorC"] == 0.0 and payload["highOutdoorC"] == 0.0 and payload["lowSetpointC"] == 0.0 and payload["highSetpointC"] == 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["kind"] == "OutdoorAirReset" and payload["highOutdoorC"] <= payload["lowOutdoorC"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not payload["schedulePresent"] and payload["scheduleId"] != 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["schedulePresent"] and (not any(entry["id"] == payload["scheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family])):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["setpoint_managers"].append({"id": payload["id"], "name": payload["name"], "kind": ({"OutdoorAirReset": {"low_outdoor_c": payload["lowOutdoorC"], "high_outdoor_c": payload["highOutdoorC"], "low_setpoint_c": payload["lowSetpointC"], "high_setpoint_c": payload["highSetpointC"]}} if payload["kind"] == "OutdoorAirReset" else payload["kind"]), "schedule_id": (payload["scheduleId"] if payload["schedulePresent"] else None)})
    return after, applied()


def _invert_create_setpoint_manager(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-setpoint-manager", {"id": payload["id"]})]


def delete_setpoint_manager(before, payload):
    """🍄️ `delete-setpoint-manager` — drops the addressed setpoint manager row."""
    item = next((entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["setpoint_managers"] = [entry for entry in after["model"]["setpoint_managers"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_setpoint_manager(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"])
    return [("create-setpoint-manager", {"id": item["id"], "name": item["name"], "kind": ("OutdoorAirReset" if isinstance(item["kind"], dict) else item["kind"]), "lowOutdoorC": (item["kind"]["OutdoorAirReset"]["low_outdoor_c"] if isinstance(item["kind"], dict) else 0.0), "highOutdoorC": (item["kind"]["OutdoorAirReset"]["high_outdoor_c"] if isinstance(item["kind"], dict) else 0.0), "lowSetpointC": (item["kind"]["OutdoorAirReset"]["low_setpoint_c"] if isinstance(item["kind"], dict) else 0.0), "highSetpointC": (item["kind"]["OutdoorAirReset"]["high_setpoint_c"] if isinstance(item["kind"], dict) else 0.0), "schedulePresent": item["schedule_id"] is not None, "scheduleId": (item["schedule_id"] if item["schedule_id"] is not None else 0)})]


def rename_setpoint_manager(before, payload):
    """🖇️ `rename-setpoint-manager` — the name of one setpoint manager row."""
    item = next((entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newName"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["id"] != payload["id"] and entry["name"] == payload["newName"] for entry in before["model"]["setpoint_managers"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == payload["newName"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["setpoint_managers"]:
        if entry["id"] == payload["id"]:
            entry["name"] = payload["newName"]
    return after, applied()


def _invert_rename_setpoint_manager(before, payload):
    """↩️ Its own inverse, with the old name read off BASE."""
    item = next(entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"])
    return [("rename-setpoint-manager", {"id": payload["id"], "newName": item["name"]})]


def replace_setpoint_manager_kind(before, payload):
    """🔀️ `replace-setpoint-manager-kind` — the whole tagged control law of one manager."""
    item = next((entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newKind"] not in ("Scheduled", "OutdoorAirReset", "WarmestZone", "ColdestZone"):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newKind"] != "OutdoorAirReset" and not (payload["newLowOutdoorC"] == 0.0 and payload["newHighOutdoorC"] == 0.0 and payload["newLowSetpointC"] == 0.0 and payload["newHighSetpointC"] == 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newKind"] == "OutdoorAirReset" and payload["newHighOutdoorC"] <= payload["newLowOutdoorC"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    kind = ({"OutdoorAirReset": {"low_outdoor_c": payload["newLowOutdoorC"], "high_outdoor_c": payload["newHighOutdoorC"], "low_setpoint_c": payload["newLowSetpointC"], "high_setpoint_c": payload["newHighSetpointC"]}} if payload["newKind"] == "OutdoorAirReset" else payload["newKind"])
    if item["kind"] == kind:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["setpoint_managers"]:
        if entry["id"] == payload["id"]:
            entry["kind"] = kind
    return after, applied()


def _invert_replace_setpoint_manager_kind(before, payload):
    """↩️ Its own inverse, with the old control law flattened back off BASE."""
    item = next(entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"])
    old = item["kind"]
    reset = old["OutdoorAirReset"] if isinstance(old, dict) else None
    return [("replace-setpoint-manager-kind", {"id": payload["id"], "newKind": "OutdoorAirReset" if reset is not None else old, "newLowOutdoorC": reset["low_outdoor_c"] if reset is not None else 0.0, "newHighOutdoorC": reset["high_outdoor_c"] if reset is not None else 0.0, "newLowSetpointC": reset["low_setpoint_c"] if reset is not None else 0.0, "newHighSetpointC": reset["high_setpoint_c"] if reset is not None else 0.0})]


def change_setpoint_manager_schedule(before, payload):
    """🎼️ `change-setpoint-manager-schedule` — the optional schedule reference of one manager."""
    item = next((entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newSchedulePresent"] and payload["newScheduleId"] != 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newSchedulePresent"] and (not any(entry["id"] == payload["newScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family])):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newScheduleId"])])
    value = payload["newScheduleId"] if payload["newSchedulePresent"] else None
    if item["schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["setpoint_managers"]:
        if entry["id"] == payload["id"]:
            entry["schedule_id"] = value
    return after, applied()


def _invert_change_setpoint_manager_schedule(before, payload):
    """↩️ Its own inverse, with the old reference read off BASE."""
    item = next(entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"])
    old = item["schedule_id"]
    return [("change-setpoint-manager-schedule", {"id": payload["id"], "newSchedulePresent": old is not None, "newScheduleId": old if old is not None else 0})]


def create_air_loop(before, payload):
    """🛞️ `create-air-loop` — one new air loop row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["name"] == payload["name"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["supplyNodeId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["returnNodeId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["designSupplyAirFlowM3S"] != payload["designSupplyAirFlowM3S"] or payload["designSupplyAirFlowM3S"] in (float("inf"), float("-inf")) or payload["designSupplyAirFlowM3S"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(payload["terminalZoneIds"][index] >= payload["terminalZoneIds"][index + 1] for index in range(len(payload["terminalZoneIds"]) - 1)):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(not any(zone["id"] == value for zone in before["model"]["zones"]) for value in payload["terminalZoneIds"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["air_loops"].append({"id": payload["id"], "name": payload["name"], "supply_node_id": payload["supplyNodeId"], "return_node_id": payload["returnNodeId"], "design_supply_air_flow_m3_s": payload["designSupplyAirFlowM3S"], "terminal_zone_ids": payload["terminalZoneIds"]})
    return after, applied()


def _invert_create_air_loop(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-air-loop", {"id": payload["id"]})]


def delete_air_loop(before, payload):
    """🥀️ `delete-air-loop` — drops the addressed air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(system["air_loop_id"] == payload["id"] for system in before["model"]["outdoor_air_systems"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["air_loops"] = [entry for entry in after["model"]["air_loops"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_air_loop(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("create-air-loop", {"id": item["id"], "name": item["name"], "supplyNodeId": item["supply_node_id"], "returnNodeId": item["return_node_id"], "designSupplyAirFlowM3S": item["design_supply_air_flow_m3_s"], "terminalZoneIds": item["terminal_zone_ids"]})]


def rename_air_loop(before, payload):
    """📇️ `rename-air-loop` — the name of one air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newName"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["id"] != payload["id"] and entry["name"] == payload["newName"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == payload["newName"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["name"] = payload["newName"]
    return after, applied()


def _invert_rename_air_loop(before, payload):
    """↩️ Its own inverse, with the old name read off BASE."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("rename-air-loop", {"id": payload["id"], "newName": item["name"]})]


def change_air_loop_supply_node(before, payload):
    """↗️ `change-air-loop-supply-node` — the supply node of one air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newSupplyNodeId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["supply_node_id"] == payload["newSupplyNodeId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["supply_node_id"] = payload["newSupplyNodeId"]
    return after, applied()


def _invert_change_air_loop_supply_node(before, payload):
    """↩️ Its own inverse, with the old supply node read off BASE."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("change-air-loop-supply-node", {"id": payload["id"], "newSupplyNodeId": item["supply_node_id"]})]


def change_air_loop_return_node(before, payload):
    """↘️ `change-air-loop-return-node` — the return node of one air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newReturnNodeId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["return_node_id"] == payload["newReturnNodeId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["return_node_id"] = payload["newReturnNodeId"]
    return after, applied()


def _invert_change_air_loop_return_node(before, payload):
    """↩️ Its own inverse, with the old return node read off BASE."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("change-air-loop-return-node", {"id": payload["id"], "newReturnNodeId": item["return_node_id"]})]


def change_air_loop_design_supply_air_flow(before, payload):
    """🍥️ `change-air-loop-design-supply-air-flow` — the design supply air flow of one air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newDesignSupplyAirFlowM3S"] != payload["newDesignSupplyAirFlowM3S"] or payload["newDesignSupplyAirFlowM3S"] in (float("inf"), float("-inf")) or payload["newDesignSupplyAirFlowM3S"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_supply_air_flow_m3_s"] == payload["newDesignSupplyAirFlowM3S"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["design_supply_air_flow_m3_s"] = payload["newDesignSupplyAirFlowM3S"]
    return after, applied()


def _invert_change_air_loop_design_supply_air_flow(before, payload):
    """↩️ Its own inverse, with the old design supply air flow read off BASE."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("change-air-loop-design-supply-air-flow", {"id": payload["id"], "newDesignSupplyAirFlowM3S": item["design_supply_air_flow_m3_s"]})]


def add_air_loop_terminal_zone(before, payload):
    """🪺️ `add-air-loop-terminal-zone` — one terminal zone id into the ascending membership list."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["zoneId"] in item["terminal_zone_ids"]:
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["zoneId"])])
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            members = entry["terminal_zone_ids"]
            position = next((index for index, value in enumerate(members) if value > payload["zoneId"]), len(members))
            members.insert(position, payload["zoneId"])
    return after, applied()


def _invert_add_air_loop_terminal_zone(before, payload):
    """↩️ The undo of an add is the removal of exactly that member."""
    return [("remove-air-loop-terminal-zone", {"id": payload["id"], "zoneId": payload["zoneId"]})]


def remove_air_loop_terminal_zone(before, payload):
    """🪹️ `remove-air-loop-terminal-zone` — one terminal zone id out of the membership list."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["zoneId"] not in item["terminal_zone_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["terminal_zone_ids"] = [value for value in entry["terminal_zone_ids"] if value != payload["zoneId"]]
    return after, applied()


def _invert_remove_air_loop_terminal_zone(before, payload):
    """↩️ The undo of a removal puts the member back at its ascending position."""
    return [("add-air-loop-terminal-zone", {"id": payload["id"], "zoneId": payload["zoneId"]})]


def create_plant_loop(before, payload):
    """⚗️ `create-plant-loop` — one new plant loop row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["plant_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["name"] == payload["name"] for entry in before["model"]["plant_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["supplyTemperatureC"] != payload["supplyTemperatureC"] or payload["supplyTemperatureC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["supplyTemperatureC"] <= 300.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["returnTemperatureC"] != payload["returnTemperatureC"] or payload["returnTemperatureC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["returnTemperatureC"] <= 300.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["designFlowKgS"] != payload["designFlowKgS"] or payload["designFlowKgS"] in (float("inf"), float("-inf")) or payload["designFlowKgS"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(payload["equipmentIds"][index] >= payload["equipmentIds"][index + 1] for index in range(len(payload["equipmentIds"]) - 1)):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(value == 0 for value in payload["equipmentIds"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["plant_loops"].append({"id": payload["id"], "name": payload["name"], "loop_type": payload["loopType"], "supply_temperature_c": payload["supplyTemperatureC"], "return_temperature_c": payload["returnTemperatureC"], "design_flow_kg_s": payload["designFlowKgS"], "equipment_ids": payload["equipmentIds"]})
    return after, applied()


def _invert_create_plant_loop(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-plant-loop", {"id": payload["id"]})]


def delete_plant_loop(before, payload):
    """💣️ `delete-plant-loop` — drops the addressed plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["plant_loops"] = [entry for entry in after["model"]["plant_loops"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_plant_loop(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("create-plant-loop", {"id": item["id"], "name": item["name"], "loopType": item["loop_type"], "supplyTemperatureC": item["supply_temperature_c"], "returnTemperatureC": item["return_temperature_c"], "designFlowKgS": item["design_flow_kg_s"], "equipmentIds": item["equipment_ids"]})]


def rename_plant_loop(before, payload):
    """📛️ `rename-plant-loop` — the name of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newName"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["id"] != payload["id"] and entry["name"] == payload["newName"] for entry in before["model"]["plant_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == payload["newName"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["name"] = payload["newName"]
    return after, applied()


def _invert_rename_plant_loop(before, payload):
    """↩️ Its own inverse, with the old name read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("rename-plant-loop", {"id": payload["id"], "newName": item["name"]})]


def change_plant_loop_type(before, payload):
    """♻️ `change-plant-loop-type` — the loop type of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["loop_type"] == payload["newLoopType"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["loop_type"] = payload["newLoopType"]
    return after, applied()


def _invert_change_plant_loop_type(before, payload):
    """↩️ Its own inverse, with the old loop type read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("change-plant-loop-type", {"id": payload["id"], "newLoopType": item["loop_type"]})]


def change_plant_loop_supply_temperature(before, payload):
    """☕️ `change-plant-loop-supply-temperature` — the supply temperature of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newSupplyTemperatureC"] != payload["newSupplyTemperatureC"] or payload["newSupplyTemperatureC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["newSupplyTemperatureC"] <= 300.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["supply_temperature_c"] == payload["newSupplyTemperatureC"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["supply_temperature_c"] = payload["newSupplyTemperatureC"]
    return after, applied()


def _invert_change_plant_loop_supply_temperature(before, payload):
    """↩️ Its own inverse, with the old supply temperature read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("change-plant-loop-supply-temperature", {"id": payload["id"], "newSupplyTemperatureC": item["supply_temperature_c"]})]


def change_plant_loop_return_temperature(before, payload):
    """🧫️ `change-plant-loop-return-temperature` — the return temperature of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newReturnTemperatureC"] != payload["newReturnTemperatureC"] or payload["newReturnTemperatureC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["newReturnTemperatureC"] <= 300.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["return_temperature_c"] == payload["newReturnTemperatureC"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["return_temperature_c"] = payload["newReturnTemperatureC"]
    return after, applied()


def _invert_change_plant_loop_return_temperature(before, payload):
    """↩️ Its own inverse, with the old return temperature read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("change-plant-loop-return-temperature", {"id": payload["id"], "newReturnTemperatureC": item["return_temperature_c"]})]


def change_plant_loop_design_flow(before, payload):
    """🚤️ `change-plant-loop-design-flow` — the design mass flow of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newDesignFlowKgS"] != payload["newDesignFlowKgS"] or payload["newDesignFlowKgS"] in (float("inf"), float("-inf")) or payload["newDesignFlowKgS"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_flow_kg_s"] == payload["newDesignFlowKgS"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["design_flow_kg_s"] = payload["newDesignFlowKgS"]
    return after, applied()


def _invert_change_plant_loop_design_flow(before, payload):
    """↩️ Its own inverse, with the old design mass flow read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("change-plant-loop-design-flow", {"id": payload["id"], "newDesignFlowKgS": item["design_flow_kg_s"]})]


def add_plant_loop_equipment(before, payload):
    """🔩️ `add-plant-loop-equipment` — one plant equipment id into the ascending membership list."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["equipmentId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["equipmentId"])])
    if payload["equipmentId"] in item["equipment_ids"]:
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["equipmentId"])])
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            members = entry["equipment_ids"]
            position = next((index for index, value in enumerate(members) if value > payload["equipmentId"]), len(members))
            members.insert(position, payload["equipmentId"])
    return after, applied()


def _invert_add_plant_loop_equipment(before, payload):
    """↩️ The undo of an add is the removal of exactly that member."""
    return [("remove-plant-loop-equipment", {"id": payload["id"], "equipmentId": payload["equipmentId"]})]


def remove_plant_loop_equipment(before, payload):
    """⚙️ `remove-plant-loop-equipment` — one plant equipment id out of the membership list."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["equipmentId"] not in item["equipment_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["equipmentId"])])
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["equipment_ids"] = [value for value in entry["equipment_ids"] if value != payload["equipmentId"]]
    return after, applied()


def _invert_remove_plant_loop_equipment(before, payload):
    """↩️ The undo of a removal puts the member back at its ascending position."""
    return [("add-plant-loop-equipment", {"id": payload["id"], "equipmentId": payload["equipmentId"]})]


def create_outdoor_air_system(before, payload):
    """🌲️ `create-outdoor-air-system` — one new outdoor air system row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["outdoor_air_systems"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["airLoopId"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["airLoopId"])])
    if payload["minOaFlowM3S"] != payload["minOaFlowM3S"] or payload["minOaFlowM3S"] in (float("inf"), float("-inf")) or payload["minOaFlowM3S"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["outdoor_air_systems"].append({"id": payload["id"], "air_loop_id": payload["airLoopId"], "min_oa_flow_m3_s": payload["minOaFlowM3S"], "economizer_enabled": payload["economizerEnabled"]})
    return after, applied()


def _invert_create_outdoor_air_system(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-outdoor-air-system", {"id": payload["id"]})]


def delete_outdoor_air_system(before, payload):
    """🍂️ `delete-outdoor-air-system` — drops the addressed outdoor air system row."""
    item = next((entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["outdoor_air_systems"] = [entry for entry in after["model"]["outdoor_air_systems"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_outdoor_air_system(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    item = next(entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"])
    return [("create-outdoor-air-system", {"id": item["id"], "airLoopId": item["air_loop_id"], "minOaFlowM3S": item["min_oa_flow_m3_s"], "economizerEnabled": item["economizer_enabled"]})]


def change_outdoor_air_system_air_loop(before, payload):
    """⛓️ `change-outdoor-air-system-air-loop` — the air loop of one outdoor air system row."""
    item = next((entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newAirLoopId"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newAirLoopId"])])
    if item["air_loop_id"] == payload["newAirLoopId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["outdoor_air_systems"]:
        if entry["id"] == payload["id"]:
            entry["air_loop_id"] = payload["newAirLoopId"]
    return after, applied()


def _invert_change_outdoor_air_system_air_loop(before, payload):
    """↩️ Its own inverse, with the old air loop read off BASE."""
    item = next(entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"])
    return [("change-outdoor-air-system-air-loop", {"id": payload["id"], "newAirLoopId": item["air_loop_id"]})]


def change_outdoor_air_system_min_oa_flow(before, payload):
    """🦋️ `change-outdoor-air-system-min-oa-flow` — the minimum outdoor air flow of one outdoor air system row."""
    item = next((entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newMinOaFlowM3S"] != payload["newMinOaFlowM3S"] or payload["newMinOaFlowM3S"] in (float("inf"), float("-inf")) or payload["newMinOaFlowM3S"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["min_oa_flow_m3_s"] == payload["newMinOaFlowM3S"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["outdoor_air_systems"]:
        if entry["id"] == payload["id"]:
            entry["min_oa_flow_m3_s"] = payload["newMinOaFlowM3S"]
    return after, applied()


def _invert_change_outdoor_air_system_min_oa_flow(before, payload):
    """↩️ Its own inverse, with the old minimum outdoor air flow read off BASE."""
    item = next(entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"])
    return [("change-outdoor-air-system-min-oa-flow", {"id": payload["id"], "newMinOaFlowM3S": item["min_oa_flow_m3_s"]})]


def change_outdoor_air_system_economizer_enabled(before, payload):
    """💰️ `change-outdoor-air-system-economizer-enabled` — the economizer setting of one outdoor air system row."""
    item = next((entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["economizer_enabled"] == payload["newEconomizerEnabled"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for entry in after["model"]["outdoor_air_systems"]:
        if entry["id"] == payload["id"]:
            entry["economizer_enabled"] = payload["newEconomizerEnabled"]
    return after, applied()


def _invert_change_outdoor_air_system_economizer_enabled(before, payload):
    """↩️ Its own inverse, with the old economizer setting read off BASE."""
    item = next(entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"])
    return [("change-outdoor-air-system-economizer-enabled", {"id": payload["id"], "newEconomizerEnabled": item["economizer_enabled"]})]


def create_electrical_load_center(before, payload):
    """🏦️ `create-electrical-load-center{index,…}` — Adds one electrical load centre — the node that sums on-site generation and storage against the building's electrical demand. `pvIds` and `batteryIds` are checked against the document; `generatorIds` is carried verbatim and NOT checked, because `Model` has no generator collection for it to reference (vocabulary §5.4), which is also why this group ships no `add-electrical-load-center-generator`."""
    rows = before["model"]["electrical_load_centers"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    missing = next((candidate for candidate in payload["pvIds"] if not any(row["id"] == candidate for row in before["model"]["pv_systems"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    missing = next((candidate for candidate in payload["batteryIds"] if not any(row["id"] == candidate for row in before["model"]["battery_storage"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["electrical_load_centers"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "generator_ids": payload["generatorIds"], "pv_ids": payload["pvIds"], "battery_ids": payload["batteryIds"]})
    return after, applied()


def _invert_create_electrical_load_center(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-electrical-load-center", {"id": payload["id"]})]


def delete_electrical_load_center(before, payload):
    """🔻️ `delete-electrical-load-center{id}` — Removes one electrical load centre. Nothing in the document references a load centre — it is the referencing end of every generation edge — so this never has to refuse for use and never cascades."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["electrical_load_centers"] = [row for row in after["model"]["electrical_load_centers"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_electrical_load_center(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["electrical_load_centers"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-electrical-load-center", {"index": index, "id": item["id"], "name": item["name"], "generatorIds": item["generator_ids"], "pvIds": item["pv_ids"], "batteryIds": item["battery_ids"]})]


def rename_electrical_load_center(before, payload):
    """🖊️ `rename-electrical-load-center{id,newName}` — Sets one electrical load center's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["electrical_load_centers"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_electrical_load_center(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"])
    return [("rename-electrical-load-center", {"id": payload["id"], "newName": item["name"]})]


def add_electrical_load_center_pv(before, payload):
    """☄️ `add-electrical-load-center-pv{id,index,pvId}` — Attaches one PV system to a load centre, so its DC output is inverted onto that centre's bus."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["pvId"] for row in before["model"]["pv_systems"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["pvId"])])
    if payload["index"] > len(item["pv_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["pvId"] in item["pv_ids"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["pv_ids"].insert(payload["index"], payload["pvId"])
    return after, applied()


def _invert_add_electrical_load_center_pv(before, payload):
    """↩️ The added member is taken back out; the owner keeps every other member in place."""
    return [("remove-electrical-load-center-pv", {"id": payload["id"], "pvId": payload["pvId"]})]


def remove_electrical_load_center_pv(before, payload):
    """🌘️ `remove-electrical-load-center-pv{id,pvId}` — Detaches one PV system from a load centre; the PV system itself survives, unattached."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["pvId"] not in item["pv_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["pvId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["pv_ids"] = [candidate for candidate in row["pv_ids"] if candidate != payload["pvId"]]
    return after, applied()


def _invert_remove_electrical_load_center_pv(before, payload):
    """↩️ The member goes back at the position it actually held in BASE, not at the end."""
    item = next(row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"])
    return [("add-electrical-load-center-pv", {"id": payload["id"], "index": item["pv_ids"].index(payload["pvId"]), "pvId": payload["pvId"]})]


def add_electrical_load_center_battery(before, payload):
    """🔋️ `add-electrical-load-center-battery{id,index,batteryId}` — Attaches one battery to a load centre, so the centre may charge and discharge it against the net bus balance."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["batteryId"] for row in before["model"]["battery_storage"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["batteryId"])])
    if payload["index"] > len(item["battery_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["batteryId"] in item["battery_ids"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["battery_ids"].insert(payload["index"], payload["batteryId"])
    return after, applied()


def _invert_add_electrical_load_center_battery(before, payload):
    """↩️ The added member is taken back out; the owner keeps every other member in place."""
    return [("remove-electrical-load-center-battery", {"id": payload["id"], "batteryId": payload["batteryId"]})]


def remove_electrical_load_center_battery(before, payload):
    """🪝️ `remove-electrical-load-center-battery{id,batteryId}` — Detaches one battery from a load centre; the battery itself survives, unattached."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["batteryId"] not in item["battery_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["batteryId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["battery_ids"] = [candidate for candidate in row["battery_ids"] if candidate != payload["batteryId"]]
    return after, applied()


def _invert_remove_electrical_load_center_battery(before, payload):
    """↩️ The member goes back at the position it actually held in BASE, not at the end."""
    item = next(row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"])
    return [("add-electrical-load-center-battery", {"id": payload["id"], "index": item["battery_ids"].index(payload["batteryId"]), "batteryId": payload["batteryId"]})]


def create_pv_system(before, payload):
    """✨️ `create-pv-system{index,…}` — Adds one photovoltaic array. Capacity, aperture area and the two efficiencies fix the DC-to-AC chain; tilt and azimuth place the plane the incident-solar model integrates over."""
    rows = before["model"]["pv_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["pv_systems"].insert(payload["index"], {"id": payload["id"], "dc_capacity_w": payload["dcCapacityW"], "area_m2": payload["areaM2"], "tilt_deg": payload["tiltDeg"], "azimuth_deg": payload["azimuthDeg"], "module_efficiency": payload["moduleEfficiency"], "inverter_efficiency": payload["inverterEfficiency"]})
    return after, applied()


def _invert_create_pv_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-pv-system", {"id": payload["id"]})]


def delete_pv_system(before, payload):
    """🌒️ `delete-pv-system{id}` — Removes one photovoltaic array. Refused while a load centre still lists it, so the electrical bus never sums over an array the document no longer defines."""
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(payload["id"] in row["pv_ids"] for row in before["model"]["electrical_load_centers"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["pv_systems"] = [row for row in after["model"]["pv_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_pv_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["pv_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-pv-system", {"index": index, "id": item["id"], "dcCapacityW": item["dc_capacity_w"], "areaM2": item["area_m2"], "tiltDeg": item["tilt_deg"], "azimuthDeg": item["azimuth_deg"], "moduleEfficiency": item["module_efficiency"], "inverterEfficiency": item["inverter_efficiency"]})]


def change_pv_system_dc_capacity(before, payload):
    """⚛️ `change-pv-system-dc-capacity{id,newDcCapacityW}` — Sets DC capacity (W) on one pv system, addressed by id."""
    value = payload["newDcCapacityW"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["dc_capacity_w"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["dc_capacity_w"] = value
    return after, applied()


def _invert_change_pv_system_dc_capacity(before, payload):
    """↩️ The dc_capacity_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-dc-capacity", {"id": payload["id"], "newDcCapacityW": item["dc_capacity_w"]})]


def change_pv_system_area(before, payload):
    """🟨️ `change-pv-system-area{id,newAreaM2}` — Sets aperture area (m²) on one pv system, addressed by id."""
    value = payload["newAreaM2"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["area_m2"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["area_m2"] = value
    return after, applied()


def _invert_change_pv_system_area(before, payload):
    """↩️ The area_m2 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-area", {"id": payload["id"], "newAreaM2": item["area_m2"]})]


def change_pv_system_tilt(before, payload):
    """📈️ `change-pv-system-tilt{id,newTiltDeg}` — Sets tilt on one pv system, addressed by id."""
    value = payload["newTiltDeg"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 90.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["tilt_deg"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["tilt_deg"] = value
    return after, applied()


def _invert_change_pv_system_tilt(before, payload):
    """↩️ The tilt_deg the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-tilt", {"id": payload["id"], "newTiltDeg": item["tilt_deg"]})]


def change_pv_system_azimuth(before, payload):
    """🧿️ `change-pv-system-azimuth{id,newAzimuthDeg}` — Sets azimuth on one pv system, addressed by id."""
    value = payload["newAzimuthDeg"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 360.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["azimuth_deg"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["azimuth_deg"] = value
    return after, applied()


def _invert_change_pv_system_azimuth(before, payload):
    """↩️ The azimuth_deg the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-azimuth", {"id": payload["id"], "newAzimuthDeg": item["azimuth_deg"]})]


def change_pv_system_module_efficiency(before, payload):
    """🎖️ `change-pv-system-module-efficiency{id,newModuleEfficiency}` — Sets module efficiency on one pv system, addressed by id."""
    value = payload["newModuleEfficiency"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["module_efficiency"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["module_efficiency"] = value
    return after, applied()


def _invert_change_pv_system_module_efficiency(before, payload):
    """↩️ The module_efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-module-efficiency", {"id": payload["id"], "newModuleEfficiency": item["module_efficiency"]})]


def change_pv_system_inverter_efficiency(before, payload):
    """♌️ `change-pv-system-inverter-efficiency{id,newInverterEfficiency}` — Sets inverter efficiency on one pv system, addressed by id."""
    value = payload["newInverterEfficiency"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["inverter_efficiency"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["inverter_efficiency"] = value
    return after, applied()


def _invert_change_pv_system_inverter_efficiency(before, payload):
    """↩️ The inverter_efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-inverter-efficiency", {"id": payload["id"], "newInverterEfficiency": item["inverter_efficiency"]})]


def create_battery(before, payload):
    """🪙️ `create-battery{index,…}` — Adds one electrical storage unit. Capacity bounds the state of charge; the two power limits bound each timestep's charge and discharge, and the round-trip efficiency is what the stored energy is debited by."""
    rows = before["model"]["battery_storage"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["battery_storage"].insert(payload["index"], {"id": payload["id"], "capacity_kwh": payload["capacityKwh"], "max_charge_w": payload["maxChargeW"], "max_discharge_w": payload["maxDischargeW"], "round_trip_efficiency": payload["roundTripEfficiency"]})
    return after, applied()


def _invert_create_battery(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-battery", {"id": payload["id"]})]


def delete_battery(before, payload):
    """♒️ `delete-battery{id}` — Removes one electrical storage unit. Refused while a load centre still lists it."""
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(payload["id"] in row["battery_ids"] for row in before["model"]["electrical_load_centers"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["battery_storage"] = [row for row in after["model"]["battery_storage"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_battery(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["battery_storage"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-battery", {"index": index, "id": item["id"], "capacityKwh": item["capacity_kwh"], "maxChargeW": item["max_charge_w"], "maxDischargeW": item["max_discharge_w"], "roundTripEfficiency": item["round_trip_efficiency"]})]


def change_battery_capacity(before, payload):
    """🥫️ `change-battery-capacity{id,newCapacityKwh}` — Sets storage capacity (kWh) on one battery, addressed by id."""
    value = payload["newCapacityKwh"]
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["capacity_kwh"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["battery_storage"]:
        if row["id"] == payload["id"]:
            row["capacity_kwh"] = value
    return after, applied()


def _invert_change_battery_capacity(before, payload):
    """↩️ The capacity_kwh the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["battery_storage"] if row["id"] == payload["id"])
    return [("change-battery-capacity", {"id": payload["id"], "newCapacityKwh": item["capacity_kwh"]})]


def change_battery_max_charge(before, payload):
    """⏫️ `change-battery-max-charge{id,newMaxChargeW}` — Sets maximum charge power (W) on one battery, addressed by id."""
    value = payload["newMaxChargeW"]
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["max_charge_w"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["battery_storage"]:
        if row["id"] == payload["id"]:
            row["max_charge_w"] = value
    return after, applied()


def _invert_change_battery_max_charge(before, payload):
    """↩️ The max_charge_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["battery_storage"] if row["id"] == payload["id"])
    return [("change-battery-max-charge", {"id": payload["id"], "newMaxChargeW": item["max_charge_w"]})]


def change_battery_max_discharge(before, payload):
    """⏬️ `change-battery-max-discharge{id,newMaxDischargeW}` — Sets maximum discharge power (W) on one battery, addressed by id."""
    value = payload["newMaxDischargeW"]
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["max_discharge_w"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["battery_storage"]:
        if row["id"] == payload["id"]:
            row["max_discharge_w"] = value
    return after, applied()


def _invert_change_battery_max_discharge(before, payload):
    """↩️ The max_discharge_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["battery_storage"] if row["id"] == payload["id"])
    return [("change-battery-max-discharge", {"id": payload["id"], "newMaxDischargeW": item["max_discharge_w"]})]


def change_battery_round_trip_efficiency(before, payload):
    """🥉️ `change-battery-round-trip-efficiency{id,newRoundTripEfficiency}` — Sets round-trip efficiency on one battery, addressed by id."""
    value = payload["newRoundTripEfficiency"]
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["round_trip_efficiency"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["battery_storage"]:
        if row["id"] == payload["id"]:
            row["round_trip_efficiency"] = value
    return after, applied()


def _invert_change_battery_round_trip_efficiency(before, payload):
    """↩️ The round_trip_efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["battery_storage"] if row["id"] == payload["id"])
    return [("change-battery-round-trip-efficiency", {"id": payload["id"], "newRoundTripEfficiency": item["round_trip_efficiency"]})]


def create_shw_system(before, payload):
    """🛀️ `create-shw-system{index,…}` — Adds one service-hot-water system: a storage tank, the heater that keeps it at setpoint, and the draw schedule the load profile is read from."""
    rows = before["model"]["shw_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["shw_systems"].insert(payload["index"], {"id": payload["id"], "heater_capacity_w": payload["heaterCapacityW"], "storage_volume_m3": payload["storageVolumeM3"], "setpoint_c": payload["setpointC"], "schedule_id": payload["scheduleId"]})
    return after, applied()


def _invert_create_shw_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-shw-system", {"id": payload["id"]})]


def delete_shw_system(before, payload):
    """🚱️ `delete-shw-system{id}` — Removes one service-hot-water system. Nothing in the document references one, so this never refuses for use."""
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["shw_systems"] = [row for row in after["model"]["shw_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_shw_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["shw_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-shw-system", {"index": index, "id": item["id"], "heaterCapacityW": item["heater_capacity_w"], "storageVolumeM3": item["storage_volume_m3"], "setpointC": item["setpoint_c"], "scheduleId": item["schedule_id"]})]


def change_shw_system_heater_capacity(before, payload):
    """🍵️ `change-shw-system-heater-capacity{id,newHeaterCapacityW}` — Sets heater capacity (W) on one service hot water system, addressed by id."""
    value = payload["newHeaterCapacityW"]
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["heater_capacity_w"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["shw_systems"]:
        if row["id"] == payload["id"]:
            row["heater_capacity_w"] = value
    return after, applied()


def _invert_change_shw_system_heater_capacity(before, payload):
    """↩️ The heater_capacity_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["shw_systems"] if row["id"] == payload["id"])
    return [("change-shw-system-heater-capacity", {"id": payload["id"], "newHeaterCapacityW": item["heater_capacity_w"]})]


def change_shw_system_storage_volume(before, payload):
    """🛢️ `change-shw-system-storage-volume{id,newStorageVolumeM3}` — Sets storage volume (m³) on one service hot water system, addressed by id."""
    value = payload["newStorageVolumeM3"]
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["storage_volume_m3"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["shw_systems"]:
        if row["id"] == payload["id"]:
            row["storage_volume_m3"] = value
    return after, applied()


def _invert_change_shw_system_storage_volume(before, payload):
    """↩️ The storage_volume_m3 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["shw_systems"] if row["id"] == payload["id"])
    return [("change-shw-system-storage-volume", {"id": payload["id"], "newStorageVolumeM3": item["storage_volume_m3"]})]


def change_shw_system_setpoint(before, payload):
    """🏹️ `change-shw-system-setpoint{id,newSetpointC}` — Sets tank setpoint (°C) on one service hot water system, addressed by id."""
    value = payload["newSetpointC"]
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 100.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["setpoint_c"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["shw_systems"]:
        if row["id"] == payload["id"]:
            row["setpoint_c"] = value
    return after, applied()


def _invert_change_shw_system_setpoint(before, payload):
    """↩️ The setpoint_c the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["shw_systems"] if row["id"] == payload["id"])
    return [("change-shw-system-setpoint", {"id": payload["id"], "newSetpointC": item["setpoint_c"]})]


def change_shw_system_schedule(before, payload):
    """🕐️ `change-shw-system-schedule{id,newScheduleId}` — Re-points one service hot water system's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["shw_systems"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_shw_system_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["shw_systems"] if row["id"] == payload["id"])
    return [("change-shw-system-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def create_solar_thermal_system(before, payload):
    """🌄️ `create-solar-thermal-system{index,…}` — Adds one solar thermal collector loop: aperture, conversion efficiency, the buffer it charges, and the plane the incident-solar model integrates over."""
    rows = before["model"]["solar_thermal_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["solar_thermal_systems"].insert(payload["index"], {"id": payload["id"], "collector_area_m2": payload["collectorAreaM2"], "efficiency": payload["efficiency"], "storage_volume_m3": payload["storageVolumeM3"], "tilt_deg": payload["tiltDeg"], "azimuth_deg": payload["azimuthDeg"]})
    return after, applied()


def _invert_create_solar_thermal_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-solar-thermal-system", {"id": payload["id"]})]


def delete_solar_thermal_system(before, payload):
    """🌆️ `delete-solar-thermal-system{id}` — Removes one solar thermal collector loop. Nothing in the document references one, so this never refuses for use."""
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["solar_thermal_systems"] = [row for row in after["model"]["solar_thermal_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_solar_thermal_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["solar_thermal_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-solar-thermal-system", {"index": index, "id": item["id"], "collectorAreaM2": item["collector_area_m2"], "efficiency": item["efficiency"], "storageVolumeM3": item["storage_volume_m3"], "tiltDeg": item["tilt_deg"], "azimuthDeg": item["azimuth_deg"]})]


def change_solar_thermal_system_collector_area(before, payload):
    """🟩️ `change-solar-thermal-system-collector-area{id,newCollectorAreaM2}` — Sets collector area (m²) on one solar thermal system, addressed by id."""
    value = payload["newCollectorAreaM2"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["collector_area_m2"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["collector_area_m2"] = value
    return after, applied()


def _invert_change_solar_thermal_system_collector_area(before, payload):
    """↩️ The collector_area_m2 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-collector-area", {"id": payload["id"], "newCollectorAreaM2": item["collector_area_m2"]})]


def change_solar_thermal_system_efficiency(before, payload):
    """🏅️ `change-solar-thermal-system-efficiency{id,newEfficiency}` — Sets collector efficiency on one solar thermal system, addressed by id."""
    value = payload["newEfficiency"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["efficiency"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["efficiency"] = value
    return after, applied()


def _invert_change_solar_thermal_system_efficiency(before, payload):
    """↩️ The efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-efficiency", {"id": payload["id"], "newEfficiency": item["efficiency"]})]


def change_solar_thermal_system_storage_volume(before, payload):
    """🧃️ `change-solar-thermal-system-storage-volume{id,newStorageVolumeM3}` — Sets buffer volume (m³) on one solar thermal system, addressed by id."""
    value = payload["newStorageVolumeM3"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["storage_volume_m3"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["storage_volume_m3"] = value
    return after, applied()


def _invert_change_solar_thermal_system_storage_volume(before, payload):
    """↩️ The storage_volume_m3 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-storage-volume", {"id": payload["id"], "newStorageVolumeM3": item["storage_volume_m3"]})]


def change_solar_thermal_system_tilt(before, payload):
    """🔼️ `change-solar-thermal-system-tilt{id,newTiltDeg}` — Sets tilt on one solar thermal system, addressed by id."""
    value = payload["newTiltDeg"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 90.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["tilt_deg"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["tilt_deg"] = value
    return after, applied()


def _invert_change_solar_thermal_system_tilt(before, payload):
    """↩️ The tilt_deg the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-tilt", {"id": payload["id"], "newTiltDeg": item["tilt_deg"]})]


def change_solar_thermal_system_azimuth(before, payload):
    """⛵️ `change-solar-thermal-system-azimuth{id,newAzimuthDeg}` — Sets azimuth on one solar thermal system, addressed by id."""
    value = payload["newAzimuthDeg"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 360.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["azimuth_deg"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["azimuth_deg"] = value
    return after, applied()


def _invert_change_solar_thermal_system_azimuth(before, payload):
    """↩️ The azimuth_deg the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-azimuth", {"id": payload["id"], "newAzimuthDeg": item["azimuth_deg"]})]


def create_refrigeration_system(before, payload):
    """❄️ `create-refrigeration-system{index,…}` — Adds one refrigeration system: the display cases it serves, their design load, and the defrost schedule that periodically reverses it."""
    rows = before["model"]["refrigeration_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["defrostScheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["defrostScheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["refrigeration_systems"].insert(payload["index"], {"id": payload["id"], "case_count": payload["caseCount"], "design_load_w": payload["designLoadW"], "defrost_schedule_id": payload["defrostScheduleId"]})
    return after, applied()


def _invert_create_refrigeration_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-refrigeration-system", {"id": payload["id"]})]


def delete_refrigeration_system(before, payload):
    """🫠️ `delete-refrigeration-system{id}` — Removes one refrigeration system. Nothing in the document references one, so this never refuses for use."""
    item = next((row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["refrigeration_systems"] = [row for row in after["model"]["refrigeration_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_refrigeration_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["refrigeration_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-refrigeration-system", {"index": index, "id": item["id"], "caseCount": item["case_count"], "designLoadW": item["design_load_w"], "defrostScheduleId": item["defrost_schedule_id"]})]


def change_refrigeration_system_case_count(before, payload):
    """🗄️ `change-refrigeration-system-case-count{id,newCaseCount}` — Sets how many display cases one refrigeration system carries."""
    value = payload["newCaseCount"]
    item = next((row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["case_count"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["refrigeration_systems"]:
        if row["id"] == payload["id"]:
            row["case_count"] = value
    return after, applied()


def _invert_change_refrigeration_system_case_count(before, payload):
    """↩️ The case_count the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"])
    return [("change-refrigeration-system-case-count", {"id": payload["id"], "newCaseCount": item["case_count"]})]


def change_refrigeration_system_design_load(before, payload):
    """🏋️ `change-refrigeration-system-design-load{id,newDesignLoadW}` — Sets design load (W) on one refrigeration system, addressed by id."""
    value = payload["newDesignLoadW"]
    item = next((row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_load_w"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["refrigeration_systems"]:
        if row["id"] == payload["id"]:
            row["design_load_w"] = value
    return after, applied()


def _invert_change_refrigeration_system_design_load(before, payload):
    """↩️ The design_load_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"])
    return [("change-refrigeration-system-design-load", {"id": payload["id"], "newDesignLoadW": item["design_load_w"]})]


def change_refrigeration_system_defrost_schedule(before, payload):
    """🕑️ `change-refrigeration-system-defrost-schedule{id,newDefrostScheduleId}` — Re-points one refrigeration system's schedule reference; a schedule the document does not define is refused."""
    value = payload["newDefrostScheduleId"]
    item = next((row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["defrost_schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["refrigeration_systems"]:
        if row["id"] == payload["id"]:
            row["defrost_schedule_id"] = value
    return after, applied()


def _invert_change_refrigeration_system_defrost_schedule(before, payload):
    """↩️ The defrost_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"])
    return [("change-refrigeration-system-defrost-schedule", {"id": payload["id"], "newDefrostScheduleId": item["defrost_schedule_id"]})]


def create_water_system(before, payload):
    """🚽️ `create-water-system{index,…}` — Adds one cold-water use system: the fixtures it serves, their combined peak flow, and the draw schedule the profile is read from."""
    rows = before["model"]["water_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["water_systems"].insert(payload["index"], {"id": payload["id"], "fixture_count": payload["fixtureCount"], "peak_flow_l_s": payload["peakFlowLS"], "schedule_id": payload["scheduleId"]})
    return after, applied()


def _invert_create_water_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-water-system", {"id": payload["id"]})]


def delete_water_system(before, payload):
    """🧼️ `delete-water-system{id}` — Removes one cold-water use system. Nothing in the document references one, so this never refuses for use."""
    item = next((row for row in before["model"]["water_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["water_systems"] = [row for row in after["model"]["water_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_water_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["water_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-water-system", {"index": index, "id": item["id"], "fixtureCount": item["fixture_count"], "peakFlowLS": item["peak_flow_l_s"], "scheduleId": item["schedule_id"]})]


def change_water_system_fixture_count(before, payload):
    """🪣️ `change-water-system-fixture-count{id,newFixtureCount}` — Sets how many fixtures one water system carries."""
    value = payload["newFixtureCount"]
    item = next((row for row in before["model"]["water_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["fixture_count"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["water_systems"]:
        if row["id"] == payload["id"]:
            row["fixture_count"] = value
    return after, applied()


def _invert_change_water_system_fixture_count(before, payload):
    """↩️ The fixture_count the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["water_systems"] if row["id"] == payload["id"])
    return [("change-water-system-fixture-count", {"id": payload["id"], "newFixtureCount": item["fixture_count"]})]


def change_water_system_peak_flow(before, payload):
    """🚾️ `change-water-system-peak-flow{id,newPeakFlowLS}` — Sets peak flow (L/s) on one water system, addressed by id."""
    value = payload["newPeakFlowLS"]
    item = next((row for row in before["model"]["water_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["peak_flow_l_s"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["water_systems"]:
        if row["id"] == payload["id"]:
            row["peak_flow_l_s"] = value
    return after, applied()


def _invert_change_water_system_peak_flow(before, payload):
    """↩️ The peak_flow_l_s the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["water_systems"] if row["id"] == payload["id"])
    return [("change-water-system-peak-flow", {"id": payload["id"], "newPeakFlowLS": item["peak_flow_l_s"]})]


def change_water_system_schedule(before, payload):
    """🕒️ `change-water-system-schedule{id,newScheduleId}` — Re-points one water system's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["water_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["water_systems"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_water_system_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["water_systems"] if row["id"] == payload["id"])
    return [("change-water-system-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def create_fault(before, payload):
    """⚠️ `create-fault{index,…}` — Adds one equipment fault. `targetEquipmentId` is checked against `ideal_loads` because that is the collection the kernel actually matches it against (`SystemSubstepStage::Fault` compares `fault.target_equipment_id == ideal.id`); the field's own type carries no discriminator, so this is the only referent the engine gives it."""
    rows = before["model"]["faults"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["targetEquipmentId"] for row in before["model"]["ideal_loads"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["targetEquipmentId"])])
    if not any(row["id"] == payload["startScheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["startScheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["faults"].insert(payload["index"], {"id": payload["id"], "target_equipment_id": payload["targetEquipmentId"], "fault_type": payload["faultType"], "severity": payload["severity"], "start_schedule_id": payload["startScheduleId"]})
    return after, applied()


def _invert_create_fault(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-fault", {"id": payload["id"]})]


def delete_fault(before, payload):
    """🩹️ `delete-fault{id}` — Removes one equipment fault, restoring the equipment it degraded to its rated behaviour. Nothing references a fault, so this never refuses for use."""
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["faults"] = [row for row in after["model"]["faults"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_fault(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["faults"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-fault", {"index": index, "id": item["id"], "targetEquipmentId": item["target_equipment_id"], "faultType": item["fault_type"], "severity": item["severity"], "startScheduleId": item["start_schedule_id"]})]


def change_fault_target_equipment(before, payload):
    """🎣️ `change-fault-target-equipment{id,newTargetEquipmentId}` — Re-points one fault at another ideal loads system; a target the document does not define is refused."""
    value = payload["newTargetEquipmentId"]
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["ideal_loads"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["target_equipment_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["faults"]:
        if row["id"] == payload["id"]:
            row["target_equipment_id"] = value
    return after, applied()


def _invert_change_fault_target_equipment(before, payload):
    """↩️ The target_equipment_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["faults"] if row["id"] == payload["id"])
    return [("change-fault-target-equipment", {"id": payload["id"], "newTargetEquipmentId": item["target_equipment_id"]})]


def change_fault_type(before, payload):
    """🐛️ `change-fault-type{id,newFaultType}` — Sets one fault's degradation mechanism."""
    value = payload["newFaultType"]
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if item["fault_type"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["faults"]:
        if row["id"] == payload["id"]:
            row["fault_type"] = value
    return after, applied()


def _invert_change_fault_type(before, payload):
    """↩️ The fault_type the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["faults"] if row["id"] == payload["id"])
    return [("change-fault-type", {"id": payload["id"], "newFaultType": item["fault_type"]})]


def change_fault_severity(before, payload):
    """🌶️ `change-fault-severity{id,newSeverity}` — Sets severity on one fault, addressed by id."""
    value = payload["newSeverity"]
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["severity"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["faults"]:
        if row["id"] == payload["id"]:
            row["severity"] = value
    return after, applied()


def _invert_change_fault_severity(before, payload):
    """↩️ The severity the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["faults"] if row["id"] == payload["id"])
    return [("change-fault-severity", {"id": payload["id"], "newSeverity": item["severity"]})]


def change_fault_start_schedule(before, payload):
    """🕓️ `change-fault-start-schedule{id,newStartScheduleId}` — Re-points one fault's schedule reference; a schedule the document does not define is refused."""
    value = payload["newStartScheduleId"]
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["start_schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["faults"]:
        if row["id"] == payload["id"]:
            row["start_schedule_id"] = value
    return after, applied()


def _invert_change_fault_start_schedule(before, payload):
    """↩️ The start_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["faults"] if row["id"] == payload["id"])
    return [("change-fault-start-schedule", {"id": payload["id"], "newStartScheduleId": item["start_schedule_id"]})]


def create_space_list(before, payload):
    """📋️ `create-space-list{index,…}` — Adds one named grouping of spaces — the handle a report or an assignment addresses many spaces by at once."""
    rows = before["model"]["space_lists"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    missing = next((candidate for candidate in payload["spaceIds"] if not any(row["id"] == candidate for row in before["model"]["spaces"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["space_lists"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "space_ids": payload["spaceIds"]})
    return after, applied()


def _invert_create_space_list(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-space-list", {"id": payload["id"]})]


def delete_space_list(before, payload):
    """🗒️ `delete-space-list{id}` — Removes one space grouping. The spaces themselves are untouched — a list owns membership, not the members."""
    item = next((row for row in before["model"]["space_lists"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["space_lists"] = [row for row in after["model"]["space_lists"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_space_list(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["space_lists"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-space-list", {"index": index, "id": item["id"], "name": item["name"], "spaceIds": item["space_ids"]})]


def rename_space_list(before, payload):
    """🪶️ `rename-space-list{id,newName}` — Sets one space list's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["space_lists"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["space_lists"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["space_lists"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_space_list(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["space_lists"] if row["id"] == payload["id"])
    return [("rename-space-list", {"id": payload["id"], "newName": item["name"]})]


def add_space_list_member(before, payload):
    """➡️ `add-space-list-member{id,index,spaceId}` — Puts one space into a grouping."""
    item = next((row for row in before["model"]["space_lists"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["spaceId"] for row in before["model"]["spaces"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["spaceId"])])
    if payload["index"] > len(item["space_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["spaceId"] in item["space_ids"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["space_lists"]:
        if row["id"] == payload["id"]:
            row["space_ids"].insert(payload["index"], payload["spaceId"])
    return after, applied()


def _invert_add_space_list_member(before, payload):
    """↩️ The added member is taken back out; the owner keeps every other member in place."""
    return [("remove-space-list-member", {"id": payload["id"], "spaceId": payload["spaceId"]})]


def remove_space_list_member(before, payload):
    """🪤️ `remove-space-list-member{id,spaceId}` — Takes one space back out of a grouping; the space itself survives."""
    item = next((row for row in before["model"]["space_lists"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["spaceId"] not in item["space_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["spaceId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["space_lists"]:
        if row["id"] == payload["id"]:
            row["space_ids"] = [candidate for candidate in row["space_ids"] if candidate != payload["spaceId"]]
    return after, applied()


def _invert_remove_space_list_member(before, payload):
    """↩️ The member goes back at the position it actually held in BASE, not at the end."""
    item = next(row for row in before["model"]["space_lists"] if row["id"] == payload["id"])
    return [("add-space-list-member", {"id": payload["id"], "index": item["space_ids"].index(payload["spaceId"]), "spaceId": payload["spaceId"]})]


def create_thermal_enclosure(before, payload):
    """🏟️ `create-thermal-enclosure{index,…}` — Adds one thermal enclosure — the named set of zones that share one continuous envelope boundary, which is what an envelope-area report normalizes over."""
    rows = before["model"]["thermal_enclosures"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    missing = next((candidate for candidate in payload["zoneIds"] if not any(row["id"] == candidate for row in before["model"]["zones"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["thermal_enclosures"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "zone_ids": payload["zoneIds"]})
    return after, applied()


def _invert_create_thermal_enclosure(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-thermal-enclosure", {"id": payload["id"]})]


def delete_thermal_enclosure(before, payload):
    """🏯️ `delete-thermal-enclosure{id}` — Removes one thermal enclosure. The zones themselves are untouched — an enclosure owns membership, not the members."""
    item = next((row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["thermal_enclosures"] = [row for row in after["model"]["thermal_enclosures"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_thermal_enclosure(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["thermal_enclosures"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-thermal-enclosure", {"index": index, "id": item["id"], "name": item["name"], "zoneIds": item["zone_ids"]})]


def rename_thermal_enclosure(before, payload):
    """🖋️ `rename-thermal-enclosure{id,newName}` — Sets one thermal enclosure's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["thermal_enclosures"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["thermal_enclosures"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_thermal_enclosure(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"])
    return [("rename-thermal-enclosure", {"id": payload["id"], "newName": item["name"]})]


def add_thermal_enclosure_zone(before, payload):
    """🔒️ `add-thermal-enclosure-zone{id,index,zoneId}` — Puts one zone inside a thermal enclosure."""
    item = next((row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["index"] > len(item["zone_ids"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["zoneId"] in item["zone_ids"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["thermal_enclosures"]:
        if row["id"] == payload["id"]:
            row["zone_ids"].insert(payload["index"], payload["zoneId"])
    return after, applied()


def _invert_add_thermal_enclosure_zone(before, payload):
    """↩️ The added member is taken back out; the owner keeps every other member in place."""
    return [("remove-thermal-enclosure-zone", {"id": payload["id"], "zoneId": payload["zoneId"]})]


def remove_thermal_enclosure_zone(before, payload):
    """🔓️ `remove-thermal-enclosure-zone{id,zoneId}` — Takes one zone back out of a thermal enclosure; the zone itself survives."""
    item = next((row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["zoneId"] not in item["zone_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["thermal_enclosures"]:
        if row["id"] == payload["id"]:
            row["zone_ids"] = [candidate for candidate in row["zone_ids"] if candidate != payload["zoneId"]]
    return after, applied()


def _invert_remove_thermal_enclosure_zone(before, payload):
    """↩️ The member goes back at the position it actually held in BASE, not at the end."""
    item = next(row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"])
    return [("add-thermal-enclosure-zone", {"id": payload["id"], "index": item["zone_ids"].index(payload["zoneId"]), "zoneId": payload["zoneId"]})]


def create_constant_schedule(before, payload):
    """🕜️ `create-constant-schedule{index,…}` — Defines one schedule that holds the same value at every timestep — the shape a fixed setpoint, a fixed fraction or an always-on availability takes."""
    rows = before["model"]["schedules"]["constants"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["value"] != payload["value"] or payload["value"] in (float("inf"), float("-inf")):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["constants"].insert(payload["index"], {"id": payload["id"], "value": payload["value"]})
    return after, applied()


def _invert_create_constant_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-constant-schedule", {"id": payload["id"]})]


def delete_constant_schedule(before, payload):
    """📍️ `delete-constant-schedule{id}` — Removes one constant schedule. Refused while any gain, thermostat, system or other schedule still resolves its id — the document would otherwise carry a reference `Model::validate` reports as severe."""
    item = next((row for row in before["model"]["schedules"]["constants"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["constants"] = [row for row in after["model"]["schedules"]["constants"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_constant_schedule(before, payload):
    """↩️ Re-defines the removed schedule at the index it actually held."""
    rows = before["model"]["schedules"]["constants"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-constant-schedule", {"index": index, "id": item["id"], "value": item["value"]})]


def change_constant_schedule_value(before, payload):
    """🕝️ `change-constant-schedule-value{id,newValue}` — Sets the one value a constant schedule returns at every timestep."""
    value = payload["newValue"]
    item = next((row for row in before["model"]["schedules"]["constants"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["value"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["constants"]:
        if row["id"] == payload["id"]:
            row["value"] = value
    return after, applied()


def _invert_change_constant_schedule_value(before, payload):
    """↩️ The value the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["constants"] if row["id"] == payload["id"])
    return [("change-constant-schedule-value", {"id": payload["id"], "newValue": item["value"]})]


def create_daily_schedule(before, payload):
    """🕞️ `create-daily-schedule{index,…}` — Defines one twenty-four-hour profile. The optional lower and upper bound are one facet — both together or neither — and the values are clamped to them on every lookup, which is why a half-stated pair is refused rather than half-applied."""
    rows = before["model"]["schedules"]["daily"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if len(payload["hourlyValues"]) != 24:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in payload["hourlyValues"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if (payload["limitsMin"] is None) != (payload["limitsMax"] is None):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["limitsMin"] is not None and payload["limitsMin"] > payload["limitsMax"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["daily"].insert(payload["index"], {"id": payload["id"], "hourly_values": list(payload["hourlyValues"]), "interpolation": payload["interpolation"], "limits": ({"min": payload["limitsMin"], "max": payload["limitsMax"]} if payload["limitsMin"] is not None else None)})
    return after, applied()


def _invert_create_daily_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-daily-schedule", {"id": payload["id"]})]


def delete_daily_schedule(before, payload):
    """🌓️ `delete-daily-schedule{id}` — Removes one daily profile. Refused while a weekly or annual schedule — or any consumer — still names it."""
    item = next((row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["daily"] = [row for row in after["model"]["schedules"]["daily"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_daily_schedule(before, payload):
    """↩️ Re-defines the removed schedule at the index it actually held."""
    rows = before["model"]["schedules"]["daily"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-daily-schedule", {"index": index, "id": item["id"], "hourlyValues": item["hourly_values"], "interpolation": item["interpolation"], "limitsMin": (item["limits"]["min"] if item["limits"] is not None else None), "limitsMax": (item["limits"]["max"] if item["limits"] is not None else None)})]


def replace_daily_schedule_hourly_values(before, payload):
    """🕔️ `replace-daily-schedule-hourly-values{id,newHourlyValues}` — Swaps a daily profile's whole twenty-four-value body. The hours are one shape, not twenty-four independent scalars, so they move together."""
    value = payload["newHourlyValues"]
    item = next((row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if len(value) != 24:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in value):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["hourly_values"] == list(value):
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["daily"]:
        if row["id"] == payload["id"]:
            row["hourly_values"] = list(value)
    return after, applied()


def _invert_replace_daily_schedule_hourly_values(before, payload):
    """↩️ The hourly_values the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"])
    return [("replace-daily-schedule-hourly-values", {"id": payload["id"], "newHourlyValues": item["hourly_values"]})]


def change_daily_schedule_interpolation(before, payload):
    """🕕️ `change-daily-schedule-interpolation{id,newInterpolation}` — Sets whether a daily profile steps between its hourly values or ramps across them."""
    value = payload["newInterpolation"]
    item = next((row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if item["interpolation"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["daily"]:
        if row["id"] == payload["id"]:
            row["interpolation"] = value
    return after, applied()


def _invert_change_daily_schedule_interpolation(before, payload):
    """↩️ The interpolation the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"])
    return [("change-daily-schedule-interpolation", {"id": payload["id"], "newInterpolation": item["interpolation"]})]


def change_daily_schedule_limits(before, payload):
    """🚧️ `change-daily-schedule-limits{id,newLimitsMin,newLimitsMax}` — the optional clamp, set or cleared as one pair."""
    item = next((row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if (payload["newLimitsMin"] is None) != (payload["newLimitsMax"] is None):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newLimitsMin"] is not None and payload["newLimitsMin"] > payload["newLimitsMax"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    limits = {"min": payload["newLimitsMin"], "max": payload["newLimitsMax"]} if payload["newLimitsMin"] is not None else None
    if item["limits"] == limits:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["daily"]:
        if row["id"] == payload["id"]:
            row["limits"] = limits
    return after, applied()


def _invert_change_daily_schedule_limits(before, payload):
    """↩️ The clamp the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"])
    old = item["limits"]
    return [("change-daily-schedule-limits", {"id": payload["id"], "newLimitsMin": (old["min"] if old is not None else None), "newLimitsMax": (old["max"] if old is not None else None)})]


def create_weekly_schedule(before, payload):
    """🗓️ `create-weekly-schedule{index,…}` — Defines one week as seven daily profiles, Sunday first — the shape `ScheduleSet::weekly_value` indexes by day of week before it reads the hour."""
    rows = before["model"]["schedules"]["weekly"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if len(payload["dailyScheduleIds"]) != 7:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    missing = next((candidate for candidate in payload["dailyScheduleIds"] if not any(row["id"] == candidate for row in before["model"]["schedules"]["daily"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["weekly"].insert(payload["index"], {"id": payload["id"], "daily_schedule_ids": list(payload["dailyScheduleIds"])})
    return after, applied()


def _invert_create_weekly_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-weekly-schedule", {"id": payload["id"]})]


def delete_weekly_schedule(before, payload):
    """🕖️ `delete-weekly-schedule{id}` — Removes one weekly schedule. Refused while any consumer still resolves its id."""
    item = next((row for row in before["model"]["schedules"]["weekly"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["weekly"] = [row for row in after["model"]["schedules"]["weekly"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_weekly_schedule(before, payload):
    """↩️ Re-defines the removed schedule at the index it actually held."""
    rows = before["model"]["schedules"]["weekly"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-weekly-schedule", {"index": index, "id": item["id"], "dailyScheduleIds": item["daily_schedule_ids"]})]


def change_weekly_schedule_day(before, payload):
    """🕗️ `change-weekly-schedule-day{id,dayIndex,newDailyScheduleId}` — one day of the week re-pointed."""
    item = next((row for row in before["model"]["schedules"]["weekly"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["dayIndex"] > 6:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["newDailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newDailyScheduleId"])])
    if item["daily_schedule_ids"][payload["dayIndex"]] == payload["newDailyScheduleId"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["weekly"]:
        if row["id"] == payload["id"]:
            row["daily_schedule_ids"][payload["dayIndex"]] = payload["newDailyScheduleId"]
    return after, applied()


def _invert_change_weekly_schedule_day(before, payload):
    """↩️ The daily profile that day named in BASE."""
    item = next(row for row in before["model"]["schedules"]["weekly"] if row["id"] == payload["id"])
    return [("change-weekly-schedule-day", {"id": payload["id"], "dayIndex": payload["dayIndex"], "newDailyScheduleId": item["daily_schedule_ids"][payload["dayIndex"]]})]


def create_annual_schedule(before, payload):
    """📚️ `create-annual-schedule{index,…}` — Defines one rule-based year. It starts with no date rules and no holidays — those are ordered and set-like collections of their own, added by `insert-annual-schedule-rule` and `add-annual-schedule-holiday` — so a create states only the two fallbacks every lookup ends at."""
    rows = before["model"]["schedules"]["annual"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["defaultDailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["defaultDailyScheduleId"])])
    if payload["holidayDailyScheduleId"] is not None and not any(row["id"] == payload["holidayDailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["holidayDailyScheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["annual"].insert(payload["index"], {"id": payload["id"], "rules": [], "default_daily_schedule_id": payload["defaultDailyScheduleId"], "holiday_daily_schedule_id": payload["holidayDailyScheduleId"], "holiday_dates": []})
    return after, applied()


def _invert_create_annual_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-annual-schedule", {"id": payload["id"]})]


def delete_annual_schedule(before, payload):
    """📕️ `delete-annual-schedule{id}` — the rule-based year, removed whole."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["annual"] = [row for row in after["model"]["schedules"]["annual"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_annual_schedule(before, payload):
    """↩️ Re-defines the year, then replays its rules in order and its holidays in order."""
    rows = before["model"]["schedules"]["annual"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    steps = [("create-annual-schedule", {"index": index, "id": item["id"], "defaultDailyScheduleId": item["default_daily_schedule_id"], "holidayDailyScheduleId": item["holiday_daily_schedule_id"]})]
    for position, rule in enumerate(item["rules"]):
        steps.append(("insert-annual-schedule-rule", {"id": item["id"], "index": position, "startMonth": rule["start_month"], "startDay": rule["start_day"], "endMonth": rule["end_month"], "endDay": rule["end_day"], "dailyScheduleId": rule["daily_schedule_id"]}))
    for position, holiday in enumerate(item["holiday_dates"]):
        steps.append(("add-annual-schedule-holiday", {"id": item["id"], "index": position, "year": holiday[0], "month": holiday[1], "day": holiday[2]}))
    return steps


def insert_annual_schedule_rule(before, payload):
    """📗️ `insert-annual-schedule-rule{id,index,…}` — one date rule at a FINAL-state position."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["index"] > len(item["rules"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not (1 <= payload["startMonth"] <= 12 and 1 <= payload["endMonth"] <= 12 and 1 <= payload["startDay"] <= 31 and 1 <= payload["endDay"] <= 31):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["dailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["dailyScheduleId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["rules"].insert(payload["index"], {"start_month": payload["startMonth"], "start_day": payload["startDay"], "end_month": payload["endMonth"], "end_day": payload["endDay"], "daily_schedule_id": payload["dailyScheduleId"]})
    return after, applied()


def _invert_insert_annual_schedule_rule(before, payload):
    """↩️ The inserted rule is taken back out at the position it was placed."""
    return [("remove-annual-schedule-rule", {"id": payload["id"], "index": payload["index"]})]


def remove_annual_schedule_rule(before, payload):
    """📙️ `remove-annual-schedule-rule{id,index}` — one date rule at a BASE-state position."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["index"] >= len(item["rules"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            del row["rules"][payload["index"]]
    return after, applied()


def _invert_remove_annual_schedule_rule(before, payload):
    """↩️ The removed rule goes back at the index it held."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    rule = item["rules"][payload["index"]]
    return [("insert-annual-schedule-rule", {"id": payload["id"], "index": payload["index"], "startMonth": rule["start_month"], "startDay": rule["start_day"], "endMonth": rule["end_month"], "endDay": rule["end_day"], "dailyScheduleId": rule["daily_schedule_id"]})]


def reorder_annual_schedule_rules(before, payload):
    """🗂️ `reorder-annual-schedule-rules{id,from,to}` — rule precedence IS list order."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["from"] >= len(item["rules"]) or payload["to"] >= len(item["rules"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["from"] == payload["to"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            rule = row["rules"].pop(payload["from"])
            row["rules"].insert(payload["to"], rule)
    return after, applied()


def _invert_reorder_annual_schedule_rules(before, payload):
    """↩️ The same move, back the other way."""
    return [("reorder-annual-schedule-rules", {"id": payload["id"], "from": payload["to"], "to": payload["from"]})]


def change_annual_schedule_default_daily_schedule(before, payload):
    """🎌️ `change-annual-schedule-default-daily-schedule{id,newDefaultDailyScheduleId}` — Re-points the profile a year falls back to on every day no rule matches."""
    value = payload["newDefaultDailyScheduleId"]
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["default_daily_schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["default_daily_schedule_id"] = value
    return after, applied()


def _invert_change_annual_schedule_default_daily_schedule(before, payload):
    """↩️ The default_daily_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    return [("change-annual-schedule-default-daily-schedule", {"id": payload["id"], "newDefaultDailyScheduleId": item["default_daily_schedule_id"]})]


def change_annual_schedule_holiday_daily_schedule(before, payload):
    """🎄️ `change-annual-schedule-holiday-daily-schedule{id,newHolidayDailyScheduleId}` — Re-points — or clears, with a null — the profile a year uses on the dates it holds as holidays."""
    value = payload["newHolidayDailyScheduleId"]
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value is not None and not any(row["id"] == value for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["holiday_daily_schedule_id"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["holiday_daily_schedule_id"] = value
    return after, applied()


def _invert_change_annual_schedule_holiday_daily_schedule(before, payload):
    """↩️ The holiday_daily_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    return [("change-annual-schedule-holiday-daily-schedule", {"id": payload["id"], "newHolidayDailyScheduleId": item["holiday_daily_schedule_id"]})]


def add_annual_schedule_holiday(before, payload):
    """🎉️ `add-annual-schedule-holiday{id,index,year,month,day}` — one dated holiday."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not (1 <= payload["month"] <= 12 and 1 <= payload["day"] <= 31):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["index"] > len(item["holiday_dates"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    date = [payload["year"], payload["month"], payload["day"]]
    if date in item["holiday_dates"]:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["holiday_dates"].insert(payload["index"], date)
    return after, applied()


def _invert_add_annual_schedule_holiday(before, payload):
    """↩️ The added holiday is taken back out."""
    return [("remove-annual-schedule-holiday", {"id": payload["id"], "year": payload["year"], "month": payload["month"], "day": payload["day"]})]


def remove_annual_schedule_holiday(before, payload):
    """🎊️ `remove-annual-schedule-holiday{id,year,month,day}` — one dated holiday, taken back out."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    date = [payload["year"], payload["month"], payload["day"]]
    if date not in item["holiday_dates"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["holiday_dates"] = [holiday for holiday in row["holiday_dates"] if holiday != date]
    return after, applied()


def _invert_remove_annual_schedule_holiday(before, payload):
    """↩️ The holiday goes back at the position it held in BASE."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    date = [payload["year"], payload["month"], payload["day"]]
    return [("add-annual-schedule-holiday", {"id": payload["id"], "index": item["holiday_dates"].index(date), "year": payload["year"], "month": payload["month"], "day": payload["day"]})]


def create_time_series_schedule(before, payload):
    """🪗️ `create-time-series-schedule{index,…}` — Defines one externally measured series, indexed by timestep rather than by calendar — the shape a metered profile or a co-simulation trace takes."""
    rows = before["model"]["schedules"]["time_series"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not payload["values"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in payload["values"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["timestepSeconds"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["time_series"].insert(payload["index"], {"id": payload["id"], "values": list(payload["values"]), "timestep_seconds": payload["timestepSeconds"]})
    return after, applied()


def _invert_create_time_series_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-time-series-schedule", {"id": payload["id"]})]


def delete_time_series_schedule(before, payload):
    """🎞️ `delete-time-series-schedule{id}` — Removes one measured series. Refused while any consumer still resolves its id."""
    item = next((row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["time_series"] = [row for row in after["model"]["schedules"]["time_series"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_time_series_schedule(before, payload):
    """↩️ Re-defines the removed schedule at the index it actually held."""
    rows = before["model"]["schedules"]["time_series"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-time-series-schedule", {"index": index, "id": item["id"], "values": item["values"], "timestepSeconds": item["timestep_seconds"]})]


def replace_time_series_schedule_values(before, payload):
    """🕘️ `replace-time-series-schedule-values{id,newValues}` — Swaps a measured series' whole body. The samples are one measurement, not independent scalars, so they move together."""
    value = payload["newValues"]
    item = next((row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in value):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["values"] == list(value):
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["time_series"]:
        if row["id"] == payload["id"]:
            row["values"] = list(value)
    return after, applied()


def _invert_replace_time_series_schedule_values(before, payload):
    """↩️ The values the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"])
    return [("replace-time-series-schedule-values", {"id": payload["id"], "newValues": item["values"]})]


def change_time_series_schedule_timestep(before, payload):
    """🕙️ `change-time-series-schedule-timestep{id,newTimestepSeconds}` — Sets how many seconds one sample of a measured series covers — what turns its index into a clock."""
    value = payload["newTimestepSeconds"]
    item = next((row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["timestep_seconds"] == value:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["time_series"]:
        if row["id"] == payload["id"]:
            row["timestep_seconds"] = value
    return after, applied()


def _invert_change_time_series_schedule_timestep(before, payload):
    """↩️ The timestep_seconds the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"])
    return [("change-time-series-schedule-timestep", {"id": payload["id"], "newTimestepSeconds": item["timestep_seconds"]})]


#: 🗺️ Catalog id -> this file's own implementation of that kind.
VOCABULARY = {
    "rename-model": rename_model,
    "change-model-version": change_model_version,
    "update-site": update_site,
    "update-ground-temperature": update_ground_temperature,
    "update-run-period": update_run_period,
    "replace-airflow-network": replace_airflow_network,
    "add-output-variable": add_output_variable,
    "remove-output-variable": remove_output_variable,
    "bind-weather-file": bind_weather_file,
    "unbind-weather-file": unbind_weather_file,
    "connect-referenced-model": connect_referenced_model,
    "disconnect-referenced-model": disconnect_referenced_model,
    "rename-zone": rename_zone,
    "change-zone-volume": change_zone_volume,
    "change-zone-multiplier": change_zone_multiplier,
    "change-zone-conditioned": change_zone_conditioned,
    "change-zone-floor-area-participation": change_zone_floor_area_participation,
    "create-zone": create_zone,
    "delete-zone": delete_zone,
    "create-space": create_space,
    "delete-space": delete_space,
    "rename-space": rename_space,
    "change-space-floor-area": change_space_floor_area,
    "change-space-zone": change_space_zone,
    "create-surface": create_surface,
    "delete-surface": delete_surface,
    "rename-surface": rename_surface,
    "change-surface-zone": change_surface_zone,
    "change-surface-class": change_surface_class,
    "replace-surface-vertices": replace_surface_vertices,
    "change-surface-construction": change_surface_construction,
    "change-surface-boundary-condition": change_surface_boundary_condition,
    "change-surface-sun-exposed": change_surface_sun_exposed,
    "change-surface-wind-exposed": change_surface_wind_exposed,
    "change-surface-multiplier": change_surface_multiplier,
    "create-fenestration": create_fenestration,
    "delete-fenestration": delete_fenestration,
    "rename-fenestration": rename_fenestration,
    "change-fenestration-surface": change_fenestration_surface,
    "change-fenestration-u-value": change_fenestration_u_value,
    "change-fenestration-shgc": change_fenestration_shgc,
    "change-fenestration-vlt": change_fenestration_vlt,
    "change-fenestration-area": change_fenestration_area,
    "change-fenestration-frame-conductance": change_fenestration_frame_conductance,
    "change-fenestration-divider-conductance": change_fenestration_divider_conductance,
    "create-shading-surface": create_shading_surface,
    "delete-shading-surface": delete_shading_surface,
    "rename-shading-surface": rename_shading_surface,
    "replace-shading-surface-vertices": replace_shading_surface_vertices,
    "change-shading-surface-transmittance-schedule": change_shading_surface_transmittance_schedule,
    "connect-surfaces": connect_surfaces,
    "disconnect-surfaces": disconnect_surfaces,
    "bind-fenestration-glazing-construction": bind_fenestration_glazing_construction,
    "clear-fenestration-glazing-construction": clear_fenestration_glazing_construction,
    "change-fenestration-height": change_fenestration_height,
    "change-fenestration-sill-height": change_fenestration_sill_height,
    "change-fenestration-overhang-depth": change_fenestration_overhang_depth,
    "change-fenestration-overhang-offset": change_fenestration_overhang_offset,
    "change-fenestration-fin-depth": change_fenestration_fin_depth,
    "change-fenestration-fin-offset": change_fenestration_fin_offset,
    "create-material": create_material,
    "delete-material": delete_material,
    "rename-material": rename_material,
    "change-material-thickness": change_material_thickness,
    "change-material-conductivity": change_material_conductivity,
    "change-material-density": change_material_density,
    "change-material-specific-heat": change_material_specific_heat,
    "change-material-thermal-absorptance": change_material_thermal_absorptance,
    "change-material-solar-absorptance": change_material_solar_absorptance,
    "change-material-visible-absorptance": change_material_visible_absorptance,
    "create-construction": create_construction,
    "delete-construction": delete_construction,
    "rename-construction": rename_construction,
    "add-construction-layer": add_construction_layer,
    "remove-construction-layer": remove_construction_layer,
    "reorder-construction-layers": reorder_construction_layers,
    "create-people-gain": create_people_gain,
    "delete-people-gain": delete_people_gain,
    "change-people-gain-zone": change_people_gain_zone,
    "change-people-gain-schedule": change_people_gain_schedule,
    "change-people-gain-activity-schedule": change_people_gain_activity_schedule,
    "change-people-gain-people-per-area": change_people_gain_people_per_area,
    "change-people-gain-sensible-fraction": change_people_gain_sensible_fraction,
    "change-people-gain-latent-fraction": change_people_gain_latent_fraction,
    "change-people-gain-radiant-fraction": change_people_gain_radiant_fraction,
    "create-lighting-gain": create_lighting_gain,
    "delete-lighting-gain": delete_lighting_gain,
    "change-lighting-gain-zone": change_lighting_gain_zone,
    "change-lighting-gain-schedule": change_lighting_gain_schedule,
    "change-lighting-gain-watts-per-area": change_lighting_gain_watts_per_area,
    "change-lighting-gain-radiant-fraction": change_lighting_gain_radiant_fraction,
    "change-lighting-gain-visible-fraction": change_lighting_gain_visible_fraction,
    "change-lighting-gain-return-air-fraction": change_lighting_gain_return_air_fraction,
    "create-equipment-gain": create_equipment_gain,
    "delete-equipment-gain": delete_equipment_gain,
    "change-equipment-gain-zone": change_equipment_gain_zone,
    "change-equipment-gain-schedule": change_equipment_gain_schedule,
    "change-equipment-gain-watts-per-area": change_equipment_gain_watts_per_area,
    "change-equipment-gain-radiant-fraction": change_equipment_gain_radiant_fraction,
    "change-equipment-gain-latent-fraction": change_equipment_gain_latent_fraction,
    "create-infiltration": create_infiltration,
    "delete-infiltration": delete_infiltration,
    "change-infiltration-zone": change_infiltration_zone,
    "change-infiltration-schedule": change_infiltration_schedule,
    "change-infiltration-flow-per-exterior-area": change_infiltration_flow_per_exterior_area,
    "change-infiltration-constant-term-coefficient": change_infiltration_constant_term_coefficient,
    "change-infiltration-temperature-term-coefficient": change_infiltration_temperature_term_coefficient,
    "change-infiltration-velocity-term-coefficient": change_infiltration_velocity_term_coefficient,
    "change-infiltration-velocity-squared-term-coefficient": change_infiltration_velocity_squared_term_coefficient,
    "create-mechanical-ventilation": create_mechanical_ventilation,
    "delete-mechanical-ventilation": delete_mechanical_ventilation,
    "change-mechanical-ventilation-zone": change_mechanical_ventilation_zone,
    "change-mechanical-ventilation-schedule": change_mechanical_ventilation_schedule,
    "change-mechanical-ventilation-design-flow": change_mechanical_ventilation_design_flow,
    "change-mechanical-ventilation-fan-total-efficiency": change_mechanical_ventilation_fan_total_efficiency,
    "change-mechanical-ventilation-fan-delta-pressure": change_mechanical_ventilation_fan_delta_pressure,
    "change-infiltration-method": change_infiltration_method,
    "change-infiltration-design-flow-ach": change_infiltration_design_flow_ach,
    "change-infiltration-effective-leakage-area": change_infiltration_effective_leakage_area,
    "change-infiltration-discharge-coefficient": change_infiltration_discharge_coefficient,
    "change-infiltration-stack-height": change_infiltration_stack_height,
    "create-thermostat": create_thermostat,
    "delete-thermostat": delete_thermostat,
    "change-thermostat-zone": change_thermostat_zone,
    "change-thermostat-heating-setpoint-schedule": change_thermostat_heating_setpoint_schedule,
    "change-thermostat-cooling-setpoint-schedule": change_thermostat_cooling_setpoint_schedule,
    "change-thermostat-heating-throttle-range": change_thermostat_heating_throttle_range,
    "change-thermostat-cooling-throttle-range": change_thermostat_cooling_throttle_range,
    "create-humidistat": create_humidistat,
    "delete-humidistat": delete_humidistat,
    "change-humidistat-zone": change_humidistat_zone,
    "change-humidistat-humidifying-setpoint-schedule": change_humidistat_humidifying_setpoint_schedule,
    "change-humidistat-dehumidifying-setpoint-schedule": change_humidistat_dehumidifying_setpoint_schedule,
    "change-humidistat-humidifying-throttle-range": change_humidistat_humidifying_throttle_range,
    "change-humidistat-dehumidifying-throttle-range": change_humidistat_dehumidifying_throttle_range,
    "create-ideal-loads-system": create_ideal_loads_system,
    "delete-ideal-loads-system": delete_ideal_loads_system,
    "change-ideal-loads-system-zone": change_ideal_loads_system_zone,
    "change-ideal-loads-system-max-heating-supply-air-temp": change_ideal_loads_system_max_heating_supply_air_temp,
    "change-ideal-loads-system-min-cooling-supply-air-temp": change_ideal_loads_system_min_cooling_supply_air_temp,
    "change-ideal-loads-system-max-heating-capacity": change_ideal_loads_system_max_heating_capacity,
    "change-ideal-loads-system-max-cooling-capacity": change_ideal_loads_system_max_cooling_capacity,
    "change-ideal-loads-system-outdoor-air-per-person": change_ideal_loads_system_outdoor_air_per_person,
    "change-ideal-loads-system-outdoor-air-per-area": change_ideal_loads_system_outdoor_air_per_area,
    "create-zone-equipment": create_zone_equipment,
    "delete-zone-equipment": delete_zone_equipment,
    "change-zone-equipment-zone": change_zone_equipment_zone,
    "change-zone-equipment-type": change_zone_equipment_type,
    "change-zone-equipment-priority": change_zone_equipment_priority,
    "change-zone-equipment-heating-capacity": change_zone_equipment_heating_capacity,
    "change-zone-equipment-cooling-capacity": change_zone_equipment_cooling_capacity,
    "create-daylight-zone": create_daylight_zone,
    "delete-daylight-zone": delete_daylight_zone,
    "change-daylight-zone-zone": change_daylight_zone_zone,
    "change-daylight-zone-illuminance-target": change_daylight_zone_illuminance_target,
    "change-daylight-zone-glare-limit": change_daylight_zone_glare_limit,
    "change-daylight-zone-window-transmittance": change_daylight_zone_window_transmittance,
    "create-sizing-object": create_sizing_object,
    "delete-sizing-object": delete_sizing_object,
    "change-sizing-object-zone": change_sizing_object_zone,
    "change-sizing-object-sizing-type": change_sizing_object_sizing_type,
    "change-sizing-object-design-day-type": change_sizing_object_design_day_type,
    "create-room-air-model-assignment": create_room_air_model_assignment,
    "delete-room-air-model-assignment": delete_room_air_model_assignment,
    "change-room-air-model": change_room_air_model,
    "create-setpoint-manager": create_setpoint_manager,
    "delete-setpoint-manager": delete_setpoint_manager,
    "rename-setpoint-manager": rename_setpoint_manager,
    "replace-setpoint-manager-kind": replace_setpoint_manager_kind,
    "change-setpoint-manager-schedule": change_setpoint_manager_schedule,
    "create-air-loop": create_air_loop,
    "delete-air-loop": delete_air_loop,
    "rename-air-loop": rename_air_loop,
    "change-air-loop-supply-node": change_air_loop_supply_node,
    "change-air-loop-return-node": change_air_loop_return_node,
    "change-air-loop-design-supply-air-flow": change_air_loop_design_supply_air_flow,
    "add-air-loop-terminal-zone": add_air_loop_terminal_zone,
    "remove-air-loop-terminal-zone": remove_air_loop_terminal_zone,
    "create-plant-loop": create_plant_loop,
    "delete-plant-loop": delete_plant_loop,
    "rename-plant-loop": rename_plant_loop,
    "change-plant-loop-type": change_plant_loop_type,
    "change-plant-loop-supply-temperature": change_plant_loop_supply_temperature,
    "change-plant-loop-return-temperature": change_plant_loop_return_temperature,
    "change-plant-loop-design-flow": change_plant_loop_design_flow,
    "add-plant-loop-equipment": add_plant_loop_equipment,
    "remove-plant-loop-equipment": remove_plant_loop_equipment,
    "create-outdoor-air-system": create_outdoor_air_system,
    "delete-outdoor-air-system": delete_outdoor_air_system,
    "change-outdoor-air-system-air-loop": change_outdoor_air_system_air_loop,
    "change-outdoor-air-system-min-oa-flow": change_outdoor_air_system_min_oa_flow,
    "change-outdoor-air-system-economizer-enabled": change_outdoor_air_system_economizer_enabled,
    "create-electrical-load-center": create_electrical_load_center,
    "delete-electrical-load-center": delete_electrical_load_center,
    "rename-electrical-load-center": rename_electrical_load_center,
    "add-electrical-load-center-pv": add_electrical_load_center_pv,
    "remove-electrical-load-center-pv": remove_electrical_load_center_pv,
    "add-electrical-load-center-battery": add_electrical_load_center_battery,
    "remove-electrical-load-center-battery": remove_electrical_load_center_battery,
    "create-pv-system": create_pv_system,
    "delete-pv-system": delete_pv_system,
    "change-pv-system-dc-capacity": change_pv_system_dc_capacity,
    "change-pv-system-area": change_pv_system_area,
    "change-pv-system-tilt": change_pv_system_tilt,
    "change-pv-system-azimuth": change_pv_system_azimuth,
    "change-pv-system-module-efficiency": change_pv_system_module_efficiency,
    "change-pv-system-inverter-efficiency": change_pv_system_inverter_efficiency,
    "create-battery": create_battery,
    "delete-battery": delete_battery,
    "change-battery-capacity": change_battery_capacity,
    "change-battery-max-charge": change_battery_max_charge,
    "change-battery-max-discharge": change_battery_max_discharge,
    "change-battery-round-trip-efficiency": change_battery_round_trip_efficiency,
    "create-shw-system": create_shw_system,
    "delete-shw-system": delete_shw_system,
    "change-shw-system-heater-capacity": change_shw_system_heater_capacity,
    "change-shw-system-storage-volume": change_shw_system_storage_volume,
    "change-shw-system-setpoint": change_shw_system_setpoint,
    "change-shw-system-schedule": change_shw_system_schedule,
    "create-solar-thermal-system": create_solar_thermal_system,
    "delete-solar-thermal-system": delete_solar_thermal_system,
    "change-solar-thermal-system-collector-area": change_solar_thermal_system_collector_area,
    "change-solar-thermal-system-efficiency": change_solar_thermal_system_efficiency,
    "change-solar-thermal-system-storage-volume": change_solar_thermal_system_storage_volume,
    "change-solar-thermal-system-tilt": change_solar_thermal_system_tilt,
    "change-solar-thermal-system-azimuth": change_solar_thermal_system_azimuth,
    "create-refrigeration-system": create_refrigeration_system,
    "delete-refrigeration-system": delete_refrigeration_system,
    "change-refrigeration-system-case-count": change_refrigeration_system_case_count,
    "change-refrigeration-system-design-load": change_refrigeration_system_design_load,
    "change-refrigeration-system-defrost-schedule": change_refrigeration_system_defrost_schedule,
    "create-water-system": create_water_system,
    "delete-water-system": delete_water_system,
    "change-water-system-fixture-count": change_water_system_fixture_count,
    "change-water-system-peak-flow": change_water_system_peak_flow,
    "change-water-system-schedule": change_water_system_schedule,
    "create-fault": create_fault,
    "delete-fault": delete_fault,
    "change-fault-target-equipment": change_fault_target_equipment,
    "change-fault-type": change_fault_type,
    "change-fault-severity": change_fault_severity,
    "change-fault-start-schedule": change_fault_start_schedule,
    "create-space-list": create_space_list,
    "delete-space-list": delete_space_list,
    "rename-space-list": rename_space_list,
    "add-space-list-member": add_space_list_member,
    "remove-space-list-member": remove_space_list_member,
    "create-thermal-enclosure": create_thermal_enclosure,
    "delete-thermal-enclosure": delete_thermal_enclosure,
    "rename-thermal-enclosure": rename_thermal_enclosure,
    "add-thermal-enclosure-zone": add_thermal_enclosure_zone,
    "remove-thermal-enclosure-zone": remove_thermal_enclosure_zone,
    "create-constant-schedule": create_constant_schedule,
    "delete-constant-schedule": delete_constant_schedule,
    "change-constant-schedule-value": change_constant_schedule_value,
    "create-daily-schedule": create_daily_schedule,
    "delete-daily-schedule": delete_daily_schedule,
    "replace-daily-schedule-hourly-values": replace_daily_schedule_hourly_values,
    "change-daily-schedule-interpolation": change_daily_schedule_interpolation,
    "change-daily-schedule-limits": change_daily_schedule_limits,
    "create-weekly-schedule": create_weekly_schedule,
    "delete-weekly-schedule": delete_weekly_schedule,
    "change-weekly-schedule-day": change_weekly_schedule_day,
    "create-annual-schedule": create_annual_schedule,
    "delete-annual-schedule": delete_annual_schedule,
    "insert-annual-schedule-rule": insert_annual_schedule_rule,
    "remove-annual-schedule-rule": remove_annual_schedule_rule,
    "reorder-annual-schedule-rules": reorder_annual_schedule_rules,
    "change-annual-schedule-default-daily-schedule": change_annual_schedule_default_daily_schedule,
    "change-annual-schedule-holiday-daily-schedule": change_annual_schedule_holiday_daily_schedule,
    "add-annual-schedule-holiday": add_annual_schedule_holiday,
    "remove-annual-schedule-holiday": remove_annual_schedule_holiday,
    "create-time-series-schedule": create_time_series_schedule,
    "delete-time-series-schedule": delete_time_series_schedule,
    "replace-time-series-schedule-values": replace_time_series_schedule_values,
    "change-time-series-schedule-timestep": change_time_series_schedule_timestep,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-zone": _invert_create_zone,
    "delete-zone": _invert_delete_zone,
    "create-space": _invert_create_space,
    "delete-space": _invert_delete_space,
    "rename-space": _invert_rename_space,
    "change-space-floor-area": _invert_change_space_floor_area,
    "change-space-zone": _invert_change_space_zone,
    "create-surface": _invert_create_surface,
    "delete-surface": _invert_delete_surface,
    "rename-surface": _invert_rename_surface,
    "change-surface-zone": _invert_change_surface_zone,
    "change-surface-class": _invert_change_surface_class,
    "replace-surface-vertices": _invert_replace_surface_vertices,
    "change-surface-construction": _invert_change_surface_construction,
    "change-surface-boundary-condition": _invert_change_surface_boundary_condition,
    "change-surface-sun-exposed": _invert_change_surface_sun_exposed,
    "change-surface-wind-exposed": _invert_change_surface_wind_exposed,
    "change-surface-multiplier": _invert_change_surface_multiplier,
    "create-fenestration": _invert_create_fenestration,
    "delete-fenestration": _invert_delete_fenestration,
    "rename-fenestration": _invert_rename_fenestration,
    "change-fenestration-surface": _invert_change_fenestration_surface,
    "change-fenestration-u-value": _invert_change_fenestration_u_value,
    "change-fenestration-shgc": _invert_change_fenestration_shgc,
    "change-fenestration-vlt": _invert_change_fenestration_vlt,
    "change-fenestration-area": _invert_change_fenestration_area,
    "change-fenestration-frame-conductance": _invert_change_fenestration_frame_conductance,
    "change-fenestration-divider-conductance": _invert_change_fenestration_divider_conductance,
    "create-shading-surface": _invert_create_shading_surface,
    "delete-shading-surface": _invert_delete_shading_surface,
    "rename-shading-surface": _invert_rename_shading_surface,
    "replace-shading-surface-vertices": _invert_replace_shading_surface_vertices,
    "change-shading-surface-transmittance-schedule": _invert_change_shading_surface_transmittance_schedule,
    "connect-surfaces": _invert_connect_surfaces,
    "disconnect-surfaces": _invert_disconnect_surfaces,
    "bind-fenestration-glazing-construction": _invert_bind_fenestration_glazing_construction,
    "clear-fenestration-glazing-construction": _invert_clear_fenestration_glazing_construction,
    "change-fenestration-height": _invert_change_fenestration_height,
    "change-fenestration-sill-height": _invert_change_fenestration_sill_height,
    "change-fenestration-overhang-depth": _invert_change_fenestration_overhang_depth,
    "change-fenestration-overhang-offset": _invert_change_fenestration_overhang_offset,
    "change-fenestration-fin-depth": _invert_change_fenestration_fin_depth,
    "change-fenestration-fin-offset": _invert_change_fenestration_fin_offset,
    "create-material": _invert_create_material,
    "delete-material": _invert_delete_material,
    "rename-material": _invert_rename_material,
    "change-material-thickness": _invert_change_material_thickness,
    "change-material-conductivity": _invert_change_material_conductivity,
    "change-material-density": _invert_change_material_density,
    "change-material-specific-heat": _invert_change_material_specific_heat,
    "change-material-thermal-absorptance": _invert_change_material_thermal_absorptance,
    "change-material-solar-absorptance": _invert_change_material_solar_absorptance,
    "change-material-visible-absorptance": _invert_change_material_visible_absorptance,
    "create-construction": _invert_create_construction,
    "delete-construction": _invert_delete_construction,
    "rename-construction": _invert_rename_construction,
    "add-construction-layer": _invert_add_construction_layer,
    "remove-construction-layer": _invert_remove_construction_layer,
    "reorder-construction-layers": _invert_reorder_construction_layers,
    "create-people-gain": _invert_create_people_gain,
    "delete-people-gain": _invert_delete_people_gain,
    "change-people-gain-zone": _invert_change_people_gain_zone,
    "change-people-gain-schedule": _invert_change_people_gain_schedule,
    "change-people-gain-activity-schedule": _invert_change_people_gain_activity_schedule,
    "change-people-gain-people-per-area": _invert_change_people_gain_people_per_area,
    "change-people-gain-sensible-fraction": _invert_change_people_gain_sensible_fraction,
    "change-people-gain-latent-fraction": _invert_change_people_gain_latent_fraction,
    "change-people-gain-radiant-fraction": _invert_change_people_gain_radiant_fraction,
    "create-lighting-gain": _invert_create_lighting_gain,
    "delete-lighting-gain": _invert_delete_lighting_gain,
    "change-lighting-gain-zone": _invert_change_lighting_gain_zone,
    "change-lighting-gain-schedule": _invert_change_lighting_gain_schedule,
    "change-lighting-gain-watts-per-area": _invert_change_lighting_gain_watts_per_area,
    "change-lighting-gain-radiant-fraction": _invert_change_lighting_gain_radiant_fraction,
    "change-lighting-gain-visible-fraction": _invert_change_lighting_gain_visible_fraction,
    "change-lighting-gain-return-air-fraction": _invert_change_lighting_gain_return_air_fraction,
    "create-equipment-gain": _invert_create_equipment_gain,
    "delete-equipment-gain": _invert_delete_equipment_gain,
    "change-equipment-gain-zone": _invert_change_equipment_gain_zone,
    "change-equipment-gain-schedule": _invert_change_equipment_gain_schedule,
    "change-equipment-gain-watts-per-area": _invert_change_equipment_gain_watts_per_area,
    "change-equipment-gain-radiant-fraction": _invert_change_equipment_gain_radiant_fraction,
    "change-equipment-gain-latent-fraction": _invert_change_equipment_gain_latent_fraction,
    "create-infiltration": _invert_create_infiltration,
    "delete-infiltration": _invert_delete_infiltration,
    "change-infiltration-zone": _invert_change_infiltration_zone,
    "change-infiltration-schedule": _invert_change_infiltration_schedule,
    "change-infiltration-flow-per-exterior-area": _invert_change_infiltration_flow_per_exterior_area,
    "change-infiltration-constant-term-coefficient": _invert_change_infiltration_constant_term_coefficient,
    "change-infiltration-temperature-term-coefficient": _invert_change_infiltration_temperature_term_coefficient,
    "change-infiltration-velocity-term-coefficient": _invert_change_infiltration_velocity_term_coefficient,
    "change-infiltration-velocity-squared-term-coefficient": _invert_change_infiltration_velocity_squared_term_coefficient,
    "create-mechanical-ventilation": _invert_create_mechanical_ventilation,
    "delete-mechanical-ventilation": _invert_delete_mechanical_ventilation,
    "change-mechanical-ventilation-zone": _invert_change_mechanical_ventilation_zone,
    "change-mechanical-ventilation-schedule": _invert_change_mechanical_ventilation_schedule,
    "change-mechanical-ventilation-design-flow": _invert_change_mechanical_ventilation_design_flow,
    "change-mechanical-ventilation-fan-total-efficiency": _invert_change_mechanical_ventilation_fan_total_efficiency,
    "change-mechanical-ventilation-fan-delta-pressure": _invert_change_mechanical_ventilation_fan_delta_pressure,
    "change-infiltration-method": _invert_change_infiltration_method,
    "change-infiltration-design-flow-ach": _invert_change_infiltration_design_flow_ach,
    "change-infiltration-effective-leakage-area": _invert_change_infiltration_effective_leakage_area,
    "change-infiltration-discharge-coefficient": _invert_change_infiltration_discharge_coefficient,
    "change-infiltration-stack-height": _invert_change_infiltration_stack_height,
    "create-thermostat": _invert_create_thermostat,
    "delete-thermostat": _invert_delete_thermostat,
    "change-thermostat-zone": _invert_change_thermostat_zone,
    "change-thermostat-heating-setpoint-schedule": _invert_change_thermostat_heating_setpoint_schedule,
    "change-thermostat-cooling-setpoint-schedule": _invert_change_thermostat_cooling_setpoint_schedule,
    "change-thermostat-heating-throttle-range": _invert_change_thermostat_heating_throttle_range,
    "change-thermostat-cooling-throttle-range": _invert_change_thermostat_cooling_throttle_range,
    "create-humidistat": _invert_create_humidistat,
    "delete-humidistat": _invert_delete_humidistat,
    "change-humidistat-zone": _invert_change_humidistat_zone,
    "change-humidistat-humidifying-setpoint-schedule": _invert_change_humidistat_humidifying_setpoint_schedule,
    "change-humidistat-dehumidifying-setpoint-schedule": _invert_change_humidistat_dehumidifying_setpoint_schedule,
    "change-humidistat-humidifying-throttle-range": _invert_change_humidistat_humidifying_throttle_range,
    "change-humidistat-dehumidifying-throttle-range": _invert_change_humidistat_dehumidifying_throttle_range,
    "create-ideal-loads-system": _invert_create_ideal_loads_system,
    "delete-ideal-loads-system": _invert_delete_ideal_loads_system,
    "change-ideal-loads-system-zone": _invert_change_ideal_loads_system_zone,
    "change-ideal-loads-system-max-heating-supply-air-temp": _invert_change_ideal_loads_system_max_heating_supply_air_temp,
    "change-ideal-loads-system-min-cooling-supply-air-temp": _invert_change_ideal_loads_system_min_cooling_supply_air_temp,
    "change-ideal-loads-system-max-heating-capacity": _invert_change_ideal_loads_system_max_heating_capacity,
    "change-ideal-loads-system-max-cooling-capacity": _invert_change_ideal_loads_system_max_cooling_capacity,
    "change-ideal-loads-system-outdoor-air-per-person": _invert_change_ideal_loads_system_outdoor_air_per_person,
    "change-ideal-loads-system-outdoor-air-per-area": _invert_change_ideal_loads_system_outdoor_air_per_area,
    "create-zone-equipment": _invert_create_zone_equipment,
    "delete-zone-equipment": _invert_delete_zone_equipment,
    "change-zone-equipment-zone": _invert_change_zone_equipment_zone,
    "change-zone-equipment-type": _invert_change_zone_equipment_type,
    "change-zone-equipment-priority": _invert_change_zone_equipment_priority,
    "change-zone-equipment-heating-capacity": _invert_change_zone_equipment_heating_capacity,
    "change-zone-equipment-cooling-capacity": _invert_change_zone_equipment_cooling_capacity,
    "create-daylight-zone": _invert_create_daylight_zone,
    "delete-daylight-zone": _invert_delete_daylight_zone,
    "change-daylight-zone-zone": _invert_change_daylight_zone_zone,
    "change-daylight-zone-illuminance-target": _invert_change_daylight_zone_illuminance_target,
    "change-daylight-zone-glare-limit": _invert_change_daylight_zone_glare_limit,
    "change-daylight-zone-window-transmittance": _invert_change_daylight_zone_window_transmittance,
    "create-sizing-object": _invert_create_sizing_object,
    "delete-sizing-object": _invert_delete_sizing_object,
    "change-sizing-object-zone": _invert_change_sizing_object_zone,
    "change-sizing-object-sizing-type": _invert_change_sizing_object_sizing_type,
    "change-sizing-object-design-day-type": _invert_change_sizing_object_design_day_type,
    "create-room-air-model-assignment": _invert_create_room_air_model_assignment,
    "delete-room-air-model-assignment": _invert_delete_room_air_model_assignment,
    "change-room-air-model": _invert_change_room_air_model,
    "create-setpoint-manager": _invert_create_setpoint_manager,
    "delete-setpoint-manager": _invert_delete_setpoint_manager,
    "rename-setpoint-manager": _invert_rename_setpoint_manager,
    "replace-setpoint-manager-kind": _invert_replace_setpoint_manager_kind,
    "change-setpoint-manager-schedule": _invert_change_setpoint_manager_schedule,
    "create-air-loop": _invert_create_air_loop,
    "delete-air-loop": _invert_delete_air_loop,
    "rename-air-loop": _invert_rename_air_loop,
    "change-air-loop-supply-node": _invert_change_air_loop_supply_node,
    "change-air-loop-return-node": _invert_change_air_loop_return_node,
    "change-air-loop-design-supply-air-flow": _invert_change_air_loop_design_supply_air_flow,
    "add-air-loop-terminal-zone": _invert_add_air_loop_terminal_zone,
    "remove-air-loop-terminal-zone": _invert_remove_air_loop_terminal_zone,
    "create-plant-loop": _invert_create_plant_loop,
    "delete-plant-loop": _invert_delete_plant_loop,
    "rename-plant-loop": _invert_rename_plant_loop,
    "change-plant-loop-type": _invert_change_plant_loop_type,
    "change-plant-loop-supply-temperature": _invert_change_plant_loop_supply_temperature,
    "change-plant-loop-return-temperature": _invert_change_plant_loop_return_temperature,
    "change-plant-loop-design-flow": _invert_change_plant_loop_design_flow,
    "add-plant-loop-equipment": _invert_add_plant_loop_equipment,
    "remove-plant-loop-equipment": _invert_remove_plant_loop_equipment,
    "create-outdoor-air-system": _invert_create_outdoor_air_system,
    "delete-outdoor-air-system": _invert_delete_outdoor_air_system,
    "change-outdoor-air-system-air-loop": _invert_change_outdoor_air_system_air_loop,
    "change-outdoor-air-system-min-oa-flow": _invert_change_outdoor_air_system_min_oa_flow,
    "change-outdoor-air-system-economizer-enabled": _invert_change_outdoor_air_system_economizer_enabled,
    "create-electrical-load-center": _invert_create_electrical_load_center,
    "delete-electrical-load-center": _invert_delete_electrical_load_center,
    "rename-electrical-load-center": _invert_rename_electrical_load_center,
    "add-electrical-load-center-pv": _invert_add_electrical_load_center_pv,
    "remove-electrical-load-center-pv": _invert_remove_electrical_load_center_pv,
    "add-electrical-load-center-battery": _invert_add_electrical_load_center_battery,
    "remove-electrical-load-center-battery": _invert_remove_electrical_load_center_battery,
    "create-pv-system": _invert_create_pv_system,
    "delete-pv-system": _invert_delete_pv_system,
    "change-pv-system-dc-capacity": _invert_change_pv_system_dc_capacity,
    "change-pv-system-area": _invert_change_pv_system_area,
    "change-pv-system-tilt": _invert_change_pv_system_tilt,
    "change-pv-system-azimuth": _invert_change_pv_system_azimuth,
    "change-pv-system-module-efficiency": _invert_change_pv_system_module_efficiency,
    "change-pv-system-inverter-efficiency": _invert_change_pv_system_inverter_efficiency,
    "create-battery": _invert_create_battery,
    "delete-battery": _invert_delete_battery,
    "change-battery-capacity": _invert_change_battery_capacity,
    "change-battery-max-charge": _invert_change_battery_max_charge,
    "change-battery-max-discharge": _invert_change_battery_max_discharge,
    "change-battery-round-trip-efficiency": _invert_change_battery_round_trip_efficiency,
    "create-shw-system": _invert_create_shw_system,
    "delete-shw-system": _invert_delete_shw_system,
    "change-shw-system-heater-capacity": _invert_change_shw_system_heater_capacity,
    "change-shw-system-storage-volume": _invert_change_shw_system_storage_volume,
    "change-shw-system-setpoint": _invert_change_shw_system_setpoint,
    "change-shw-system-schedule": _invert_change_shw_system_schedule,
    "create-solar-thermal-system": _invert_create_solar_thermal_system,
    "delete-solar-thermal-system": _invert_delete_solar_thermal_system,
    "change-solar-thermal-system-collector-area": _invert_change_solar_thermal_system_collector_area,
    "change-solar-thermal-system-efficiency": _invert_change_solar_thermal_system_efficiency,
    "change-solar-thermal-system-storage-volume": _invert_change_solar_thermal_system_storage_volume,
    "change-solar-thermal-system-tilt": _invert_change_solar_thermal_system_tilt,
    "change-solar-thermal-system-azimuth": _invert_change_solar_thermal_system_azimuth,
    "create-refrigeration-system": _invert_create_refrigeration_system,
    "delete-refrigeration-system": _invert_delete_refrigeration_system,
    "change-refrigeration-system-case-count": _invert_change_refrigeration_system_case_count,
    "change-refrigeration-system-design-load": _invert_change_refrigeration_system_design_load,
    "change-refrigeration-system-defrost-schedule": _invert_change_refrigeration_system_defrost_schedule,
    "create-water-system": _invert_create_water_system,
    "delete-water-system": _invert_delete_water_system,
    "change-water-system-fixture-count": _invert_change_water_system_fixture_count,
    "change-water-system-peak-flow": _invert_change_water_system_peak_flow,
    "change-water-system-schedule": _invert_change_water_system_schedule,
    "create-fault": _invert_create_fault,
    "delete-fault": _invert_delete_fault,
    "change-fault-target-equipment": _invert_change_fault_target_equipment,
    "change-fault-type": _invert_change_fault_type,
    "change-fault-severity": _invert_change_fault_severity,
    "change-fault-start-schedule": _invert_change_fault_start_schedule,
    "create-space-list": _invert_create_space_list,
    "delete-space-list": _invert_delete_space_list,
    "rename-space-list": _invert_rename_space_list,
    "add-space-list-member": _invert_add_space_list_member,
    "remove-space-list-member": _invert_remove_space_list_member,
    "create-thermal-enclosure": _invert_create_thermal_enclosure,
    "delete-thermal-enclosure": _invert_delete_thermal_enclosure,
    "rename-thermal-enclosure": _invert_rename_thermal_enclosure,
    "add-thermal-enclosure-zone": _invert_add_thermal_enclosure_zone,
    "remove-thermal-enclosure-zone": _invert_remove_thermal_enclosure_zone,
    "create-constant-schedule": _invert_create_constant_schedule,
    "delete-constant-schedule": _invert_delete_constant_schedule,
    "change-constant-schedule-value": _invert_change_constant_schedule_value,
    "create-daily-schedule": _invert_create_daily_schedule,
    "delete-daily-schedule": _invert_delete_daily_schedule,
    "replace-daily-schedule-hourly-values": _invert_replace_daily_schedule_hourly_values,
    "change-daily-schedule-interpolation": _invert_change_daily_schedule_interpolation,
    "change-daily-schedule-limits": _invert_change_daily_schedule_limits,
    "create-weekly-schedule": _invert_create_weekly_schedule,
    "delete-weekly-schedule": _invert_delete_weekly_schedule,
    "change-weekly-schedule-day": _invert_change_weekly_schedule_day,
    "create-annual-schedule": _invert_create_annual_schedule,
    "delete-annual-schedule": _invert_delete_annual_schedule,
    "insert-annual-schedule-rule": _invert_insert_annual_schedule_rule,
    "remove-annual-schedule-rule": _invert_remove_annual_schedule_rule,
    "reorder-annual-schedule-rules": _invert_reorder_annual_schedule_rules,
    "change-annual-schedule-default-daily-schedule": _invert_change_annual_schedule_default_daily_schedule,
    "change-annual-schedule-holiday-daily-schedule": _invert_change_annual_schedule_holiday_daily_schedule,
    "add-annual-schedule-holiday": _invert_add_annual_schedule_holiday,
    "remove-annual-schedule-holiday": _invert_remove_annual_schedule_holiday,
    "create-time-series-schedule": _invert_create_time_series_schedule,
    "delete-time-series-schedule": _invert_delete_time_series_schedule,
    "replace-time-series-schedule-values": _invert_replace_time_series_schedule_values,
    "change-time-series-schedule-timestep": _invert_change_time_series_schedule_timestep,
}
# endregion 🔖️Vocabulary


# region 🔖️Inverse
def invert(kind, before, payload):
    """↩️ The undo steps a kind owes, always read off BASE — never by inverting a delta. A refused or
    no-op forward step owes nothing (taxonomy.md's addressing convention)."""
    after, outcome = VOCABULARY[kind](before, payload)
    if outcome["status"] == "rejected" or after == before:
        return []
    if kind in EXTRA_INVERT:
        return EXTRA_INVERT[kind](before, payload)
    if kind == "rename-model":
        return [("rename-model", {"newName": before["model"]["name"]})]
    if kind == "change-model-version":
        return [("change-model-version", {"newVersion": before["model"]["version"]})]
    if kind == "update-site":
        site = before["model"]["site"]
        return [("update-site", {"latitudeDeg": site["latitude_deg"], "longitudeDeg": site["longitude_deg"], "elevationM": site["elevation_m"], "timeZoneHours": site["time_zone_hours"], "northAxisDeg": site["north_axis_deg"]})]
    if kind == "update-ground-temperature":
        ground = before["model"]["ground_temperature"]
        return [("update-ground-temperature", {"buildingSurfaceC": ground["building_surface_c"], "shallowC": ground["shallow_c"], "deepC": ground["deep_c"]})]
    if kind == "update-run-period":
        run_period = before["model"]["run_period"]
        return [("update-run-period", {"startMonth": run_period["start_month"], "startDay": run_period["start_day"], "endMonth": run_period["end_month"], "endDay": run_period["end_day"], "year": run_period["year"]})]
    if kind == "replace-airflow-network":
        network = before["model"]["airflow_network"]
        if network is None:
            return [("replace-airflow-network", {"present": False, "zoneIds": [], "nodeIds": [], "outdoorNodeId": 0, "linkIds": []})]
        return [("replace-airflow-network", {"present": True, "zoneIds": [pair[0] for pair in network["zone_node_ids"]], "nodeIds": [pair[1] for pair in network["zone_node_ids"]], "outdoorNodeId": network["outdoor_node_id"], "linkIds": network["link_ids"]})]
    if kind == "add-output-variable":
        return [("remove-output-variable", {"name": payload["name"], "key": payload["key"]})]
    if kind == "remove-output-variable":
        spec = next(spec for spec in before["model"]["output_variables"] if spec["name"] == payload["name"] and spec["key"] == payload["key"])
        return [("add-output-variable", {"name": spec["name"], "key": spec["key"], "reportingFrequency": spec["reporting_frequency"]})]
    if kind in ("bind-weather-file", "unbind-weather-file"):
        existing = before.get("weatherLink")
        return [("bind-weather-file", {"targetUri": _uri_of(existing)})] if existing else [("unbind-weather-file", {})]
    if kind in ("connect-referenced-model", "disconnect-referenced-model"):
        existing = before.get("referencedModel")
        return [("connect-referenced-model", {"targetUri": _uri_of(existing)})] if existing else [("disconnect-referenced-model", {})]
    zone = _zone(before, payload["id"])
    if kind == "rename-zone":
        return [("rename-zone", {"id": payload["id"], "newName": zone["name"]})]
    if kind == "change-zone-volume":
        return [("change-zone-volume", {"id": payload["id"], "newVolumeM3": zone["volume_m3"]})]
    if kind == "change-zone-multiplier":
        return [("change-zone-multiplier", {"id": payload["id"], "newMultiplier": zone["multiplier"]})]
    if kind == "change-zone-conditioned":
        return [("change-zone-conditioned", {"id": payload["id"], "newConditioned": zone["conditioned"]})]
    if kind == "change-zone-floor-area-participation":
        return [("change-zone-floor-area-participation", {"id": payload["id"], "newPartOfTotalFloorArea": zone["part_of_total_floor_area"]})]
    raise AssertionError(f"no inverse is written for {kind!r}")


def _uri_of(link):
    """🔗️ Flattens a link's `ArtifactRef` back to the URI form `bind`/`connect` payloads carry."""
    target = link["target"]
    dialect = target["dialect"]
    return f"{target['artifactId']}!{dialect['artifactKind']}@{dialect['standard']}/{dialect['subset']}"
# endregion 🔖️Inverse


# region 🔖️Oracle
def _kind_of(scenario, wire):
    tag, payload = unwrap(wire)
    for kind in VOCABULARY:
        if wire_tag(kind) == tag:
            return kind, payload
    raise AssertionError(f"unexpected wire tag {tag!r} for scenario {scenario!r}")


def _mutate_for(scenario):
    def handler(ctx: Context) -> Outcome:
        before, wire, expected_after, expected_outcome = _vector(ctx, scenario)
        kind, payload = _kind_of(scenario, wire)
        after, outcome = VOCABULARY[kind](before, payload)
        assert after == expected_after, f"mutate-{scenario}: {after} != committed after-snapshot {expected_after}"
        assert outcome == expected_outcome, f"mutate-{scenario}: {outcome} != committed outcome {expected_outcome}"
        return Outcome(projection=after, raw=json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8"))

    return handler


def _inverse_for(scenario):
    def handler(ctx: Context) -> Outcome:
        before, wire, _expected_after, _expected_outcome = _vector(ctx, scenario)
        kind, payload = _kind_of(scenario, wire)
        after, _outcome = VOCABULARY[kind](before, payload)
        restored = after
        for step_kind, step_payload in invert(kind, before, payload):
            restored, step_outcome = VOCABULARY[step_kind](restored, step_payload)
            assert step_outcome["status"] != "rejected", f"inverse-{scenario}: the undo step {step_kind} was itself refused"
        assert restored == before, f"inverse-{scenario}: {restored} != committed before-snapshot {before}"
        return Outcome(projection=restored, raw=json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8"))

    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only: registering these handlers as subjects too would make the
    reference its own subject and manufacture a guaranteed-green self-comparison."""
    built = Adapter("python")
    for scenario in VECTOR_ROOTS:
        built = built.oracle(f"mutate-{scenario}", _mutate_for(scenario)).oracle(f"inverse-{scenario}", _inverse_for(scenario))
    return built
# endregion 🔖️Registration
