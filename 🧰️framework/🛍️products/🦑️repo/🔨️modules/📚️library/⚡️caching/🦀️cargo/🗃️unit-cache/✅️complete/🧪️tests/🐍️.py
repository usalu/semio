#!/usr/bin/env python3
import json,sys
D={'darwin':'dylib','win32':'dll'}
def expected(platform,args):
    if '-o' in args or any(a.startswith('-o') and not a.startswith('--') and len(a)>2 for a in args):return []
    def opt(flag):
        out=[]
        for i,a in enumerate(args):
            if a==flag and i+1<len(args):out.append(args[i+1])
            elif a.startswith(flag+'='):out.append(a[len(flag)+1:])
        return out
    crate=(opt('--crate-name') or [''])[0]
    extra=''
    for i,a in enumerate(args):
        v=args[i+1] if a=='-C' and i+1<len(args) else (a[2:] if a.startswith('-C') and len(a)>2 else None)
        if v and v.startswith('extra-filename='):extra=v[len('extra-filename='):]
    emit={p.split('=')[0] for v in opt('--emit') for p in v.split(',')}
    types=[t for v in opt('--crate-type') for t in v.split(',')] or ['bin']
    host=not opt('--target')
    out=[]
    if 'dep-info' in emit:out.append(f'{crate}{extra}.d')
    if 'metadata' in emit:out.append(f'lib{crate}{extra}.rmeta')
    if 'link' in emit:
        if 'lib' in types or 'rlib' in types:out.append(f'lib{crate}{extra}.rlib')
        if 'proc-macro' in types:out.append(('' if platform=='win32' else 'lib')+f'{crate}{extra}.'+D.get(platform,'so'))
        if host and 'bin' in types:out.append(f'{crate}{extra}'+('.exe' if platform=='win32' else ''))
    return list(dict.fromkeys(out))
rows=json.load(sys.stdin)
print(json.dumps([expected(r['platform'],r['arguments']) for r in rows]))
