"""🗂️ Content-bound read-only visual snapshot contracts."""

import copy
import importlib
import json
from pathlib import Path
from unittest.mock import patch

import numpy as np
import jsonschema

G = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.📐️geometry")


def test_visual_snapshots_follow_content_producers_and_isolate_consumers():
    """🧊 Equal JSON shares immutable snapshots; actual revisions invalidate them."""
    case = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
    jsonschema.validate(case,json.loads(Path(__file__).with_name("🧬️.schema.json").read_text(encoding="utf-8")))
    calls = []
    source = {"value":1,"points":[[1,2,3]],"models":[]}
    def build(m):
        calls.append(m)
        return {"points":copy.deepcopy(m["points"]),"value":m["value"]}
    with patch.object(G._model,"building_geometry",build),patch.object(G._model,"geometry_producers",lambda: (build,)):
        a,ga = G.geometry_context(source)
        b,gb = G.geometry_context(json.loads(json.dumps(source)))
        assert a is b and ga is gb and len(calls)==case["same_content_builds"]
        np.testing.assert_array_equal(np.asarray(ga["points"]),np.asarray(source["points"]))
        for mutate in (lambda: a.__setitem__("value",2),lambda: a["points"].append([4,5,6]),lambda: ga["points"][0].__setitem__(0,9)):
            try:
                mutate()
            except TypeError:
                pass
            else:
                raise AssertionError("mutable shared snapshot")
        owned = copy.deepcopy(ga)
        assert type(owned) is dict and type(owned["points"]) is list
        owned["points"][0][0]=90
        assert ga["points"][0][0]==1
        source["value"]=2
        before = G.geometry_context(source)[1]
        assert before["value"]==2
        changed = object()
        with patch.object(G._model,"geometry_producers",lambda: (build,changed)):
            after = G.geometry_context(source)[1]
            assert after is not before
            assert G.geometry_context(source)[1] is after
        for i in range(4):
            G.geometry_context({**source,"value":i})
        assert len(G._snapshots)<=case["retained_revisions"]
        assert len(G._sources)<=case["retained_revisions"]


def test_visual_default_and_membership_share_the_resolved_source_without_geometry():
    """🧭 Default source changes refresh; membership never constructs geometry."""
    value = [{"value":7,"points":[[7,8,9]],"models":[]}]
    stamp = [1]
    with patch.object(G,"_source_revision",lambda: stamp[0]),patch.object(G._model,"source_text",lambda: json.dumps(value[0])),patch.object(G._model,"building_geometry",side_effect=AssertionError("membership built geometry")),patch.object(G._model,"partial_model",lambda name,m: {"element_guids":[str(m["value"])]}):
        a = G.source_model()
        assert G.membership(a)["7"]==frozenset(G.DISCIPLINES)
        with patch.object(G.json,"dumps",side_effect=AssertionError("snapshot reserialized")):
            for _ in range(1000):
                assert G.source_model(a) is a
                assert G.membership(a)["7"]==frozenset(G.DISCIPLINES)
        assert G.source_model() is a
        assert G.source_model(json.loads(json.dumps(value[0]))) is a
        value[0]["value"]=8
        stamp[0]=2
        b = G.source_model()
        assert b is not a and b["value"]==8
        assert G.membership(b)["8"]==frozenset(G.DISCIPLINES)


def test_actual_source_snapshot_membership_matches_independent_source_lists():
    """🏛️ Actual discipline membership reads the immutable source without building geometry."""
    raw=json.loads(G._model.source_text())
    registry={o['tag']:o['guid'] for group in ('elements','types','spaces','storeys','materials') for o in raw[group]}
    expected={}
    for spec in raw['models']:
        if spec['id'] not in G.DISCIPLINES:
            continue
        for tag in spec['element_tags']:
            expected.setdefault(registry[tag],set()).add(spec['id'])
    with patch.object(G._model,'building_geometry',side_effect=AssertionError('source membership built geometry')):
        model=G.source_model(raw)
        table=G.membership(model)
    assert {k:set(v) for k,v in table.items()}==expected
    np.testing.assert_array_equal(np.sort(list(table)),np.sort(list(expected)))
