# 重み付きグラフの1辺を表現するクラス
class Gedge:
    # interface var.
    # instance var.
    # s(int):source
    # t(int):target
    # c(?):cost
    # id(int):id of edge
    def __init__(self, s, t, c, id):
        self.s = s
        self.t = t
        self.c = c
        self.id = id


# 重み付きグラフを表現するクラス
class WeightedGraph:
    # interface var.
    # instance var.
    # edge([][]):edge[s]は点sから出る辺Gedgeのリスト
    # N:頂点数
    def __init__(self, N):
        self.N = N
        self.edge = [[] for i in range(N)]

    # 有向辺s->tを追加する
    def add_edge(self, s, t, c, id=-1):
        self.edge[s].append(Gedge(s, t, c, id))

    # 有向辺s->tとt->sを追加する
    def add_binary_edge(self, s, t, c, id=-1):
        self.add_edge(s, t, c, id)
        self.add_edge(t, s, c, id)


# 木の1頂点を表現するクラス
class TNode:
    # interface var.
    # instance var.
    # parent(int)
    # childs([]):all int
    # cost:この頂点から親への辺の重み
    # int id:この頂点から親への辺のid
    def __init__(self, p: int, c, id: int = -1):
        self.parent = p
        self.cost = c
        self.id = id
        self.childs = []


# 木をTNodeのリストとして表す
class Tree:
    # interface var.
    # instance var.
    # N(int): number of nodes
    # nodes[]:list of TNode
    # root(int)
    # WeightedGraphから木を生成する
    def __init__(self, g: WeightedGraph, root: int = 0):
        self.N = g.N
        self.nodes = [0 for _ in range(self.N)]
        self.root = root
        q = queue.Queue()
        q.put(root)
        self.nodes[root] = TNode(-1, 0, -1)
        while not q.empty():
            cur = q.get()
            for node in g.edge[cur]:
                if node.t == self.nodes[cur].parent:
                    continue
                self.nodes[node.t] = TNode(cur, node.c, node.id)
                self.nodes[cur].childs.append(node.t)
                q.put(node.t)


# 木になっている無向グラフGの直径を求める
# return tuple<int,int,int>:最も遠い頂点の組、直径
def get_diameter_pair(G: WeightedGraph, T: Tree) -> tuple:
    root = T.root
    N = G.N
    v1 = [0] * N
    q = queue.Queue()
    q.put(root)
    newroot = 0
    while not q.empty():
        n = q.get()
        for ch in T.nodes[n].childs:
            v1[ch] = v1[n] + T.nodes[ch].cost
            if v1[ch] >= v1[newroot]:
                newroot = ch
            q.put(ch)
    T2 = Tree(G, newroot)
    v2 = [0] * N
    q.put(newroot)
    far = newroot
    while not q.empty():
        n = q.get()
        for ch in T2.nodes[n].childs:
            v2[ch] = v2[n] + T2.nodes[ch].cost
            if v2[ch] >= v2[far]:
                far = ch
            q.put(ch)
    return newroot, far, v2[far]


