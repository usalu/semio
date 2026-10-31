"""🧪 Private native slab and wall-cutter entries bind transitive surface producers."""
import importlib,json
from pathlib import Path
from copy import deepcopy
P='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.'
a=importlib.import_module(P+'🐍️')


def _check_private_caches(verify):
    """🧊 Source-identical private entries must observe replaced exact surface helpers."""
    case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));m=a.load_model();slab=a._find(m,'SL-1');column=deepcopy(next(e for e in m['elements'] if e['class']=='IfcColumn'));m['elements']=[column,slab]
    targets={name:(a._analysis,'_orientation') if name=='_analysis._orientation' else (a,name) for name in (*case['helpers'],'_slab_geometry_uncached','column_geometry')}
    saved={name:getattr(owner,key) for name,(owner,key) in targets.items()}
    active=[]
    def exercise():
        """📐 Exercise one actual helper through its declared standard geometry interface."""
        name=active[0]
        if name=='_surface_difference':result=a._surface_difference([case['triangle']],[])
        elif name=='_surface_separated':result=a._surface_separated(case['triangle'],case['other_triangle'])
        elif name=='_surface_support_edges':result=a._surface_support_edges(tuple(map(tuple,case['triangle'])),a._analysis._orientation)
        else:result=a._analysis._orientation(*case['triangle'])
        verify(name,result,case)
        return result
    def slab_value(model,obj):
        """🧪 Resolve the currently selected transitive surface operation."""
        return {'marker':exercise()}
    def column_value(model,obj):
        """🧪 Resolve a valid native member through that same current operation."""
        exercise()
        return {'parts':[{'outline':case['triangle'],'z0':0.,'z1':1.}]}
    try:
        a._slab_geometry_uncached=slab_value;a.column_geometry=column_value
        for scope in case['scopes']:
            for helper in case['helpers']:
                active[:]=[helper];calls=[];original=saved[helper]
                def initial(*args,**kwargs):
                    """🧪 Count the first helper without altering its physical output."""
                    calls.append('initial');return original(*args,**kwargs)
                target,key=targets[helper];setattr(target,key,initial);a._SLAB_GEOMETRIES.clear();a._WALL_CUTTERS.clear()
                query=(lambda:a._slab_geometry_entry(m,slab)) if scope=='slab' else (lambda:a._wall_cutters({**m,'elements':[column]}))
                first=query();count=len(calls);assert count==case['expected']['initial_calls'],(scope,helper,'not exercised');assert query()==first and len(calls)==count
                def replacement(*args,**kwargs):
                    """🧪 A distinct helper identity must recompute its private consumer."""
                    calls.append('replacement');return original(*args,**kwargs)
                setattr(target,key,replacement);assert query()==first and 'replacement' in calls,(scope,helper,'stale private cache')
                setattr(target,key,original)
    finally:
        for name,value in saved.items():
            owner,key=targets[name];setattr(owner,key,value)
        a._SLAB_GEOMETRIES.clear();a._WALL_CUTTERS.clear()


def test_private_slab_and_wall_cutter_surface_replacement_invalidates_cached_values():
    """🧪 Independent GEOS/NumPy outputs remain stable while producer replacement invalidates both caches."""
    import numpy as np
    from shapely.geometry import Polygon,LineString
    def verify(name,result,case):
        """📐 Verify the same actual native helper result with independent geometry libraries."""
        polygon=Polygon(case['triangle'])
        if name=='_surface_difference':assert sum(Polygon(ring).area for ring in result)==polygon.area
        elif name=='_surface_separated':assert result==(polygon.intersection(Polygon(case['other_triangle'])).area==0)
        elif name=='_surface_support_edges':assert len(result)==len(case['triangle']) and all(polygon.boundary.covers(LineString([start,end])) for start,end,side in result)
        else:assert result==int(np.sign(np.linalg.det(np.asarray(case['triangle'][1:])-case['triangle'][0])))
    _check_private_caches(verify)
