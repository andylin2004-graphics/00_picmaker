use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Copy, Clone)]
struct ColorInfo{
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl ColorInfo{
    fn new() -> ColorInfo{
        ColorInfo{r: 0, g:0, b:0}
    }

    fn to_string(&self) -> String{
        return format!("{} {} {}", self.r, self.g, self.b);
    }
}

fn main(){
    let path = Path::new("imageFile.ppm");

    let mut file = match File::create(&path){
        Err(error) => panic!("failed to create image file because {}", error),
        Ok(file) => file,
    };

    let result = create_data();

    match file.write_all(result.as_bytes()){
        Err(error) => panic!("failed to write image file because {}", error),
        Ok(_) => print!(""),
    };
}

fn create_data() -> String{
    let mut result: String = "P3\n500 500\n255\n".to_owned();
    let mut array: [[ColorInfo; 500]; 500] = [[ColorInfo::new(); 500]; 500];
    let mut counter = 0;
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for i in 0..array.len(){
        for v in 0..array[i].len(){
            match counter%3{
                0=>r += 1,
                1=>g += 1,
                _=>b += 1,
            }
            array[i][v].r = r;
            array[i][v].g = g;
            array[i][v].b = b;
        }
        counter = 0;
        r = 0;
        g = 0;
        b = 0;
    }
 
    for i in 0..array.len(){
        for v in 0..array[i].len(){
            result.push_str(&array[i][v].to_string().to_owned());
            result.push_str(" ");
        }
        result.push_str("\n");
    }

    return result;
}