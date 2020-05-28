use crate::algorithms::linked_list_queue::LinkedQueue;
use crate::algorithms::linked_list_stack::LinkedStack;

struct DGraph {
    num_vertices: usize,
    edges: Vec<LinkedStack<usize>>,
}

struct DFSPaths {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    origin: usize,
}

struct BFSPaths {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    origin: usize,
}

struct TopologicalSort {
    marked: Vec<bool>,
    reverse_post: LinkedStack<usize>,
}

struct DirectedCycle {
    marked: Vec<bool>,
    on_stack: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    cycle: LinkedStack<usize>,
}

struct KosarajuSharirSCC {
    marked: Vec<bool>,
    id: Vec<Option<usize>>,
    pub count: usize,
}

impl DGraph {
    pub fn new(num_vertices: usize) -> Self {
        let mut edges = Vec::with_capacity(num_vertices);

        for v in 0..num_vertices {
            edges.insert(v, LinkedStack::new());
        }

        DGraph {
            num_vertices,
            edges,
        }
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.edges[v].push(w);
    }

    pub fn get_adj_edges<'a>(&'a self, v: usize) -> &'a LinkedStack<usize> {
        &self.edges[v]
    }

    pub fn get_adj_edges_mut<'a>(&'a mut self, v: usize) -> &'a mut LinkedStack<usize> {
        &mut self.edges[v]
    }

    pub fn reverse(&self) -> DGraph {
        let mut edges = Vec::with_capacity(self.num_vertices);

        for v in 0..self.num_vertices {
            edges.insert(v, LinkedStack::new());
        }

        for v in 0..self.num_vertices {
            for w in self.edges[v].iter() {
                edges[*w].push(v);
            }
        }

        DGraph {
            num_vertices: self.num_vertices,
            edges,
        }
    }
}

impl DFSPaths {
    pub fn new(g: &DGraph, origin: usize) -> Self {
        let mut paths = DFSPaths {
            marked: vec![false; g.num_vertices],
            edge_to: vec![None; g.num_vertices],
            origin,
        };

        paths.dfs(g, origin);

        paths
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Option<LinkedStack<usize>> {
        if self.has_path_to(v) {
            let mut path = LinkedStack::new();

            let mut x = self.edge_to[v].unwrap();
            while x != self.origin {
                path.push(x);
                x = self.edge_to[x].unwrap();
            }

            path.push(self.origin);

            Some(path)
        } else {
            None
        }
    }

    fn dfs(&mut self, g: &DGraph, v: usize) {
        self.marked[v] = true;

        for w in g.get_adj_edges(v).iter() {
            if !self.marked[*w] {
                self.dfs(g, *w);
                self.edge_to[*w] = Some(v);
            }
        }
    }
}

impl BFSPaths {
    pub fn new(g: &DGraph, origin: usize) -> Self {
        let mut paths = BFSPaths {
            marked: vec![false; g.num_vertices],
            edge_to: vec![None; g.num_vertices],
            origin,
        };

        paths.bfs(g, origin);

        paths
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Option<LinkedStack<usize>> {
        if self.has_path_to(v) {
            let mut path = LinkedStack::new();

            let mut x = self.edge_to[v].unwrap();
            while x != self.origin {
                path.push(x);
                x = self.edge_to[x].unwrap();
            }

            path.push(self.origin);

            Some(path)
        } else {
            None
        }
    }

    fn bfs(&mut self, g: &DGraph, v: usize) {
        let mut q = LinkedQueue::new();

        q.enqueue(self.origin);
        self.marked[self.origin] = true;

        while !q.is_empty() {
            let v = q.dequeue().unwrap();

            for w in g.get_adj_edges(v).iter() {
                if !self.marked[*w] {
                    q.enqueue(*w);
                    self.marked[*w] = true;
                    self.edge_to[*w] = Some(v);
                }
            }
        }
    }
}

impl TopologicalSort {
    pub fn new(g: &DGraph) -> Self {
        let mut ts = TopologicalSort {
            marked: vec![false; g.num_vertices],
            reverse_post: LinkedStack::new(),
        };

        for v in 0..g.num_vertices {
            if !ts.marked[v] {
                ts.dfs(g, v);
            }
        }

        ts
    }

    pub fn reverse_post(&self) -> &LinkedStack<usize> {
        &self.reverse_post
    }

    pub fn reverse_post_mut(&mut self) -> &mut LinkedStack<usize> {
        &mut self.reverse_post
    }

    fn dfs(&mut self, g: &DGraph, v: usize) {
        self.marked[v] = true;

        for w in g.get_adj_edges(v).iter() {
            if !self.marked[*w] {
                self.dfs(g, *w);
            }
        }

        self.reverse_post.push(v);
    }
}

impl DirectedCycle {
    pub fn new(g: &DGraph) -> Self {
        let mut dc = DirectedCycle {
            marked: vec![false; g.num_vertices],
            on_stack: vec![false; g.num_vertices],
            edge_to: vec![None; g.num_vertices],
            cycle: LinkedStack::<usize>::new(),
        };

        println!("Bitch please {}", dc.cycle.is_empty());

        for v in 0..g.num_vertices {
            if !dc.marked[v] {
                dc.dfs(g, v);
            }
        }

        dc
    }

    pub fn has_cycle(&self) -> bool {
        !self.cycle.is_empty()
    }

    pub fn cycle(&self) -> &LinkedStack<usize> {
        &self.cycle
    }

    pub fn cycle_mut(&mut self) -> &mut LinkedStack<usize> {
        &mut self.cycle
    }

    fn dfs(&mut self, g: &DGraph, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;

        for w in g.get_adj_edges(v).iter() {
            if self.has_cycle() {
                return;
            }

            if !self.marked[*w] {
                self.edge_to[*w] = Some(v);
                self.dfs(g, *w);
            } else if self.on_stack[*w] {
                let mut x = self.edge_to[v].unwrap();
                while x != *w {
                    self.cycle.push(x);
                    x = self.edge_to[x].unwrap();
                }

                self.cycle.push(*w);
                self.cycle.push(v);
            }
        }

        self.on_stack[v] = false;
    }
}

impl KosarajuSharirSCC {
    pub fn new(g: &DGraph) -> Self {
        let mut cc = KosarajuSharirSCC {
            marked: vec![false; g.num_vertices],
            id: vec![None; g.num_vertices],
            count: 0,
        };

        let top_sort = TopologicalSort::new(&g.reverse());

        for v in top_sort.reverse_post().iter() {
            if !cc.marked[*v] {
                cc.dfs(g, *v);
                cc.count = cc.count + 1;
            }
        }

        cc
    }

    pub fn id(&self, v: usize) -> Option<usize> {
        self.id[v]
    }

    pub fn strongly_connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    fn dfs(&mut self, g: &DGraph, v: usize) {
        self.marked[v] = true;
        self.id[v] = Some(self.count);

        for w in g.get_adj_edges(v).iter() {
            if !self.marked[*w] {
                self.dfs(g, *w);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::directed_graph::{
        BFSPaths, DFSPaths, DGraph, DirectedCycle, KosarajuSharirSCC, TopologicalSort,
    };
    use crate::algorithms::linked_list_stack::LinkedStack;

    #[test]
    fn dfs_interface_operations_should_work_as_expected() {
        let mut g = DGraph::new(13);

        g.add_edge(4, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 2);
        g.add_edge(6, 0);
        g.add_edge(0, 1);
        g.add_edge(2, 0);
        g.add_edge(11, 12);
        g.add_edge(12, 9);
        g.add_edge(9, 10);
        g.add_edge(9, 11);
        g.add_edge(7, 9);
        g.add_edge(10, 12);
        g.add_edge(11, 4);
        g.add_edge(4, 3);
        g.add_edge(3, 5);
        g.add_edge(6, 8);
        g.add_edge(8, 6);
        g.add_edge(5, 4);
        g.add_edge(0, 5);
        g.add_edge(6, 4);
        g.add_edge(6, 9);
        g.add_edge(7, 6);

        let paths = DFSPaths::new(&g, 0);

        assert!(paths.has_path_to(0));
        assert!(paths.has_path_to(1));
        assert!(paths.has_path_to(2));
        assert!(paths.has_path_to(3));
        assert!(paths.has_path_to(4));

        let paths = DFSPaths::new(&g, 9);

        assert!(paths.has_path_to(9));
        assert!(paths.has_path_to(10));
        assert!(paths.has_path_to(11));
        assert!(paths.has_path_to(12));

        let paths = DFSPaths::new(&g, 7);

        assert!(paths.has_path_to(7));
        assert!(paths.has_path_to(6));
        assert!(paths.has_path_to(8));
        assert!(paths.has_path_to(0));
        assert!(paths.has_path_to(9));
        assert!(paths.has_path_to(10));
        assert!(paths.has_path_to(11));
        assert!(paths.has_path_to(12));
    }

    #[test]
    fn bfs_interface_operations_should_work_as_expected() {
        let mut g = DGraph::new(13);

        g.add_edge(4, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 2);
        g.add_edge(6, 0);
        g.add_edge(0, 1);
        g.add_edge(2, 0);
        g.add_edge(11, 12);
        g.add_edge(12, 9);
        g.add_edge(9, 10);
        g.add_edge(9, 11);
        g.add_edge(7, 9);
        g.add_edge(10, 12);
        g.add_edge(11, 4);
        g.add_edge(4, 3);
        g.add_edge(3, 5);
        g.add_edge(6, 8);
        g.add_edge(8, 6);
        g.add_edge(5, 4);
        g.add_edge(0, 5);
        g.add_edge(6, 4);
        g.add_edge(6, 9);
        g.add_edge(7, 6);

        let paths = BFSPaths::new(&g, 0);

        assert!(paths.has_path_to(0));
        assert!(paths.has_path_to(1));
        assert!(paths.has_path_to(2));
        assert!(paths.has_path_to(3));
        assert!(paths.has_path_to(4));

        let paths = BFSPaths::new(&g, 9);

        assert!(paths.has_path_to(9));
        assert!(paths.has_path_to(10));
        assert!(paths.has_path_to(11));
        assert!(paths.has_path_to(12));

        let paths = BFSPaths::new(&g, 7);

        assert!(paths.has_path_to(7));
        assert!(paths.has_path_to(6));
        assert!(paths.has_path_to(8));
        assert!(paths.has_path_to(0));
        assert!(paths.has_path_to(9));
        assert!(paths.has_path_to(10));
        assert!(paths.has_path_to(11));
        assert!(paths.has_path_to(12));
    }

    #[test]
    fn it_creates_topological_sort_on_gad() {
        let mut g = DGraph::new(7);

        g.add_edge(0, 5);
        g.add_edge(0, 2);
        g.add_edge(0, 1);
        g.add_edge(3, 6);
        g.add_edge(3, 5);
        g.add_edge(3, 4);
        g.add_edge(5, 2);
        g.add_edge(6, 4);
        g.add_edge(6, 0);
        g.add_edge(3, 2);
        g.add_edge(1, 4);

        let mut paths = TopologicalSort::new(&g);
        let mut sorting = paths.reverse_post_mut();

        assert!(sorting.pop() == Some(3));
        assert!(sorting.pop() == Some(6));
        assert!(sorting.pop() == Some(0));
        assert!(sorting.pop() == Some(5));
        assert!(sorting.pop() == Some(2));
        assert!(sorting.pop() == Some(1));
        assert!(sorting.pop() == Some(4));
    }

    #[test]
    fn it_detects_directed_cycles() {
        let mut g = DGraph::new(6);

        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(1, 3);
        g.add_edge(1, 4);
        g.add_edge(4, 3);

        let paths = DirectedCycle::new(&g);

        assert!(!paths.has_cycle());

        let mut g = DGraph::new(5);

        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(1, 3);
        g.add_edge(3, 1);
        g.add_edge(1, 4);
        g.add_edge(4, 3);

        let mut paths = DirectedCycle::new(&g);

        assert!(paths.has_cycle());

        let mut c = paths.cycle_mut();

        assert!(c.pop() == Some(3));
        assert!(c.pop() == Some(1));
        assert!(c.pop() == Some(4));
    }

    #[test]
    fn it_strongly_connected_components() {
        let mut g = DGraph::new(13);

        g.add_edge(4, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 2);
        g.add_edge(6, 0);
        g.add_edge(0, 1);
        g.add_edge(2, 0);
        g.add_edge(11, 12);
        g.add_edge(12, 9);
        g.add_edge(9, 10);
        g.add_edge(9, 11);
        g.add_edge(7, 9);
        g.add_edge(10, 12);
        g.add_edge(11, 4);
        g.add_edge(4, 3);
        g.add_edge(3, 5);
        g.add_edge(6, 8);
        g.add_edge(8, 6);
        g.add_edge(5, 4);
        g.add_edge(0, 5);
        g.add_edge(6, 4);
        g.add_edge(6, 9);
        g.add_edge(7, 6);

        let mut cc = KosarajuSharirSCC::new(&g);

        assert!(!cc.strongly_connected(0, 1));
        assert!(!cc.strongly_connected(2, 6));
        assert!(!cc.strongly_connected(6, 9));
        assert!(!cc.strongly_connected(7, 6));
        assert!(!cc.strongly_connected(7, 9));

        assert!(cc.strongly_connected(0, 2));
        assert!(cc.strongly_connected(0, 3));
        assert!(cc.strongly_connected(0, 4));
        assert!(cc.strongly_connected(0, 5));
        assert!(cc.strongly_connected(2, 3));
        assert!(cc.strongly_connected(2, 4));

        assert!(cc.strongly_connected(6, 8));

        assert!(cc.strongly_connected(9, 10));
        assert!(cc.strongly_connected(9, 12));
    }
}
