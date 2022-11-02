import networkx as nx


class Structure:
    def __init__(self,Gs):
        if len(Gs)>0 and not isinstance(Gs[0],nx.DiGraph):
            Gs = [nx.DiGraph(G) for G in Gs]
        self.nodes = set(Gs[0].nodes)
        for G in Gs:
            self.nodes.update(set(G.nodes))
        self.Gs = []
        for G in Gs:
            Gc = G.copy()
            Gc.add_nodes_from(self.nodes)
            self.Gs += [Gc]
    def remove_nodes_from(self,rmv):
        self.nodes.difference_update(rmv)
        for G in self.Gs:
            G.remove_nodes_from(rmv)

    def remove_node(self,v):
        self.remove_nodes_from({v})

    def copy(self):
        return Structure([G.copy() for G in self.Gs])

    def subgraph(self,nodes):
        G=self.copy()
        G.remove_nodes_from(self.nodes.difference(nodes))
        return G

    def add_node(self,v):
        for G in self.Gs:
            G.add_node(v)
        self.nodes.add(v)

    def edges(self):
        return [list(G.edges) for G in self.Gs]

def arcConStruck():
    return False
    #use arcCon for every graph

