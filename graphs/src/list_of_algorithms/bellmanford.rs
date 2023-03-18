pub mod bellmanford {
    use std::{io::{stdin, stdout, Write}, collections::{HashSet, BinaryHeap}, cmp::Ordering};
    use std::usize;
    #[derive(Clone, Eq, PartialEq, PartialOrd)]
    struct Node {
        vertex: usize,
        dist: i32,
    }
    struct Graph {
        adj_list: Vec<Vec<Node>>, //adjacency list representation using node structure
        vertices: usize, //total no of vertices
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            other.dist.cmp(&self.dist)
        }
    }
    
    impl Graph {
        fn new(vertices: usize) -> Self {
            Graph {
                adj_list: vec![Vec::new(); vertices],
                vertices,
            }
        }
    
        //Adding edges to the graph
        fn add_edge(&mut self, u: usize, v: usize, w: i32) {
            self.adj_list[u].push(Node { vertex: v, dist: w });
            self.adj_list[v].push(Node { vertex: u, dist: w });
        }
         fn bellman_ford(&self, src: usize)
         {
            //implement bellmanford
            
         }
    }   
    pub fn bellmanford() 
    {
        //read the number of vertices and source from the console
        let mut vertex = String::new();
        let mut source = String::new();
        println!("**********Bellman Ford*************");
        println!("****************************************************");
        print!("Please Enter Number of Vertices : ");
        let _= stdout().flush();
        stdin().read_line(&mut vertex).expect("Enter valid number of vertices");
        let vertices: usize = vertex.trim().parse().expect("Invalid input");
        print!("Enter Source Vertex : ");
        let _= stdout().flush();
        stdin().read_line(&mut source).expect("Enter valid source vertex ");
        let source: usize = source.trim().parse().expect("Invalid input for source");
        //reading number of edges in the graph
        let mut n_edges = String::new();
        print!("Please Enter Number of edges in the graph : ");
        let _= stdout().flush();
        stdin().read_line(&mut n_edges).expect("Enter Valid Input");
        let n_edges: i32 = n_edges.trim().parse().expect("Invalid input for source");
        //assign the weights to each edge from the console
        let mut e= add_weights(vertices, source, n_edges);
        //print!("If you want add edges press 1")
        // add new edges if the user want to add more edges at this point
        //implementation of bellman ford algorithm
    }

     fn add_weights(vertices: usize, source: usize, edges: i32) -> Graph
    {   let mut g = Graph::new(vertices);
        let mut cnt = 0;
        //let mut weighted_edges = vec![];
        for i in 0..(edges)
        {   
            let mut s = String::new();
            let mut d = String::new();
            let mut w = String::new();
            print!("Source : ");
            let _= stdout().flush();
            stdin().read_line(&mut s).expect("Please Enter Valid Input for .");
            let s: usize = s.trim().parse().expect("Invalid input for source");
            print!("Destination : ");
            let _= stdout().flush();
            stdin().read_line(&mut d).expect("Please Enter Valid Input for .");
            let d: usize = d.trim().parse().expect("Invalid input for source");
            print!("Weight : ");
            let _= stdout().flush();
            stdin().read_line(&mut w).expect("Please Enter Valid Input for .");
            let w: i32 = w.trim().parse().expect("Invalid input for source");
            //weighted_edges.push((s, d, w));
            // add edge code goes here
            g.add_edge(s, d, w);
            cnt = cnt + 1;
         }
        return g;
         //return vector in the form containing source,destination and weight of the edge
    }
}
