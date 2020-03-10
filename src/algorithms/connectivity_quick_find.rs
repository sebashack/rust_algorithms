#[repr(transparent)]
pub struct Network(Vec<usize>);

impl Network {
    pub fn new(num_elts: usize) -> Network {
        let mut ids = Vec::with_capacity(num_elts);

        for id in 0..num_elts {
            ids.push(id);
        }

        Network(ids)
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.0[p] == self.0[q]
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pid = self.0[p];
        let qid = self.0[q];

        for i in 0..self.0.len() {
            if self.0[i] == pid {
                self.0[i] = qid;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::connectivity_quick_find::Network;

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
