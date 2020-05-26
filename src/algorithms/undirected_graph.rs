use crate::algorithms::linked_list_queue::LinkedQueue;
use crate::algorithms::linked_list_stack::LinkedStack;

struct UGraph {
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

struct ConnectedComponents {
    marked: Vec<bool>,
    id: Vec<Option<usize>>,
    pub count: usize,
}

impl UGraph {
    pub fn new(num_vertices: usize) -> Self {
        let mut edges = Vec::with_capacity(num_vertices);

        for v in 0..num_vertices {
            edges.insert(v, LinkedStack::new());
        }

        UGraph {
            num_vertices,
            edges,
        }
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.edges[v].push(w);
        self.edges[w].push(v);
    }

    pub fn get_adj_edges<'a>(&'a self, v: usize) -> &'a LinkedStack<usize> {
        &self.edges[v]
    }

    pub fn get_adj_edges_mut<'a>(&'a mut self, v: usize) -> &'a mut LinkedStack<usize> {
        &mut self.edges[v]
    }
}

impl DFSPaths {
    pub fn new(g: &UGraph, origin: usize) -> Self {
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

    fn dfs(&mut self, g: &UGraph, v: usize) {
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
    pub fn new(g: &UGraph, origin: usize) -> Self {
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

    fn bfs(&mut self, g: &UGraph, v: usize) {
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

impl ConnectedComponents {
    pub fn new(g: &UGraph) -> Self {
        let mut cc = ConnectedComponents {
            marked: vec![false; g.num_vertices],
            id: vec![None; g.num_vertices],
            count: 0,
        };

        for v in 0..g.num_vertices {
            if !cc.marked[v] {
                cc.dfs(g, v);
                cc.count = cc.count + 1;
            }
        }

        cc
    }

    pub fn id(&self, v: usize) -> Option<usize> {
        self.id[v]
    }

    fn dfs(&mut self, g: &UGraph, v: usize) {
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
    use crate::algorithms::linked_list_stack::LinkedStack;
    use crate::algorithms::undirected_graph::{BFSPaths, ConnectedComponents, DFSPaths, UGraph};

    #[test]
    fn dfs_interface_operations_should_work_as_expected() {
        let mut g = UGraph::new(10);

        g.add_edge(0, 1);

        g.add_edge(0, 2);

        g.add_edge(0, 6);
        g.add_edge(6, 4);
        g.add_edge(4, 5);
        g.add_edge(5, 3);

        g.add_edge(7, 8);

        let paths = DFSPaths::new(&g, 0);

        assert!(paths.has_path_to(0));
        assert!(paths.has_path_to(6));
        assert!(paths.has_path_to(4));
        assert!(paths.has_path_to(5));
        assert!(paths.has_path_to(3));

        let path = paths.path_to(3).unwrap();
        let mut expected_verts = LinkedStack::<usize>::new();

        expected_verts.push(5);
        expected_verts.push(4);
        expected_verts.push(6);
        expected_verts.push(0);

        for v in path.iter() {
            println!("{}", v);
            assert!(*v == expected_verts.pop().unwrap());
        }

        assert!(!paths.has_path_to(8));
    }

    #[test]
    fn bfs_interface_operations_should_work_as_expected() {
        let mut g = UGraph::new(10);

        g.add_edge(0, 1);

        g.add_edge(0, 2);

        g.add_edge(0, 6);
        g.add_edge(6, 4);
        g.add_edge(4, 5);
        g.add_edge(5, 3);

        g.add_edge(7, 8);

        let paths = BFSPaths::new(&g, 0);

        assert!(paths.has_path_to(0));
        assert!(paths.has_path_to(6));
        assert!(paths.has_path_to(4));
        assert!(paths.has_path_to(5));
        assert!(paths.has_path_to(3));

        let path = paths.path_to(3).unwrap();
        let mut expected_verts = LinkedStack::<usize>::new();

        expected_verts.push(5);
        expected_verts.push(4);
        expected_verts.push(6);
        expected_verts.push(0);

        for v in path.iter() {
            println!("{}", v);
            assert!(*v == expected_verts.pop().unwrap());
        }

        assert!(!paths.has_path_to(8));
    }

    #[test]
    fn cc_interface_operations_should_work_as_expected() {
        let mut g = UGraph::new(17);

        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 6);
        g.add_edge(6, 4);
        g.add_edge(4, 5);
        g.add_edge(5, 3);

        g.add_edge(7, 8);

        g.add_edge(9, 10);
        g.add_edge(9, 11);
        g.add_edge(9, 12);
        g.add_edge(11, 12);

        g.add_edge(13, 14);
        g.add_edge(13, 15);
        g.add_edge(13, 16);
        g.add_edge(15, 16);
        g.add_edge(14, 15);

        let cc = ConnectedComponents::new(&g);

        assert!(cc.count == 4);

        assert!(cc.id(0) == Some(0));
        assert!(cc.id(5) == Some(0));
        assert!(cc.id(6) == Some(0));
        assert!(cc.id(4) == Some(0));
        assert!(cc.id(7) == Some(1));
        assert!(cc.id(9) == Some(2));
        assert!(cc.id(10) == Some(2));
        assert!(cc.id(11) == Some(2));
        assert!(cc.id(13) == Some(3));
        assert!(cc.id(14) == Some(3));
        assert!(cc.id(16) == Some(3));
    }
}
