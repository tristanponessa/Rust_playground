macro_rules! str{
    // macth like arm for macro
       ($a:expr)=>{
    // macro expand to this code
           {
   // $a and $b will be templated using the value/variable provided to macro
               String::from($a)
           }
       }
}


fn main() {
    ;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::path::{Path, PathBuf};
    use std::str::LinesAny;
    use std::{io::Stdout, fs::File, process::Stdio};
    use std::{fs, io};


    struct DirWalker;
    impl  DirWalker {


        fn list_dir(src: String, which: String) -> io::Result<Vec<PathBuf>> {
            let mut content : Vec<PathBuf> = vec![];
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() && which == "dir" {
                    content.push(path);
                } else if path.is_file() && which == "file" {
                    content.push(path);
                } else if which == "all" {
                    content.push(path);
                }
            }
            Ok(content)
        }



        fn walk_dir(d:String) -> io::Result<HashMap<PathBuf, Vec<PathBuf>>> {
            //content.keys().collect(); causes random data 
            let mut keys : Vec<PathBuf> = vec![];
            //let mut vals : Vec<Vec<PathBuf>> = vec![];

            keys.push(PathBuf::from(&d));
            let mut index = 0;
            let mut cur_key = d;
            loop {
                
                let dirs = Self::list_dir(cur_key.clone(), str!("dir"))?;
                for dir in dirs{
                    //content.insert(PathBuf::from(dir), vec![]);
                    keys.push(PathBuf::from(dir));
                }

                index += 1;
                if index >= keys.len() {
                    break;
                } else {
                    let k = keys[index].to_str().unwrap();
                    cur_key = str!(k)
                }            
            }
            
            let mut content : HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
            for k in keys {
                content.insert(k, vec![]);
            }

            for (dir, elems) in &mut content {
                let s = dir.to_str().unwrap();
                let files = Self::list_dir(str!(s), str!("file"))?;
                *elems = files;
            }
            return Ok(content);
        }


    }

    #[test]
    fn list_dir() {
        let src = "./test_media/media/lvl1/";
        let dst = "./dst_media";

        let content = DirWalker::list_dir(str!(src), str!("all")).unwrap();
        let content_dirs = DirWalker::list_dir(str!(src), str!("all")).unwrap();
        let content_files = DirWalker::list_dir(str!(src), str!("all")).unwrap();

        let expected_all = [str!("cpp2.jpg"),str!("lvl2")];
        let expected_files  = str!("cpp2.jpg");
        let expected_dirs = str!("lvl2");

        assert_eq!();
        assert_eq!();
        assert_eq!();
    }

 
    #[test]
    fn walk_dir() {
        //dependant of list_dir
        
        let src = "./test_media/media";
        let dst = "./dst_media";

        //let content = list_dir(str!(src), str!("all")).unwrap();
        let content = DirWalker::walk_dir(str!(src)).unwrap();

        let expected_dirs = [PathBuf::from(src), PathBuf::from("./test_media/media/lvl1"),PathBuf::from("./test_media/media/lvl1/lvl2")];
        let expected_nb_files:  [usize;3] = [11,1,2];

        for i in 0..3 {
            assert!(content.contains_key(&expected_dirs[i]));
            assert_eq!(content.get(&expected_dirs[i]).unwrap().len(), expected_nb_files[i]);
        }
    }
}

