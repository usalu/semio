"""🔢 Native STEP real tokens preserve finite binary64 values without geometry quantization."""
from pathlib import Path
import importlib,json,math,re,struct
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
exchange=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def test_native_real_codec_preserves_binary64_and_independent_numpy_parse():
    """🔢 Every neutral finite token follows STEP real syntax and has independently parsed exact IEEE754 bits."""
    import numpy as np
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(CASE)
    for row in CASE['cases']:
        token=exchange._real(row['value'])
        if 'minimal_token' in row:assert token==row['minimal_token']
        assert re.fullmatch(r'[+-]?[0-9]+\.[0-9]*(?:E[+-]?[0-9]+)?',token),token
        actual=np.fromstring(token,dtype=np.float64,sep=' ');assert len(actual)==1
        assert struct.pack('>d',float(actual[0])).hex()==row['expected_bits'],(row,token)
        assert math.isfinite(float(actual[0]))

def test_native_real_codec_rejects_nonfinite_values():
    """🚫 Nonfinite numbers cannot enter a native STEP coordinate or measure."""
    for text in CASE['invalid']:
        try:exchange._real(float(text))
        except ValueError:continue
        raise AssertionError(text)

def test_oblique_closed_native_faces_keep_exact_coordinate_planes_after_codec():
    """🔷 A real oblique rounded stock stays closed and convex after native token decoding, with independent SciPy volume."""
    import numpy as np
    from scipy.spatial import ConvexHull
    part=CASE['mesh'];raw=part['mesh']['vertices']
    vertices=[[float(np.fromstring(exchange._real(value),dtype=np.float64,sep=' ')[0]) for value in point] for point in raw]
    assert vertices==raw
    native=api._polyhedron_part(vertices,part['mesh']['faces'])
    points=np.asarray(vertices)
    assert abs(api._polyhedron_volume(native)-ConvexHull(points-points[0]).volume)<1e-12
