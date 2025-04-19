use rayon::prelude::*;
use thiserror::Error;

mod subdomains;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: tricoder <kerkour.com>")]
    CliUsage,
}

fn main() -> Result<(), anyhow::Error> {
    // println!("Hello, world!");
    // we use custom threadpool to improve speed
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    // pool.install is required to use our custom threadpool, instead of rayon's default one
    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomains in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!(" {}", port.port);
            }

            println!("");
        }
    });
    Ok(())
}
