pub mod dfs {
    
    use std::{io::{stdin, stdout, Write}, collections::HashSet};
    use std::usize;

    struct Graph {
        ///representation using adjacency list
        edges : Vec<Vec<usize>>, 
        ///total no of vertices
        vertices: usize, 
    }

    impl Graph {
        fn new(vertices: usize) -> Self {
            Graph {
                edges: vec![Vec::new(); vertices],
                vertices,
            }
        }

        ///Adding edges to the graph
        fn add_edge(&mut self, u: usize, v: usize) {
            self.edges[u].push(v);
        }

        ///DFS algorithm
        fn dfs(&self, u: usize, visited: &mut HashSet<usize>) {
            visited.insert(u);
            println!("Visited node: {}", u);

            for &v in &self.edges[u] {
                if !visited.contains(&v) {
                    self.dfs(v, visited);
                }
            }
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
        g.dfs(source, &mut visited);
    }

    ///to return the vertices of each edge as a graph
    fn add_edges(vertices: usize, edges: i32) -> Graph
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