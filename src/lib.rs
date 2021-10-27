use std::error::Error;
use std::fs;
use std::collections::HashMap;

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { query, filename })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
   // println!("With text:\n{}", contents);
    stat(&contents);
    Ok(())
}
pub fn stat (contents: &str) {
    
    let mut vect:Vec<&str> = Vec::new();
    let mut vectlower:Vec<String> = Vec::new();
    let mut map = HashMap::new();
    
    for line in contents.lines() {
        
        let split = line.split_whitespace();
        for s in split {
            let mut m = s;
           // let mut m = &s.to_lowercase().as_str();
            
            if value_have_bad_end(s) == true { m = rem_last(s); }
          //  println!("{}  - {}  -    {}", m , m.len(),s.chars().last().unwrap())
          vect.push(m);
          vectlower.push(low(m));
        }
        
    }
    
    for word in vectlower {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
    
    
    
}
pub fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
   // chars.next();
    chars.next_back();
    chars.as_str()
}
pub fn value_have_bad_end(value: &str) ->bool {
    let s = value.chars().last().unwrap();
    let mut z: bool = false;
    if s == ',' { z = true; }
    if s == '.' { z = true; }
    if s == '?' { z = true; }
    if s == '!' { z = true; }
    if s == ';' { z = true; }
    z
    
}
pub fn low(s: &str) -> String {
 let st:String = String::from(s)   ;
 let mut g = st.to_lowercase();
 g
    
}
