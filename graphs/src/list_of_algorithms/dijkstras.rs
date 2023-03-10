
pub mod dijkstras {
    use std::{io::{stdin, stdout, Write}};
    pub fn dijkstras() {
        let mut ve = String::new();
        let mut so = String::new();
        println!("***************Dijkstars Algorithm******************");
        println!("****************************************************");
        print!("Please Enter Number of Vertices : ");
        let _= stdout().flush();
        stdin().read_line(&mut ve).expect("Please Enter Valid number for vertices.");
        let vertices: usize = ve.trim().parse().expect("Invalid input");
        print!("Please Source Vertex : ");
        let _= stdout().flush();
        stdin().read_line(&mut so).expect("Please Enter Valid Input for .");
        let source: usize = so.trim().parse().expect("Invalid input for source");
        let mut edges = String::new();
        print!("Please Enter Number of edges in the graph : ");
        let _= stdout().flush();
        stdin().read_line(&mut edges).expect("Please Enter Valid Input for .");
        let edges: i32 = edges.trim().parse().expect("Invalid input for source");
        // let mut g = Graph::new(vertices);
        let mut cnt = 0;
        while cnt < edges {
            println!("Please Enter Edge {} values ",cnt+1);
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

            // add edge code goes here
            cnt = cnt + 1;
        }
       
    }



}