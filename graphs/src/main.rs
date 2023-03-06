use std::io::{stdin, stdout, Write};

fn main() {
    let mut choice = String::new();

    loop {
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
        match number {
            1 => dijkstras(),
            2 => bellmanford(),
            3 => dfs(),
            4 => kosaraju(),
            5 => bfs(),
            6 => break,
            _ => println!("Please Make a Valid Selection"),
        }
    }
}

fn dijkstras() {
    println!("dijkstars selected.");
    
}

fn bellmanford() {
    println!("bell-man ford selected.");
}

fn dfs() {
    println!("dfs selected.");
}

fn kosaraju() {
    println!("kosaraju selected.");
}

fn bfs(){
    println!("bfs selected.");
}
