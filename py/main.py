import sys
import copy
import queue
from collections import deque
import heapq

sys.setrecursionlimit(1 << 25)

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

# 返り値res[n] := 頂点startからnまでの最短距離
# 到達不可能 -> -1
def dijkstra(g: WeightedGraph, start: int):
    N = g.N
    res = [-1] * N
    res[start] = 0
    q = [(0, start)]
    heapq.heapify(q)
    while len(q) != 0:
        cost, cur = heapq.heappop(q)
        if res[cur] != -1 and res[cur] < cost:
            continue
        for e in g.edge[cur]:
            if res[e.t] == -1 or res[e.t] > cost + e.c:
                res[e.t] = cost + e.c
                heapq.heappush(q, (res[e.t], e.t))
    return res



# l[i][j]<=Pなる組(i,j)の数を数える
def count(l,P:int ):
    ans = 0
    for i in range(len(l)):
        for j in range(i+1,len(l[i])):
            if l[i][j] != -1 and l[i][j] <= P:
                ans += 1
    return ans

N,P,K = tuple(map(int,input().split()))
A = [[0]*N for _ in range(N)]
for y in range(N):
    A[y] = list(map(int,input().split()))
G = WeightedGraph(N)
# コストXである辺 添え字は始点
costx = [[] for _ in range(N)]
for i in range(N):
    for j in range(N):
        if i==j:
            continue
        if A[i][j] == -1:
            G.add_edge(i,j,4000000000)
            costx[i].append(len(G.edge[i])-1)
        else:
            G.add_edge(i,j,A[i][j])
dp = [[] for _ in range(N)]
for n in range(N):
    dp[n] = dijkstra(G,n)
inires = count(dp,P)
if inires > K:
    print(str(0))
    sys.exit()
elif inires == K:
    print("Infinity")
    sys.exit()
# X=s => K個以上、X=e => K個未満
s = 0
e=4000000000
while e-s > 1:
    m = (e+s)//2
    for n in range(N):
        for t in costx[n]:
            G.edge[n][t].c = m
    for n in range(N):
        dp[n] = dijkstra(G,n)
    if count(dp,P) >= K:
        s = m
    else:
        e = m
# X=ss => K+1個以上 X=ee => K+1個未満
ss = 0
ee = 4000000000
while ee-ss > 1:
    m = (ee+ss)//2
    for n in range(N):
        for t in costx[n]:
            G.edge[n][t].c = m
    for n in range(N):
        dp[n] = dijkstra(G,n)
    if count(dp,P) >= K+1:
        ss = m
    else:
        ee = m
print(str(s-ss))

