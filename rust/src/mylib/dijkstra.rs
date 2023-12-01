use cargo_snippet::snippet;

#[snippet("dijkstra")]
#[derive(Clone, PartialEq, Eq)]
struct DijkstraState<T> {
    pub index: usize,
    pub priority: T,
}

#[snippet("dijkstra")]
impl<T> PartialOrd for DijkstraState<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[snippet("dijkstra")]
impl<T> Ord for DijkstraState<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

#[snippet("dijkstra")]
#[snippet(include = "graph")]
//(startからの最短距離、最短パスにおける直前の点(startの場合None))のtupleを返す
fn dijkstra<T>(g: &Graph<T>, start: usize, zero: T) -> (Vec<Option<T>>, Vec<Option<usize>>)
where
    T: Copy + Ord + std::ops::Add<T, Output = T>,
{
    let mut dists: Vec<Option<T>> = vec![None; g.len()];
    let mut prevs: Vec<Option<usize>> = vec![None; g.len()];
    //(頂点、距離)
    let mut q = std::collections::BinaryHeap::<DijkstraState<T>>::new();
    q.push(DijkstraState {
        index: start,
        priority: zero,
    });
    dists[start] = Some(zero);
    while let Some(DijkstraState { index, priority }) = q.pop() {
        if dists[index].unwrap_or(zero) < priority {
            continue;
        }
        for e in g[index].iter() {
            let dst_dist = dists[e.dst];
            if dst_dist.is_some() && dst_dist.unwrap_or(zero) <= priority + e.cost {
                continue;
            }
            dists[e.dst] = Some(priority + e.cost);
            prevs[e.dst] = Some(index);
            q.push(DijkstraState {
                index: e.dst,
                priority: priority + e.cost,
            });
        }
    }
    return (dists, prevs);
}
