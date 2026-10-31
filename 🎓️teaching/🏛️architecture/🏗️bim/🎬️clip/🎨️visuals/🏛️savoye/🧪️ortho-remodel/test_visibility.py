"""🧪 Production physical-height plans and true spatial structural elevations."""
import importlib,json,unittest,sys
from pathlib import Path
from types import SimpleNamespace
import numpy as np
from shapely.geometry import Polygon,MultiPoint
from shapely.ops import unary_union

sys.path.insert(0,str(Path.cwd()))
BASE=Path(__file__).parent
PREFIX='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
H=importlib.import_module(PREFIX+'🎨️visuals.🏛️savoye.🧪️ortho-remodel.test_frustums')
CASES=json.loads((BASE/'visibility.json').read_text(encoding='utf8'))


def plan_output(data,case):
    """📋 Execute the actual production solid loop through the recording drawing interface."""
    scope=H._scope();scene=scope['Plan']();scene._pair=lambda *a:(0,0,1,0);recorded=[]
    ctx={'bands':{case['level']:case['band']},'level':case['level'],'g':{'storeys':{'ground':0},'details':[data]},'catalogue':[],'look':None,'use':{'architecture'},'below':True,'cut_z':case['cut'],'screen':lambda ring:np.c_[ring,np.zeros(len(ring))],'put':lambda *a:recorded.append(a[-1])}
    scene._plan_solids(ctx)
    return unary_union([p for group in recorded for p in H._polygons(group)])


def elevation_output(data,kind):
    """🏛️ Execute the complete structural elevation provider with actual world vertices."""
    scope=H._scope();scene=scope['Section']();scene._pair=lambda *a:(0,0,1,0)
    class View:
        """📽️ South orthographic projection with an independent NumPy facing calculation."""
        def __init__(self,*a,**k):pass
        def __call__(self,x,y,z):return np.array([x,z,0])
        def facing(self,normal):return np.dot(normal,[0,-1,0])
        def depth(self,x,y,z):return -y
    scope['_d'].View=View;scope['P']=lambda p:np.asarray(p,float)
    scope['_s'].Look=lambda *a:SimpleNamespace(state=lambda item:'normal')
    scope['_s'].Sheet=lambda *a:H.Shape();scope['_s'].expand_names=lambda names,g:names
    g={'box':{'length':20,'width':22,'height':7},'storeys':{'cellar':-2.4},'slabs':[],kind:[data]}
    scope['_c'].geometry=lambda ignored:g;scope['_s'].items=lambda ignored:[{'tag':data['tag'],'kind':kind,'data':data}]
    drawing=scene.villa_elevation('south',scale=1,centre=(0,0),model=g,ground=False,levels=False,layers=('structure',))
    return unary_union(list(H._polygons(drawing.parts[data['tag']])))


class PhysicalVisibility(unittest.TestCase):
    """🧱 Ownership storeys must not substitute for occupied physical extents."""
    def test_cross_storey_architectural_material_and_floor_ownership(self):
        """📐 Compare real production polygons with declarative GEOS occupied contours."""
        part={'outline':[[0,0],[1,0],[1,1],[0,1]],'z0':-2.4,'z1':6.34}
        data={'tag':'CROSS','guid':'cross','class':'IfcBuildingElementProxy','domains':['architecture'],'kind':'tube','storey':'ground','parts':[part]}
        for case in CASES['plan']['cases']:
            with self.subTest(level=case['level']):
                output=plan_output(data,case)
                self.assertEqual(not output.is_empty,case['visible'])
                if case['visible']:self.assertEqual(output.symmetric_difference(Polygon(part['outline'])).area,0)
        floor={**data,'class':'IfcCovering','predefined_type':'FLOORING','kind':'composite'}
        self.assertTrue(plan_output(floor,CASES['plan']['cases'][1]).is_empty)
        short={**data,'parts':[{**part,'z1':1}]}
        self.assertTrue(plan_output(short,CASES['plan']['cases'][1]).is_empty)

    def test_structural_mesh_projects_actual_inclined_faces(self):
        """📏 A mesh cannot be replaced by its rectangular height envelope."""
        part=CASES['elevation']['part'];data={'tag':'INCLINE','guid':'incline','class':'IfcBeam','domains':['structure'],'kind':'beam','parts':[part]}
        actual=elevation_output(data,'beams');expected=Polygon(CASES['elevation']['expected_xz'])
        self.assertLess(actual.symmetric_difference(expected).area,1e-15)
        self.assertNotEqual(actual.area,4)
        section={k:value for k,value in part.items() if k!='mesh'}
        section['section']={'plane':'XZ','sweep':[0,1],'profile':CASES['elevation']['expected_xz']}
        actual=elevation_output({**data,'parts':[section]},'beams')
        self.assertLess(actual.symmetric_difference(expected).area,1e-15)

    def test_actual_tube_and_structural_native_products(self):
        """🏠 Preserve the exact current source tube, inclined stringer and capped L support."""
        api=H.MODEL;m=api.load_model();tube=api.solid_geometry(CASES['source']['tube'],m)
        self.assertEqual(len(tube['parts']),64)
        tube.update(domains=['architecture'],storey='ground')
        for case in CASES['plan']['cases'][:2]:self.assertFalse(plan_output(tube,case).is_empty)
        for key,kind in (('stringer','beams'),('column','pilotis')):
            tag=CASES['source'][key]
            body=api.solid_geometry(tag,m) if key=='stringer' else api.column_geometry(m,api._find(m,tag))
            actual=elevation_output(body,kind)
            faces=[Polygon(np.asarray(api.part_vertices(part),float)[face][:,[0,2]]) for part in body['parts'] for face in api.part_faces(part)]
            expected=unary_union([polygon for polygon in faces if polygon.area>0])
            self.assertLess(actual.symmetric_difference(expected).area,1e-13)


if __name__=='__main__':unittest.main()
