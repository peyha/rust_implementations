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

struct Graph<V, E>{
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>,
    ids: HashSet<i64>,
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb() {
        let g = Graph::<i64, i64>{
            vertices: Vec::new(),
            edges: Vec::new(),
            ids: HashSet::new(),
        };

        assert_eq!(g.nb_vertices(), 0);
        assert_eq!(g.nb_edges(), 0);
    }
}
