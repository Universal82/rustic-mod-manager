// code I stole from my other testing environment

#[derive(Debug, Clone)]
struct SymLink {
    from: PathBuf,
    to: PathBuf
}

impl SymLink {
    fn rm_conflicts(input: Vec<Self>) -> Vec<Self> {
        let mut known = vec![];
        let mut out = vec![];

        for (_, to, original) in input.iter().rev().map(|v|{v.split()}) {
            if !known.contains(&to) {
                known.push(to);
                out.push(original.clone());
            }
        }

        return out;
    }

    fn split(&self) -> (&PathBuf, &PathBuf, &Self) {
        return (&self.from, &self.to, self);
    }

    fn make(self) {
        match std::fs::hard_link(&self.from, &self.to) {
            Ok(()) => {},
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    let meta1 = std::fs::metadata(&self.from).unwrap();
                    let meta2 = std::fs::metadata(&self.to).unwrap();
                    if !((meta1.ino() == meta2.ino()) && (meta1.dev() == meta2.dev())) {
                        println!("Failed to link {} to {} because destination exists!", self.from.display(), self.to.display())
                    } else {
                        println!("Link from {} to {} already exists and is fine.", self.from.display(), self.to.display())
                    }
                },
                e => println!("Failed to link {} to {} because {e}", self.from.display(), self.to.display())
            }
        }
    }
}

fn prep_and_get_links(inputs: Vec<PathBuf>, output: PathBuf) -> Vec<SymLink> {
    let mut links_to_make = vec![];
    for path in inputs {
        println!("Reading dir at path: {}", path.display());
        for item in std::fs::read_dir(path).unwrap() {
            if let Ok(item) = item {
                let file_type = item.file_type().unwrap();
                if file_type.is_file() {
                    println!("{} is file!", item.path().display());
                    let filename = item.file_name().into_string().unwrap();
                    links_to_make.push(SymLink { from: item.path(), to: (output.clone().to_str().unwrap().to_string()+&filename).into() })
                } else if file_type.is_dir() {
                    // oh dear, it's getting recursive
                    let mut new_output = output.clone();
                    new_output.push(item.file_name());
                    let _ = std::fs::create_dir(&new_output);
                    new_output.push("./");
                    
                    let new_inputs: Vec<PathBuf> = vec![item.path()];
                    
                    println!("Dir {} found, recursing to {} from {:#?}", item.path().display(), new_output.display(), new_inputs);

                    let mut more_links = prep_and_get_links(new_inputs, new_output);
                    links_to_make.append(&mut more_links);
                }
            }
        }
    }
    return links_to_make;
}