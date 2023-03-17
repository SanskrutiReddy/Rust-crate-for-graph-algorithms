pub mod dijkstras {
    use std::{
        cmp::Ordering,
        collections::{BinaryHeap, HashSet},
        io::{stdin, stdout, Write},
    };

    #[derive(Clone, Eq, PartialEq, PartialOrd)]
    struct Node {
        vertex: usize,
        dist: i32,
    }
    //graph structure
    struct Graph {
        adj_list: Vec<Vec<Node>>, //adjacency list representation using node structure
        vertices: usize,          //total no of vertices
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

        fn dijkstra(&self, src: usize) -> Vec<i32> {
            let mut dist = vec![i32::max_value(); self.vertices]; //initialize all distances to max value
            let mut visited_vertices = HashSet::new(); //to store the visited vertices
            dist[src] = 0; //initialize distance from source vertex to the source as 0
            let mut pq = BinaryHeap::new();
            pq.push(Node {
                vertex: src,
                dist: dist[src],
            });

            //Loop till the binary heap is empty
            while !pq.is_empty() {
                let Node { vertex: u, dist: _ } = pq.pop().unwrap();

                //checks if the vertex is already visited or else the vertex is marked as visited vertices by inserting into the visited HashSet
                if visited_vertices.contains(&u) {
                    continue;
                } else {
                    visited_vertices.insert(u);
                }

                //For every adjacent vertex of u, relax the edge
                for Node { vertex: v, dist: w } in &self.adj_list[u] {
                    let new_dist = dist[u] + *w;
                    if new_dist < dist[*v] {
                        //relaxing the distances
                        dist[*v] = new_dist;
                        pq.push(Node {
                            //push the item into binary heap if the vertex is relaxed
                            vertex: *v,
                            dist: dist[*v],
                        });
                    }
                }
            }
            //returns the distances from source to every other vertex
            dist
        }
    }
    pub fn dijkstras() {
        let mut ve = String::new();
        let mut so = String::new();
        println!("***************Dijkstras Algorithm******************");
        println!("****************************************************");
        print!("Please Enter Number of Vertices : ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut ve)
            .expect("Please Enter Valid number for vertices.");
        let vertices: usize = ve
            .trim()
            .parse()
            .expect("Invalid input for number of vertices");
        print!("Please Enter Source Vertex : ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut so)
            .expect("Please Enter Valid Input for .");
        let source: usize = so.trim().parse().expect("Invalid input for source");
        let mut edges = String::new();
        print!("Please Enter Number of edges in the graph : ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut edges)
            .expect("Please Enter Valid Input for .");
        let edges: i32 = edges
            .trim()
            .parse()
            .expect("Invalid input for number of edges");
        let mut g = Graph::new(vertices);
        let mut cnt = 0;
        while cnt < edges {
            println!("Please Enter Edge {} values ", cnt + 1);
            let mut s = String::new();
            let mut d = String::new();
            let mut w = String::new();
            print!("Source : ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut s)
                .expect("Please Enter Valid Input for .");
            let s: usize = s.trim().parse().expect("Invalid input for source");
            print!("Destination : ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut d)
                .expect("Please Enter Valid Input for .");
            let d: usize = d.trim().parse().expect("Invalid input for destination");
            print!("Weight : ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut w)
                .expect("Please Enter Valid Input for .");
            let w: i32 = w.trim().parse().expect("Invalid input for weight");

            // add edge code goes here
            g.add_edge(s, d, w);
            cnt = cnt + 1;
        }
        //calling dijkstra's function
        let dist = g.dijkstra(source);

        for (v, d) in dist.iter().enumerate() {
            println!("Distance from vertex {} to vertex {} is {}", source, v, d);
        }
    }
}
