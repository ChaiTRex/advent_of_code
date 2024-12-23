use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    static INPUT: &str = include_str!("../../../day23.txt");

    let mut graph = HashMap::new();
    for (a, b) in INPUT.lines().map(|line| line.split_once('-').unwrap()) {
        graph.entry(a).or_insert(HashSet::new()).insert(b);
        graph.entry(b).or_insert(HashSet::new()).insert(a);
    }

    let nodes = graph.keys().copied().collect::<HashSet<_>>();
    let t_nodes = nodes
        .iter()
        .copied()
        .filter(|&name| name.starts_with('t'))
        .collect::<HashSet<_>>();

    let mut solutions = HashSet::new();
    for node_1 in t_nodes.iter().copied() {
        for node_2 in graph.get(node_1).unwrap().iter().copied() {
            if node_1 != node_2 {
                for node_3 in graph.get(node_1).unwrap().iter().copied() {
                    if node_1 != node_3
                        && node_2 != node_3
                        && graph.get(node_2).unwrap().contains(&node_3)
                    {
                        let mut nodes = [node_1, node_2, node_3];
                        nodes.sort_unstable();
                        solutions.insert(nodes);
                    }
                }
            }
        }
    }
    let part1 = solutions.len();

    println!("{part1}");

    println!("Node count: {}", nodes.len());
    println!(
        "Maximum degree: {}",
        graph.values().map(|value| value.len()).max().unwrap()
    );

    let mut part2 = String::new();
    let mut prev_solutions = nodes
        .iter()
        .copied()
        .map(|node| core::iter::once(node).collect::<BTreeSet<_>>())
        .collect::<HashSet<_>>();

    loop {
        let mut solutions = prev_solutions
            .iter()
            .flat_map(|prev_solution| {
                nodes
                    .iter()
                    .copied()
                    .filter(|&node| {
                        prev_solution.iter().copied().all(|sol_node| {
                            sol_node != node && graph.get(sol_node).unwrap().contains(node)
                        })
                    })
                    .map(|node| {
                        let mut new_solution = prev_solution.clone();
                        new_solution.insert(node);
                        new_solution
                    })
            })
            .collect::<HashSet<_>>();

        if solutions.is_empty() {
            let solution = prev_solutions
                .iter()
                .next()
                .unwrap()
                .iter()
                .copied()
                .collect::<Vec<_>>();
            part2 = solution.join(",");
            break;
        }

        prev_solutions = solutions;
    }
    println!("{part2}");
}
