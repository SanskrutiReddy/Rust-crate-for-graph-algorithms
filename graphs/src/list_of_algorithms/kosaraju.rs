pub mod kosaraju {
    use std::{
        io::{stdin, stdout, Write},
    };

    fn kosaraju_algorithm(adj_list: &Vec<Vec<usize>>){
        // Creating a reversed graph
        let mut adj_list_reversed = vec![Vec::new(); adj_list.len()]; //creating empty adjacency list for the reversed graph
        for (u, e) in adj_list.iter().enumerate() { //iterating through each vertex
            for &v in e { //iterating through each neighbor adjacent to u
                adj_list_reversed[v].push(u); //adding each vertex to the neighboring list in reversed graph
            }
        }
    }
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

            //neighbors input for each vertex
            let mut neighbors: Vec<usize> = Vec::new();
            for j in 0..num_neighbors {
                print!("Please enter the next neighbor for vertex {} : ", i);
                let _ = stdout().flush();
                stdin()
                    .read_line(&mut buffer)
                    .expect("Please Enter Valid number for next neighbor.");
                let neighbor: usize = buffer.trim().parse().expect("Invalid input for neighbor");
                buffer.clear();
                neighbors.push(neighbor);
            }
            adj_list[i] = neighbors;
        }

        // Call the kosaraju function with the adjacency list
        kosaraju_algorithm(&adj_list);

    }
}
