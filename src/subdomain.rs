use crate::{
    Error,
    model::{CrtShEntry, Subdomain},
};
use std::time::Duration;

use reqwest::blocking::Client;
use std::collections::HashSet;
use trust_dns_resolver::{
    Resolver,
    config::{ResolverConfig, ResolverOpts},
};

pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .map(|entry| {
            entry
                .name_value
                .lines()
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|s| !s.is_empty())
        .filter(|s| !s.contains(&'*'.to_string()))
        .collect();
    subdomains.insert(target.to_string());
    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter(resolves)
        .collect();
    Ok(subdomains);
}
pub fn resolves(domain: &Subdomain) -> bool {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    let dns_resolver = Resolver::new(ResolverConfig::default(), opts)
        .expect("subdomain resolver: building DNS client");
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
