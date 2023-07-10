use dreamlogic::open_url;
fn main() {
    let url = "http://dreamlogic.io";
    println!("Opening the {url}...");
    open_url::open_url(url);
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_test()
    {
        // super::main();
    }
}