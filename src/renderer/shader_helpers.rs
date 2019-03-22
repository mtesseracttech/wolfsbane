use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub struct Shader {
    vertex: &'static str,
    fragment: &'static str,
    geometry: Option<&'static str>,
}

impl Shader {
    #[allow(dead_code)]
    pub const PHONG: Self = Self {
        vertex: include_str!("../../res/shaders/phong/phong.vert"),
        fragment: include_str!("../../res/shaders/phong/phong.frag"),
        geometry: None,
    };

    #[allow(dead_code)]
    pub const GOURAUD: Self = Self {
        vertex: include_str!("../../res/shaders/gouraud/gouraud.vert"),
        fragment: include_str!("../../res/shaders/gouraud/gouraud.frag"),
        geometry: None,
    };

    #[allow(dead_code)]
    pub const COLORED2D: Self = Self {
        vertex: include_str!("../../res/shaders/colored2d/colored2d.vert"),
        fragment: include_str!("../../res/shaders/colored2d/colored2d.frag"),
        geometry: None,
    };

    #[allow(dead_code)]
    pub const NORMALS: Self = Self {
        vertex: include_str!("../../res/shaders/normals/normals.vert"),
        fragment: include_str!("../../res/shaders/normals/normals.frag"),
        geometry: None,
    };


    #[allow(dead_code)]
    pub fn load(display: &glium::Display, shader_type: Self) -> Option<glium::Program> {
        Shader::compile_shader(display, shader_type.vertex, shader_type.fragment, shader_type.geometry)
    }

    #[allow(dead_code)]
    pub fn load_dynamic(display: &glium::Display, shader_name: &str) -> Option<glium::Program> {
        let folder_location = format!("res/shaders/{}/", shader_name);
        if !Shader::path_exists(folder_location.as_str()) {
            println!("There is no folder for {}", shader_name);
            None
        } else {
            let shader_location = format!("{}{}", folder_location, shader_name);
            let vert_shader_path = format!("{}.vert", shader_location);
            let frag_shader_path = format!("{}.frag", shader_location);
            let geom_shader_path = format!("{}.geom", shader_location);
            let geom_shader = if Shader::path_exists(geom_shader_path.as_str()) { Some(geom_shader_path.as_str()) } else { None };
            Shader::read_shader(display, vert_shader_path.as_str(), frag_shader_path.as_str(), geom_shader)
        }
    }

    fn path_exists(file_path: &str) -> bool {
        let filepath = Path::new(file_path);
        filepath.exists()
    }

    fn read_shader(display: &glium::Display, vert_path: &str, frag_path: &str, geom_path: Option<&str>) -> Option<glium::Program> {
        let vert_source = match Shader::read_file(vert_path) {
            Ok(source) => source,
            Err(e) => {
                println!("Error reading vertex shader source file:\n{:?}", e);
                return None;
            }
        };
        let frag_source = match Shader::read_file(frag_path) {
            Ok(source) => source,
            Err(e) => {
                println!("Error reading fragment shader source file:\n{:?}", e);
                return None;
            }
        };

        let mut geom_src = String::new();
        let geom_source = if geom_path.is_none() {
            None
        } else {
            match Shader::read_file(geom_path.unwrap()) {
                Ok(source) => {
                    geom_src = source;
                    Some(geom_src.as_str())
                }
                Err(e) => {
                    println!("Error reading geometry shader source file:\n{:?}", e);
                    return None;
                }
            }
        };
        Shader::compile_shader(display, &vert_source, &frag_source, geom_source)
    }

    fn compile_shader(display: &glium::Display, vert_src: &str, frag_src: &str, geom_src: Option<&str>) -> Option<glium::Program> {
        let result = glium::Program::from_source(display, vert_src, frag_src, geom_src);
        match result {
            Ok(r) => Some(r),
            Err(err) => {
                match err {
                    glium::CompilationError(msg) => println!("{:?}", msg),
                    glium::LinkingError(msg) => println!("{:?}", msg),
                    _ => println!("{:?}", err)
                }
                None
            }
        }
    }

    fn read_file(file_path: &str) -> std::io::Result<String> {
        let file = File::open(file_path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}