"""🚗 Independent physical parking controls and explicit rejection of unverified three-car demonstrations."""
import hashlib
import importlib
import json
import math
from collections import Counter
from pathlib import Path
import unittest

import numpy as np
from shapely.affinity import rotate,translate
from shapely.geometry import Polygon,MultiPoint,box
from shapely.strtree import STRtree

api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
PATH=Path(__file__).with_suffix('.json')
_overlay=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚗️overlay')


def _polygon_rings(geometry):
    if geometry.is_empty or geometry.area == 0:
        return []
    if geometry.geom_type == "Polygon":
        return [[[list(map(float, point)) for point in ring.coords] for ring in [geometry.exterior, *geometry.interiors]]]
    if geometry.geom_type in ("MultiPolygon", "GeometryCollection"):
        return [polygon for part in geometry.geoms for polygon in _polygon_rings(part)]
    raise ValueError("Expected polygonal occupied area")


def _overlay_cases(cases):
    response = json.loads(_overlay.overlay_json(json.dumps({"cases":cases}, allow_nan=False, separators=(",", ":"))))
    if response.get("engine") != "fraction-sweep" or [case["id"] for case in response["cases"]] != [case["id"] for case in cases]:
        raise ValueError("Overlay batch identity mismatch")
    return response["cases"]


def _level(source,value):
    if isinstance(value,(int,float)):return value
    found=next((s for s in source['storeys'] if s['tag']==value),None)
    if found:return found['elevation']['value']
    if value.startswith('soffit:'):
        slab=next(e for e in source['elements'] if e['tag']==value[7:]);base=_level(source,slab['geometry']['level'])
        return min(base+r['top_offset']['value']-r['thickness']['value'] for r in slab['geometry']['regions'])
    raise ValueError(value)


def _columns(source,vehicle):
    result=[];types={t['tag']:t for t in source['types']}
    for e in source['elements']:
        if e['class']!='IfcColumn':continue
        g=e['geometry']
        for section in g.get('sections',[dict(profile=types[e['type']]['profile'],axis=g['axis'],base_level=g['base_level'],top_level=g['top_level'])]):
            z0,z1=_level(source,section['base_level']),_level(source,section['top_level'])
            if min(z1,vehicle['z0']+vehicle['height'])-max(z0,vehicle['z0'])<=1e-9:continue
            p=section['profile'];angle=math.radians(p.get('rotation',0));c,s=math.cos(angle),math.sin(angle)
            if p['shape'] in ['circle','ellipse']:
                w=p['diameter']['value'] if p['shape']=='circle' else p['width']['value'];d=w if p['shape']=='circle' else p['depth']['value']
                ring=np.array([[w/2*math.cos(i*math.pi/48),d/2*math.sin(i*math.pi/48)] for i in range(96)])
            elif p['shape']=='rectangle':ring=np.array([[-1,-1],[1,-1],[1,1],[-1,1]])*np.array([p['width']['value'],p['depth']['value']])/2
            elif p['shape']=='polygon':ring=np.array(p['outline'])
            else:raise ValueError(p['shape'])
            outline=ring@np.array([[c,s],[-s,c]])+np.array(section.get('axis',g['axis']))
            result.append({'tag':e['tag'],'outline':outline.tolist(),'z0':z0,'z1':z1})
    return result


def _signature(source,vehicle):
    active={p['tag'] for p in _columns(source,vehicle)}
    def overlap(g):
        try:
            base=_level(source,g.get('base_level','ground'))
            def bounds(p):
                if 'mesh' in p:return min(v[2] for v in p['mesh']['vertices']),max(v[2] for v in p['mesh']['vertices'])
                if 'section' in p:return min(v[1] for v in p['section']['profile']),max(v[1] for v in p['section']['profile'])
                return p['z0'],p['z1']
            return any(min(base+hi,vehicle['z0']+vehicle['height'])-max(base+lo,vehicle['z0'])>1e-9 for lo,hi in map(bounds,g['parts'])) if 'parts' in g else True
        except (KeyError,ValueError):return True
    def pick(obj,keys):return {k:obj[k] for k in keys if k in obj}
    elements=sorted([pick(e,['tag','class','predefined_type','storey','type','material','geometry','attributes']) for e in source['elements'] if e['tag']=='SL-1' or e['tag'] in active or e['storey'] in ['ground','cellar'] and overlap(e['geometry'])],key=lambda e:e['tag'])
    wanted={e['type'] for e in elements if 'type' in e}
    types=sorted([pick(t,['tag','class','material','profile','filling','attributes']) for t in source['types'] if t['tag'] in wanted],key=lambda e:e['tag'])
    materials={v for e in elements+types for v in [e.get('material'),e.get('filling',{}).get('frame_material'),e.get('filling',{}).get('panel_material')] if v}
    changed=True
    while changed:
        before=set(materials)
        for m in source['materials']:
            if m['tag'] in materials:materials.update(l['material'] for l in m.get('layers',[]))
        changed=before!=materials
    selected=sorted([pick(m,['tag','class','layers']) for m in source['materials'] if m['tag'] in materials],key=lambda e:e['tag'])
    data={'scenario_vehicle':vehicle,'elements':elements,'types':types,'materials':selected,'storeys':[{'tag':s['tag'],'elevation':s['elevation']['value']} for s in source['storeys']],'car':source['car']}
    def normalize(v):
        if isinstance(v,bool) or v is None:return v
        if isinstance(v,(int,float)):return math.floor(v*1e9+.5)
        if isinstance(v,list):return [normalize(x) for x in v]
        if isinstance(v,dict):return {k:normalize(v[k]) for k in sorted(v)}
        return v
    return hashlib.sha256(json.dumps(normalize(data),ensure_ascii=False,separators=(',',':')).encode()).hexdigest()


def _rectangle(pose,vehicle):
    a=math.radians(pose['angle_degrees']);c,s=math.cos(a),math.sin(a)
    points=np.array([[-1,-1],[1,-1],[1,1],[-1,1]])*np.array([vehicle['length'],vehicle['width']])/2
    return Polygon(points@np.array([[c,s],[-s,c]])+np.array(pose['center']))


def evaluate(fixture,source,poses):
    """📐 Compute exact GEOS intersections and distances from independently transformed rectangles."""
    cars=[_rectangle(p,fixture['car']) for p in poses];garage=Polygon(fixture['outline']);columns=[(c['tag'],Polygon(c['outline'])) for c in _columns(source,fixture['car'])]
    physical=[(o,Polygon(o['outline'])) for o in fixture['physical_obstacles'] if o['kind']!='column'];shapes=[p for _,p in physical];tree=STRtree(shapes)
    violations=[];metrics=[]
    for i,car in enumerate(cars):
        outside=car.difference(garage).area;boundary=car.boundary.distance(garage.boundary)
        if outside>1e-8:violations.append({'kind':'outside','car':i,'value':outside})
        if boundary+1e-8<fixture['clearance']['boundary']:violations.append({'kind':'boundary','car':i,'value':boundary})
        column=min(car.distance(shape) for _,shape in columns)
        for tag,shape in columns:
            if car.distance(shape)+1e-8<fixture['clearance']['column_surface']:violations.append({'kind':'column','car':i,'tag':tag,'value':car.distance(shape)})
        nearest=car.distance(shapes[int(tree.nearest(car))])
        for index in tree.query(car):
            record,shape=physical[int(index)];area=car.intersection(shape).area
            if area>1e-8 or car.distance(shape)+1e-8<fixture['clearance']['physical_surface']:violations.append({'kind':'physical','car':i,'tag':record['tag'],'value':car.distance(shape),'area':area})
        for reserve in fixture['door_access_reserves']:
            area=car.intersection(Polygon(reserve['polygon'])).area
            if area>1e-8:violations.append({'kind':'reserve','car':i,'tag':reserve['opening'],'value':area})
        metrics.append({'outside_area':outside,'boundary_distance':boundary,'column_surface_distance':column,'physical_surface_distance':nearest})
    pairs=[]
    for i,car in enumerate(cars):
        for j,other in enumerate(cars[:i]):
            gap=car.distance(other);pairs.append(gap)
            if gap+1e-8<fixture['clearance']['car_pair']:violations.append({'kind':'pair','cars':[j,i],'value':gap})
    return {'valid':not violations,'violations':violations,'cars':metrics,'pair_gaps':pairs}


class ParkingContract(unittest.TestCase):
    """🅿️ A verified positive control never becomes a claim that the target three-car fleet fits."""
    @classmethod
    def setUpClass(cls):
        cls.fixture=json.loads(PATH.read_text(encoding='utf-8'));cls.source=json.loads(PATH.parent.parent.joinpath('🔣️.json').read_text(encoding='utf-8'))
        cls.results={c['id']:evaluate(cls.fixture,cls.source,c['poses']) for c in [cls.fixture['verified_control']]+cls.fixture['rejected_demonstrations']}

    def test_fixture_is_bound_to_current_physical_source(self):
        self.assertEqual(_signature(self.source,self.fixture['car']),self.fixture['source_signature'])
        self.assertEqual(next(s['outline'] for s in self.source['spaces'] if s['tag']==self.fixture['garage']),self.fixture['outline'])
        self.assertEqual(sorted({c['tag'] for c in _columns(self.source,self.fixture['car'])}),self.fixture['active_column_tags'])
        self.assertEqual(len(self.fixture['active_column_tags']),35)
        registry={e['tag']:e for e in self.source['elements']+self.source['types']}
        for reserve in self.fixture['door_access_reserves']:
            op=registry[reserve['opening']];wall=registry[op['geometry']['wall']];a,b=wall['geometry']['path'];length=math.dist(a,b)
            filling=next(e for e in self.source['elements'] if e['geometry'].get('opening')==op['tag']);spec=registry[filling['type']]['filling'];w=spec['frame_width']['value']
            for i,station in enumerate([op['attributes']['start']['value']+w,op['attributes']['end']['value']-w]):
                point=[a[j]+(b[j]-a[j])*station/length for j in (0,1)];self.assertLess(math.dist(point,reserve['polygon'][reserve['portal_indices'][i]]),1e-7)
        self.assertTrue(any(o['tag']=='GARAGE-SLIDING' and o['role']=='panel' for o in self.fixture['physical_obstacles']))
        self.assertTrue(any(o['tag']=='CW-1' and o['role']=='glass' for o in self.fixture['physical_obstacles']))

    def test_two_car_control_is_physically_verified(self):
        control=self.fixture['verified_control'];self.assertEqual(len(control['poses']),2)
        self.assertTrue(self.results[control['id']]['valid'],self.results[control['id']]['violations'])
        self.assertEqual(self.fixture['car']['length'],4.6);self.assertEqual(self.fixture['car']['width'],1.75)

    def test_three_car_demonstrations_are_rejected_without_infeasibility_claim(self):
        self.assertEqual(self.fixture['target']['status'],'unverified-current-source')
        self.assertEqual(self.fixture['target']['fleet_size'],3);self.assertFalse(self.fixture['target']['global_infeasibility_proven'])
        self.assertEqual(self.fixture['historical_capacity']['fleet_size'],3)
        for case in self.fixture['rejected_demonstrations']:
            result=self.results[case['id']];self.assertGreaterEqual(len(case['poses']),2);self.assertFalse(result['valid'])
            for tag in case.get('required_collision_tags',[]):self.assertTrue(any(v.get('tag')==tag for v in result['violations']),tag)
            for kind in case.get('required_violation_kinds',[]):self.assertTrue(any(v['kind']==kind for v in result['violations']),kind)

    def test_numpy_corners_and_javascript_metrics_match_geos(self):
        for case in [self.fixture['verified_control']]+self.fixture['rejected_demonstrations']:
            for pose in case['poses']:
                shape=translate(rotate(box(-2.3,-.875,2.3,.875),pose['angle_degrees'],origin=(0,0)),*pose['center'])
                self.assertLess(shape.symmetric_difference(_rectangle(pose,self.fixture['car'])).area,1e-8)
            expected=self.fixture['expected_results'][case['id']];actual=self.results[case['id']]
            self.assertEqual(actual['valid'],expected['valid'])
            for a,b in zip(actual['cars'],expected['cars']):
                for key in a:self.assertAlmostEqual(a[key],b[key],places=6)
            for a,b in zip(actual['pair_gaps'],expected['pair_gaps']):self.assertAlmostEqual(a,b,places=6)

    def test_exact_physical_overlay_matches_portable_and_source_cases(self):
        from shapely.ops import unary_union

        fixture = json.loads(PATH.with_name("🚗️overlays.json").read_text(encoding="utf-8"))
        cases = [{key:case[key] for key in ("id", "actual", "expected")} for case in fixture["cases"]]
        source = fixture["source_group"]["polygons"]
        cases += [{"id":"source-union", "actual":source, "expected":[]}, {"id":"source-identity", "actual":source, "expected":source}]
        areas = {case["id"]:case["xor_area"] for case in _overlay_cases(cases)}
        for case in fixture["cases"]:
            actual = unary_union([Polygon(value[0], value[1:]) for value in case["actual"]])
            expected = unary_union([Polygon(value[0], value[1:]) for value in case["expected"]])
            self.assertAlmostEqual(areas[case["id"]], actual.symmetric_difference(expected).area, places=10, msg=case["id"])
            if "xor_area" in case:
                self.assertAlmostEqual(areas[case["id"]], case["xor_area"], places=10, msg=case["id"])
            else:
                self.assertLess(areas[case["id"]], case["xor_max"], case["id"])
        self.assertGreater(areas["tiny-positive-area"], 0)
        self.assertLess(abs(areas["source-union"] - fixture["source_group"]["independent_union_area"]), fixture["total_xor_tolerance"])
        self.assertLess(areas["source-identity"], fixture["total_xor_tolerance"])

    def test_physical_oracle_matches_native_height_intersections(self):
        from collections import defaultdict
        from scipy.spatial import ConvexHull
        from shapely.ops import unary_union
        actual,expected=defaultdict(list),defaultdict(list)
        def key(record,lo,hi):return record['tag'],record['kind'],record['role'],round(lo,6),round(hi,6)
        for record in api._vehicle_parts(api.load_model()):
            part=record['geometry'];lo=max(part['z0'],self.fixture['car']['z0']);hi=min(part['z1'],self.fixture['car']['z0']+self.fixture['car']['height'])
            if hi-lo<=1e-10:continue
            if 'mesh' in part:
                vertices=np.asarray(part['mesh']['vertices']);hull=ConvexHull(vertices);points=[p[:2] for p in vertices if lo<=p[2]<=hi]
                for face in hull.simplices:
                    for i,j in zip(face,np.roll(face,1)):
                        a,b=vertices[i],vertices[j]
                        for z in (lo,hi):
                            if (a[2]-z)*(b[2]-z)<0:points.append((a+(b-a)*(z-a[2])/(b[2]-a[2]))[:2])
                shapes=[MultiPoint(points).convex_hull]
            elif part.get('holes'):shapes=[Polygon(part['outline'],part['holes'])]
            elif 'section' in part:
                sec=part['section'];cut=Polygon(sec['profile']).intersection(box(-1e6,lo,1e6,hi));components=list(cut.geoms) if hasattr(cut,'geoms') else [cut];shapes=[]
                for shape in components:
                    if shape.area<=1e-12:continue
                    u,_,v,_=shape.bounds;a,b=sec['sweep']
                    shapes.append(Polygon([[u,a],[v,a],[v,b],[u,b]] if sec['plane']=='XZ' else [[a,u],[b,u],[b,v],[a,v]]))
            elif 'top_outline' in part:
                alpha=(lo-part['z0'])/(part['z1']-part['z0']);beta=(hi-part['z0'])/(part['z1']-part['z0']);bottom=np.asarray(part['outline']);delta=np.asarray(part['top_outline'])-bottom
                shapes=[MultiPoint(np.concatenate([bottom+alpha*delta,bottom+beta*delta])).convex_hull]
            else:shapes=[Polygon(part['outline'])]
            actual[key(record,lo,hi)].extend(shapes)
        for record in self.fixture['physical_obstacles']:expected[key(record,record['z0'],record['z1'])].append(Polygon(record['outline']))
        self.assertEqual(set(actual),set(expected))
        cases = [{"id":json.dumps(group), "actual":[polygon for geometry in actual[group] for polygon in _polygon_rings(geometry)], "expected":[polygon for geometry in expected[group] for polygon in _polygon_rings(geometry)]} for group in actual]
        for case in _overlay_cases(cases):self.assertLess(case["xor_area"],1e-8,case["id"])

    def test_native_collision_band_projects_actual_section_occupancy(self):
        data=self.fixture['band_cases']
        from shapely.ops import unary_union
        for case in data['cases']:
            actual=unary_union([Polygon(r) for r in api.part_band_footprints(case['part'],*data['band'])]);expected=unary_union([Polygon(r) for r in case['expected']])
            self.assertLess(actual.symmetric_difference(expected).area,1e-9,case['id'])
        self.assertFalse(any(o['tag'] in self.fixture['operational_reserves']['covers'] for o in api.vehicle_obstacles(api.load_model())))

    def test_native_vehicle_checks_actual_hoist_and_reuses_isolated_geometry(self):
        model=api.load_model();case=next(c for c in self.fixture['rejected_demonstrations'] if c['id']=='retired-pre-hoist-two-control');car={'length':4.6,'width':1.75,'height':1.65,'ground':0}
        results=api._vehicle_obstacles(model,[[*p['center'],p['angle_degrees']] for p in case['poses']],car)
        self.assertEqual(next(r['clearance'] for r in results if r['tag']=='HOIST-JIB'),0)
        records=api.vehicle_obstacles(model);records.clear();self.assertGreater(len(api.vehicle_obstacles(model)),1000)

    def test_vehicle_checks_synthetic_height_intersecting_service(self):
        from copy import deepcopy
        model=deepcopy(api.load_model());pipe=deepcopy(next(e for e in model['elements'] if e['class']=='IfcPipeSegment'))
        pipe['tag']='TEST-VEHICLE-PIPE';pipe['geometry']={'points':[[0,0,0],[0,0,2]],'tube':{k:v for k,v in pipe['geometry']['tube'].items() if k not in ('clip_start','clip_end')}};model['elements'].append(pipe)
        car={'length':4.6,'width':1.75,'height':1.65,'ground':0}
        results=api._vehicle_obstacles(model,[[0,0,0]],car)
        self.assertEqual(next(r['clearance'] for r in results if r['tag']==pipe['tag']),0)


    def test_source_signature_selects_native_mesh_height_band(self):
        from copy import deepcopy
        empty={'elements':[],'types':[],'materials':[],'storeys':[{'tag':'ground','elevation':{'value':0}}],'car':{}}
        baseline=_signature(empty,self.fixture['car'])
        for case in self.fixture['signature_cases']:
            model=deepcopy(empty)
            model['elements']=[{'tag':'TEST-MESH','class':'IfcRailing','storey':'ground','geometry':{'kind':'solid','base_level':case['base_level'],'parts':[case['part']]}}]
            vertices=np.asarray(case['part']['mesh']['vertices'])[:,2]+case['base_level']
            occupied=bool(min(np.max(vertices),self.fixture['car']['z0']+self.fixture['car']['height'])-max(np.min(vertices),self.fixture['car']['z0'])>1e-9)
            self.assertEqual(occupied,case['included'],case['id'])
            self.assertEqual(_signature(model,self.fixture['car'])!=baseline,case['included'],case['id'])

if __name__=='__main__':unittest.main()
      