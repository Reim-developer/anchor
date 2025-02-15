use yaml_rust::YamlLoader;

#[test]
pub fn check_yaml_syntax() -> Result<(), String> {
    let yaml_content = r#"
 doe: "a deer, a female deer"
 ray: "a drop of golden sun"
 pi: 3.14159
 xmas: true
 french-hens: 3
 calling-birds:
   - huey
   - dewey
   - louie
   - fred
 xmas-fifth-day:
   calling-birds: four
   french-hens: 3
   golden-rings: 5
   partridges:
     count: 1
     location: "a pear tree"
   turtle-doves: two
    "#;

    match YamlLoader::load_from_str(yaml_content) {
        Ok(_) => Ok(()),
        Err(error) => {
            let line_number = error.marker().line();
            let column = error.marker().col();
            Err(format!(
                "YAML syntax error at line: {}, column: {}",
                line_number, column
            ))
        }
    }
}

#[test]
pub fn yml_parser() {}
