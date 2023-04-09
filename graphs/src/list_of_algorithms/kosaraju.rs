pub mod kosaraju {
    use std::{
        io::{stdin, stdout, Write}, collections::VecDeque,
    };

    fn kosaraju_algorithm(adj_list: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
        // Creating a reversed graph
        let mut adj_list_reversed = vec![Vec::new(); adj_list.len()]; //creating empty adjacency list for the reversed graph
        for (u, e) in adj_list.iter().enumerate() {
            //iterating through each vertex
            for &v in e {
                //iterating through each neighbor adjacent to u
                adj_list_reversed[v].push(u); //adding each vertex to the neighboring list in reversed graph
            }
        }

        let mut visited = vec![false; adj_list.len()];
        let mut order = VecDeque::new();
        for u in 0..adj_list.len() {
            if !visited[u] {
                dfs_reversed(u, &adj_list_reversed, &mut visited, &mut order);
            }
        }

        fn dfs_reversed(u: usize, adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, order: &mut VecDeque<usize>,) {
            visited[u] = true;
            for &v in &adj_list[u] {
                if !visited[v] {
                    dfs_reversed(v, adj_list, visited, order);
                }
            }
            order.push_front(u);
        }

        //perform DFS on the graph obtained above
        let mut visited = vec![false; adj_list.len()];
        let mut list_of_scc = Vec::new(); //to store strongly connected components
        while let Some(u) = order.pop_front() {
            if !visited[u] {
                let mut scc = Vec::new(); //to store nodes in the current SCC   
                dfs(u, adj_list, &mut visited, &mut scc);
                list_of_scc.push(scc); 
            }
        }
        fn dfs(u: usize, adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, scc: &mut Vec<usize>) {
            visited[u] = true;
            scc.push(u);
            for &v in &adj_list[u] {
                if !visited[v] {
                    dfs(v, adj_list, visited, scc);
                }
            }
        }

        list_of_scc

    }
    pub fn kosaraju() {
        println!("******Kosaraju Algorithm*******");
        println!("******************");
        let mut buffer = String::new();

        // Read the number of vertices from the user
        print!("Please Enter Number of Vertices : ");
        let _ = stdout().flush();
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
            for _j in 0..num_neighbors {
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
        let res = kosaraju_algorithm(&adj_list);
        println!("******************");
        println!("{:?}",res);
        
    }
}
