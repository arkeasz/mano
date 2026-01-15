use rocket::http::Status;
use rocket_dyn_templates::{Template, context};
use markdown;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::{self, File};
use std::ffi::OsString;
use regex::Regex;

fn directory() -> Result<Vec<Vec<String>>, Status> {
    let path = Path::new("./docs");
    let mut titles: Vec<Vec<String>> = vec![
        vec![], // title
        vec![]  // filename
    ];
    if !path.exists() || !path.is_dir() {
        return Err(Status::NotFound);
    }

    let mut entries: Vec<fs::DirEntry> = fs::read_dir(path)
        .map_err(|_| Status::InternalServerError)?
        .filter_map(|en| en.ok()) 
        .collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let file_name: OsString = entry.file_name();
        let re = Regex::new(r"^\d+_(.+)$").unwrap();
        if let Some(caps) = re.captures(&file_name.to_string_lossy().to_string()) {
            titles[1].push(caps[1].to_string());
        }
        let str_path = format!("./docs/{}", file_name.as_os_str().to_str().unwrap());
        println!("This is the path: {:?}", str_path);
        let file_path = Path::new(&str_path);
        let open = File::open(file_path);
        match open {
            Ok(open) => {
                let reader = BufReader::new(open);
                if let Some(first_line) = reader.lines().next() {
                    match first_line {
                        Ok(line) => titles[0].push(line),
                        Err(e) => println!("Error {}", e),
                    }
                }    
            },
            Err(e) => println!("Cant open the file. {}", e)
        }   
    }

    Ok(titles)
}

#[get("/")]
pub fn index() -> Result<Template, Status> {
    let mut ctx = HashMap::new();
    
    match directory() {
        Ok(directory) => ctx.insert("directory", directory),
        Err(e) => {
            println!("Ocurr an error {}", e);
            ctx.insert("directory", vec![vec!["Unable to load documents".to_string()], vec!["#".to_string()]])
        },
    };

    Ok(Template::render("docs", &ctx))
}


#[get("/<post>")]
pub fn posts(post: String) -> Result<Template, Status> {
    let path = Path::new("./docs");
    if !path.exists() || !path.is_dir() {
        return Err(Status::NotFound);
    }
    let mut contents: String = String::from("empty");

    for entry in fs::read_dir(path).map_err(|_| Status::InternalServerError)? {
        match entry {
            Ok(entry) => {
                if entry.file_name().to_string_lossy().to_string().contains(&post) {
                    let content = fs::read_to_string(entry.path()).map_err(|_| Status::NotFound);                    
                    contents = markdown::to_html(&content?);
                }
            },
            Err(e) => print!("Error")
        }
    }

    let dir: Vec<Vec<String>>;
    match directory() {
        Ok(directory) => dir =  directory,
        Err(e) => {
            println!("Ocurr an error {}", e);
            dir =  vec![vec!["Unable to load documents".to_string()], vec!["#".to_string()]]
        },
    };

    Ok(Template::render("post", context! {
        contents: contents,
        directory: dir
    }))
}