use std::collections::{HashMap, HashSet, LinkedList};

use crate::shared::file_reader;

fn bag_bfs(
    mut visited: HashSet<String>,
    mut queue: LinkedList<String>,
    bag_map: HashMap<String, Vec<String>>,
) -> HashSet<String> {
    if let Some(bag) = queue.pop_front() {
        visited.insert(bag.clone());

        for v in bag_map.get(&bag).unwrap_or(&Vec::new()) {
            queue.push_back(v.clone());
        }

        // println!("Calling bfs: {:?}, {:?}", visited, queue);
        return bag_bfs(visited, queue, bag_map);
    } else {
        // println!("Stop bfs: {:?}, {:?}", visited, queue);

        return visited;
    }
}

fn count_bags(bag: &String, map: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    if let Some(children) = map.get(bag) {
        let mut result = 0;
        for (count, child) in children {
            result += count + count * count_bags(child, &map);
        }
        return result;
    }
    return 0;
}

pub fn solve() {
    let lines = match file_reader::read_lines("files/day7/input.txt") {
        Ok(ls) => ls,
        Err(e) => panic!("{}", e),
    };

    let mut reverse_bag_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut bag_map: HashMap<String, Vec<(i32, String)>> = HashMap::new();

    for line in lines.map_while(Result::ok) {
        let mut line_split = line.split(" contain ");
        let parent_bag = line_split
            .next()
            .unwrap()
            .split(' ')
            .take_while(|s| !s.contains("bag"))
            .fold("".to_string(), |acc, s| {
                if acc == "" {
                    return acc + s;
                } else {
                    return acc + " " + s;
                }
            }); // Remove the word 'bag' or 'bags' at the end.

        let mut child_bags_str = line_split.next().unwrap();
        child_bags_str = &child_bags_str[..child_bags_str.len() - 1];
        if child_bags_str == "no other bags" {
            continue;
        }

        let mut child_bag_split = child_bags_str.split(", ");
        while let Some(child_bag_str) = child_bag_split.next() {
            let child_bag = &child_bag_str[2..]
                .split(" ")
                .take_while(|s| !s.contains("bag"))
                .fold("".to_string(), |acc, s| {
                    if acc == "" {
                        return acc + s;
                    } else {
                        return acc + " " + s;
                    }
                }); // Remove leading bag count and the word 'bag' or 'bags' at the end.
            reverse_bag_map
                .entry(child_bag.to_string())
                .or_insert(Vec::new())
                .push(parent_bag.to_string());

            let child_count: i32 = child_bag_str
                .chars()
                .nth(0)
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap();

            bag_map
                .entry(parent_bag.to_string())
                .or_insert(Vec::new())
                .push((child_count, child_bag.to_string()));
        }
    }

    let gold_bag: String = "shiny gold".to_string();
    let mut queue: LinkedList<String> = LinkedList::new();
    for bag in reverse_bag_map.get(&gold_bag).unwrap() {
        queue.push_back(bag.clone());
    }
    let visited: HashSet<String> = bag_bfs(HashSet::new(), queue, reverse_bag_map);

    println!("Day 7 - part 1: {}", visited.len());
    println!("Day 7 - part 2: {}", count_bags(&gold_bag, &bag_map));
}
