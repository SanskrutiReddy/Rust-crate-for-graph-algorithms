pub mod kosaraju {
    use std::{
        io::{stdin, stdout, Write},
    };

    pub fn kosaraju() {
        println!("******Kosaraju Algorithm*******");
        println!("******************");
        let mut buffer = String::new();

        // Read the number of vertices from the user
        print!("Please Enter Number of Vertices : ");
        let _= stdout().flush();
        stdin()
            .read_line(&mut buffer)
            .expect("Please Enter Valid number for vertices.");
        let vertices: usize = buffer
            .trim()
            .parse()
            .expect("Invalid input for number of vertices");
        buffer.clear();

        // Read the adjacency list for each vertex
        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); vertices];
        for i in 0..vertices {
            print!("Please enter the number of neighbors for vertex {} : ", i);
            let _ = stdout().flush();
            stdin()
                .read_line(&mut buffer)
                .expect("Please Enter Valid number for neighbors.");
            let num_neighbors: usize = buffer
                .trim()
                .parse()
                .expect("Invalid input for number of neighbors");
            buffer.clear();
        }
    }
}