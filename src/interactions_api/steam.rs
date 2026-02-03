use std::{path::Path, vec};

static KNOWN_LIBRARIES_LOCATIONS: [&'static str; 1] = ["~/.local/share/Steam/steamapps/libraryfolders.vdf"];

/// simply takes a file path beginning with a tilde (~) and qualifies it to one that can be read by std::fs
fn qualify_file_path(path: &str) -> String {
    let mut val = std::env::home_dir()
        .expect("Expected to be able to get user's home directory")
        .to_str()
        .expect("Expected to be able to convert PathBuf to str")
        .to_string();
    if path.contains('~') {
        val.push_str(path[1..path.len()].as_ref());
    }
    val
}

/// gets the install directory of the app given via it's steam appid
pub fn find_game(appid: i32) -> Option<String> {

    // get libraryfolders.vdf
    let library = {
        let mut library = None;
        for lib in KNOWN_LIBRARIES_LOCATIONS {
            let lib = qualify_file_path(lib);
            if std::fs::exists(&lib).expect("Expected to be able to read known library location paths") {
                library = Some(lib.as_str().to_owned());
                break;
            }
        }

        let content = match library {
            Some(v) => std::fs::read_to_string(v).expect("Expected to be able to read the libraryfolders.vdf file"),
            None => panic!("Expected to find a libraryfolders.vdf in one of the known locations")
        };

        content
    };

    // extract library paths from the libraryfolders.vdf
    let library_paths = {
        let mut library_paths: Vec<String> = vec![];
        for line in library.lines() {
            if line.contains("\"path\"") {
                let temp = line.trim().split_at(6).1.trim();
                let temp = temp[1..temp.len()-1].to_owned();
                library_paths.push(temp + "/steamapps");
            }
        }

        library_paths
    };

    // search the folders for the appmanifest to find the correct one
    let library_folder = {
        let mut library_folder = None;
        for folder in library_paths {
            let manifest_path = format!("{folder}/appmanifest_{appid}.acf");
            if std::fs::exists(&manifest_path).expect("Expected to be able to check for file in library directory") {
                library_folder = Some(folder);
            }
        }
        library_folder
    };

    // get the appmanifest's content
    let app_manifest =  {
        let mut app_manifest = None;
        if let Some(folder) = &library_folder {
            let manifest_path = format!("{folder}/appmanifest_{appid}.acf");
            if std::fs::exists(&manifest_path).expect("Expected to be able to check for file in library directory") {
                    app_manifest = Some(std::fs::read_to_string(manifest_path).expect("Expected to be able to read appmanifet file"));
            }
        }
        app_manifest
    };

    // get the app's install directory, parsed from the data from the appmanifest
    let install_dir = {
        let mut install_dir: Option<String> = None;
        if let Some(folder) = library_folder {
            if let Some(manifest) = app_manifest {
                for line in manifest.lines() {
                    if line.contains("\"installdir\"") {
                        let temp = line.trim().split_at(12).1.trim();
                        let temp = temp[1..temp.len()-1].to_owned();
                        install_dir = Some(format!("{folder}/common/{temp}"))
                    }
                }
            }
        }
        install_dir
    };

    // finally return the install dir
    return install_dir;
}