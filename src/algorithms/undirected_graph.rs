use crate::algorithms::linked_list_queue::LinkedQueue;

struct UGraph {
    num_vertices: usize,
    edges: Vec<LinkedQueue<usize>>,
}

impl UGraph {
    pub fn new(num_vertices: usize) -> Self {
        let mut edges = Vec::with_capacity(num_vertices);

        for v in 0..num_vertices {
            edges.insert(v, LinkedQueue::new());
        }

        UGraph {
            num_vertices,
            edges,
        }
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.edges[v].enqueue(w);
        self.edges[w].enqueue(v);
    }

    pub fn get_adj_edges<'a>(&'a self, v: usize) -> &'a LinkedQueue<usize> {
        &self.edges[v]
    }

    pub fn get_adj_edges_mut<'a>(&'a mut self, v: usize) -> &'a mut LinkedQueue<usize> {
        &mut self.edges[v]
    }
}
