use std::collections::HashSet;

struct Vertex<T>{
    id: i64,
    value: T,
}
struct Edge<T>{
    id_a: i64,
    id_b: i64,
    value: T,
}

// Graphs are not oriented by default
struct Graph<V, E>{
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>,
    ids: HashSet<i64>,
    vertex_ids: HashSet<(i64, i64)>,
}

impl<V, E> Graph<V, E>{
    pub fn nb_vertices(&self) -> usize{
        self.vertices.len()
    }

    pub fn nb_edges(&self) -> usize{
        self.edges.len()
    }

    pub fn add_vertex(&mut self, id: i64, value: V){
        if self.ids.contains(&id){
            return;
        }
        self.vertices.push(Vertex::<V>{
            id: id,
            value: value
        });
        self.ids.insert(id);
    }

    pub fn add_edge(&mut self, id_a: i64, id_b: i64, value: E){
        if self.ids.contains(&id_a) && self.ids.contains(&id_b) && !self.vertex_ids.contains(&(id_a, id_b)){
            self.edges.push(Edge::<E>{
                id_a: id_a,
                id_b: id_b,
                value: value
            });
            self.vertex_ids.insert((id_a, id_b));
        }
    }

    pub fn get_connected_component(&self, id: i64) -> Vec<i64>{
        let mut cc = Vec::new();
        let mut cc_set = HashSet::<i64>::new();
        
        if !self.ids.contains(&id){
            return cc;
        }
        //Not opti, potentially O(|V||E|)
        self.dfs_cc(&mut cc, &mut cc_set, id);

        cc
    }

    pub fn dfs_cc(&self, cc: &mut Vec<i64>, cc_set: &mut HashSet<i64>, id: i64){
        if cc_set.contains(&id){
            return;
        }
        cc_set.insert(id);
        cc.push(id);

        for edge in self.edges.iter(){
            if edge.id_a == id{
                self.dfs_cc(cc, cc_set, edge.id_b);
            }
        }
    }
}

impl<V, E> Default for Graph<V, E>{
    fn default() -> Self{
        Graph::<V, E>{
            vertices: Vec::new(),
            edges: Vec::new(),
            ids: HashSet::new(),
            vertex_ids: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb() {
        let g = Graph::<i64, i64>::default();

        assert_eq!(g.nb_vertices(), 0);
        assert_eq!(g.nb_edges(), 0);
    }

    #[test]
    fn test_cc() {
        let mut g = Graph::<i64, i64>::default();

        g.add_vertex(0, 0);
        g.add_vertex(1, 0);
        g.add_vertex(2, 0);

        g.add_edge(0, 1, 1);

        let cc = g.get_connected_component(0);

        assert_eq!(cc.len(), 2);

        let cc = g.get_connected_component(-1);
        
        assert_eq!(cc.len(), 0);
    }
}
