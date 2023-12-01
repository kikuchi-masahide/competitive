use cargo_snippet::snippet;
use std::collections::VecDeque;

#[snippet("construct_tree")]
#[snippet(include = "graph")]
#[snippet(include = "deque")]
fn construct_tree<T>(g: &Graph<T>, root: usize) -> Vec<usize>
where
    T: Copy,
{
    let mut parents = vec![usize::MAX; g.len()];
    let mut q = std::collections::VecDeque::<usize>::new();
    q.push_back(root);
    while !q.is_empty() {
        let index = match q.pop_front() {
            Some(v) => v,
            None => usize::MAX,
        };
        for v in g[index].iter() {
            if v.dst != parents[index] {
                parents[v.dst] = index;
                q.push_back(v.dst);
            }
        }
    }
    return parents;
}
