import copy
import math

import networkx as nx
import time
import random
import Structures

#import pptester

start = 0



def getMinimalNoHomSubgraph(G: nx.DiGraph, H, f=None, ACWorks=False):
    if isinstance(G, nx.DiGraph):
        C = nx.weakly_connected_components(G)
    else:
        C = [G]
    for Svs in C:
        if isinstance(G, nx.DiGraph):
            S = G.subgraph(Svs)
        else:
            S = G

        if existsHom(S, {v: f[v] for v in f if v in S.nodes}, H, ACWorks, componentwise=False):
            continue
        vs = list(S.nodes)
        g = {v: f[v] for v in f if v in vs}
        for v in vs:
            print('nodes', len(S.nodes))
            S2 = S.copy()
            S2.remove_node(v)
            if not existsHom(S2, {v: g[v] for v in g if v in S2.nodes}, H, ACWorks, componentwise=False):
                S = S2
                g = {v: g[v] for v in g if v in S.nodes}
                continue
            # S3 = getMinimalNoHomSubgraph(S2,H,ACWorks)
            # if S3 is not None:
        return S
    return None


# use:
# H,q=Identities.getIndicatorGraph(G,Identities.Siggers,True)
# K=ArcConFast.getMinimalNoHomSubgraphFast(H,G,{q[('f',v,v,v,v)]:{v} for v in G.nodes})

def getMinimalNoHomSubgraphFast(G: nx.DiGraph, H, f=None, ACWorks=False, dropRatio=0.3):
    if f is None:
        f=dict()
    if isinstance(G,nx.DiGraph):
        C = nx.weakly_connected_components(G)
    else:
        C=[G]
    for Svs in C:
        if isinstance(G,nx.DiGraph):
            S = G.subgraph(Svs)
        else:
            S=G

        if existsHom(S, {v: f[v] for v in f if v in S.nodes}, H, ACWorks, componentwise=False):
            continue
        g = {v: f[v] for v in f if v in S.nodes}
        while dropRatio*len(S.nodes) > 5 and len(S.nodes) > 50:
            rvs = random.sample(S.nodes, math.floor(len(S.nodes) * dropRatio) + 1)

            print('nodes', len(S.nodes), dropRatio)
            S2 = S.copy()
            S2.remove_nodes_from(rvs)
            try:
                if not existsHom(S2, {v: g[v] for v in g if v in S2.nodes}, H, ACWorks, componentwise=True,timelimit=2):
                    S = S2
                    g = {v: g[v] for v in g if v in S.nodes}
                    dropRatio = min(dropRatio * 2, 0.60)
                else:
                    dropRatio = max(0, dropRatio * 0.95)
            except:
                dropRatio = max(0, dropRatio * 0.95)
                print('timeout')
        return getMinimalNoHomSubgraph(S, H, g, ACWorks)
    return None



def existsHom(G, f, H, ACWorks=False, componentwise=True, timelimit=float('inf'),workingSet=None):
    if not isinstance(G, nx.DiGraph):
        componentwise = False
    if componentwise:
        C = nx.weakly_connected_components(G)
        for Svs in C:
            ff = dict()
            S = G.subgraph(Svs)
            for v in S.nodes:
                if f is not None and v in f:
                    ff[v] = f[v]

            if not existsHom(S, ff, H, ACWorks, False, timelimit,workingSet):
                return False
        return True
    else:
        if ACWorks:
            f = initF(G, H, f)
            f = arcCon(G, f, H,workingSet=workingSet)[0]
            if set() not in [f[k] for k in f.keys()]:
                return True
            return False
        return findHom(G, H, f, ACWorks, timelimit) is not None


def reduceHomComponentsACWorks(G: nx.DiGraph):
    C = nx.weakly_connected_components(G)
    print('comps:', len(list(C)))
    for Svs in C:
        S = G.subgraph(Svs)
        G2 = G.subgraph(set(G.nodes).difference(Svs))
        print(len(S.nodes), len(G2.nodes), len(G.nodes))
        if existsHom(S, None, G2, ACWorks=False):  # G2 might not have TSn for all n
            return reduceHomComponentsACWorks(G2)
    return G


def getGinGout(G, f, H):
    gin = dict()
    gout = dict()
    kold = None
    koldold = None
    for k in f.keys():
        inN = set()
        out = set()
        if kold and f[k] == f[kold]:
            gin[k] = copy.deepcopy(gin[kold])
            gout[k] = copy.deepcopy(gout[kold])
        elif koldold and f[k] == f[koldold]:
            gin[k] = copy.deepcopy(gin[koldold])
            gout[k] = copy.deepcopy(gout[koldold])
        elif G == H and len(f[k]) == len(G.nodes):
            gin[k] = {v for v in G.nodes if G.out_degree[v] > 0}
            gout[k] = {v for v in G.nodes if G.in_degree[v] > 0}
        else:
            for w in f[k]:
                inN = inN.union(set(H.predecessors(w)))
                out = out.union(set(H.neighbors(w)))
            gin[k] = inN
            gout[k] = out
            kold, koldold = k, kold
    return gin, gout


def isHomEqACworks(G, H, h1=None, h2=None):
    if not existsHom(G, None, H, ACWorks=True):
        return False
    if not existsHom(H, None, G, ACWorks=True):
        return False
    return True


def isHomEq(G, H, h1=None, h2=None, timelimit=float('inf')):
    global start
    h1 = initF(H, G, h1)
    start = time.time()
    h1 = canExtend(H, h1, G, timelimit=timelimit)
    if h1 is not None:
        h2 = initF(G, H, h2)
        start = time.time()
        h2 = canExtend(G, h2, H, timelimit=timelimit)
        if h2 is not None:
            return True
    return False


def singletonArcCon(G, H):
    f = initF(G, H, None)
    changed = True
    while changed:
        f = arcCon(G, f, H)[0]
        changed = False
        print('loop')
        for v in f:
            for u in f[v]:
                ff = copy.deepcopy(f)
                ff[v] = {u}
                ff = arcCon(G, ff, H)[0]
                if ff is None:
                    f[v].remove(u)
                    print(u)
                    changed = True
                    break
    return f


# timeAC(Identities.getIndicatorGraph(pptester.T6,Identities.Sigma5),pptester.T6)
# def timeAC(G=Identities.getIndicatorGraph(pptester.T5, Identities.Sigma5), H=pptester.T5):
def timeAC(G, H):
    f = initF(G, H)
    start = time.time()
    g1 = arcCon(G, f, H)[0]
    print('arcCon time:', time.time() - start)

    f = initF(G, H)
    start = time.time()
    g2 = arcConFast(G, f, H)[0]
    print('arcConFast time:', time.time() - start)
    print([(v, g1[v], g2[v]) for v in g1.keys() if not (g1[v].issubset(g2[v]) and g2[v].issubset(g1[v]))])

    f = initF(G, H)
    start = time.time()
    g1 = arcConSlow(G, f, H)[0]
    print('arcConSlow time:', time.time() - start)
    return g1, g2


# saves about 90%
def arcConFast(G: nx.DiGraph, f, H: nx.DiGraph, gin=None, gout=None):
    workingSet = set(f.keys())
    while len(workingSet) > 0:
        # print(workingSet,f)
        v = workingSet.pop()
        PossibleOutVertices = set()
        PossibleInVertices = set()

        # todo do this using dynamic programming
        for h in f[v]:
            PossibleOutVertices.update(H.successors(h))
            PossibleInVertices.update(H.predecessors(h))
        for uOut in G.successors(v):
            if not PossibleOutVertices.issuperset(f[uOut]):
                workingSet.add(uOut)
                f[uOut] = f[uOut].intersection(PossibleOutVertices)
        for uIn in G.predecessors(v):
            if not PossibleInVertices.issuperset(f[uIn]):
                workingSet.add(uIn)
                f[uIn] = f[uIn].intersection(PossibleInVertices)
    return f, None, None


# last argument is only for downwards compatibility
def arcConOld2(G: nx.DiGraph, f, H: nx.DiGraph, dyn=None, deprecated=None,workingSet=None):
    if dyn is None:
        dyn = dict()
    if workingSet is None:
        workingSet = set(G.nodes)

    while len(workingSet) > 0:
        # print(len(dyn.keys()))
        # print(workingSet,f)
        v = workingSet.pop()

        # some dynamic programming saves about 10 percent
        if v not in f:
            f[v] = set(H.nodes)
        if frozenset(f[v]) in dyn:
            PossibleOutVertices, PossibleInVertices = dyn[frozenset(f[v])]
        else:
            PossibleOutVertices = set()
            PossibleInVertices = set()

            for h in f[v]:
                PossibleOutVertices.update(H.successors(h))
                PossibleInVertices.update(H.predecessors(h))
            dyn[frozenset(f[v])] = (PossibleOutVertices, PossibleInVertices)
        for uOut in G.successors(v):
            if uOut not in f or not PossibleOutVertices.issuperset(f[uOut]):
                workingSet.add(uOut)
                if uOut in f:
                    f[uOut] = f[uOut].intersection(PossibleOutVertices)
                else:
                    f[uOut] = PossibleOutVertices
                if len(f[uOut]) == 0:
                    return {v: set() for v in G.nodes}, dyn, None
        for uIn in G.predecessors(v):
            if uIn not in f or not PossibleInVertices.issuperset(f[uIn]):
                workingSet.add(uIn)
                if uIn in f:
                    f[uIn] = f[uIn].intersection(PossibleInVertices)
                else:
                    f[uIn] = PossibleInVertices
                if len(f[uIn]) == 0:
                    return {v: set() for v in G.nodes}, dyn, None
    return f, dyn, None

def arcCon(G: nx.DiGraph, f, H: nx.DiGraph, dyn=None, deprecated=None,workingSet=None):
    if f is None:
        f = dict()

    if not isinstance(G,nx.DiGraph):
        if len(G.Gs) != len(H.Gs):
            return {v: set() for v in G.nodes}, dyn, None
        while True:
            g = copy.deepcopy(f)
            for i in range(len(G.Gs)):
                g, _, _ = arcCon(G.Gs[i], g, H.Gs[i])
            #print(g)
            if g == f:
                return f, None, None
            else:
                f=g

    else:
        if dyn is None:
            dyn = dict()
        if workingSet is None:
            workingSet = set(G.nodes)
            highPworkingSet = set(f.keys())
        else:
            highPworkingSet = workingSet

        while len(workingSet) > 0 or len(highPworkingSet) > 0:
            # print(len(dyn.keys()))
            # print(workingSet,f)
            if len(highPworkingSet)>0:
                v=highPworkingSet.pop()
                if v in workingSet:
                    workingSet.remove(v)
            else:
                v = workingSet.pop()
            # some dynamic programming saves about 10 percent
            if v not in f:
                f[v] = set(H.nodes)
            if frozenset(f[v]) in dyn:
                PossibleOutVertices, PossibleInVertices = dyn[frozenset(f[v])]
            else:
                PossibleOutVertices = set()
                PossibleInVertices = set()

                for h in f[v]:
                    PossibleOutVertices.update(H.successors(h))
                    PossibleInVertices.update(H.predecessors(h))
                dyn[frozenset(f[v])] = (PossibleOutVertices, PossibleInVertices)
            for uOut in G.successors(v):
                if uOut not in f or not PossibleOutVertices.issuperset(f[uOut]):
                    highPworkingSet.add(uOut)
                    if uOut in f:
                        f[uOut] = f[uOut].intersection(PossibleOutVertices)
                    else:
                        f[uOut] = PossibleOutVertices
                    if len(f[uOut]) == 0:
                        return {v: set() for v in G.nodes}, dyn, None
            for uIn in G.predecessors(v):
                if uIn not in f or not PossibleInVertices.issuperset(f[uIn]):
                    highPworkingSet.add(uIn)
                    if uIn in f:
                        f[uIn] = f[uIn].intersection(PossibleInVertices)
                    else:
                        f[uIn] = PossibleInVertices
                    if len(f[uIn]) == 0:
                        return {v: set() for v in G.nodes}, dyn, None
        return f, dyn, None

#@arcCon.register
#def arcCon(G: Structures.Structure, f, H:Structures.Structure , dyn=None, deprecated=None,workingSet=None):



def arcConSlow(G, f, H, gin=None, gout=None):
    if gin is None:
        gin, gout = getGinGout(G, f, H)
    done = False
    i = 0
    while not done:
        (f, done, gin, gout) = arcConStep(G, f, H, gin, gout)
        i += 1

    return f, gin, gout


def arcConStep(G, f, H, gin, gout):
    done = True
    for k in f.keys():

        # out neig
        for outN in G.neighbors(k):
            if outN in f.keys() and not f[k].issubset(gin[outN]):
                f[k] = f[k].intersection(gin[outN])
                done = False
                inN = set()
                out = set()
                for w in f[k]:
                    inN = inN.union(set(H.predecessors(w)))
                    out = out.union(set(H.neighbors(w)))
                gin[k] = inN
                gout[k] = out

        # in neig
        for inNN in G.predecessors(k):
            if inNN in f.keys() and not f[k].issubset(gout[inNN]):
                f[k] = f[k].intersection(gout[inNN])
                done = False
                inN = set()
                out = set()
                for w in f[k]:
                    inN = inN.union(set(H.predecessors(w)))
                    out = out.union(set(H.neighbors(w)))
                gin[k] = inN
                gout[k] = out

    return f, done, gin, gout


def initF(G, H, f=None,initLoops = True):

    if not f: f = dict()

    if not isinstance(G,nx.DiGraph) and initLoops and len(G.Gs)==len(H.Gs):
        for i,GE in enumerate(G.Gs):
            Hloops = {u for u in H.Gs[i].nodes if H.Gs[i].has_edge(u,u)}
            for v in GE.nodes:
                if GE.has_edge(v,v):
                    if v in f:
                        f[v] = f[v].intersection(Hloops)
                    else:
                        f[v] = Hloops.copy()
    #for v in G.nodes:
    #    if v not in f.keys():
    #        f[v] = copy.deepcopy(set(H.nodes))
    return f


def findHom(G, H, f=None, ACworks=False, timelimit=float("inf")):
    # init f
    if ACworks:
        print('ACWorks implemenation for finding hom is buggy, I set ACWorks to False :)')
        ACworks = False

    global start
    start = time.time()
    if not f: #f = dict()
        f = initF(G, H, f)
    res = canExtend(G, f, H, ACworks, timelimit=timelimit)
    #print('G nodes',len(G.nodes),'time all',time.time()-start)
    return res

def canExtendslow(G, f, H, ACworks=False, im=None, gin=None, gout=None, timelimit=float("inf"), depth=0, debug=False):
    global start
    # print(time.time() - start)
    if (time.time() - start) > timelimit:
        raise Exception('timeout')

    if im is None:
        im = set()
    f, gin, gout = arcCon(G, f, H, gin, gout)
    #    print([(set(k),f[k]) for k in f.keys() if len(f[k]) != 1])
    # find the first vertex thats not mapped to a singleton and try all values
    ks = list(f.keys())
    ks.sort(key=lambda k: len(f[k]))  # backtrack on small lists first
    for k in ks:
        if len(f[k]) == 0:
            if debug:
                print('depth: ' + str(depth))
            return None
        if len(f[k]) > 1:
            for u in [v for v in f[k] if v in im] + [v for v in f[k] if v not in im]:
                if ACworks:
                    ff = f
                    ggin = gin
                    ggout = gout
                else:
                    ff = copy.deepcopy(f)
                    ggin = copy.deepcopy(gin)
                    ggout = copy.deepcopy(gout)

                ff[k] = {u}
                # ggin[k] = set(H.predecessors(u))
                # ggout[k] = set(H.neighbors(u))

                g = canExtend(G, ff, H, ACworks, im.union({u}), ggin, ggout, timelimit, depth + 1, debug)
                if g is not None:
                    return g
                # else:
                #    print(H,"counterexample",f) #only f there is one
            #
            return None
    # all sets are singletons and f is already a function
    ff = dict()
    for k in f.keys():
        ff[k] = list(f[k])[0]
    return ff


def getAllHom(G,H,componentwise=True):
    if not isinstance(G, nx.DiGraph):
        componentwise = False
    if componentwise:
        C = nx.weakly_connected_components(G)
        homs = [dict()]
        for Svs in C:
            S = G.subgraph(Svs)
            newhoms = getAllHom(S,H,componentwise=False)
            #homs += newhoms TODO this is viable in practice
            homs = [dict(list(f.items())+list(g.items())) for f in homs for g in newhoms]
        return homs
    else:
        # G should be connected
        f0 = arcCon(G,dict(),H)[0]
        homs= []
        workingset = [f0]
        while len(workingset)>0:
            f=workingset[0]
            workingset=workingset[1:]
            ks = list(f.keys())
            ks.sort(key=lambda k: len(f[k]))  # backtrack on small lists first
            print('workingset',len(workingset),'lists in f',len([k for k in ks if len(f[k])>1]),'homs found',len(homs))
            for k in ks:
                if len(f[k])==0:
                    break
                if len(f[k]) > 1:
                    for v in f[k]:
                        ff = copy.deepcopy(f)
                        ff[k]={v}
                        ff = arcCon(G,ff,H)[0]
                        workingset=[ff]+workingset
                    break
            if len(f[ks[-1]])==1:
                homs+=[f]
        return homs



def canExtend(G, f, H, ACworks=False, im=None, gin=None, gout=None, timelimit=float("inf"), depth=0, debug=False,maxBreitensuche=2):
    maxdepth = 1
    f, gin, gout = arcCon(G, f, H, gin, gout)
    #try greedy search (one branch oft depth first search) first
    g = copy.deepcopy(f)
    ggin = copy.deepcopy(gin)
    ggout = copy.deepcopy(gout)
    if debug:
        print('greedy search')
    try:
        g = canExtendFast(G, g, H,ACworks=True,gin=ggin,gout=ggout, timelimit=timelimit*1/3, depth=depth + 1, debug=debug, maxdepth=1000)
        if g is not None:
            return g
    except:
        print('timeout')
    try:
        if debug:
            print('breitensuche')
        while True:
            g = copy.deepcopy(f)
            ggin = copy.deepcopy(gin)
            ggout = copy.deepcopy(gout)
            g = canExtendFast(G, g, H, ACworks, gin=ggin,gout=ggout,timelimit=timelimit*1/2 if timelimit < float('inf') else 3, depth=depth + 1, debug=debug, maxdepth=maxdepth)
            if g != 'tooDeep':
                return g
            if maxdepth > maxBreitensuche:
                print('tooDeep',maxdepth)
            maxdepth += 1
    except:
        try:
            if debug:
                print('proper greedy search')
            g = copy.deepcopy(f)
            ggin = copy.deepcopy(gin)
            ggout = copy.deepcopy(gout)
            g = canExtendGreedy(G, g, H, ACworks=True,gin=ggin,gout=ggout, timelimit=timelimit*2/3, depth=depth + 1, debug=debug,
                                maxdepth=len(G.nodes))
            if g is not None:
                return g
        except:
            if debug:
                print('timeout greedy')

        if debug:
            print('exhaustive search, previous maxdepth',maxdepth)

        g = copy.deepcopy(f)
        ggin = copy.deepcopy(gin)
        ggout = copy.deepcopy(gout)
        return canExtendFast(G, g, H, ACworks, gin=ggin,gout=ggout,timelimit=timelimit, depth=depth + 1, debug=debug, maxdepth=len(G.nodes))



def canExtendFast(G, f, H, ACworks=False, im=None, gin=None, gout=None, timelimit=float("inf"), depth=0, debug=False,
                  maxdepth=0,workingSet=None):
    global start
    #print('depth',depth,'f',f)#,'G',list(G.edges),'H',list(H.edges))
    timePassed= time.time() - start
    digit = (timePassed*100)%100
    if timePassed > 5 and digit==0:
        print('canExtendFast time:',timePassed,ACworks,timelimit,'depth:',depth,'max',maxdepth,'limit',timelimit)
    if timePassed > timelimit:
        raise Exception('timeout')


    if im is None:
        im = set()
    timeAC = time.time()
    f, gin, gout = arcCon(G, f, H, gin, gout,workingSet=workingSet)
    #print('depth',depth,'actime',time.time()-timeAC)
    #    print([(set(k),f[k]) for k in f.keys() if len(f[k]) != 1])
    # find the first vertex thats not mapped to a singleton and try all values
    ks = list(f.keys())
    ks.sort(key=lambda k: len(f[k]))  # backtrack on small lists first

    if len(ks)>0 and len(f[ks[0]]) == 0:
        return None

    if depth > maxdepth:
        if debug:
            print('tooDeep', im)
        return 'tooDeep'
    if debug:
        print('depth:', depth, 'length keys with not singleton lists', len([k for k in ks if len(f[k]) > 1]))

    for i, k in enumerate(ks):
        #if len(f[k]) == 0:
            # if debug:
            #    print('depth: ' + str(depth))
        #    return None
        if len(f[k]) > 1:
            if debug:
                print('depth', depth, 'index', i, 'len(f[k])im', len(f[k]), k)
            tooDeep = False
            for u in [v for v in f[k] if v in im] + [v for v in f[k] if v not in im]:
                if ACworks:
                    ff = f
                    ggin = gin
                    ggout = gout
                else:
                    ff = copy.deepcopy(f)
                    ggin = copy.deepcopy(gin)
                    ggout = copy.deepcopy(gout)

                ff[k] = {u}
                # ggin[k] = set(H.predecessors(u))
                # ggout[k] = set(H.neighbors(u))

                g = canExtendFast(G, ff, H, ACworks, im.union({u}), ggin, ggout, timelimit, depth + 1, debug, maxdepth,{k})
                if g == 'tooDeep':
                    if debug:
                        print('tooDeep', k, u, 'depth:', depth)
                    tooDeep = True
                    continue
                if g is not None:
                    return g
            #
            if tooDeep:
                continue
            return None
    # all sets are singletons and f is already a function
    ff = dict()
    for k in f.keys():
        if len(f[k])>1:
            return 'tooDeep'

        ff[k] = list(f[k])[0]
    return ff


def canExtendGreedy(G, f, H, ACworks=False, im=None, gin=None, gout=None, timelimit=float("inf"), depth=0, debug=False,
                  maxdepth=0,workingSet=None):
    global start


    if im is None:
        im = set()
    timeAC = time.time()
    f, gin, gout = arcCon(G, f, H, gin, gout,workingSet=workingSet)
    #print('depth',depth,'actime',time.time()-timeAC)
    #    print([(set(k),f[k]) for k in f.keys() if len(f[k]) != 1])
    # find the first vertex thats not mapped to a singleton and try all values
    ks = list(f.keys())
    ks.sort(key=lambda k: len(f[k]))  # backtrack on small lists first

    while len(f[ks[-1]])>1:
        depth+=1
        ks = list(f.keys())
        ks.sort(key=lambda k: len(f[k]))  # backtrack on small lists first

        if len(f[ks[0]]) == 0:
            return None


        for i, k in enumerate(ks):

            #if len(f[k]) == 0:
                # if debug:
                #    print('depth: ' + str(depth))
            #    return None
            if len(f[k]) > 1:

                if time.time() - start > 5:

                    print('canExtendGreedy steps', depth, 'singletons', i, 'len(f[k])', len(f[k]),'time:', time.time() - start, ACworks, timelimit)
                if (time.time() - start) > timelimit:
                    raise Exception('timeout')
                foundU=False
                for u in [v for v in f[k] if v in im] + [v for v in f[k] if v not in im]:

                    ff = copy.deepcopy(f)
                    ggin = copy.deepcopy(gin)
                    ggout = copy.deepcopy(gout)

                    ff[k] = {u}

                    ff,ggin,ggout=arcCon(G,ff,H,ggin,ggout,workingSet={k})

                    if len(ff[k])>0:
                        foundU=True
                        im.add(u)
                        f = ff
                        gin=ggin
                        gout=ggout
                        break
                if not foundU:
                    return None
                break
                #
    # all sets are singletons and f is already a function
    ff = dict()
    for k in f.keys():
        if len(f[k])>1:
            return None
        ff[k] = list(f[k])[0]
    return ff



def canExtendGreedyOld(G, H, f=None, im=None, debug=True):
    if im is None:
        im = set()

    f = initF(G, H, f)
    f, gin, gout = arcCon(G, f, H)
    i = 0
    for k in f.keys():
        i += 1
        if len(f[k]) == 0:
            return None
        if len(f[k]) > 1:
            if debug:
                print('nodes completed', i, 'list length', len(f[k]))
            for u in [v for v in f[k] if v in im] + [v for v in f[k] if v not in im]:
                if debug:
                    print(u)
                ff = copy.deepcopy(f)
                ggin = copy.deepcopy(gin)
                ggout = copy.deepcopy(gout)

                ff[k] = {u}
                #ggin[k] = set(H.predecessors(u))
                #ggout[k] = set(H.neighbors(u))

                ff, ggin, ggout = arcCon(G, ff, H, ggin, ggout)
                if 0 not in [len(ff[v]) for v in ff]:
                    f = ff
                    gin = ggin
                    gout = ggout
                    break
    # all sets are singletons
    ff = dict()
    for k in f.keys():
        if len(f[k]) > 1:
            return None
        ff[k] = list(f[k])[0]
    return ff


def canExtendACWorks(G, H, f=None, debug=False):
    f = initF(G, H, f)
    f, gin, gout = arcCon(G, f, H)
    i = 0
    for k in f.keys():
        i += 1
        if len(f[k]) == 0:
            return None
        if len(f[k]) > 1:
            if debug:
                print('nodes completed', i, 'list length', len(f[k]))
            u = list(f[k])[0]
            if debug:
                print(u)

            f[k] = {u}
            gin[k] = set(H.predecessors(u))
            gout[k] = set(H.neighbors(u))

            f, gin, gout = arcCon(G, f, H, gin, gout)
    return f


def findHomTestSigletonNoBacktracking(G, H, f=None):
    # init f
    if not f: f = dict()
    f = initF(G, H, f)

    return canExtendTestSigletonNoBacktracking(G, f, H)


def canExtendTestSigletonNoBacktracking(G, f, H, choices=None):
    if not choices: choices = []
    f = arcCon(G, f, H)[0]
    #    print([(set(k),f[k]) for k in f.keys() if len(f[k]) != 1])
    # find the first vertex thats not mapped to a singleton and try all values
    for k in f.keys():
        if len(f[k]) == 0:
            return None
        if len(f[k]) > 1:
            found = None
            for u in f[k]:
                ff = copy.deepcopy(f)
                ff[k] = {u}

                g = canExtendTestSigletonNoBacktracking(G, ff, H, choices + [(set(k), u)])
                if g != None:
                    # return g
                    found = True
                else:
                    print(H, choices + [(set(k), u)])  # only f there is one
            #
            return found
    # all sets are singletons and f is already a function
    ff = dict()
    for k in f.keys():
        ff[k] = list(f[k])[0]
    return ff


# works for trees
def isTreeCore(t, f=None,workingSet = None):
    t = t.copy()

    if not f: f = dict()
    #f = initF(t, t, f)
    f = arcCon(t, f, t,workingSet=workingSet)[0]

    for v in f.keys():
        if len(f[v]) > 1:
            # print(v, f[v])
            return False
    return True


# how to use
# Gs=[Identities.getIndicatorGraph(pptester.T6,Identities.getHM(i)) for i in range(3,25)]
# tab = ArcConFast.timeACTable(Gs,[pptester.T6])
def timeACTable(Gs, Hs):
    res = []
    for G in Gs:
        for H in Hs:
            start = time.time()
            f = initF(G, H)
            c = arcCon(G, f, H)
            res += [(len(G.nodes), len(H.nodes), int((time.time() - start) * 1000) / 1000)]

    return res
