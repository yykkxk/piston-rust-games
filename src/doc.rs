use std::io::{Write, Read, Seek, SeekFrom};
use std::fs::{File, OpenOptions};

fn read_file(path: &str, seek_start: u64) -> Result<String, String> {
    File::open(path).map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.seek(SeekFrom::Start(seek_start)).unwrap();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
}

fn write_new_file(data: &str, dest: &str) {
    File::create(dest)
        .and_then(|mut file| {
            file.write_all(data.as_bytes())
        }).unwrap();
}

fn append_file(data: &str, dest: &str) {
    let w = OpenOptions::new().write(true).create(true).open(dest)
    .map_err(|err| {
        err
    }).and_then(|mut file| {
        file.seek(SeekFrom::End(-10)).unwrap();
        file.write_all(data.as_bytes())
    });

    match w {
        Err(e) => println!("{:?}", e),
        Ok(ok) => println!("{:?}", ok),
    }
}

fn main() {
    let doc_name = "doc_tmp/5.00.asc".to_string();
    let contents = read_file(&doc_name, 175).unwrap();
    let mut vec_str = contents.trim().split("\r\n").collect::<Vec<_>>();
    println!("{:?}", vec_str.pop());

    let mut ss = String::new();
    for i in &vec_str {
        if i.len() > 12 {
            let sl = &i[11..];
            let tr = sl.trim();
            ss.push_str(&tr[0..6]);
            ss.push_str("\n");
        }
    }
    println!("{:?}", ss);

    // write_new_file(&ss, "111_reflect.asc");
    write_new_file(&ss, "000trim.asc");
    // append_file("append", "reflect.asc");

    // let mut ss2 = String::new();
    // for i in 350..1051 {
    //     ss2.push_str(&format!("{}", i));
    //     ss2.push_str("\n");
    // }
    // println!("{:?}", ss2);
    // write_new_file(&ss2, "350_1050.asc");
}
