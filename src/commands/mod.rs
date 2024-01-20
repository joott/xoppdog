use std::{
    path::PathBuf,
    process::{Command, Stdio},
    collections::HashMap,
    io::Write,
    fs,
};
use indoc::formatdoc;

mod sit;
pub use sit::start_sit;

fn open_xournal(path: &PathBuf) {
    let _ = Command::new("xournalpp")
        .arg(path.as_os_str())
        .arg("--name=xoppdoggin")
        .arg("--disable-audio")
        .stderr(Stdio::null())
        .spawn()
        .expect("Unable to launch xournal!");
}

fn latex_template(name: &String, path: &PathBuf) -> String {
    formatdoc! {r"
        \begin{{figure}}[ht]
            \centering
            \incfig{{{}}}
            \caption{{{name}}}
            \label{{fig:{name}}}
        \end{{figure}}
        ",
        path.with_extension("").file_name().unwrap().to_string_lossy()
    }
}

pub fn shake_figure(name: String, root: PathBuf) {
    let template_path = dirs::config_dir()
                   .expect("Could not resolve config directory.")
                   .join("xoppdog/template.xopp");

    if !template_path.try_exists().unwrap() {
        panic!("Please make a template at {}!", template_path.to_str().unwrap());
    }

    if !root.try_exists().unwrap() {
        fs::create_dir_all(&root).unwrap();
    }

    let filename = name.trim().replace(" ", "-");
    let target = root.join(filename).with_extension("xopp");

    fs::copy(template_path, &target).expect("Could not copy template to target file.");

    open_xournal(&target);
    print!("{}", latex_template(&name, &target));
}

pub fn fetch_figure(root: PathBuf) {
    let figures = find_figures(&root);
    let figure_path = pick_figure(&figures);

    open_xournal(&figure_path);
}

fn find_figures(root: &PathBuf) -> HashMap<String, PathBuf> {
    if !root.is_dir() {
        panic!("Root path not a directory!");
    }

    let directory = fs::read_dir(root)
        .expect("Can't sniff out the given figures directory.");

    let mut figures = HashMap::new();

    for entry in directory {
        match entry {
            Ok(entry) => {
                if entry.path().extension().unwrap() == "xopp" {
                    let name = entry.path()
                        .with_extension("")
                        .file_name()
                        .unwrap().to_string_lossy()
                        .replace("-", " ").replace("_", " ");
                    figures.insert(name, entry.path());
                }
            },
            Err(error) => {
                eprintln!("Error reading file in figures directory: {:?}", error);
                continue;
            },
        }
    }

    return figures;
}

fn pick_figure<'a>(figures: &'a HashMap<String, PathBuf>) -> &'a PathBuf{
    let mut options = String::new();
    for key in figures.keys() {
        options.push_str(key);
        options.push_str("\n");
    }

    let mut rofi = Command::new("rofi")
        .arg("-dmenu")
        .args(["-p", "Figures"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().expect("Rofi failed to launch.");

    rofi.stdin.as_mut().unwrap().write(options.trim().as_bytes())
        .expect("Could not send input to Rofi process.");

    let rofi_output = rofi.wait_with_output()
        .expect("Waiting on Rofi failed.");
    let choice = String::from_utf8(rofi_output.stdout)
        .expect("Couldn't make sense of Rofi output.");
    let choice_path = figures.get(choice.trim())
        .expect("Somehow Rofi output didn't match with a figure.");

    return choice_path;
}
