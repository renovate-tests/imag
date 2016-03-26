use libimagstore::storeid::StoreId;
use libimagstore::store::Entry;
use libimagstore::store::EntryHeader;
use libimagstore::store::Result as StoreResult;

use error::{LinkError, LinkErrorKind};
use result::Result;

use toml::Value;

pub type Link = String;

pub trait InternalLinker {

    /// Get the internal links from the implementor object
    fn get_internal_links(&self) -> Result<Vec<Link>>;

    /// Set the internal links for the implementor object
    fn set_internal_links(&mut self, links: Vec<&mut Entry>) -> Result<Vec<Link>>;

    /// Add an internal link to the implementor object
    fn add_internal_link(&mut self, link: &mut Entry) -> Result<()>;

    /// Remove an internal link from the implementor object
    fn remove_internal_link(&mut self, link: &mut Entry) -> Result<()>;

}

impl InternalLinker for Entry {

    fn get_internal_links(&self) -> Result<Vec<Link>> {
        process_rw_result(self.get_header().read("imag.links"))
    }

    /// Set the links in a header and return the old links, if any.
    fn set_internal_links(&mut self, links: Vec<&mut Entry>) -> Result<Vec<Link>> {
        let self_location = self.get_location().clone();
        let mut new_links = vec![];

        for link in links {
            if let Err(e) = add_foreign_link(link, self_location.clone()) {
                return Err(e);
            }
            let link = link.get_location().clone();
            let link = link.to_str();
            if link.is_none() {
                return Err(LinkError::new(LinkErrorKind::InternalConversionError, None));
            }
            let link = link.unwrap();

            new_links.push(Value::String(String::from(link)));
        }

        process_rw_result(self.get_header_mut().set("imag.links", Value::Array(new_links)))
    }

    fn add_internal_link(&mut self, link: &mut Entry) -> Result<()> {
        let new_link = link.get_location().clone();
        let new_link = new_link.to_str();
        if new_link.is_none() {
            return Err(LinkError::new(LinkErrorKind::InternalConversionError, None));
        }
        let new_link = new_link.unwrap();

        add_foreign_link(link, self.get_location().clone())
            .and_then(|_| {
                self.get_internal_links()
                    .and_then(|mut links| {
                        links.push(String::from(new_link));
                        let links = links.into_iter().map(|s| Value::String(s)).collect();
                        let process = self.get_header_mut().set("imag.links", Value::Array(links));
                        process_rw_result(process)
                            .map(|_| ())
                    })
            })
    }

    fn remove_internal_link(&mut self, link: &mut Entry) -> Result<()> {
        let own_loc = link.get_location().clone();
        let own_loc = own_loc.to_str();
        if own_loc.is_none() {
            return Err(LinkError::new(LinkErrorKind::InternalConversionError, None));
        }
        let own_loc = own_loc.unwrap();

        let other_loc = link.get_location().clone();
        let other_loc = other_loc.to_str();
        if other_loc.is_none() {
            return Err(LinkError::new(LinkErrorKind::InternalConversionError, None));
        }
        let other_loc = other_loc.unwrap();

        link.get_internal_links()
            .and_then(|mut links| {
                let links = links.into_iter()
                    .filter(|l| l.clone() != own_loc)
                    .map(|s| Value::String(s))
                    .collect();
                process_rw_result(link.get_header_mut().set("imag.links", Value::Array(links)))
                    .map(|_| ())
            })
            .and_then(|_| {
                self.get_internal_links()
                    .and_then(|mut links| {
                        let links = links
                            .into_iter()
                            .filter(|l| l.clone() != other_loc)
                            .map(|s| Value::String(s))
                            .collect();
                        process_rw_result(self.get_header_mut().set("imag.links", Value::Array(links)))
                            .map(|_| ())
                    })
            })
    }

}

/// When Linking A -> B, the specification wants us to link back B -> A.
/// This is a helper function which does this.
fn add_foreign_link(target: &mut Entry, from: StoreId) -> Result<()> {
    let from = from.to_str();
    if from.is_none() {
        debug!("Cannot convert pathbuf '{:?}' to String", from);
        return Err(LinkError::new(LinkErrorKind::InternalConversionError, None));
    }
    let from = from.unwrap();

    target.get_internal_links()
        .and_then(|mut links| {
            links.push(String::from(from));
            let links = links.into_iter().map(|s| Value::String(s)).collect();
            process_rw_result(target.get_header_mut().set("imag.links", Value::Array(links)))
                .map(|_| ())
        })
}

fn process_rw_result(links: StoreResult<Option<Value>>) -> Result<Vec<Link>> {
    if links.is_err() {
        let lerr  = LinkError::new(LinkErrorKind::EntryHeaderReadError,
                                   Some(Box::new(links.err().unwrap())));
        return Err(lerr);
    }
    let links = links.unwrap();

    if links.iter().any(|l| match l { &Value::String(_) => true, _ => false }) {
        return Err(LinkError::new(LinkErrorKind::ExistingLinkTypeWrong, None));
    }

    let links : Vec<Link> = links.into_iter()
        .map(|link| {
            match link {
                Value::String(s) => String::from(s),
                _ => unreachable!(),
            }
        })
        .collect();

    Ok(links)
}
