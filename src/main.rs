// mod output;
// mod variables;
// mod flow_control;
// mod structures;
mod adventofcode2019;

fn main() {
    // output::hello_world();
    // variables::basic();
    // flow_control::basic();
    // structures::basic();
    let res = adventofcode2019::day1();
    println!("{:?}", res);
}
