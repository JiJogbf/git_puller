pub mod repository;
pub mod repositories;
pub mod file;

fn main() {
    repositories::Repositories::from(
        file::File::content(
            String::from(
                "repositores.txt"
            )
        )
    ).pull();
}
