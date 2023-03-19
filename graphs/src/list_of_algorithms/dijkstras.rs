pub mod dijkstras {
    use std::{
        cmp::Ordering,                      // Importing Ordering to make node structure comparision based on distances
        collections::{BinaryHeap, HashSet}, // Importing BinaryHeap and HashSet to get priority Node and to mark visited Nodes 
        io::{stdin, stdout, Write},         // Importing input/output library for reading user input and for printing output
    };

    #[derive(Clone, Eq, PartialEq, PartialOrd)]
    struct Node {
        vertex: usize, // Vertex name in the Graph
        dist: i32,     // Distance from source to vertex
    }

    struct Graph {
        adj_list: Vec<Vec<Node>>, // Adjacency list representation using Node structure
        vertices: usize,          // Total number of vertices in the Graph
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering { // Custom comparison function for Node to compare on dist
            other.dist.cmp(&self.dist) // Nodes will be compared by their distance from the source
        }
    }

    impl Graph {
        fn new(vertices: usize) -> Self { // Constructor for Graph structure
            Graph {
                adj_list: vec![Vec::new(); vertices], // Initializing adjacency list with empty vectors with size equal to vertices number
                vertices,
            }
        }

        fn add_edge(&mut self, u: usize, v: usize, w: i32) { // Function `add_edge` to add edges to the graphs
            self.adj_list[u].push(Node { vertex: v, dist: w }); // Adding Vertex `v` as adjacent vertex of Vertex `u`
            self.adj_list[v].push(Node { vertex: u, dist: w }); // Add Vertex `u` as adjacent vertex of Vertex `v`
        }

        fn dijkstra(&self, src: usize) -> Vec<i32> {
            let mut dist = vec![i32::max_value(); self.vertices]; // Initializing all distances to max value So that we can select min distance and update the graph
            let mut visited_vertices = HashSet::new(); // To store the visited vertices
            dist[src] = 0; // Initializing distance from source to the source to 0
            let mut pq = BinaryHeap::new(); // Creating a BinaryHeap to store nodes for priority queue

            pq.push(Node { vertex: src, dist: dist[src] }); // Pushing the source node into priority queue

            // Loop till the BinaryHeap is empty
            while !pq.is_empty() {
                let Node { vertex: u, dist: _ } = pq.pop().unwrap(); // Getting the node with minimum distance from the priority queue

                // Checking if the vertex is already visited
                if visited_vertices.contains(&u) {
                    continue;
                } else {
                    visited_vertices.insert(u); // marking the vertex as visited by inserting into the HashSet
                }

                // For every adjacent vertex of u, relax the edge
                for Node { vertex: v, dist: w } in &self.adj_list[u] {
                    let new_dist = dist[u] + *w; // Calculating the new distance
                    if new_dist < dist[*v] { // Check if the new distance is less than current distance
                        dist[*v] = new_dist; // Relax the distance
                        pq.push(Node { vertex: *v, dist: dist[*v] }); // Push the node into priority queue
                    }
                }
            }

            // Return the distances from source to every other vertex
            dist
        }
    }

   pub fn dijkstras() {
    // Create two empty strings to store user input(Source & Vertices Count)
    let mut ve = String::new();
    let mut so = String::new();
    
    // Printing the introduction message
    println!("******Dijkstras Algorithm*******");
    println!("******************");
    
    // Prompting user to input number of vertices in the Graph
    print!("Please Enter Number of Vertices in the Graaph : ");
    let _ = stdout().flush();
    
    // Reading user input for number of vertices, parsing it into an integer and handling errors
    stdin()
        .read_line(&mut ve)
        .expect("Please Enter Valid number for vertices.");
    let vertices: usize = ve
        .trim()
        .parse()
        .expect("Invalid input for number of vertices");
    
    // Prompting user to input the source vertex
    print!("Please Enter Source Vertex : ");
    let _ = stdout().flush();
    
    // Reading user input for source vertex and parsing it into an integer
    stdin()
        .read_line(&mut so)
        .expect("Please Enter Valid Input for Source.");
    let source: usize = so.trim().parse().expect("Invalid input for source");
    
    // Prompting user to input the number of edges
    let mut edges = String::new();
    print!("Please Enter Number of edges in the graph : ");
    let _ = stdout().flush(); // Flushing the output buffer to ensure prompt is displayed before taking input from user
    
    // Reading user input for number of edges and parsing it into an integer
    stdin()
        .read_line(&mut edges)
        .expect("Please Enter Valid Input for number of Edges.");
    let edges: i32 = edges
        .trim()
        .parse()
        .expect("Invalid input for number of edges");
    
    // Creating a new graph with the number of vertices entered by the user
    let mut g = Graph::new(vertices);
    
    // Initializing counter variable to keep track of the number of edges added
    let mut cnt = 0;
    
    // Loop Prompting user to input all edges source, destination and weight
    while cnt < edges {
        println!("Please Enter Edge {} values ", cnt + 1);
        let mut s = String::new();
        let mut d = String::new();
        let mut w = String::new();
        
        // Prompting user to input source vertex of current edge
        print!("Source : ");
        let _ = stdout().flush();
        
        // Reading user input for source vertex and parsing it into an integer
        stdin()
            .read_line(&mut s)
            .expect("Please Enter Valid Input for Source.");
        let s: usize = s.trim().parse().expect("Invalid input for source");
        
        // Prompting user to input destination vertex of current edge
        print!("Destination : ");
        let _ = stdout().flush();
        
        // Reading user input for destination vertex and parsing it into an integer
        stdin()
            .read_line(&mut d)
            .expect("Please Enter Valid Input for destination.");
        let d: usize = d.trim().parse().expect("Invalid input for destination");
        
        // Prompting user to input weight of current edge
        print!("Weight : ");
        let _ = stdout().flush();
        
        // Reading user input for edge weight and parsing it into an integer
        stdin()
            .read_line(&mut w)
            .expect("Please Enter Valid Input for weight.");
        let w: i32 = w.trim().parse().expect("Invalid input for weight");

        // Adding the current edge to the graph
        g.add_edge(s, d, w);
        
        // Increasing the edge counter
        cnt = cnt + 1;
    }
    
    // Calling Dijkstra's algorithm to find the shortest path from th e Source
    let dist = g.dijkstra(source);
    println!("******************");
    // Looping and printing the distances from Source to respective vertices
    for (v, d) in dist.iter().enumerate() {
        println!("Distance from vertex {} to vertex {} is {}", source, v, d);
    }
    }
}
// stdin, stdout and Write trait from std::io module