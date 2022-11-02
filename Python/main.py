#author: Florian Starke (florian.starke@tu-dresden.de)
import treeGeneration
import Identities


#test ids from the paper for a singe tree
def testIdentitiesforTree(T, Ids=None, isTree=True):
    if Ids is None:
        Ids = [Identities.Majority, Identities.KMM, Identities.WNU3u4, Identities.getHM(10), Identities.getHMcK(10),
               Identities.getKK(10), Identities.getJohn(10)]
    isTS = treeGeneration.isTotallySymmetric(T,skip=True,isTree=isTree)
    print('tree' if isTree else 'Digraph', 'has' if isTS else 'does not have', 'TS(n) for all n')


    if isTree: levels=treeGeneration.getLevelsOfBalancedGraph(T)

    for Id in Ids:
        if isTree:
            sat = Identities.satisfysIdentity(T, Id,ACWorks=isTS,partition=treeGeneration.getLevelsOfBalancedGraph(T))
        else:
            sat = Identities.satisfysIdentity(T, Id, ACWorks=isTS)

        print('tree' if isTree else 'Digraph', 'satisfies' if sat else 'does not satisfy', Id)




if __name__ == "__main__":
    print('Please enter the number of vertices (at least 1)')
    n = int(input())

    if n<10:
        rootedCoreTrees = []
        for d in range(n):
            rootedCoreTrees += treeGeneration.getRootedCoreTrees(n, d)
        print(len(rootedCoreTrees), 'rooted Core Trees with', n, 'vertices')
    else:
        print('computing number rooted core trees would take a lot of time so I skipped it')

    if n>13:
        print('consider reading the trees from a file with treeGeneration.getCoreTreesFromFile(n)')
    coreTrees = list(treeGeneration.getCoreTrees(n))
    #TODO if you use this you have to remove the line "print(len(coreTrees)...." because getCoreTreesFromFile returns a generator and not a list to save memory
    #TODO change directory such that reading from file works
    #coreTrees = treeGeneration.getCoreTreesFromFile(n)
    #treeGeneration.getNoMajorityCoreTreesFromFile(n)



    print(len(coreTrees), 'Core Trees with', n, 'vertices')
    noTS2CoreTrees = list(treeGeneration.filterForNoTS2(coreTrees))
    print(len(noTS2CoreTrees), 'of those do not have a commutative polymorphism')
    noMajCoreTrees = list(treeGeneration.filterForNoMajority(noTS2CoreTrees))
    print(len(noMajCoreTrees), 'of those without commutative polymorphism also do not have a majority polymorphism')
    noKMMCoreTrees = list(treeGeneration.filterForNoKMM(noMajCoreTrees))
    print(len(noKMMCoreTrees), 'of those do not have a KMM polymorphism')

    print('testing tree',coreTrees[0].edges)
    testIdentitiesforTree(coreTrees[0])
