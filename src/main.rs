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
                "repositories.txt"
            ).as_str()
        ).content()
    ).pull();
}
