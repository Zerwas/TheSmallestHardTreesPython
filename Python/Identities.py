import copy
import time

import networkx as nx
import ArcConFast as ArcCon
import itertools

import Structures
#import pptester
import random


class Identity:
    def __init__(self, Ids, Name=None):
        self.Ids = Ids
        self.functions = set()
        self.arity = dict()

        for Id in Ids:
            for term in Id:
                self.functions.add(term[0])
                self.arity[term[0]] = len(term) - 1

        self.Name = str(self.Ids)
        if Name != None:
            self.Name = Name

    def __str__(self):
        return self.Name

    def __repr__(self):
        return self.__str__()

    def __eq__(self, other):
        return self.Name == other.Name

#    def __lt__(self,other):
#        return self.__str__() < other.__str__()

    def __hash__(self):
        return self.Name.__hash__()

    # important convention: q(f,u,u,u,u) = u = q(g,u,u,u)
    def getQuotientMap(self, G):
        q = dict()

    def match(self, G, v, term):
        if v[0] != term[0]:
            return None
        assignment = dict()
        for i in range(1, len(term)):
            if term[i] not in assignment:
                # print(i,term,v,assignment)
                assignment[term[i]] = v[i]
            elif assignment[term[i]] != v[i]:
                return None
        return assignment

    def getInstances(self, G, term, f):
        insts = self.getInstancesInternal(G, term[1:], f)
        return {(term[0],) + u for u in insts}

    def getInstancesInternal(self, G, term, f):
        u = []
        for i in range(len(term)):
            if term[i] in f:
                u += [f[term[i]]]
            else:
                res = set()
                for v in G.nodes:
                    ff = f.copy()
                    ff[term[i]] = v
                    res = res.union({tuple(u) + (v,) + w for w in self.getInstancesInternal(G, term[i + 1:], ff)})
                return res
        return {tuple(u)}

    def getEqClass(self, G, v):
        eqclassWorking = {v}
        eqclass = set()
        while len(eqclassWorking) > 0:
            u = eqclassWorking.pop()
            eqclass.add(u)
            # print(self.Ids,eqclassWorking)
            for Id in self.Ids:
                for term in Id:
                    f = self.match(G, u, term)
                    # print(Id,term,f)
                    if f:
                        # add new nodes to working set
                        for term2 in Id:
                            for w in self.getInstances(G, term2, f):
                                if w not in eqclass:
                                    eqclassWorking.add(w)
                # todo optimze this
        return frozenset(eqclass)



alph=[chr(i) for i in range(10000)]

def getIndicatorGraph(G, Id, returnq=False):
    if not isinstance(G, nx.DiGraph):
        IGs = []
        qs = dict()
        for i in range(len(G.Gs)):
            IG,q = getIndicatorGraph(G.Gs[i],Id,True)
            IGs += [IG]
            qs.update(q)
        if returnq:
            return Structures.Structure(IGs),qs
        else:
            return Structures.Structure(IGs)
    else:
        IG = nx.DiGraph()
        # q= Id.getQuotientMap(G)
        q = dict()
        for f in Id.functions:
            productEdges = itertools.product(G.edges, repeat=Id.arity[f])
            for es in productEdges:
                u = tuple([f] + [e[0] for e in es])
                v = tuple([f] + [e[1] for e in es])
                #            if u not in q:
                #                c = Id.getEqClass(G,u)
                #                if #todo fwww in q[u]:
                #                    q[u]= w
                #                else:
                #                    q[u] = c
                if u not in q:
                    q[u] = Id.getEqClass(G, u)
                    for w in q[u]:
                        q[w] = q[u]
                if v not in q:
                    q[v] = Id.getEqClass(G, v)
                    for w in q[v]:
                        q[w] = q[v]
                IG.add_edge(q[u], q[v])
        if returnq:
            return IG, q
        return IG

#the partition should be a congruence
def getIndicatorGraphRespPartition(G, Id, partition=dict(), returnq=False):
    IG = nx.DiGraph()
    #compute edge partition
    edges = dict()
    for p in partition:
        for q in partition:
            edges[(p,q)]={e for e in G.edges if e[0] in partition[p] and e[1] in partition[q]}
    # q= Id.getQuotientMap(G)
    q = dict()

    for f in Id.functions:
        productEdges = []
        for p in edges:
            productEdges += list(itertools.product(edges[p], repeat=Id.arity[f]))
        for es in productEdges:
            u = tuple([f] + [e[0] for e in es])
            v = tuple([f] + [e[1] for e in es])
            #            if u not in q:
            #                c = Id.getEqClass(G,u)
            #                if #todo fwww in q[u]:
            #                    q[u]= w
            #                else:
            #                    q[u] = c
            if u not in q:
                q[u] = Id.getEqClass(G, u)
                for w in q[u]:
                    q[w] = q[u]
            if v not in q:
                q[v] = Id.getEqClass(G, v)
                for w in q[v]:
                    q[w] = q[v]
            IG.add_edge(q[u], q[v])
    if returnq:
        return IG, q
    return IG



def satisfysIdentity(G, Id, ACWorks=False, Idempotent=True,timelimit = float('inf'),partition=None,conservative=False):#,diagonal=False):
    #timeInd = time.time()
    #print('cons',conservative)
    if partition is None:
        IG, q = getIndicatorGraph(G, Id, returnq=True)
    else:
        IG, q = getIndicatorGraphRespPartition(G, Id, partition, returnq=True)
    #print('indi time', time.time() - timeInd)
    #timeAC = time.time()
    #g=None
    #arity=0
    fMap = None
    if Idempotent:
        fMap = dict()
        for f in Id.functions:
            a = Id.arity[f]
    #        arity=a
    #        g=f
            fMap.update({q[tuple([f] + [u] * a)]: {u} for u in G.nodes})
    if conservative:
        fMap=dict()
        for v in IG.nodes:
            sets = [set(u[1:]) for u in v]
            res = sets[0]
            for s in sets:
                res.intersection_update(s)
            fMap[v]=res
            #print(v,res)

    #if diagonal:
    #    u=list(G.nodes)[0]
    #    S=nx.node_connected_component(IG.to_undirected(),q[tuple([g]+[u]*arity)])
    #    IG=IG.subgraph(S)
    return ArcCon.existsHom(IG, fMap, G, ACWorks,timelimit=timelimit)
    print('AC',time.time()-timeAC)
    print('gesamt',time.time()-timeInd)
    return ret


Sigma2 = Identity([['fxy', 'fyx']], 'S2')
Sigma3 = Identity([['fxyz', 'fzxy']], 'S3')
Sigma5 = Identity([['fabcde', 'fbcdea']], 'S5')

TS3 = Identity([['fxyz', 'fzxy','fyxz'],['fxxy','fxyy']], 'TS3')
TS5 = Identity([['fabcde', 'fbcdea','fbacde'],['faacde','faccde']], 'TS5')

FS3 = Identity([['fxyz', 'fzxy', 'fyxz']], 'FS3')

Minxvxz = Identity([['tabcxyz','tbcaxyz','tbacxyz','tabcxzy'],['taaaxyz','tabbxyz'], ['tabcxyy', 'tabcxxx'], ['tabcxxy', 'tabcyyx']],'Min+xv(y+z)')

Malt = Identity([['mxxx', 'mxyy', 'myyx']], 'HM1')
HM2 = Identity([['pyyx', 'pxxx'], ['pxyy', 'qxxy'], ['qxyy', 'qxxx']], 'HM2')
HM3 = Identity([['pyyx', 'pxxx'], ['pxyy', 'qxxy'], ['qxyy', 'rxxy'], ['rxyy', 'rxxx']], 'HM3')
HM4 = Identity([['pyyx', 'pxxx'], ['pxyy', 'qxxy'], ['qxyy', 'rxxy'], ['rxyy', 'sxxy'], ['sxyy', 'sxxx']], 'HM4')
HM5 = Identity(
    [['pyyx', 'pxxx'], ['pxyy', 'qxxy'], ['qxyy', 'rxxy'], ['rxyy', 'sxxy'], ['sxyy', 'txxy'], ['txyy', 'txxx']], 'HM5')
HM6 = Identity(
    [['pyyx', 'pxxx'], ['pxyy', 'qxxy'], ['qxyy', 'rxxy'], ['rxyy', 'sxxy'], ['sxyy', 'txxy'], ['txyy', 'uxxy'],
     ['uxyy', 'uxxx']], 'HM6')
HM7 = Identity(
    [['pyyx', 'pxxx'], ['pxyy', 'qxxy'], ['qxyy', 'rxxy'], ['rxyy', 'sxxy'], ['sxyy', 'txxy'], ['txyy', 'uxxy'],
     ['uxyy', 'vxxy'], ['vxyy', 'vxxx']], 'HM7')
HM8 = Identity(
    [['pyyx', 'pxxx'], ['pxyy', 'qxxy'], ['qxyy', 'rxxy'], ['rxyy', 'sxxy'], ['sxyy', 'txxy'], ['txyy', 'uxxy'],
     ['uxyy', 'vxxy'], ['vxyy', 'wxxy'], ['wxyy', 'wxxx']], 'HM8')
HM9 = Identity(
    [['pyyx', 'pxxx'], ['pxyy', 'qxxy'], ['qxyy', 'rxxy'], ['rxyy', 'sxxy'], ['sxyy', 'txxy'], ['txyy', 'uxxy'],
     ['uxyy', 'vxxy'], ['vxyy', 'wxxy'], ['wxyy', 'axxy'], ['axyy', 'axxx']], 'HM9')



KMM = Identity([['p011', 'q100', 'q001'], ['p010', 'q010']], 'Sig3')

HM2Maj = Identity([['pyyx', 'pxxx'], ['pxyy', 'qxxy'], ['qxyy', 'qxxx'],['pxyx','pxxx','qxyx']], 'HM2Maj')

def getJohn(n):
    alph=[chr(i) for i in range(100000)]
    ids = [[alph[1] + 'xxy', alph[1] + 'xxx']]
    #ids = [[alph[1] + 'xyz', alph[1] + 'xxx']]
    for i in range(1,n+1):
        ids += [[alph[2*i-1]+'xyy',alph[2*i]+'xyy']]
        ids += [[alph[2*i]+'xxy',alph[2*i+1]+'xxy']]
    for i in range(1,2*n+2):
        ids += [[alph[i] + 'xyx', alph[i] + 'xxx']]
    ids += [[alph[2*n+1]+'xyy',alph[2*n+1]+'yyy']]
    #ids += [[alph[2 * n + 1] + 'xyz', alph[2 * n + 1] + 'zzz']]
    return Identity(ids,'John'+str(n))
#j_i(x,y,x)  = j_i(x,x,x)  for all i \in {1,\dots,2n+1}
#j_{2i-1}(x,y,y)  = j_{2i}(x,y,y) for all i \in {1,\dots,n}
#j_{2i}(x,x,y)  = j_{2i+1}(x,x,y) for all i  \in {1,\dots,n}
#j_{2n+1}(x,y,y)  = j_{2n+1}(y,y,y).

def getKK(n):#same as getSDJoin
    alph = [chr(i) for i in range(100000)]
    if n<2:
        print('n should be at least 2')
        return 'n should be at least 2'
    ids = [[alph[0] + 'xyz', alph[0] + 'xxx']]
    for i in range(n):
        if i%2==0:
            ids += [[alph[i]+'xyy',alph[i+1]+'xyy']]
            ids += [[alph[i]+'xyx',alph[i+1]+'xyx']]
        else:
            ids += [[alph[i] + 'xxy', alph[i + 1] + 'xxy']]

    ids += [[alph[n]+'xyz',alph[n]+'zzz']]
    return Identity(ids,'KK'+str(n))

#d_0(x,y,z)  = d_0(x,x,x)
#d_i(x,y,y)  = d_{i+1}(x,y,y) for even  i \in {0,1,\dots,n-1}
#d_i(x,y,x)  = d_{i+1}(x,y,x) for even  i \in {0,1,\dots,n-1}
#d_i(x,x,y)  = d_{i+1}(x,x,y) for odd  i  \in {1,\dots,n-1}
#d_n(x,y,z) = d_n(z,z,z)

#TODO https://marcinkozik.staff.tcs.uj.edu.pl/Directed.SDv.terms.pdf
def getDKK(n):#same as getSDJoin
    alph = [chr(i) for i in range(100000)]
    if n<2:
        print('n should be at least 2')
        return 'n should be at least 2'
    ids = [[alph[0] + 'xyz', alph[0] + 'xxx']]
    for i in range(n):
        ids += [[alph[i] + 'xyx', alph[i] + 'xxx']]
        if i%2==0:
            ids += [[alph[i]+'xyy',alph[i+1]+'xyy']]
            ids += [[alph[i]+'xyx',alph[i+1]+'xyx']]
        else:
            ids += [[alph[i] + 'xxy', alph[i + 1] + 'xxy']]

    ids += [[alph[n]+'xyz',alph[n]+'zzz']]
    ids += [[alph[n] + 'xyx', alph[n] + 'xxx']]
    return Identity(ids,'DKK'+str(n))




def getHM(n, alph='abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ'):
    ids = [[alph[0] + 'xxx', alph[0] + 'yyx']]
    for i in range(n - 1):
        ids += [[alph[i] + 'xyy', alph[i + 1] + 'xxy']]
    ids += [[alph[n - 1] + 'xyy', alph[n - 1] + 'xxx']]
    return Identity(ids, 'HM' + str(n))



def getHMMaj(n, alph='abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ'):
    ids = [[alph[0] + 'xxx', alph[0] + 'yyx']]
    for i in range(n - 1):
        ids += [[alph[i] + 'xyy', alph[i + 1] + 'xxy']]
        ids += [[alph[i] + 'xyx', alph[i] + 'xxx']]
    ids += [[alph[n - 1] + 'xyy', alph[n - 1] + 'xxx']]
    ids += [[alph[n-1] + 'xyx', alph[n-1] + 'xxx']]
    return Identity(ids, 'HM' + str(n)+'Maj')

Majority = Identity([['mxxx', 'mxxy', 'mxyx', 'myxx']], 'Maj')
Minority = Identity([['myyy', 'mxxy', 'mxyx', 'myxx']], 'Min')
SymMajority = Identity([['txyz','tyxz','tyzx'],['txxy','txxx']],'SymMaj')
TwoSymMajority = Identity([['txyz','tyxz'],['txxy','txyx','tyxx','txxx']],'2SymMaj')

S2Min = Identity([['fxyabc','fyxabc','fxybac','fxybca'],['fxyabb','fxyaaa']],'S2+Min')

Const = Identity([['fx', 'fy']], 'x=y')

def getDJohn(n):
    global alph
    ids = [[alph[1] + 'xxy', alph[1] + 'xxx']]
    for i in range(1,n):
        ids += [[alph[i] + 'xyy', alph[i + 1] + 'xxy']]
        ids += [[alph[i] + 'xyx', alph[i + 1] + 'xxx']]

    ids += [[alph[n] + 'xyx', alph[n] + 'xxx']]
    ids += [[alph[n] + 'xyy', alph[n] + 'yyy']]
    return Identity(ids, 'DJohn' + str(n))

WNU3 = Identity([['fxxy', 'fxyx', 'fyxx']], 'WNU3')
WNU4 = Identity([['fxxxy', 'fxxyx', 'fxyxx', 'fyxxx']], 'WNU4')

WNU3u4 = Identity([['gxxy', 'gxyx', 'gyxx'],['fxxxy', 'fxxyx', 'fxyxx', 'fyxxx'], ['gyxx','fyxxx']], 'WNU3u4')

Edge4 = Identity([['fyyxx','fyxyx','fxxxx','fxxxy']],'Edge4')
Edge5 = Identity([['fyyxxx','fyxyxx','fxxxxx','fxxxyx','fxxxxy']],'Edge5')

NU3 = Identity([['fxxy', 'fxyx', 'fyxx', 'fxxx']], 'NU3')
NU4 = Identity([['fxxxy', 'fxxyx', 'fxyxx', 'fyxxx', 'fxxxx']], 'NU4')
NU5 = Identity([['fxxxxy', 'fxxxyx', 'fxxyxx', 'fxyxxx', 'fyxxxx', 'fxxxxx']], 'NU5')
NU6 = Identity([['fxxxxxy', 'fxxxxyx', 'fxxxyxx', 'fxxyxxx', 'fxyxxxx', 'fyxxxxx', 'fxxxxxx']], 'NU6')
NU7 = Identity([['fxxxxxxy', 'fxxxxxyx', 'fxxxxyxx', 'fxxxyxxx', 'fxxyxxxx', 'fxyxxxxx', 'fyxxxxxx', 'fxxxxxxx']], 'NU7')

Pix2 = Identity([['pxyy', 'pxxx', 'pxyx'], ['pxxy', 'qxyy'], ['qyxy', 'qxxy', 'qyyy']], 'Px2')

SN123 = Identity([['f000412', 'f341323']], 'SN123')

# SP313 = Identity([['f0124456', 'f1233567']],'SP313') # already unsat for P2

SN1234 = Identity([['f0000412567', 'f3415323673']], 'SN1234')  # arity too high

ST4 = Identity([['f000112', 'f123233']])  # should be equivalent to HM2

NUFS3 = Identity([['fxxy', 'fxyx', 'fyxx', 'fxxx'], ['fxyz', 'fzxy', 'fyxz']], 'NUFS3')
NUFS4 = Identity([['fxxxy', 'fxxxx'], ['fabcd', 'fbcda', 'fbacd']], 'NUFS4')
NUFS5 = Identity([['fxxxxy', 'fxxxxx'], ['fabcde', 'fbcdea', 'fbacde']], 'NUFS5')
NUFS6 = Identity([['fxxxxxy', 'fxxxxxx'], ['fabcdef', 'fbcdefa', 'fbacdef']], 'NUFS6')
NUFS7 = Identity([['fxxxxxxy', 'fxxxxxxx'], ['fabcdefg', 'fbcdefga', 'fbacdefg']], 'NUFS7')
NUFS8 = Identity([['fxxxxxxxy', 'fxxxxxxxx'], ['fabcdefgh', 'fbcdefgha', 'fbacdefgh']], 'NUFS8')


GuardedFS1 = Identity([['faax', 'fbbx', 'fxaa']], 'GFS1') #=HM1?
GuardedFS2 = Identity([['faaxy', 'fbbyx', 'fxaay']], 'GFS2')
GuardedFS3 = Identity([['faaxyz', 'fbbyxz', 'fbbyzx', 'fxaayz']], 'GFS3')
GuardedFS4 = Identity([['faaxyzu', 'fbbyxzu', 'fbbyzux', 'fxaayzu']], 'GFS4')
GuardedFS5 = Identity([['faaxyzuv', 'fbbyxzuv', 'fbbyzuvx', 'fxaayzuv']], 'GFS5')
GuardedFS6 = Identity([['faaxyzuvw', 'fbbyxzuvw', 'fbbyzuvwx', 'fxaayzuvw']], 'GFS6')

Guarded2FS3 = Identity([['faaxyzcc', 'fbbyxzdd', 'fbbyzxdd','fxaayzcc','fbbxyddz']],'2GFS3')

Guarded2 = Identity([['faaxy', 'fbbxy', 'fxaay','fyaxa']], 'G2')

Guarded3 = Identity([['faaxyz', 'fbbxyz', 'fxaayz','fyaxaz','fzaxya']], 'G3')


GuardedSigma3 = Identity([['fxxxx', 'fxxxy'], ['f123x', 'f231x']], 'GS3')

Siggers = Identity([['farea', 'frare']], 'Sigg')

# noname terms to seperate structures in L
NNT = Identity([['0xyyz', '0xxxx'], ['2xxyz', '2zzzz'], ['0xxyx', '1xyyx'], ['1xxyx', '2xyyx'], ['0xxyy', '1xyyy'],
                ['1xxyy', '2xyyy']], 'NNT')


def getHMcK(n):
    global alph
    p=0
    d= 1
    e=d+n+1
    ids = [[alph[d] + 'xyz', alph[d] + 'xxx']]
    ids += [[alph[d+n] + 'xyy', alph[p] + 'xyy']]
    ids += [[alph[p] + 'xxy', alph[e] + 'xxy']]
    for i in range(n):
        if i%2==0:
            ids += [[alph[d+i] + 'xyy', alph[d+i+1] + 'xyy']]
            ids += [[alph[e+i] + 'xyy', alph[e+i+1] + 'xyy']]
            ids += [[alph[e+i] + 'xyx', alph[e+i+1] + 'xyx']]
        else:
            ids += [[alph[d+i] + 'xxy', alph[d+i+1] + 'xxy']]
            ids += [[alph[d+i] + 'xyx', alph[d+i+1] + 'xyx']]
            ids += [[alph[e+i] + 'xxy', alph[e+i+1] + 'xxy']]

    ids += [[alph[e+n] + 'xyz', alph[e+n] + 'zzz']]
    return Identity(ids, 'HMcK' + str(n))
# d0(x, y, z) = x
# di(x, y, y) = di+1(x, y, y) for even i < n
# di(x, x, y) = di+1(x, x, y) for odd i < n
# di(x, y, x) = di+1(x, y, x) for odd i < n
# dn(x, y, y) = p(x, y, y)
# p(x, x, y) = e0(x, x, y)
# ei(x, y, y) = ei1(x, y, y) for even i < n
# ei(x, x, y) = ei1(x, x, y) for odd i < n
# ei(x, y, x) = ei1(x, y, x) for even i < n
# en(x, y, z) = z

# [['fxzyx', 'fzyzy', 'fxxyz']]
# [['fyyxx', 'fyzxz', 'fxzzy']]

def getArguments(arity=3, maxVar=3, func='f'):
    res = func
    for k in range(arity):
        res += 'xyzabcde'[random.randint(0, maxVar - 1)]

    return res

#Ids = [Identities.getIdsSingleFunc(3,2,4) for _ in range(100)]
def getIdsSingleFunc(arity=3, maxVar=2, maxIds=4, classes=1, func='f'):
    Id = []
    for i in range(classes):
        nIds = random.randint(2, maxIds)
        c = [func] * nIds
        for k in range(arity):
            for l in range(nIds):
                c[l] += "xyzabcde"[random.randint(0, maxVar - 1)]
        Id += [c]
    if isSatByProjection(Identity(Id)):
        return getIdsSingleFunc(arity, maxVar, maxIds, classes, func)
    return Identity(Id)


# def getIds(maxVar=2,maxFunc=1):

def getTwoFunctionId(arity=3, maxVar=2, maxIds=4, classes=1):
    Id1 = getIdsSingleFunc(arity, maxVar, maxIds, classes, 'f')
    Id2 = getIdsSingleFunc(arity, maxVar, maxIds, classes, 'g')
    argf = getArguments(arity, maxVar, 'f')
    argg = getArguments(arity, maxVar, 'g')
    return Identity(Id1.Ids + Id2.Ids + [[argf, argg]])

#Ids = [Identities.getRandomId(3,2,4,1,1) for _ in range(100)]
def getRandomId(arity=3, maxVar=2, maxIds=4, classes=1, functions=1, Ids=None):
    if Ids is None:
        Ids = []
    fs = 'fghijklmnopqrstuvwxyz'
    for i in range(functions):
        Ids += getIdsSingleFunc(arity, maxVar, maxIds, classes, fs[i]).Ids
    for i in range(functions-1):
        Ids += [[getArguments(arity,maxVar,fs[i]),getArguments(arity,maxVar,fs[i+1])]]
    return Identity(Ids)

def isSatByProjection(Id: Identity):
    fs = {f: set(range(1, Id.arity[f] + 1)) for f in Id.functions}
    for eq in Id.Ids:
        for a in eq:
            for b in eq:
                if a[0] == b[0]:
                    fs[a[0]] = fs[a[0]].intersection({i for i in fs[a[0]] if a[i] == b[i]})
    for f in fs:
        if len(fs[f]) == 0:
            return False

    return True

def notSatisfysIdentityMinimalSubgraph(G,Id, ACWorks=False, Idempotent=True):
    IG, q = getIndicatorGraph(G, Id, returnq=True)
    fMap = None
    if Idempotent:
        fMap = dict()
        for f in Id.functions:
            a = Id.arity[f]
            fMap.update({q[tuple([f] + [u] * a)]: {u} for u in G.nodes})
    return ArcCon.getMinimalNoHomSubgraphFast(IG, G,fMap, ACWorks)

def addId(Id:Identity,G,maxVar=3,ACWorks=False):
    while True:
        arg = getArguments(Id.arity['f'],maxVar)
        if arg not in Id.Ids[0]:
            Idnew = copy.deepcopy(Id)
            Idnew.Ids[0]+=[arg]
            if satisfysIdentity(G,Idnew,ACWorks):
                return Idnew

