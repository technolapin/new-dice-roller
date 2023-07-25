extern crate lalrpop;

fn main()
{
    // lalrpop::Configuration::new()
    //     .process_root()
    //     .unwrap();
    // lalrpop::Configuration::new()
    //     .process_file("src/expression.lalrpop")
    //     .unwrap();
    lalrpop::process_root().unwrap();
}
