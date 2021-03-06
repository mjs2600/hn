mod sites;

fn main() {
    match sites::hacker_news() {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching Hacker News data."),
    }

    match sites::datatau() {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching DataTau data."),
    }

    match sites::lobsters() {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching DataTau data."),
    }
}
