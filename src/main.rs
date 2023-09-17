use std::env::{args, Args};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::{File, metadata};
use std::io::Read;

type TemplateContext = (Vec<PathBuf>, HashMap<String, String>);

fn main() {
    let mut args = args(); args.next();
    let result = output_from_args(args);

    match result {
        Ok(generated_string) => print!("{generated_string}"),
        Err(err) => {
            eprintln!("{err}\n");
            eprintln!("{USAGE}");
        }
    }
}

fn output_from_args(args: Args) -> Result<String, String> {
    let (files, variables) = parse_args(args)?;
    let contents = concat_files(&files).map_err(handle_io_error)?;
    let generated_string = gen_from_template(&variables, &contents);
    
    return Ok(generated_string);
}

fn parse_args(mut arg_iter: Args) -> Result<TemplateContext, &'static str> {
    let mut variables = HashMap::new();        
    let mut file_paths = Vec::new();

    while let Some(arg) = arg_iter.next() {
        if arg == "--var" || arg == "-v" {
            let pair_str = arg_iter.next().ok_or("Unexpected end to `var` argument")?;
            let mut pair_iter = pair_str.split("=").map(str::trim);
            let key = pair_iter.next().ok_or("Given `var` assignment is not a pair.")?.to_string();
            let value = pair_iter.next().ok_or("No given value in `var` assignment.")?.to_string();

            variables.insert(key, value);
        } else {
            file_paths.push(PathBuf::from(arg));
        }
    }

    return if file_paths.is_empty() {
        Err("No provided path(s).")
    } else {
        Ok((file_paths, variables))
    };
}

fn concat_files(file_paths: &[PathBuf]) -> std::io::Result<String> {
    let file_iter = file_paths.iter().map(File::open);
    let meta_iter = file_paths.iter().map(metadata);
    let mut size = 0;
    let mut file_contents;

    for meta in meta_iter {
        size += meta?.len()
    }

    file_contents = String::with_capacity(size as usize);

    for file in file_iter {
        file?.read_to_string(&mut file_contents)?;
        file_contents.push('\n');
    }

    return Ok(file_contents);
}

fn gen_from_template(variables: &HashMap<String, String>, s: &str) -> String {
    let start_indices = s.match_indices("${");
    let end_indices = s.match_indices("}");
    let indices = std::iter::zip(start_indices, end_indices);
    let mut ret = String::with_capacity(s.len());
    let mut substring_start = 0;

    for ((var_start, _), (var_end, _)) in indices {
        let substring = &s[substring_start..var_start];
        let name = &s[var_start + 2..var_end];
        let value = variables
            .get(name)
            .map(String::as_ref)
            .unwrap_or(&s[var_start..=var_end]);

        ret.push_str(substring);
        ret.push_str(value);
        substring_start = var_end + 1;
    }

    ret.push_str(&s[substring_start..]);

    return ret;
}

const USAGE: &'static str = 
r#"Usage: template [--var pair]... template...
Options:
    template      A list of template paths. These files will be concatanated together.
    -v | --var    A key/value pair (i.e. 'foo=bar')
"#;

fn handle_io_error(error: std::io::Error) -> String {
    return match error {
        _ => error.to_string()
    };
}