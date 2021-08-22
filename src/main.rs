pub mod repository;
pub mod repositories;
pub mod file;

///
/// Entry point 
/// 
fn main() {
    repositories::Repositories::from(
        file::File::from(
            "repositories.txt"
        ).content()
    ).pull();
}
