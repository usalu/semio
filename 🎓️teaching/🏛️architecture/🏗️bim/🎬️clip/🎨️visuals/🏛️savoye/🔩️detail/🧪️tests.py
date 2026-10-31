"""🧱 Preserve actual stepped material sections in native wall details."""
from copy import deepcopy
import importlib,json,os
from pathlib import Path
from unittest.mock import patch


def test_native_wall_detail_sections_actual_material_instead_of_poisoned_gross_thickness():
    import numpy as np
    from shapely.geometry import Polygon
    from jsonschema import Draft202012Validator
    from manim import tempconfig
    p=Path(__file__);case=json.loads(p.with_suffix('.json').read_text())
    Draft202012Validator(json.loads(p.with_suffix('.schema.json').read_text())).validate(case)
    visuals=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️')
    detail=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🔩️detail.🐍️')
    g={'walls':[deepcopy(case['wall'])],'fillings':[deepcopy(case['filling'])]};before=deepcopy(g)
    for language in case['languages']:
        with patch.dict(os.environ,{'SEMIO_TEACHING_LANGUAGE':language}),tempconfig({'dry_run':True,'pixel_width':320,'pixel_height':180}),patch.object(detail._c,'geometry',return_value=g):
            scene=visuals.ClipScene();drawing=scene.wall_detail((0,0,0),case['scale'],model={},tag=case['wall']['tag'],dims=False)
            wall=drawing.parts[case['wall']['tag']];glass=drawing.parts[case['filling']['tag']]
            assert abs(wall.width-case['expected']['wall_width'])<1e-12
            assert abs(glass.width-case['expected']['glass_width'])<1e-12
            def area(group):
                return sum(Polygon(np.asarray(shape.get_start_anchors())[:,:2]).area for shape in group.submobjects)
            assert abs(area(wall)-case['expected']['wall_area'])<1e-12
            assert abs(area(glass)-case['expected']['glass_area'])<1e-12
            assert g==before
