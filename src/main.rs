use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer.pest"]
pub struct FeynmanParser;

fn main() {
    let diagram = "
    exterline e1, e2, mu1, mu2;
    vertex a, b;

    e1 -> a ~p~ b -> mu1;
    e2 -> a;
    b -> mu2;
";

    let successful_parse = FeynmanParser::parse(Rule::diagram, diagram).unwrap();
    println!("{:?}", successful_parse);
}
