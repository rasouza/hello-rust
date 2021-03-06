mod basics;
mod utils;
mod structs;
mod modules;
mod functions;

fn main() {

    basics::string_interpolation();
    basics::literals();
    basics::tuples();
    basics::arrays();
    basics::casting();
    basics::inference();
    basics::aliasing();
    basics::expressions();
    basics::conversion();

    basics::flow_of_control::loop_clause();
    basics::flow_of_control::if_clause();
    basics::flow_of_control::while_clause();
    basics::flow_of_control::for_clause();
    basics::flow_of_control::match_clause();

    structs::basics();
    structs::enums();

    modules::visibility();

    functions::closures();
    functions::input_argument();
    // functions::output_parameter();
    // functions::high_order();
    // functions::diverging();
}