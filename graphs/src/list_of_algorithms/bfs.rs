pub mod bfs {
    
    use std::{io::{stdin, stdout, Write}, collections::{HashSet, VecDeque}};
    use std::usize;

    struct Graph {
        edges : Vec<Vec<usize>>, //representation using adjacency list
        vertices: usize, //total no of vertices
    }

    impl Graph {
        fn new(vertices: usize) -> Self {
            Graph {
                edges: vec![Vec::new(); vertices],
                vertices,
            }
        }

        //Adding edges to the graph
        fn add_edge(&mut self, u: usize, v: usize) {
            self.edges[u].push(v);
            self.edges[v].push(u); // for undirected graphs
        }

        //BFS algorithm
        fn bfs(&self, start: usize) {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();

            visited.insert(start);
            queue.push_back(start);

            while let Some(u) = queue.pop_front() {
                println!("Visited node: {}", u);

                for &v in &self.edges[u] {
                    if !visited.contains(&v) {
                        visited.insert(v);
                        queue.push_back(v);
                    }
                }
            }
        }
    }   

    pub fn bfs() 
    {
        //read the number of vertices from the console
        let mut vertex = String::new();
        println!("*****BFS********");
        println!("****************************************************");
        //get the number of vertices
        print!("Please Enter Number of Vertices : ");
        let _= stdout().flush();
        stdin().read_line(&mut vertex).expect("Enter valid number of vertices");
        let vertices: usize = vertex.trim().parse().expect("Invalid input");
        //get number of edges in the graph
        let mut n_edges = String::new();
        print!("Please Enter Number of edges in the graph : ");
        let _= stdout().flush();
        stdin().read_line(&mut n_edges).expect("Enter Valid Input");
        let n_edges: i32 = n_edges.trim().parse().expect("Invalid input for source");
        //assign the vertices to each edge from the console
        let g = add_edges(vertices, n_edges);
        //get the source vertex
        let mut start = String::new();
        print!("Enter Starting Vertex : ");
        let _= stdout().flush();
        stdin().read_line(&mut start).expect("Enter valid starting vertex ");
        let start: usize = start.trim().parse().expect("Invalid input for starting vertex");
        //call BFS implementation
        g.bfs(start);
    }

    //to return the vertices of each edge as a graph
    fn add_edges(vertices: usize, edges: i32) -> Graph
    {   //intialize a new graph with the required number of vertices
        let mut g = Graph::new(vertices);
        for i in 0..(edges)
        {   //intialize source and destination
            let mut s = String::new();
            let mut d = String::new();
            //get the source
            print!("Source : ");
            let _= stdout().flush();
            stdin().read_line(&mut s).expect("Please Enter Valid Input for .");
            let s: usize = s.trim().parse().expect("Invalid input for source");
           //get the destination
            print!("Destination : ");
            let _= stdout().flush();
            stdin().read_line(&mut d).expect("Please Enter Valid Input for .");
            let d: usize = d.trim().parse().expect("Invalid input for source");
            // add edge with source and destination
            g.add_edge(s, d);
            g.add_edge(d, s); // for undirected graphs
        }
        return g;
        //return graph in the form containing vertices of the
    }
}