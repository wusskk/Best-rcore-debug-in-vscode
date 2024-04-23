use std::io::{Result, Write};
use std::fs::{File, read_dir, write};

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
    load_app_dbg();
}

static TARGET_PATH: &str = "../user/build/elf/";

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<_> = read_dir("../user/src/bin/")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    apps.sort();

    writeln!(f, r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#, apps.len())?;

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    writeln!(f, r#"
    .global _app_names
_app_names:"#)?;
    for app in apps.iter() {
        writeln!(f, r#"    .string "{}""#, app)?;
    }

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(f, r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
    .align 3
app_{0}_start:
    .incbin "{2}{1}.elf"
app_{0}_end:"#, idx, app, TARGET_PATH)?;
    }
    Ok(())
}

fn load_app_dbg() {
    let paths = read_dir("../user/build/dbg").unwrap();
    let mut commands = String::new();

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap() == "dbg" {
            commands += &format!("add-symbol-file {}\n", path.canonicalize().unwrap().display());
        }
    }

    write("../user/build/dbg/commands.gdb", commands).unwrap();
}