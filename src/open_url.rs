/// Opens a url in default browser
/// # Arguments
/// * url - A url to open in default browser
pub fn open_url(url: &str) {

    use std::process::Command;

    if cfg!(target_os = "windows") {
        panic!("Not implemented yet");
    } else {
        Command::new("xdg-open").arg(url).spawn().expect("Could not open url!");
    }    
}

#[cfg(test)]
mod tests {

    #[test]
    fn open_sample_url()
    {
        // open_url("http://dreamlogic.io");    
    }
}