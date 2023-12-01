use cargo_snippet::snippet;

#[snippet("edge")]
#[derive(Clone, Copy)]
struct Edge<T> {
    src: usize,
    dst: usize,
    cost: T,
}

#[snippet("graph")]
#[snippet(include = "edge")]
struct Graph<T> {
    pub edges: Vec<Vec<Edge<T>>>,
}

#[snippet("graph")]
#[snippet(include = "edge")]
impl<T> Graph<T>
where
    T: Copy,
{
    pub fn new(n: usize) -> Graph<T> {
        let mut g = Graph {
            edges: Vec::<Vec<Edge<T>>>::new(),
        };
        for _ in 0..n {
            g.edges.push(Vec::<Edge<T>>::new());
        }
        return g;
    }
    pub fn add_directed_edge(self: &mut Graph<T>, src: usize, dst: usize, cost: T) {
        self.edges[src].push(Edge { src, dst, cost });
    }
    pub fn add_undirected_edge(self: &mut Graph<T>, u: usize, t: usize, cost: T) {
        self.add_directed_edge(u, t, cost);
        self.add_directed_edge(t, u, cost);
    }
    pub fn len(self: &Graph<T>) -> usize {
        return self.edges.len();
    }
}

#[snippet("graph")]
#[snippet(include = "edge")]
impl<T> std::ops::Index<usize> for Graph<T> {
    type Output = Vec<Edge<T>>;

    fn index(&self, index: usize) -> &Self::Output {
        self.edges.get(index).unwrap()
    }
}

#[snippet("graph")]
#[snippet(include = "edge")]
impl<T> std::ops::IndexMut<usize> for Graph<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.edges.get_mut(index).unwrap()
    }
}
