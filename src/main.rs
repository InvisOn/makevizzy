use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    io::{BufRead, stdin},
    process::exit,
    rc::Rc,
};

fn main() {
    let lines = &mut stdin().lock().lines().map_while(Result::ok);

    let rules = match parse_make_p(lines) {
        Ok(rules) => rules,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    print_dot_graph(rules);
}

type Targets = Vec<(Rc<String>, Vec<Rc<String>>)>;

fn parse_make_p(lines: &mut impl Iterator<Item = String>) -> Result<Targets, String> {
    if !lines.any(|x| x.starts_with("# Make data base, printed on ")) {
        return Err("Input seems not compatible with `LANG=C make -p`.".to_string());
    };

    if !lines.any(|x| x.starts_with("# Files")) {
        return Err("No files defined in Makefile.".to_string());
    };

    let mut targets: Vec<(Rc<String>, Vec<Rc<String>>)> = Vec::new();

    while let Some(line) = lines.next() {
        if line == "# Not a target:" {
            for line in lines.by_ref() {
                if line.is_empty() {
                    break;
                }
            }
            continue;
        }

        if line.starts_with("# files hash-table stats:") {
            break;
        }

        if !line.contains(':') || line.starts_with('#') {
            continue;
        }

        let [target, prerequisites] = match line.splitn(2, ':').collect::<Vec<&str>>().as_slice() {
            &[head, tail] => [head, tail],
            _ => continue,
        };

        let prerequisites: Vec<Rc<String>> = prerequisites
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| Rc::new(s.to_string()))
            .collect();

        targets.push((Rc::new(target.to_string()), prerequisites));
    }

    Ok(targets)
}

fn print_dot_graph(targets: Vec<(Rc<String>, Vec<Rc<String>>)>) {
    let dot_pre = "digraph G {
        graph [
        rankdir=RL,
    ]

    node [
        shape=box,
        style=solid,
        margin=\"0.3,0.1\",
    ]

    edge [
        color=\"#00000088\",
        dir=back,
        penwidth=1.2,
        minlen=1    
    ]
    subgraph cluster1{";

    println!("{}", dot_pre);

    let mut name_to_node: HashMap<Rc<String>, Rc<String>> = HashMap::new();
    let mut parents: HashSet<String> = HashSet::new();
    let mut i = 2;

    for (target, _) in &targets {
        let target_str = escape(target.clone());
        parents.insert(target_str);
    }

    for (target, deps) in targets {
        let target_str = Rc::new(escape(target.clone()));

        register_node(target_str.clone(), &mut i, &mut name_to_node);

        for dep_str in deps {
            if parents.contains(&*dep_str) {
                register_node(dep_str.clone(), &mut i, &mut name_to_node);
            } else {
                register_filled_node(dep_str.clone(), &mut i, &mut name_to_node);
            }

            if let (Some(target_node), Some(dep_node)) =
                (name_to_node.get(&target_str), name_to_node.get(&dep_str))
            {
                println!("{} -> {}", target_node, dep_node);
            };
        }
    }

    println!("}}\n}}");
}

fn register_node(
    name: Rc<String>,
    i: &mut i32,
    name_to_node: &mut HashMap<Rc<String>, Rc<String>>,
) {
    if let Entry::Vacant(e) = name_to_node.entry(name.clone()) {
        let node = Rc::new(format!("n{}", i));
        *i += 1;
        e.insert(node.clone());
        println!("{}[label=\"{}\"]", node, name);
    }
}

fn register_filled_node(
    name: Rc<String>,
    i: &mut i32,
    name_to_node: &mut HashMap<Rc<String>, Rc<String>>,
) {
    if let Entry::Vacant(e) = name_to_node.entry(name.clone()) {
        let node = Rc::new(format!("n{}", i));
        *i += 1;
        e.insert(node.clone());
        println!("{}[label=\"{}\", style = \"solid,filled\"]", node, name);
    }
}

fn escape(s: Rc<String>) -> String {
    s.replace('"', "\\\"")
}
