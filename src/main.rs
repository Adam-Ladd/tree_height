fn main() {
    let node_numbers = get_stdin_input();
    println!("{:?}", node_numbers)
}

/// take input for program from stdin
/// the first line should be a single number containing the length (n) of the input.
/// the second line should be n valid i32 numbers separated by whitespaces.
/// e.g.
/// 5
/// 0 72 82 1755 -28762
/// -->
/// [0, 72, 82, 1755, -28762]
fn get_stdin_input() -> Vec<i32> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("could not read stdin line 1");
    let mut node_numbers = Vec::with_capacity(buffer.trim().parse::<usize>().expect("could not parse number of nodes as usize"));
    buffer.clear();

    // read the node numbers and parse them into a Vec of i32's
    std::io::stdin().read_line(&mut buffer).expect("could not read stdin line 1");
    node_numbers = buffer.trim().split_whitespace().map(|x| x.parse::<i32>().expect("Could not parse character(s) as i32")).collect();
    node_numbers
}


/// take an input vector and build a tree
/// the index of a value is it's node number and the value itself is the number of the parent node.
/// e.g. if node_mapping[3] == 1 then node 3 is a child of node 1
fn find_tree_depth(node_mapping: Vec<i32>) -> i32 {
    // 
    let checked: Vec<bool> = vec![false, node_mapping.len()];
    todo!();
}

