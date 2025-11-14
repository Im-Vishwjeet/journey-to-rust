mod modules;
fn main() {
    modules::mutability::mutability_example();
    modules::shadowing::shadowing_example();
    modules::declare_first::declare_first_example();
    modules::freezing::freezing_example();
    println!("All examples completed successfully!");
}
