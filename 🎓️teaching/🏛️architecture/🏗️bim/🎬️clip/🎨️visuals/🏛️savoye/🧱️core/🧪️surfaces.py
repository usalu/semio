"""🎨 Portable material-role contracts without a graphical runtime."""
import importlib
import json
from pathlib import Path
import unittest


class SurfaceRoleContract(unittest.TestCase):
    """🪟 Glazing and finish surfaces must preserve their physical role in every provider."""

    def test_portable_material_dispatch(self):
        directory = Path(__file__).parent
        physical = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.📐️geometry")
        for case in json.loads((directory / "🧪️surfaces.json").read_text(encoding="utf-8"))["cases"]:
            self.assertEqual(physical.surface_role({"kind": case["kind"]}, case.get("part")), case["role"])


    def test_authored_finish_palette_is_shared_by_plan_section_and_axon(self):
        """🔷 Preserve explicit authored material hues through actual shared palette functions."""
        import ast
        from types import SimpleNamespace
        from PIL import Image
        directory = Path(__file__).parent
        cases = json.loads((directory / '🧪️surfaces.json').read_text(encoding='utf-8'))['mosaics']
        physical = importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.📐️geometry')
        source = ast.parse((directory / '🐍️.py').read_text(encoding='utf-8'))
        function = next(node for node in source.body if isinstance(node, ast.FunctionDef) and node.name == 'surface_color')
        draft = SimpleNamespace(GLASS='#abcdef',CONCRETE='#999999',INK='#ffffff',GOOD='#33bb55',BACKGROUND='#04161a',GREEN_GROUND='#228833',SERV='#4499aa',MUTED='#777777',mix=lambda a,b,t:a)
        scope = {'_d': draft, 'surface_role': physical.surface_role}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[function],type_ignores=[])),'actual-native-palette','exec'),scope)
        axon = ast.parse((directory.parent / '🧊️axon/🐍️.py').read_text(encoding='utf-8'))
        roles = next(node for node in axon.body if isinstance(node,ast.Assign) and any(isinstance(target,ast.Name) and target.id=='ROLES' for target in node.targets))
        scope['_c'] = SimpleNamespace(surface_color=scope['surface_color'])
        exec(compile(ast.fix_missing_locations(ast.Module(body=[roles],type_ignores=[])),'actual-axon-palette','exec'),scope)
        for case in cases:
            color = scope['surface_color']({'kind':'tile'},{'material':case['material']})
            self.assertEqual(color,case['color'])
            self.assertEqual(scope['ROLES'][case['role']],color)
            self.assertEqual(Image.new('RGB',(1,1),color).getpixel((0,0)),tuple(case['rgb']))


    def test_service_parts_preserve_authored_finish_palette(self):
        """🛁 Execute native service face dispatch while preserving explicit mosaic finishes."""
        import ast
        import numpy as np
        from types import SimpleNamespace
        from PIL import Image
        directory=Path(__file__).parent
        physical=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.📐️geometry')
        tree=ast.parse((directory/'🐍️.py').read_text(encoding='utf-8'))
        functions=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in ('surface_color','service_surface_color')]
        self.assertEqual(len(functions),2)
        scope={'_d':SimpleNamespace(SERV='#4499aa',GLASS='#abcdef',CONCRETE='#999999',MUTED='#777777',INK='#ffffff',GOOD='#33bb55',mix=lambda a,b,t:a),'surface_role':physical.surface_role,'service_surface_role':physical.service_surface_role}
        exec(compile(ast.fix_missing_locations(ast.Module(body=functions,type_ignores=[])),'actual-service-palette','exec'),scope)
        source=ast.parse((directory.parent/'🧊️axon/🧩️primitives.py').read_text(encoding='utf-8'))
        function=next(n for n in source.body if isinstance(n,ast.FunctionDef) and n.name=='_service_prims')
        scope.update(np=np,_c=SimpleNamespace(service_surface_role=physical.service_surface_role,_model=SimpleNamespace(part_outline_at=lambda p,z:p['outline'])),_part_faces=lambda p,r:r,_prism=lambda rings,z0,z1,r:r,Prim=lambda tag,info,stage,centre,faces,**kwargs:faces,STAGE_TOP=10,STAGE_UNDER=2,STAGE_FIRST=3)
        exec(compile(ast.fix_missing_locations(ast.Module(body=[function],type_ignores=[])),'actual-service-faces','exec'),scope)
        cases=json.loads((directory/'🧪️surfaces.json').read_text(encoding='utf-8'))['mosaics']+[{'material':'MAT-METAL','role':'service','color':'#4499aa','rgb':[68,153,170]}]
        for case in cases:
            for shape in ({},{'mesh':{}},{'top_outline':[[0,0],[1,0],[1,1],[0,1]]}):
                part={'material':case['material'],'outline':[[0,0],[1,0],[1,1],[0,1]],'z0':0.,'z1':.2,**shape}
                service={'tag':'NEUTRAL-FIXTURE','kind':'bath','parts':[part]}
                self.assertEqual(scope['_service_prims'](service,{},0,1),[case['role']])
                color=scope['service_surface_color'](service,part)
                self.assertEqual(color,case['color'])
                self.assertEqual(Image.new('RGB',(1,1),color).getpixel((0,0)),tuple(case['rgb']))
        for folder,methods in (('📋️plan',('_plan_services',)),('✂️section',('_sec_services','villa_elevation'))):
            tree=ast.parse((directory.parent/folder/'🐍️.py').read_text(encoding='utf-8'))
            for name in methods:
                function=next(n for n in ast.walk(tree) if isinstance(n,ast.FunctionDef) and n.name==name)
                self.assertTrue(any(isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr=='service_surface_color' for n in ast.walk(function)))


if __name__ == "__main__":
    unittest.main()


class FloorPlanSurfaceContract(unittest.TestCase):
    """🪵 Keep physical layer partitions out of the visible floor finish pattern."""

    def test_actual_plan_floor_visibility_order_and_no_internal_ink(self):
        """🖼️ Execute actual plan dispatch and compare filled adjacent cells with independent Pillow pixels."""
        import ast
        from types import SimpleNamespace
        from PIL import Image,ImageDraw
        directory=Path(__file__).parent;case=json.loads((directory/'🧪️surfaces.json').read_text())['floor_plan'];tree=ast.parse((directory.parent/'📋️plan/🐍️.py').read_text());names=('plan_part_rank','plan_part_visible','_plan_solids');nodes=[n for n in ast.walk(tree) if isinstance(n,ast.FunctionDef) and n.name in names]
        calls=[]
        class Group:
            def __init__(self):self.submobjects=[]
            def add(self,*items):self.submobjects.extend(items)
        def shape(kind,ring,*args,**kwargs):
            calls.append((kind,ring));return kind
        scope={'_engine':SimpleNamespace(VGroup=Group),'_d':SimpleNamespace(polygon=lambda r,*x,**k:shape('fill',r),polyline=lambda r,*x,**k:shape('stroke',r)),'_c':SimpleNamespace(surface_color=lambda d,p:'#B58B54'),'_s':SimpleNamespace(pseudo=lambda *x:None),'part_plan_outline':lambda p,z:p['outline'],'INK':'#000000','W_HAIR':.01,'W_THIN':.02}
        exec(compile(ast.fix_missing_locations(ast.Module(body=nodes,type_ignores=[])),'actual-floor-plan-controller','exec'),scope)
        data={**case,'tag':'NEUTRAL-FLOOR','guid':'neutral','domains':['architecture']};self.assertEqual(scope['plan_part_rank'](data),case['rank'])
        for row in case['parts']:self.assertEqual(scope['plan_part_visible'](data,row,'first',(3.17,6.6),False),row['visible'])
        data['parts']=[{**row,'outline':[[i,0],[i+1,0],[i+1,1],[i,1]]} for i in range(2) for row in case['parts']]
        ctx={'bands':{'first':(3.17,6.6)},'level':'first','g':{'details':[data],'storeys':{'first':3.17}},'use':{'architecture'},'catalogue':[{'tag':data['tag']}],'look':SimpleNamespace(state=lambda item:'base'),'below':False,'cut_z':4.3,'screen':lambda ring:ring,'put':lambda *args:None}
        scope['_plan_solids'](SimpleNamespace(_pair=lambda *args:('#B58B54','#000000',1,0)),ctx);self.assertEqual(sum(k=='stroke' for k,r in calls),case['internal_strokes']);self.assertEqual(sum(k=='fill' for k,r in calls),2)
        image=Image.new('RGB',(80,40),'white');draw=ImageDraw.Draw(image)
        for kind,ring in calls:
            if kind=='fill':draw.polygon([(round(x*39),round(y*39)) for x,y in ring],fill='#B58B54')
        self.assertEqual(image.getpixel((39,20)),(181,139,84));self.assertEqual(image.getpixel((38,20)),image.getpixel((40,20)))

    def test_actual_face_appearance_only_colours_source_bound_surface(self):
        """🖌️ Real wall primitives and the actual painter retain face-local inferred RGB with user-look precedence."""
        from PIL import Image,ImageColor,ImageDraw
        from types import SimpleNamespace
        prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
        model=importlib.import_module(prefix+'🧱️model.🎨️finishes.🧪️tests.🐍️')
        api=importlib.import_module(prefix+'🧱️model.🐍️')
        primitives=importlib.import_module(prefix+'🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives')
        axon=importlib.import_module(prefix+'🎨️visuals.🏛️savoye.🧊️axon.🐍️')
        m,wall=model.example();geometry=api._wall_geometry(m,wall)
        faces=[face for _,group in primitives._wall_solids(geometry,{}) for face in group]
        painted=[face for face in faces if isinstance(face[3],dict)]
        self.assertTrue(painted)
        backend=api.surface_finishes()
        self.assertAlmostEqual(sum(backend.face_area(face[1][0]) for face in painted),model.CASE['expected']['face_area'],places=12)
        self.assertTrue(all(backend.face_matches(face[1][0],next(p['surface_finish'] for p in geometry['parts'] if 'surface_finish' in p)) for face in painted))
        painter=axon._Painter(None,None,False)
        expected=tuple(round(value*255) for value in model.CASE['material']['surface_finish']['display_rgb']['value'])
        for face in painted:
            colour=painter.base(SimpleNamespace(base=None),face[3],'architecture')
            self.assertEqual(ImageColor.getrgb(colour),expected)
            self.assertEqual(painter.base(SimpleNamespace(base='#112233'),face[3],'architecture'),'#112233')
        self.assertEqual(painter.base(SimpleNamespace(base=None),"wall","architecture"),axon.ROLES["wall"])
        painter.discipline_mode=True
        self.assertEqual(painter.base(SimpleNamespace(base=None),painted[0][3],"architecture"),axon._d.DISCIPLINES["architecture"])
        painter.discipline_mode=False
        image=Image.new('RGB',(400,300),'white');draw=ImageDraw.Draw(image)
        for face in painted:draw.polygon([(round(p[0]*100),round((3-p[2])*100)) for p in face[1][0]],fill=painter.base(SimpleNamespace(base=None),face[3],'architecture'))
        self.assertEqual(image.getpixel((120,100)),expected)
        self.assertEqual(image.getpixel((200,200)),(255,255,255))
        self.assertEqual(image.getpixel((50,100)),(255,255,255))

