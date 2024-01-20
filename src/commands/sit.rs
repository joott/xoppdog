use notify::{
    Watcher,
    RecursiveMode,
    event::{Event, EventKind, ModifyKind},
};
use std::{
    path::PathBuf,
    io::{self, Error},
    process::Command,
    fs
};

pub fn start_sit(root: PathBuf) {
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
           Ok(event) => handle_event(event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();

    if !root.try_exists().unwrap() {
        fs::create_dir_all(&root).unwrap();
    }

    watcher.watch(&root, RecursiveMode::NonRecursive).unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input.");
}

fn handle_event(event: Event) {
    let _event_result = match event.kind {
        EventKind::Modify(ModifyKind::Data(_)) => compile(event),
        EventKind::Create(_) => compile(event),
        _ => Ok(()),
    };
}

fn compile(event: Event) -> io::Result<()> {
    let path = &event.paths[0];
    if path.extension().unwrap() == "xopp" {
        let pdf_path = path.with_extension("pdf");
        let pdf_path_str = pdf_path.to_str().unwrap();

        let xournal_status = Command::new("xournalpp")
            .arg(path.as_os_str())
            .args(["-p", pdf_path_str])
            .arg("--export-no-background")
            .arg("--export-no-ruling")
            .status()?;
        
        match xournal_status.success() {
            true => println!("exported to {}", pdf_path_str),
            false => return Err(Error::other("couldn't export xopp to pdf.")),
        }

        let inkscape_status = Command::new("inkscape")
            .arg(pdf_path_str)
            .arg("--export-type=pdf")
            .arg("--export-latex")
            .arg(format!("--export-filename={}", pdf_path_str))
            .status()?;
        
        match inkscape_status.success() {
            true => println!("created pdf_tex from {}", pdf_path_str),
            false => return Err(Error::other("couldn't export pdf to pdf_tex.")),
        }
    }

    Ok(())
}
