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

#[cfg(test)]
mod tests {
    use std::{io::Stdout, fs::File, process::Stdio};

    fn get_paths() -> std::io::Result<> {
        //sort from smallest to biggest
        let mut entries = fs::read_dir(src)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
        entries.sort();

        //seperate  errors  results

        Ok(entries, )
    }
 
    #[test]
    fn test_1() {
        
        
     let src = "";
     let dst = "";

 
        
        
    }
}

