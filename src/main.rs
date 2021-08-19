pub mod repository;
pub mod repositories;
pub mod file;

///
/// Entry point 
/// 
fn main() {
    repositories::Repositories::from(
        file::File::from(
            String::from(
                "repositores.txt"
            ).as_str()
        ).content()
    ).pull();
}
