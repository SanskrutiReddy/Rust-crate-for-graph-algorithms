pub mod dfs {
    //Importng necessary libraries
    use std::{io::{stdin, stdout, Write}, collections::HashSet};
    use std::usize;

    pub struct Graph {
        ///representation using adjacency list
        pub edges : Vec<Vec<usize>>, 
        ///total no of vertices
        pub vertices: usize, 
    }

    impl Graph {
        // Constructor method for creating a new graph
        pub fn new(vertices: usize) -> Self {
            Graph {
                edges: vec![Vec::new(); vertices], // Initializing adjacency list with empty vectors with size equal to vertices number
                vertices,
            }
        }


        ///Adding edges to the graph
        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.edges[u].push(v);
        }

        ///DFS algorithm
        pub fn d_fs(&self, u: usize, visited: &mut HashSet<usize>) -> Vec<usize> {
            visited.insert(u);
            println!("Visited node: {}", u);
            let mut visited_nodes = vec![u];
        
            for &v in &self.edges[u] {
                if !visited.contains(&v) {
                    visited_nodes.extend(self.d_fs(v, visited));
                }
            }
        
            visited_nodes
        }
    }   

    pub fn dfs() 
    {
        ///read the number of vertices from the console
        let mut vertex = String::new();
        println!("******DFS Traversal*********");
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
        let mut source = String::new();
        print!("Enter Source Vertex : ");
        let _= stdout().flush();
        stdin().read_line(&mut source).expect("Enter valid source vertex ");
        let source: usize = source.trim().parse().expect("Invalid input for source");

        ///call DFS implementation

        let mut visited = HashSet::new();
        g.d_fs(source, &mut visited);
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
            /// add edge with source and destination
            g.add_edge(s, d);
            g.add_edge(d, s); // for undirected graphs
        }
        ///return graph in the form containing vertices of the
        return g;
    
    }
}
#[cfg(test)]
mod tests {
    use super::dfs::*;
    use std::collections::HashSet;

    #[test]
    fn test_dfs() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        g.add_edge(2, 3);
        g.add_edge(3, 3);

        let mut visited = HashSet::new();
        let x = g.d_fs(2, &mut visited);
        ///Check that the visited nodes match the expected set
        let expected= vec![2, 0, 1, 3];
        assert_eq!(x, expected);
        assert!(visited.contains(&0));
        assert!(visited.contains(&1));
        assert!(visited.contains(&2));
        assert!(visited.contains(&3));
        assert_eq!(visited.len(), 4);
    }
    #[test]
    fn test_dfs1() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 4);

        let mut visited = HashSet::new();
        let x = g.d_fs(0, &mut visited);
        ///Check that the visited nodes match the expected set
        let expected= vec![0,1,3,2,4];
        assert_eq!(x, expected);
        assert!(visited.contains(&0));
        assert!(visited.contains(&1));
        assert!(visited.contains(&2));
        assert!(visited.contains(&3));
        assert!(visited.contains(&4));
        assert_eq!(visited.len(), 5);
    }
}
