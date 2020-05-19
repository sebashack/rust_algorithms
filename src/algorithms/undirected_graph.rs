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
            marked: Vec::new(),
            edge_to: Vec::new(),
            origin,
        };

        for v in 0..g.num_vertices {
            paths.marked.push(false);
            paths.edge_to.push(None);
        }

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

#[cfg(test)]
mod tests {
    use crate::algorithms::linked_list_stack::LinkedStack;
    use crate::algorithms::undirected_graph::{DFSPaths, UGraph};

    #[test]
    fn interface_operations_should_work_as_expected() {
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
}
