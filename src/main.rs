use i3ipc::{reply::{Node, Workspace, Workspaces}, I3Connection};
use std::env;
use itertools::Itertools;

fn main() {
    let mut con = I3Connection::connect().unwrap();

    let workspaces = con.get_workspaces().unwrap();

    // for ws in &workspaces.workspaces {
    //     println!("Workspace: {:?}", ws);
    // }

    let tree = con.get_tree().unwrap();

    let out = get_current_output(&tree);
    let ws = current_ws(&workspaces);
    let ws_nodes = get_node_by_name(&out, &ws.name).unwrap();

    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    match cmd.as_str() {
        "next" => next(&mut con, &ws_nodes),
        "prev" => prev(&mut con, &ws_nodes),
        "0" => top_at(&mut con, &ws_nodes, 0),
        "1" => top_at(&mut con, &ws_nodes, 1),
        "2" => top_at(&mut con, &ws_nodes, 2),
        "3" => top_at(&mut con, &ws_nodes, 3),
        "s0" => semi_top_at(&mut con, &ws_nodes, 0),
        "s1" => semi_top_at(&mut con, &ws_nodes, 1),
        "s2" => semi_top_at(&mut con, &ws_nodes, 2),
        "s3" => semi_top_at(&mut con, &ws_nodes, 3),
        "move0" => move_to_top_at(&mut con, &ws_nodes, 0),
        "move1" => move_to_top_at(&mut con, &ws_nodes, 1),
        "move2" => move_to_top_at(&mut con, &ws_nodes, 2),
        "move3" => move_to_top_at(&mut con, &ws_nodes, 3),
        "swap0" => swap_top(&mut con, &ws_nodes, 0),
        "swap1" => swap_top(&mut con, &ws_nodes, 1),
        "swap2" => swap_top(&mut con, &ws_nodes, 2),
        "swap3" => swap_top(&mut con, &ws_nodes, 3),
        "cycle" => cycle_other(&mut con, &ws_nodes, CycleDirection::Forward),
        "cycler" => cycle_other(&mut con, &ws_nodes, CycleDirection::Backward),
        "cycle_at" => cycle_at(&mut con, &ws_nodes, CycleDirection::Forward),
        "cycle_atr" => cycle_at(&mut con, &ws_nodes, CycleDirection::Backward),
        &_ => eprintln!("Unknown command: {}", cmd),
    }

    // let topmost = get_node_by_id(&ws_nodes, ws_nodes.focus[0]).unwrap();
    // let topmost = &topmost.focus;
    // let other = topmost.last().unwrap();
    // let other_leaf = get_leaf(&get_node_by_id(&ws_nodes, *other).expect("Node not found"));
    // let cmd = format!("[con_id=\"{}\"] focus", other_leaf.id);
    // println!("Running command {}", cmd.as_str());
    // match con.run_command(cmd.as_str()) {
    //     Ok(_) => println!("Command ran successfully"),
    //     Err(e) => println!("Error: {:?}", e),
    // }
}

enum CycleDirection {
    Forward,
    Backward,
}

fn cycle_at(con: &mut I3Connection, ws_nodes: &Node, direction: CycleDirection) {
    let parent = get_parent(&ws_nodes);
    let topmost = &parent.focus;
    let rest = match direction {
        CycleDirection::Forward => &topmost[1..].iter().collect::<Vec<_>>(),
        CycleDirection::Backward => &topmost[0..].iter().rev().collect::<Vec<_>>(),
    };
    for (prev, next) in rest.iter().tuple_windows() {
        swap(con, **prev, **next);
    }
}

fn cycle_other(con: &mut I3Connection, ws_nodes: &Node, direction: CycleDirection) {
    let topmost = get_node_by_id(&ws_nodes, ws_nodes.focus[0]).unwrap();
    let topmost = &topmost.focus;
    let rest = match direction {
        CycleDirection::Forward => &topmost[1..].iter().collect::<Vec<_>>(),
        CycleDirection::Backward => &topmost[1..].iter().rev().collect::<Vec<_>>(),
    };
    for (prev, next) in rest.iter().tuple_windows() {
        swap(con, **prev, **next);
    }
}

fn swap_top(con: &mut I3Connection, ws_nodes: &Node, idx: usize) {
    let topmost = get_node_by_id(&ws_nodes, ws_nodes.focus[0]).unwrap();
    let current_id = &topmost.focus[0];
    let other = &topmost.nodes[idx];
    // let cmd = format!("[con_id=\"{}\"] swap container with con_id {}", current_id, other.id);
    // println!("Running command {}", cmd.as_str());
    // match con.run_command(cmd.as_str()) {
    //     Ok(_) => println!("Command ran successfully"),
    //     Err(e) => println!("Error: {:?}", e),
    // }
    swap(con, *current_id, other.id);
}

fn swap(con: &mut I3Connection, id_a: i64, id_b: i64) {
    let cmd = format!("[con_id=\"{}\"] swap container with con_id {}", id_a, id_b);
    println!("Running command {}", cmd.as_str());
    match con.run_command(cmd.as_str()) {
        Ok(_) => println!("Command ran successfully"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn top_at(con: &mut I3Connection, ws_nodes: &Node, idx: usize) {
    let topmost = get_node_by_id(&ws_nodes, ws_nodes.focus[0]).unwrap();
    let other = &topmost.nodes[idx];
    // println!("Topmost: {:?}, Other: {:?}", topmost, other);
    let other_leaf = get_leaf(other);
    let cmd = format!("[con_id=\"{}\"] focus", other_leaf.id);
    println!("Running command {}", cmd.as_str());
    match con.run_command(cmd.as_str()) {
        Ok(_) => println!("Command ran successfully"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn semi_top_at(con: &mut I3Connection, ws_nodes: &Node, idx: usize) {
    let topmost = get_node_by_id(&ws_nodes, ws_nodes.focus[0]).unwrap();
    let topmost = get_node_by_id(&ws_nodes, topmost.focus[0]).unwrap();
    let other = &topmost.nodes[idx];
    // println!("Topmost: {:?}, Other: {:?}", topmost, other);
    let other_leaf = get_leaf(other);
    let cmd = format!("[con_id=\"{}\"] focus", other_leaf.id);
    println!("Running command {}", cmd.as_str());
    match con.run_command(cmd.as_str()) {
        Ok(_) => println!("Command ran successfully"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn move_to_top_at(con: &mut I3Connection, ws_nodes: &Node, idx: usize) {
    let topmost = get_node_by_id(&ws_nodes, ws_nodes.focus[0]).unwrap();
    let other = &topmost.nodes[idx];
    // println!("Topmost: {:?}, Other: {:?}", topmost, other);
    // let other_leaf = get_leaf(other);
    let cmd = format!("[con_id=\"{}\"] mark --add x; move container to mark x; [con_id=\"{}\"] mark --toggle x", other.id, other.id);
    println!("Running command {}", cmd.as_str());
    match con.run_command(cmd.as_str()) {
        Ok(_) => println!("Command ran successfully"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn next(con: &mut I3Connection, ws_nodes: &Node) {
    let topmost = get_node_by_id(&ws_nodes, ws_nodes.focus[0]).unwrap();
    let topmost = &topmost.focus;
    let other = topmost.last().unwrap();
    println!("Topmost: {:?}, Other: {:?}", topmost, other);
    let other_leaf = get_leaf(&get_node_by_id(&ws_nodes, *other).expect("Node not found"));
    let cmd = format!("[con_id=\"{}\"] focus", other_leaf.id);
    println!("Running command {}", cmd.as_str());
    match con.run_command(cmd.as_str()) {
        Ok(_) => println!("Command ran successfully"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn prev(con: &mut I3Connection, ws_nodes: &Node) {
    let topmost = get_node_by_id(&ws_nodes, ws_nodes.focus[0]).unwrap();
    let topmost = &topmost.focus;
    let other = topmost[1];
    println!("Topmost: {:?}, Other: {:?}", topmost, other);
    let other_leaf = get_leaf(&get_node_by_id(&ws_nodes, other).expect("Node not found"));
    let cmd = format!("[con_id=\"{}\"] focus", other_leaf.id);
    println!("Running command {}", cmd.as_str());
    match con.run_command(cmd.as_str()) {
        Ok(_) => println!("Command ran successfully"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn get_current_output(tree: &Node) -> &Node {
    // for id in tree.focus.iter() {
    //     output = output.nodes.iter().filter(|n| n.id == *id).next().unwrap();
    // }
    tree.nodes.iter().filter(|n| n.id == tree.focus[0]).next().unwrap()
}

fn current_ws(ws: &Workspaces) -> &Workspace {
    ws.workspaces.iter().filter(|w| w.focused).next().unwrap()
}

// fn get_current_ws(con: &mut I3Connection, tree: &Node) -> Option<String> {
//     let output = get_node(tree, tree.focus[0]);
//     let outputs = con.get_outputs().unwrap();
//     for o in outputs.outputs.into_iter() {
//         if &o.name == output.name.as_ref().unwrap() {
//             return o.current_workspace;
//         }
//     }
//     None
// }

// fn get_nodes(tree: &mut Node, ws: &Workspace) -> Result<Vec<Node>, &'static str> {
//     let mut nodes = Vec::new();
//     for node in tree.nodes.iter() {
//         match node.name {
//             Some(ref name) => {
//                 if name == &ws.name {
//                     nodes.push(node.clone());
//                 }
//                 if node.nodes.len() > 0 {
//                     let mut subnodes = get_nodes(&mut node.clone(), ws)?;
//                     nodes.append(&mut subnodes);
//                 }
//             }
//             None => {}
//         }
//     }
//     Ok(nodes)
// }

// fn current_output(tree: &mut Node) -> &Node {
//     let mut output = tree;
//     for id in tree.focus.iter() {
//         output = output.nodes.iter().filter(|n| n.id == *id).next().unwrap();
//     }
//     output
// }

fn get_node_by_id(tree: &Node, id: i64) -> Option<&Node> {
    // let node = tree.nodes.iter().filter(|n| n.id == id).next().unwrap();
    // node
    for n in tree.nodes.iter() {
        if n.id == id {
            return Some(n);
        }
        match get_node_by_id(n, id) {
            Some(node) => return Some(node),
            None => {}
        }
    }
    None
}

fn get_node_by_name<'a>(tree: &'a Node, name: &'a String) -> Option<&'a Node> {
    // let node = tree.nodes.iter().filter(|n| n.name.as_ref().unwrap() == name).next().unwrap();
    for n in tree.nodes.iter() {
        match n.name {
            Some(ref nn) => {
                // println!("Node: {:?}", n);
                if nn == name {
                    return Some(n);
                }
                match get_node_by_name(n, name) {
                    Some(node) => return Some(node),
                    None => {}
                }
            }
            None => {}
        }
    }
    None
}

fn get_leaf(tree: &Node) -> &Node {
    let mut node = tree;
    while node.nodes.len() > 0 {
        node = get_node_by_id(&node, node.focus[0]).unwrap();
    }
    node
}

fn get_focused(tree: &Node) -> &Node {
    let mut node = tree;
    while !node.focused {
        node = get_node_by_id(&node, node.focus[0]).unwrap();
    }
    node
}


fn get_parent(tree: &Node) -> &Node {
    let mut node = tree;
    let mut last_node = tree;
    while !node.focused {
        last_node = node;
        node = get_node_by_id(&node, node.focus[0]).unwrap();
    }
    last_node
}
