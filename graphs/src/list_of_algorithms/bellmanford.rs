pub mod bellmanford {
    
    use std::{io::{stdin, stdout, Write}, collections::{HashSet, BinaryHeap}, cmp::Ordering};
    use std::usize;
    #[derive(Clone, Eq, PartialEq, PartialOrd)]
    struct Node {
        vertex: usize,
        dist: i32,
    }
    struct Graph {
        edges : Vec<(usize, usize, i32)>, //representation using edge list
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
                edges: Vec::new(),
                vertices,
            }
        }
    
        //Adding edges to the graph
        fn add_edge(&mut self, u: usize, v: usize, w: i32) {
            self.edges.push((u, v, w));
        }
        // Bellman-Ford algorithm
        fn bellman_ford(&self, src: usize) -> Vec<i32> {
            let mut dist = vec![i32::max_value(); self.vertices]; // initialize all distances to max value
            dist[src] = 0; // initialize distance from source vertex to the source as 0

            // loop for (vertices - 1) times
            for _ in 0..self.vertices - 1 {
                //For every edge (u, v) with weight w, relax the edge
                for (u, v, w) in &self.edges {
                    if dist[*u] != i32::max_value() && dist[*u] + *w < dist[*v] {
                        //relaxing the distances
                        dist[*v] = dist[*u] + *w;
                    }
                }
            }

            // check for negative cycles
            let mut negative_cycle = false;
            for (u, v, w) in &self.edges {
                if dist[*u] != i32::max_value() && dist[*u] + *w < dist[*v] {
                    panic!("Negative weight cycle detected");
                }
            }

            // return the distances from source to every other vertex
                dist
           
        }

    }   
    pub fn bellmanford() 
    {
        //read the number of vertices and source from the console
        let mut vertex = String::new();
        let mut source = String::new();
        println!("********Bellman Ford***********");
        println!("****************************************************");
        //get the number of vertices
        print!("Please Enter Number of Vertices : ");
        let _= stdout().flush();
        stdin().read_line(&mut vertex).expect("Enter valid number of vertices");
        let vertices: usize = vertex.trim().parse().expect("Invalid input");
        //get the source vertex
        print!("Enter Source Vertex : ");
        let _= stdout().flush();
        stdin().read_line(&mut source).expect("Enter valid source vertex ");
        let source: usize = source.trim().parse().expect("Invalid input for source");
        //get number of edges in the graph
        let mut n_edges = String::new();
        print!("Please Enter Number of edges in the graph : ");
        let _= stdout().flush();
        stdin().read_line(&mut n_edges).expect("Enter Valid Input");
        let n_edges: i32 = n_edges.trim().parse().expect("Invalid input for source");
        //assign the weights to each edge from the console
        let e = add_weights(vertices, source, n_edges);
        //call bellman_ford implementation
        let dist = e.bellman_ford(source);
        //print the distances from the source vertex
        for (v, d) in dist.iter().enumerate() 
        {
            println!("Distance from vertex {} to vertex {} is {}", source, v, d);
        }
    }
     //to return the weights of each branch as a graph containing source,destination and weight
     fn add_weights(vertices: usize, source: usize, edges: i32) -> Graph
    {   //intialize a new graph with the required number of vertices
        let mut g = Graph::new(vertices);
        for i in 0..(edges)
        {   //intialize source,destination and weights
            let mut s = String::new();
            let mut d = String::new();
            let mut w = String::new();
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
            //get the weight
            print!("Weight : ");
            let _= stdout().flush();
            stdin().read_line(&mut w).expect("Please Enter Valid Input for .");
            let w: i32 = w.trim().parse().expect("Invalid input for source");
            // add edge with source,destination and weight
            g.add_edge(s, d, w);
         }
        return g;
         //return graph in the form containing source,destination and weight of the edge
    }
}
