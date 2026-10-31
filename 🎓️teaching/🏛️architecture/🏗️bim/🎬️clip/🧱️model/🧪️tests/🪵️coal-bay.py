"""🪵 Continuous indoor coal-bay flooring and fixed-cupboard bearing."""

import importlib
import json
from pathlib import Path

import jsonschema
from shapely.geometry import Polygon
from shapely.ops import unary_union

A=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def test_coal_bay_indoor_finished_floor_supports_all_cupboard_feet_and_front():
    """🪑 Actual wearing surfaces support the cupboard without moving the lower service plate."""
    case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf-8'))
    jsonschema.validate(case,json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf-8')))
    m=A.load_model()
    slab=A._find(m,case['slab'])
    raw=next(p for p in slab['geometry']['regions'] if p['tag']==case['plate_tag'])
    assert raw['top_offset']['value']==case['plate_top']
    cover=A._slab_geometry(m,A._find(m,case['cover']))
    assert max(p['z1'] for p in cover['parts'])==case['plate_top']
    surfaces=[]
    for tag in case['floor_tags']:
        for p in A._solid_geometry(m,A._find(m,tag))['parts']:
            if p['z1']==case['finished_top']:
                surfaces.extend(Polygon(r) for r in A.part_regions(p,case['finished_top']))
    support=unary_union(surfaces)
    host=A._solid_geometry(m,A._find(m,case['cabinet']))
    feet=[p for p in host['parts'] if p['z0']==case['finished_top']]
    assert len(feet)==4
    for ring in [p['outline'] for p in feet]+[case['board_strip']]:
        shape=Polygon(ring)
        assert support.intersection(shape).area>=shape.area*case['minimum_supported_fraction'],ring
