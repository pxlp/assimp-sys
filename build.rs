extern crate pkg_config;

fn main() {
    if let Ok(..) = pkg_config::Config::new().atleast_version("3.2.0").find("assimp") {
        return
    }
}
