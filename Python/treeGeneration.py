import itertools
import random

import networkx as nx

import ArcConFast
import ArcConFast as ArcCon
from pyvis.network import Network
import copy
import time
import ast
import os.path

import Identities
import Structures


def drawGraph(G, phi=("asd", "")):
    nt = Network('700px', '1000px', directed=True, heading=phi)
    relabel = {v: str(v) for v in G.nodes}
    GStr = nx.relabel_nodes(G, relabel)
    nt.from_nx(GStr)
    # nt.set_options()
    nt.show_buttons(filter_=['physics', 'layout'])
    nt.show('nx.html')

    time.sleep(0.5)


def getCoreTreesFromFile(n, path=r'tripolys_data-master\\',filename=r'\cores.edges', filterSomeForOrientation=False):

    ls = open(path + str(n) + filename).readlines()

    for l in ls:
        T = nx.DiGraph(ast.literal_eval(l))
        if filterSomeForOrientation:
            lvl = getLevelsOfBalancedGraph(T)
            i = 0
            height = max(v for v in lvl)
            while len(lvl[i]) == len(lvl[height - i]) and i < height / 2:
                i += 1
            if len(lvl[i]) >= len(lvl[height - i]):
                yield T
        else:
            yield T


def getNoMajorityCoreTreesFromFile(n,path=r'tripolys_data-master\\',filename=r'\majority_n.edges', filterSomeForOrientation=False):
    ls = open(path + str(n) + filename).readlines()
    res = [nx.DiGraph(ast.literal_eval(l)) for l in ls]
    if filterSomeForOrientation:
        resNew = []
        for T in res:
            lvl = getLevelsOfBalancedGraph(T)
            i = 0
            height = max(v for v in lvl)
            while len(lvl[i]) == len(lvl[height - i]) and i < height / 2:
                i += 1
            if len(lvl[i]) >= len(lvl[height - i]):
                resNew += [T]
        res = resNew
    return res


def getOrientation(T):
    lvl = getLevelsOfBalancedGraph(T)
    i = 0
    height = max(v for v in lvl)
    while len(lvl[i]) == len(lvl[height - i]) and i < height / 2:
        i += 1
    if len(lvl[i]) > len(lvl[height - i]):
        return 0
    if len(lvl[i]) == len(lvl[height - i]):
        return 1
    return 2


def treesToTikz(Ts):
    header = r'''\tikzstyle{bullet}=[circle,fill,inner sep=0pt,minimum size=3pt]
\usetikzlibrary{arrows}
\def\scale{0.5}
\def\hdist{1cm}
\def\vdist{1cm}
'''
    text = r'''\begin{figure}
\centering'''
    for i, T in enumerate(Ts):

        text += treeToTikz(T, i + 1)
        if (i + 1) % 4 == 0:
            text += '\n\n' + r'\vspace{\vdist}' + '\n'
        else:
            text += r'\hspace{\hdist}' + '\n'
    text += r'''\caption{NP-hard trees with 20 vertices.}
\label{fig:my_label}
\end{figure}'''
    return header + '\n' + text


def treeToTikz(T, name):
    # print(name)
    levels = 2
    f = ArcCon.findHom(T, getPath('1' * levels))
    while f is None:
        levels += 1
        f = ArcCon.findHom(T, getPath('1' * levels))
    # print('edgelevels:', levels)
    text = r'\begin{tikzpicture}[scale=\scale]' + '\n'

    counter = {l: 0 for l in range(levels + 1)}
    levelsTxt = {l: '' for l in range(levels + 1)}
    for v in T.nodes:
        levelsTxt[f[v]] += r'\node[bullet] (' + str(v) + ') at (' + str(counter[f[v]]) + ',' + str(
            f[v]) + ') {};' + '\n'
        counter[f[v]] += 1
    for i in range(levels + 1):
        text += r'% Level ' + str(i) + '\n'
        text += levelsTxt[i]
    text += r'\path[->,>=stealth' + '\']' + '\n'
    for e in T.edges:
        text += '(' + str(e[0]) + ') edge (' + str(e[1]) + ')\n'
    text += ';\n'
    text += r'\node at (' + str((max([counter[j] for j in counter]) - 1) / 2) + ',-1) {Tree ' + str(name) + '};\n'
    text += r'\end{tikzpicture}' + '\n'
    # print(text)
    return text


def upToIsomorphism(Gs):
    Ugs = set()
    Gs = set(Gs)
    while len(Gs) > 0:
        G = Gs.pop()
        Ugs.add(G)
        Gs = {H for H in Gs if not nx.is_isomorphic(H, G)}

    #        print(len(Ugs))
    return Ugs


def computeHMTableForTreesUpTo(n, treesFromFile=True, append=False, appendafter=0):
    k = 2
    numbers = dict()
    paths = dict()
    while k <= n and os.path.exists(r'treesHM\\' + str(k)):

        numbers[k] = dict()
        f = open(r'treesHM\\' + str(k))
        for line in f:
            l = line.rstrip().split(',')
            mmin = int(l[-2])
            if mmin not in numbers[k]:
                numbers[k][mmin] = 0
            numbers[k][mmin] += 1 if 'True' == (l[-1].replace(' ', '')) else 2
        if append and k >= appendafter:
            # todo numberofTrees is too large as it is not clear whether dual tree has been skipped
            numberOfTrees = sum([numbers[k][mm] for mm in numbers[k]])
            ps = getCoreTreesFromFile(k)
            numOfTrees = len(open(r'tripolys_data-master\\' + str(k) + r'\edges').readlines())
            if numberOfTrees < numOfTrees:
                print('append', k, numberOfTrees, numOfTrees, appendafter)
                f.close()
                ps = getCoreTreesFromFile(k)
                for i in range(numberOfTrees):
                    next(ps)
                f = open(r'treesHM\\' + str(k), 'a')
                for p in ps:
                    lvl = getLevelsOfBalancedGraph(p)
                    o = getOrientation(p)
                    if o == 2:
                        continue

                    mmin = 2
                    max = 29
                    HM = 4
                    while mmin < max:
                        if Identities.satisfysIdentity(p, Identities.getHM(HM), True, partition=lvl):
                            max = HM
                        else:
                            mmin = HM + 1
                        if mmin > 18:  # todo remove later
                            mmin = max
                        HM = (mmin + max) // 2

                    f.writelines([str(list(p.edges)) + ', ' + str(mmin) + ', ' + str(o == 1) + '\n'])
                    if mmin not in numbers[k]:
                        numbers[k][mmin] = 0
                    numbers[k][mmin] += 1 if o == 1 else 2

        k += 1

    while k <= n:
        numbers[k] = dict()
        f = open(r'treesHM\\' + str(k), 'w+')
        if treesFromFile:
            ps = getCoreTreesFromFile(k)
        else:
            ps = getCoreTrees(k)
        for p in ps:

            lvl = getLevelsOfBalancedGraph(p)
            o = getOrientation(p)
            if o == 2:
                continue

            mmin = 2
            max = 29
            HM = 4
            while mmin < max:
                if Identities.satisfysIdentity(p, Identities.getHM(HM), True, partition=lvl):
                    max = HM
                else:
                    mmin = HM + 1
                HM = (mmin + max) // 2

            f.writelines([str(list(p.edges)) + ', ' + str(mmin) + ', ' + str(o == 1) + '\n'])
            if mmin not in numbers[k]:
                numbers[k][mmin] = 0
            numbers[k][mmin] += 1 if o == 1 else 2
        k += 1
        f.close()
    return numbers


def printTable(numbers):
    lines = [''] * 30
    for k in numbers:
        for i in range(1, 30):
            if i in numbers[k]:
                lines[i - 1] += str(numbers[k][i]) + ','
            else:
                lines[i - 1] += '0,'
    for l in lines:
        print(l)


def evaluate(G, phi):
    if (phi[0] + phi[1]).isnumeric():
        return [phi]
    res = []
    for l in "abcdefghijklmnopqrstuvwxyz":
        if l in phi[0] + phi[1]:
            break

    if l.capitalize() in phi[0] + phi[1]:
        for (u, v) in G.edges:
            res += evaluate(G, (phi[0].replace(l, str(u)).replace(l.capitalize(), str(v)),
                                phi[1].replace(l, str(u)).replace(l.capitalize(), str(v))))
    else:
        for v in G.nodes:
            res += evaluate(G, (phi[0].replace(l, str(v)), phi[1].replace(l, str(v))))
    return res


def pppower(G, phi):
    if len(G.nodes) > 10:
        print("Warning: too many nodes", phi, G.edges)
    if not isinstance(G, nx.DiGraph):
        Hs = []
        for i in range(len(G.Gs)):
            Hs += [pppower(G.Gs[i], phi[i])]
        return Structures.Structure(Hs)
    else:
        H = nx.DiGraph()
        edges = evaluate(G, phi)
        # print(edges)
        H.add_edges_from(edges)
        return H


def getLevelsOfBalancedGraphSlow(T):
    levels = 2
    f = ArcCon.findHom(T, getPath('1' * levels))
    while f is None:
        levels += 1
        f = ArcCon.findHom(T, getPath('1' * levels))

    lvl = dict()
    for i in range(levels + 1):
        lvl[i] = {v for v in f if f[v] == i}
    return lvl


def getLevelsOfBalancedGraph(T):
    nodes = list(T.nodes)
    levels = 2 * len(nodes)
    f = ArcCon.arcCon(T, {nodes[0]: {levels // 2}}, getPath('1' * levels), workingSet={nodes[0]})[0]

    lvl = dict()
    for i in range(levels + 1):
        lvl[0] = {v for v in f if f[v] == {i}}
        if len(lvl[0]) > 0:
            s = i
            break
    for i in range(s + 1, levels + 1):
        lvl[i - s] = {v for v in f if f[v] == {i}}
        if len(lvl[i - s]) == 0:
            del lvl[i - s]
            break

    return lvl


def relabelGraph(G):
    nodes = list(G.nodes)
    relabel = {nodes[i]: i for i in range(len(nodes))}
    return nx.relabel_nodes(G, relabel)


def getWords(n, alph):
    if n == 0:
        return [""]
    return [w + l for w in getWords(n - 1, alph) for l in alph]


def getTuples(n, alph):
    if n == 0:
        return [()]
    return [w + (l,) for w in getTuples(n - 1, alph) for l in alph]


def concat(ls):
    if len(ls) == 0:
        return ""
    return ls[0] + concat(ls[1:])


def getLowerNoDublicates(w):
    w = [a for a in w if a.islower()]
    res = ''
    for a in w:
        if a not in res:
            res += a
    return res


def getOrderedWords(alph, n, inc):
    if n == 0:
        return [""]
    ws = []
    for i in range(min(inc + 1, len(alph))):
        ws += [alph[i] + w for w in getOrderedWords(alph[i:], n - 1, inc)]
    return ws


def getFormulaFirstTuple(vs, n):
    fst = [getOrderedWords("abcdefghijklmnopqrstuvwxyz", k, 1) for k in range(n)]
    snd = [getOrderedWords("ABCDEFGHIJKLMNOPQRSTUVWXYZ"[:n], k, n) for k in range(n)]
    trd = [getOrderedWords(vs, k, len(vs)) for k in range(n)]

    ws = []
    for i in range(0, n):
        for j in range(0, n - i):
            ws += ['a' + u + v + w for u in fst[i] for v in snd[j] for w in trd[n - j - i - 1]]
    return ws


# for tikzing
def whichTreesAreAlmostTheSame(Ts):
    TTs = []
    for T in Ts:
        l = getLevelsOfBalancedGraph(T)
        if len(l[0]) == 1:
            v = l[0].pop()
            T = T.copy()
            T.remove_node(v)
        if len(l[max(l.keys())]) == 1:
            v = l[max(l.keys())].pop()
            T = T.copy()
            T.remove_node(v)
        TTs += [T]
    for i in range(len(TTs)):
        for j in range(i):
            if nx.is_isomorphic(TTs[i], TTs[j]) or nx.is_isomorphic(TTs[i], TTs[j].reverse()):
                print(i, '=', j)


def getTriad(w1, w2, w3):
    G1 = getPath(w1)
    G2 = getPath(w2)
    G3 = getPath(w3)
    nx.relabel_nodes(G2, lambda v: 0 if v == 0 else v + len(w1), False)
    nx.relabel_nodes(G3, lambda v: 0 if v == 0 else v + len(w1) + len(w2), False)
    G1.add_edges_from(G2.edges)
    G1.add_edges_from(G3.edges)
    return G1


def hasLoop(G):
    return len([n for n in G.nodes if (n, n) in G.edges]) > 0


def getCore(G, timelimit=float('inf')):
    G = G.copy()
    if isinstance(G, nx.DiGraph) and hasLoop(G):
        return nx.DiGraph([(0, 0)])
    # G = removeStrayEdges(G)
    changed = True
    while changed:
        # print(str((len(G.nodes), len(G.edges))))
        changed = False
        # arccon on G
        fs = ArcCon.initF(G, G)
        fs = ArcCon.arcCon(G, fs, G)[0]
        for v in G.nodes:
            for u in fs[v]:
                if u != v:
                    # print(u,v,changed)
                    # try to contract u and v by finding endo with f(u)=f(v)=u or f(u)=f(v)=v
                    # f = dict()
                    f = copy.deepcopy(fs)

                    f[u] = {u}
                    f[v] = {u}
                    f = ArcCon.findHom(G, G, f, timelimit=timelimit)
                    if f is not None:
                        # print(f)
                        changed = True
                        # reduce G to im(f)
                        im = {f[k] for k in f.keys()}
                        rest = {v for v in G.nodes if v not in im}
                        G.remove_nodes_from(rest)
                        break
            if changed:
                break
    return G


# todo verify corectness and compare time with getCore
def getCore2(G, timelimit=float('inf')):
    G = G.copy()
    if isinstance(G, nx.DiGraph) and hasLoop(G):
        return nx.DiGraph([(0, 0)])
    # G = removeStrayEdges(G)
    changed = True
    while changed:
        # print(str((len(G.nodes), len(G.edges))))
        changed = False
        # arccon on G
        fs = ArcCon.initF(G, G)
        fs = ArcCon.arcCon(G, fs, G)[0]
        for v in G.nodes:
            # print(u,v,changed)
            # try to contract u and v by finding endo with f(u)=f(v)=u or f(u)=f(v)=v
            # f = dict()
            f = copy.deepcopy(fs)

            [f[u].remove(v) for u in f]
            f = ArcCon.findHom(G, G, f, timelimit=timelimit)
            if f is not None:
                # print(f)
                changed = True
                # reduce G to im(f)
                im = {f[k] for k in f.keys()}
                rest = {v for v in G.nodes if v not in im}
                G.remove_nodes_from(rest)
                break
    return G


def getCoreC3Conservative(G, zeros={3}, timelimit=float('inf')):
    G = G.copy()
    if hasLoop(G):
        return nx.DiGraph([(0, 0)])
    # G = removeStrayEdges(G)
    changed = True
    while changed:
        # print(str((len(G.nodes), len(G.edges))))
        changed = False
        # arccon on G
        fs = ArcCon.initF(G, G)
        unRel = {k for k in G.nodes if k[0] == k[1] and {k[s] for s in zeros} == {'0'}}
        print(unRel)
        for k in G.nodes:
            if k[0] == k[1] and {k[s] for s in zeros} == {'0'}:
                fs[k] = unRel.copy()

        fs = ArcCon.arcCon(G, fs, G)[0]
        for v in G.nodes:
            for u in fs[v]:
                if u != v:
                    # print(u,v,changed)
                    # try to contract u and v by finding endo with f(u)=f(v)=u or f(u)=f(v)=v
                    # f = dict()
                    f = copy.deepcopy(fs)

                    f[u] = {u}
                    f[v] = {u}
                    f = ArcCon.findHom(G, G, f, timelimit=timelimit)
                    if f is not None:
                        # print(f)
                        changed = True
                        # reduce G to im(f)
                        im = {f[k] for k in f.keys()}
                        rest = {v for v in G.nodes if v not in im}
                        G.remove_nodes_from(rest)
                        break
            if changed:
                break
    return G


rct = dict()


def getWordsMaxLen(n, alph, maxLen, letter, length):
    if n == 0:
        return ['']
    ws = []
    for a in alph:
        if a == letter and length < maxLen:
            ws += [a + w for w in getWordsMaxLen(n - 1, alph, maxLen, a, length + 1)]
        elif a != letter:
            ws += [a + w for w in getWordsMaxLen(n - 1, alph, maxLen, a, 1)]
    return ws


def getCoreTrees(n):
    Ts = []
    # center
    for i in range(1, n):
        for d in range(1, n):
            if d + 1 <= n - i:
                rootedCoreTreesd = getRootedCoreTrees(i, d)
                if len(rootedCoreTreesd) > 0:
                    rootedCoreTreesd2 = getRootedCoreTrees(n - i, d + 1)
                    for T1, r1 in rootedCoreTreesd:
                        for T2, r2 in rootedCoreTreesd2:
                            newT = T1.copy()
                            nodesT = list(newT.nodes)
                            newT = nx.relabel_nodes(newT, {nodesT[j]: j for j in range(len(nodesT))})
                            T2 = T2.copy()
                            nodes2 = list(T2.nodes)
                            T2 = nx.relabel_nodes(T2, {nodes2[j]: j + len(nodesT) for j in range(len(nodes2))})
                            newT.add_edges_from(T2.edges)
                            newT1 = newT.copy()
                            newT1.add_edge(nodesT.index(r1), nodes2.index(r2) + len(nodesT))
                            newT.add_edge(nodes2.index(r2) + len(nodesT), nodesT.index(r1))
                            if ArcConFast.isTreeCore(newT) and True not in [nx.is_isomorphic(newT, T) for T in Ts]:
                                Ts += [newT]
                            if ArcConFast.isTreeCore(newT1) and True not in [nx.is_isomorphic(newT1, T) for T in Ts]:
                                Ts += [newT1]

    # bicenter
    for d in range(n):
        for i in range(n):
            if d <= n - i:
                rootedCoreTreesd = getRootedCoreTrees(i, d)
                if len(rootedCoreTreesd) > 0:
                    rootedCoreTreesd2 = getRootedCoreTrees(n - i, d)
                    for T1, r1 in rootedCoreTreesd:
                        for T2, r2 in rootedCoreTreesd2:
                            newT = T1.copy()
                            nodesT = list(newT.nodes)
                            newT = nx.relabel_nodes(newT, {nodesT[j]: j for j in range(len(nodesT))})
                            T2 = T2.copy()
                            nodes2 = list(T2.nodes)
                            T2 = nx.relabel_nodes(T2, {nodes2[j]: j + len(nodesT) for j in range(len(nodes2))})
                            newT.add_edges_from(T2.edges)
                            newT.add_edge(nodesT.index(r1), nodes2.index(r2) + len(nodesT))
                            if ArcConFast.isTreeCore(newT):
                                Ts += [newT]
    return Ts

    # for d in range(n):
    #    Ts += getRootedCoreTrees(n,d)
    # return [T for T,_ in Ts if ArcConFast.isTreeCore(T)]


def getRootedCoreTrees(n, d, onlyCores=True):
    # print('start',n,d)
    if n == 1 and d == 0:
        T = nx.DiGraph()
        T.add_node(0)
        return [(T, 0)]
    if d > n + 1 or d == 0:
        return []
    global rct
    if (n, d, onlyCores) in rct:
        return rct[(n, d, onlyCores)]

    Ts = []
    count = 0
    combTime = 0
    testTime = 0
    for i in range(1, n):
        # print('T', i, d - 1)
        for T, t in getRootedCoreTrees(i, d - 1, onlyCores):
            for d2 in range(0, d + 1)[::-1]:
                # print('S', n - 1, d2)
                for S, s in getRootedCoreTrees(n - i, d2, onlyCores):
                    count += 1
                    start = time.time()
                    newT = T.copy()
                    nodesT = list(newT.nodes)
                    newT = nx.relabel_nodes(newT, {nodesT[j]: j for j in range(len(nodesT))})
                    S = S.copy()
                    nodesS = list(S.nodes)
                    S = nx.relabel_nodes(S, {nodesS[j]: j + len(nodesT) for j in range(len(nodesS))})
                    newT.add_edges_from(S.edges)
                    newt = nodesS.index(s) + len(nodesT)
                    newT1 = newT.copy()
                    newT1.add_edge(nodesT.index(t), newt)
                    newT2 = newT.copy()
                    newT2.add_edge(newt, nodesT.index(t))
                    combTime += time.time() - start
                    start = time.time()
                    if addCoreToTreeList(Ts, newT1, newt, d2 == d, onlyCores):
                        Ts += [(newT1, newt)]
                    if addCoreToTreeList(Ts, newT2, newt, d2 == d, onlyCores):
                        Ts += [(newT2, newt)]
                    testTime += time.time() - start
    # print(n, d, 'iterations:', count, 'combine time:', int(combTime * 1000), 'ms test time:', int(testTime * 1000), 'ms')
    rct[(n, d, onlyCores)] = copy.deepcopy(Ts)
    return Ts


def addCoreToTreeList(Ts, newT, newt, homeqpossible=True, onlyCores=True):
    if not onlyCores:
        return True
    if ArcCon.isTreeCore(newT, {newt: {newt}}, workingSet={newt}):
        if homeqpossible:
            for (T, t) in Ts:
                #                if nx.is_isomorphic(newT,T,{newt:t}):
                if ArcCon.existsHom(T, {t: {newt}}, newT, ACWorks=True, componentwise=False,
                                    workingSet={t}) and ArcCon.existsHom(newT, {newt: {t}}, T, ACWorks=True,
                                                                         componentwise=False, workingSet={newt}):
                    # print('hom eq')
                    return False
        return True
    return False

def filterForNoTS2(Ts):
    for T in Ts:
        levels = getLevelsOfBalancedGraph(T)
        if not Identities.satisfysIdentity(T, Identities.Sigma2,partition=levels):
            yield T

def filterForNoMajority(Ts):
    for T in Ts:
        if not Identities.satisfysIdentity(T, Identities.Majority):
            yield T


def filterForNoKMM(Ts):
    for T in Ts:
        if not Identities.satisfysIdentity(T, Identities.KMM):
            yield T


def getPath(w):
    n = len(w)
    return nx.DiGraph(
        [(i, (i + 1)) for i in range(n) if w[i] == '1'] + [((i + 1), i) for i in range(n) if w[i] == '0'])


def getCycle(w):
    n = len(w)
    return nx.DiGraph(
        [(i, (i + 1) % n) for i in range(n) if w[i] == '1'] + [((i + 1) % n, i) for i in range(n) if w[i] == '0'])


def getSubSets(A: set, k):
    if k == 0:
        return {frozenset()}
    ss = getSubSets(A, k - 1)
    res = set()
    for s in ss:
        for a in A.difference(s):
            res.add(frozenset(s.union({a})))
    return res




def PowerSetGraphHasEdge(G, u, v):
    for Gu in u:
        if len(set(G.successors(Gu)).intersection(v)) == 0:
            return False
    for Gv in v:
        if len(set(G.predecessors(Gv)).intersection(u)) == 0:
            return False
    return True


def getPowerSetGraphSlow(G, n=None, relabel=True):
    if n is None:
        n = len(G.nodes)
    PG = nx.DiGraph()

    sources = {v for v in G.nodes if G.out_degree[v] == 0}
    sinks = {v for v in G.nodes if G.in_degree[v] == 0}
    # add nodes
    for k in range(1, n + 1):
        for S in getSubSets(set(G.nodes), k):
            # PG.add_nodes_from(getSubSets(set(G.nodes), k))
            if len(S.intersection(sinks)) == 0 or len(S.intersection(sources)) == 0:
                PG.add_node(S)
    # add edges
    # print('start',PG.nodes)
    for u in PG.nodes:
        v = set()
        for Gu in u:
            v = v.union(set(G.successors(Gu)))
        workingLs = [frozenset(v)]
        while len(workingLs) > 0:
            v = workingLs.pop()
            if PowerSetGraphHasEdge(G, u, v):
                if len(v) <= n:
                    PG.add_edge(u, v)
                for Gv in v:
                    workingLs += [v.difference({Gv})]

    if relabel:
        return nx.relabel_nodes(PG, lambda v: str(set(v)))
    return PG


def getPowerSetGraph(G, n=None, relabel=True):
    if n is None:
        n = len(G.nodes)
    PG = nx.DiGraph()

    sources = {v for v in G.nodes if G.out_degree[v] == 0}
    sinks = {v for v in G.nodes if G.in_degree[v] == 0}
    smooth = {v for v in G.nodes if G.in_degree[v] != 0 and G.out_degree[v] != 0}
    # add nodes
    for k in range(1, n + 1):
        for ks in range(0, k + 1):
            for Ssmooth in getSubSets(smooth, k - ks):
                for Ssinks in getSubSets(sinks, ks):
                    PG.add_node(Ssmooth.union(Ssinks))
                for Ssources in getSubSets(sources, ks):
                    PG.add_node(Ssmooth.union(Ssources))

    # add edges
    # print('start',PG.nodes)
    for u in PG.nodes:
        v = set()
        for Gu in u:
            v = v.union(set(G.successors(Gu)))
        workingLs = [frozenset(v)]
        while len(workingLs) > 0:
            v = workingLs.pop()
            if PowerSetGraphHasEdge(G, u, v):
                if len(v) <= n:
                    PG.add_edge(u, v)
                for Gv in v:
                    workingLs += [v.difference({Gv})]

    if relabel:
        return nx.relabel_nodes(PG, lambda v: str(set(v)))
    return PG


def testgetIncomparableSubsets():
    sets = getSubSets({1, 2, 3}, 1)
    sets.update(getSubSets({1, 2, 3}, 2))
    sets.update(getSubSets({1, 2, 3}, 3))
    s = getIncomparableSubsets(sets)
    return s


def getIncomparableSubsets(sets):
    print('sets', [set(s) for s in sets])
    res = [frozenset()]
    sets2 = sets.copy()
    for S in sets:
        print(S)
        sets2.remove(S)
        subsets = getIncomparableSubsets({R for R in sets2 if not R.issubset(S) and not R.issuperset(S)})
        print(subsets)
        res += [frozenset({S}.union(T)) for T in subsets]
    return res


def getkLayeredABSGraphTreeSameLvlComponent(G, k, relabel=True):
    BlockSymG = nx.DiGraph()
    ACG = getPowerSetGraphTreeSameLvlComponent(G, relabel=False)  # use for edges
    lvl = getLevelsOfBalancedGraph(G)
    # print(lvl)
    for l in lvl:
        sets = set()
        for k in range(1, len(G.nodes) + 1):
            sets.update(getSubSets(lvl[l], k))
        [BlockSymG.add_nodes_from(getSubSets(sets, m)) for m in range(1, len(sets) + 1)]
    print(len(BlockSymG.nodes), 65790)
    for vs in BlockSymG.nodes:
        print(vs)
        for ws in BlockSymG.nodes:
            hasEdge = True
            for v in vs:
                if len(set(ACG.successors(v)).intersection(ws)) == 0:
                    hasEdge = False
                    break
            if hasEdge:
                for w in ws:
                    if len(set(ACG.predecessors(w)).intersection(vs)) == 0:
                        hasEdge = False
                        break
            if hasEdge:
                BlockSymG.add_edge(vs, ws)
    return BlockSymG

    # compute all outgoing edges
    # for v in vs:
    #    ACG.successors(v)

    # print(Sss)
    # PG.add_nodes_from([s for s in getIncomparableSubsets(sets) if len(s)>0])


def getPowerSetGraphTreeSameLvlComponent(G, relabel=True):
    PG = nx.DiGraph()

    lvl = getLevelsOfBalancedGraph(G)
    for k in range(1, len(G.nodes) + 1):
        for l in lvl:
            PG.add_nodes_from(getSubSets(lvl[l], k))

    for u in PG.nodes:
        v = set()
        for Gu in u:
            v = v.union(set(G.successors(Gu)))
        workingLs = [frozenset(v)]
        while len(workingLs) > 0:
            v = workingLs.pop()
            if PowerSetGraphHasEdge(G, u, v):
                PG.add_edge(u, v)
                for Gv in v:
                    workingLs += [v.difference({Gv})]

    if relabel:
        return nx.relabel_nodes(PG, lambda v: str(set(v)))
    return PG


def isTotallySymmetric(G, n=None, skip=False, printres=False, isTree=False):
    #    print(2,G.edges)
    if len(G.nodes) <= 3:
        return Identities.satisfysIdentity(G, Identities.TS3)
    if n is None:
        n = len(G.nodes)
    if not skip:
        PG = getPowerSetGraph(G, 2)
        if not ArcCon.existsHom(PG, {str({v}): {v} for v in G.nodes}, G):
            return False
        #    print(3)
        PG = getPowerSetGraph(G, 3)
        if not ArcCon.existsHom(PG, {str({v}): {v} for v in G.nodes}, G):
            return False

    #    print(4)
    if isTree:
        PG = getPowerSetGraphTreeSameLvlComponent(G)
    else:
        PG = getPowerSetGraph(G)
    if ArcCon.existsHom(PG, {str({v}): {v} for v in G.nodes}, G):
        if printres:
            print('\033[92m', True, '\033[0m')
        return True
    if printres:
        print('\033[91m', False, '\033[0m')
    return False



