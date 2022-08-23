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

    #test[]
}

