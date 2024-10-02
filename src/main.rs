use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer.pest"]
pub struct FeynmanParser;

fn main() {
    let diagram = "vertex photon1 photon2 axion
axion ~> photon1
axion ~> photon2";

    let successful_parse = FeynmanParser::parse(Rule::diagram, diagram).unwrap();
    println!("{:?}", successful_parse);
}
