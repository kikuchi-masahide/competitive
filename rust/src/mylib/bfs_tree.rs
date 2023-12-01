use cargo_snippet::snippet;

#[snippet("deque")]
use std::collections::VecDeque;

#[snippet("bfs_tree")]
#[snippet(include = "graph")]
#[snippet(include = "deque")]
//fに渡される辺は、親から自身に対する辺
fn bfs_tree<T>(g: &Graph<T>, parents: &Vec<usize>, root: usize, mut f: impl FnMut(&Edge<T>) -> ())
where
    T: Copy,
{
    let mut q = VecDeque::<usize>::new();
    q.push_back(root);
    while !q.is_empty() {
        let index = match q.pop_front() {
            Some(v) => v,
            None => usize::MAX,
        };
        for v in g[index].iter() {
            let c = v.dst;
            if c != parents[index] {
                f(v);
                q.push_back(c);
            }
        }
    }
}
