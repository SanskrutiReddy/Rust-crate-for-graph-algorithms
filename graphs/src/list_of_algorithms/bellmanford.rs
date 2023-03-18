pub mod bellmanford {
    use std::{io::{stdin, stdout, Write}};
    use std::usize;
    pub fn bellmanford() 
    {
        //read the number of vertices and source from the console
        let mut vertex = String::new();
        let mut source = String::new();
        println!("***********Bellman Ford**************");
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
        let mut edges = add_weights(vertices, source, n_edges);
        print!("If you want add edges press 1")
        // add new edges if the user want to add more edges at this point
        //implementation of bellman ford algorithm
    }



    pub fn add_weights(vertices: usize, source: usize, edges: i32) -> Vec<(usize, usize, i32)>
    {   let mut weighted_edges = vec![];
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
            weighted_edges.push((s, d, w));
         }
        return weighted_edges;
         //return vector in the form containing source,destination and weight of the edge
    }
}    

