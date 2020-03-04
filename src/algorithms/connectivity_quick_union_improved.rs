pub struct Network {
    tree: Vec<usize>,
    weights: Vec<usize>,
}

impl Network {
    pub fn new(num_elts: usize) -> Network {
        let mut tree = Vec::with_capacity(num_elts);
        let mut weights = Vec::with_capacity(num_elts);

        for id in 0..(num_elts) {
            tree.push(id);
            weights.push(0)
        }

        Network { tree, weights }
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.root(p);
        let root_q = self.root(q);

        if root_p == root_q { return; }

        if self.weights[root_p] < self.weights[root_q] {
            self.tree[root_p] = self.tree[root_q];
            self.weights[root_q] += self.weights[root_p];
        } else {
            self.tree[root_q] = self.tree[root_p];
            self.weights[root_p] += self.weights[root_q];
        }
    }

    fn root(&mut self, node: usize) -> usize {
        let mut current_node = node;

        while current_node != self.tree[current_node] {
            self.tree[current_node] = self.tree[self.tree[current_node]];

            current_node = self.tree[current_node];
        }

        current_node
    }
}

#[cfg(test)]
mod tests {
    use  crate::algorithms::connectivity_quick_union_improved::Network;

    #[test]
    fn correctly_determines_connections() {
        let mut network = Network::new(100);

        assert!(network.connected(0, 0));
        assert!(network.connected(50, 50));
        assert!(network.connected(99, 99));

        assert!(!network.connected(0, 1));
        assert!(!network.connected(0, 99));

        network.union(0, 10);

        assert!(network.connected(0, 10));

        network.union(10, 20);
        network.union(20, 30);

        assert!(network.connected(10, 20));
        assert!(network.connected(20, 10));
        assert!(network.connected(10, 30));

        network.union(10, 40);
        network.union(30, 40);

        assert!(network.connected(10, 40));
        assert!(network.connected(20, 40));

        network.union(50, 90);
        network.union(50, 55);
        network.union(55, 70);
        network.union(90, 70);

        assert!(!network.connected(0, 55));
        assert!(!network.connected(10, 70));
        assert!(!network.connected(20, 90));
        assert!(!network.connected(30, 50));
        assert!(!network.connected(40, 55));

        network.union(30, 50);

        assert!(network.connected(0, 55));
        assert!(network.connected(10, 70));
        assert!(network.connected(20, 90));
        assert!(network.connected(30, 50));
        assert!(network.connected(40, 55));
    }
}
