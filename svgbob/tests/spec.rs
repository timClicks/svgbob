extern crate svgbob;



#[test]
fn test_dash_alone(){
    use svgbob::{Grid,Settings};

    let g = Grid::from_str("-", &Settings::separate_lines());
    let result = g.get_svg_nodes_only();
    let expected = r#"<text>-</text>"#;
    assert_eq!(result, expected);
}
#[test]
fn test_dash_line2(){
    use svgbob::{Grid,Settings};

    let g = Grid::from_str("--", &Settings::separate_lines());
    let result = g.get_svg_nodes_only();
    let expected = r#"<line x1="0" x2="16" y1="8" y2="8"/>"#;
    assert_eq!(result, expected);
}
