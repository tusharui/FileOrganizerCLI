use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path , PathBuf};
use walkdir::WalkDir ;

fn categories () -> HashMap<&'static str, &'static str>{
    let mut map = HashMap::new();
    let images = ["jpg","jpeg", "png", "gif","bmp"];
    let docs = ["txt" , "pdf","doc","docx","ppt","pptx"];
    let videos = ["mp4","mkv","avi","mov"];
    let music = ["mp3", "wav", "flac"];

    for ext in images{map.insert(ext, "Images");}
    for ext in docs { map.insert(ext, "Documents");}
    for ext in videos {map.insert(ext, "Videos");}
    for ext in music{map.insert(ext, "Music");}
    map
} 



fn unique_path(mut path:PathBuf )-> PathBuf{
    let mut count =1;
    while path.exists(){
        let file_stem = path.file_stem().unwrap().to_string_lossy();
        let ext = path.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
        let parent = path.parent().unwrap();
        path = parent.join(format!("{}_{}{}", file_stem, count, ext));
        count +=1;
        }
        path 
}


fn organize_files(dir: &Path){
    if !dir.exists(){
        fs::create_dir_all(dir).expect("failed tp create directory");
    }
    let cat_map = categories();
    for entry in WalkDir::new(dir).min_depth(1){
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file(){
            let folder_name = path
            .extension()
            .and_then(|e| e.to_str() )
            .map(|ext| cat_map.get(&ext.to_lowercase()[..]).unwrap_or(&ext).to_string())
            .unwrap_or("others".to_string());

            let folder_path = dir.join(folder_name);
            if !folder_path.exists(){
                fs::create_dir(&folder_path).unwrap();

            }

            let file_name = path.file_name().unwrap();
            let new_path = unique_path(folder_path.join(file_name));

            fs::rename(path,&new_path).unwrap();
            println!("moved: {}-> {}", path.display(), new_path.display());
        }
    }

    println!("all files organized successfully!");
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let dir = if args.len()> 1{
        PathBuf::from(&args[1])
    }else{
        PathBuf::from("./files")
    };

    organize_files(&dir);

}