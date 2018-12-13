use std::io;

/// Download [image] from the url and put it to the stdout
pub fn download(base_url: &str, path: &str) {
    let url = format!("{}{}", base_url, path);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    reqwest::get(&url)
        .expect(&format!("Cannot download {}", url))
        .copy_to(&mut handle)
        .expect("Cannot copy to stdout");
}
