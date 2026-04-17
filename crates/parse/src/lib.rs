mod tests;
use std::rc::Rc;

type Targets = Vec<(Rc<String>, Vec<Rc<String>>)>;

pub fn parse_make_p(lines: &mut impl Iterator<Item = String>) -> Result<Targets, String> {
    if !lines.any(|x| x.starts_with("# Make data base, printed on ")) {
        return Err("Input is not compatible with `LANG=C make -p`.".to_string());
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
