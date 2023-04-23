pub mod bfs {
    
    use std::{io::{stdin, stdout, Write}, collections::{HashSet, VecDeque}};
    use std::usize;

    pub struct Graph {
        ///representation using adjacency list
        pub edges : Vec<Vec<usize>>, 
        ///total no of vertices
        pub vertices: usize, 
    }

    impl Graph {
        pub fn new(vertices: usize) -> Self {
            Graph {
                edges: vec![Vec::new(); vertices],
                vertices,
            }
        }

        ///Adding edges to the graph
        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.edges[u].push(v);
            ///for undirected graphs
            self.edges[v].push(u); 
        }

        ///BFS algorithm
        pub fn b_fs(&self, start: usize) -> Vec<usize> {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            let mut visited_vec = Vec::new();
        
            visited.insert(start);
            queue.push_back(start);
        
            while let Some(u) = queue.pop_front() {
                println!("Visited node: {}", u);
                visited_vec.push(u);
                for &v in &self.edges[u] {
                    if !visited.contains(&v) {
                        visited.insert(v);
                        queue.push_back(v);
                    }
                }
            }
            visited_vec
            
        }
    }   

    pub fn bfs() 
    {
        ///read the number of vertices from the console
        let mut vertex = String::new();
        println!("*****BFS********");
        println!("****************************************************");
        ///get the number of vertices
        print!("Please Enter Number of Vertices : ");
        let _= stdout().flush();
        stdin().read_line(&mut vertex).expect("Enter valid number of vertices");
        let vertices: usize = vertex.trim().parse().expect("Invalid input");
        ///get number of edges in the graph
        let mut n_edges = String::new();
        print!("Please Enter Number of edges in the graph : ");
        let _= stdout().flush();
        stdin().read_line(&mut n_edges).expect("Enter Valid Input");
        let n_edges: i32 = n_edges.trim().parse().expect("Invalid input for source");
        ///assign the vertices to each edge from the console
        let g = add_edges(vertices, n_edges);
        ///get the source vertex
        let mut start = String::new();
        print!("Enter Starting Vertex : ");
        let _= stdout().flush();
        stdin().read_line(&mut start).expect("Enter valid starting vertex ");
        let start: usize = start.trim().parse().expect("Invalid input for starting vertex");
        ///call BFS implementation
        g.b_fs(start);
    }

    ///to return the vertices of each edge as a graph
    pub fn add_edges(vertices: usize, edges: i32) -> Graph
    {   ///intialize a new graph with the required number of vertices
        let mut g = Graph::new(vertices);
        for i in 0..(edges)
        {   ///intialize source and destination
            let mut s = String::new();
            let mut d = String::new();
            ///get the source
            print!("Source : ");
            let _= stdout().flush();
            stdin().read_line(&mut s).expect("Please Enter Valid Input for .");
            let s: usize = s.trim().parse().expect("Invalid input for source");
           ///get the destination
            print!("Destination : ");
            let _= stdout().flush();
            stdin().read_line(&mut d).expect("Please Enter Valid Input for .");
            let d: usize = d.trim().parse().expect("Invalid input for source");
            ///add edge with source and destination
            g.add_edge(s, d);
            ///for undirected graphs
            g.add_edge(d, s); 
        }
        ///return graph in the form containing vertices of the
        return g;
    }
}
#[cfg(test)]
mod tests {
    use super::bfs::*;
    use std::collections::HashSet;

    #[test]
    fn test_new_graph() {
        let g = Graph::new(5);
        assert_eq!(g.vertices, 5);
        assert_eq!(g.edges.len(), 5);
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 4);
        ///Run the BFS algorithm starting from vertex 2
        let visited = g.b_fs(0);

        ///Check that the visited nodes match the expected set
        let expected= vec![0,1,2,3,4];
        assert_eq!(visited, expected);
    }

    #[test]
    fn test_bfs() {
    ///Create a new graph with 5 vertices
    let mut g = Graph::new(5);

    ///Add edges to the graph
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 2);
    g.add_edge(2, 0);
    g.add_edge(2, 3);
    g.add_edge(3, 3);

    ///Run the BFS algorithm starting from vertex 2
    let visited = g.b_fs(2);

    ///Check that the visited nodes match the expected set
    let expected= vec![2, 0, 1, 3];
    assert_eq!(visited, expected);
}

}

