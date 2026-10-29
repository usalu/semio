"""🚘 Language-neutral vehicle/profile clearance checked against independent Shapely polygons."""
import importlib.util
import json
import math
from pathlib import Path
from shapely.geometry import Polygon, Point
from shapely.affinity import rotate, translate


def run(root):
    """🛞 Check exact physical profiles and semantic roles, including vertical exclusions."""
    directory = Path(root)/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model'
    spec = importlib.util.spec_from_file_location('vehicle_profiles_core',directory/'🐍️.py')
    core = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(core)
    fixture = json.loads((directory/'🧪️tests/🚘️profiles.json').read_text())
    car = fixture['car']
    checked = []
    for case in fixture['cases']:
        pose = case['pose']
        ring = core._car_outline(pose,car)
        independent = translate(rotate(Polygon([[-2.1,-.825],[2.1,-.825],[2.1,.825],[-2.1,.825]]),pose[2],origin=(0,0)),pose[0],pose[1])
        assert independent.symmetric_difference(Polygon(ring)).area < 1e-12
        obstacle = Polygon(case['outline'])
        actual = core._part_pose_distance(pose,car,{'outline':case['outline']})
        assert math.isclose(max(0,actual),independent.distance(obstacle),abs_tol=1e-12),(case['name'],actual)
        if independent.intersects(obstacle):
            assert actual <= 0
        checked.append(case['name'])
    for angle in range(0,360,13):
        pose = [1.4,-2,angle]
        circle = {'outline':core.profile_geometry({'shape':'circle','diameter':{'value':.28}},[3.,-2.])['outline'],'radius':.14}
        actual = core._part_pose_distance(pose,car,circle)
        oracle = Polygon(core._car_outline(pose,car)).distance(Point(3.,-2.))-.14
        assert math.isclose(actual,oracle,abs_tol=1e-12)
    ellipse = core.profile_geometry({'shape':'ellipse','width':{'value':.15},'depth':{'value':.4},'rotation':29},[3.,-2.])
    dense = core.profile_geometry(ellipse['profile'],[3.,-2.],4096)
    for angle in range(0,360,13):
        pose = [1.4,-2,angle]
        actual = core._part_pose_distance(pose,car,ellipse)
        exact = Polygon(core._car_outline(pose,car)).distance(Polygon(dense['outline']))
        assert actual <= exact+1e-10
        assert actual >= exact-2*ellipse['curve_error']-1e-10
    assert core._vehicle_part({'z0':-.2,'z1':1.0},car)
    assert not core._vehicle_part({'z0':2.91,'z1':6.34},car)
    assert not core._vehicle_part({'z0':-2.42,'z1':-.1},car)
    m = core.load_model()
    obj = next(e for e in m['elements'] if e['class']=='IfcColumn')
    m['roles'] = {'column_example':obj['tag'],'column_type_example':obj['type'],'floor':'SL-1','roof':'SL-R','entrance_glazing':'CW-1','ramp':'RM-1'}
    assert core.role('column_example',m) == obj['tag']
    assert core.role('column_type_example',m) == obj['type']
    try:
        core.role('missing',m)
    except ValueError:
        pass
    else:
        assert False
    m['types'] = [t for t in m['types'] if t['tag'] != 'TY-PILOTI']
    assert core._special(m,'car.width')[1]['value'] == m['car']['width']['value']
    assert core._special(m,'not-known') is None
    return checked + ['circle analytic distance','ellipse conservative curve bound','vertical occupancy','semantic roles','lazy information']
   