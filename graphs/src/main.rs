mod list_of_algorithms;
use std::io::{stdin, stdout, Write};
pub use crate::list_of_algorithms::{
    dijkstras::dijkstras::dijkstars,
    bellmanford::bellmanford::bellmanford,
    dfs::dfs::dfs,
    kosaraju::kosaraju::kosaraju,
    bfs::bfs::bfs
};
fn main() {
    let mut choice = String::new();
    loop {
        choice.clear(); //added to avoid panic error so that parse operates on a clear string
        println!("****************************************************");
        println!("Please Select Any One of the Algorithms Below: ");
        println!("1. Dijkstra’s algorithm");
        println!("2. Bellman ford algorithm");
        println!("3. Depth-First Search algorithm");
        println!("4. Kosaraju's algorithm");
        println!("5. Breadth-First Search algorithm");
        println!("6. Exit");
        println!("****************************************************");
        print!("Please Enter your choice (ex:1): ");
         // The flush() method is used to flush any buffered data from the stream, ensuring that it is written to the output immediately.
        let _= stdout().flush();
        stdin().read_line(&mut choice).expect("Please Enter Valid Input.");
        let number: i32 = choice.trim().parse().expect("Invalid input");
        println!("****************************************************");
        match number {
            1 => dijkstars(),
            2 => bellmanford(),
            3 => dfs(),
            4 => kosaraju(),
            5 => bfs(),
            6 => break,
            _ => println!("Please Make a Valid Selection"),
        }
    }
}
