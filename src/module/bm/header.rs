use module::helpers::header as headerhelpers;
use storage::file::header::data::FileHeaderData as FHD;
use storage::file::header::spec::FileHeaderSpec as FHS;

pub fn get_spec() -> FHS {
    FHS::Map {
        keys: vec![
            headerhelpers::tags::spec::url_key(),
            headerhelpers::tags::spec::tags_key(),
        ]
    }
}

pub fn build_header(url: String, tags: Vec<String>) -> FHD {
    FHD::Map {
        keys: vec![
            FHD::Key {
                name: String::from("URL"),
                value: Box::new(FHD::Text(url.clone()))
            },
            FHD::Key {
                name: String::from("TAGS"),
                value: Box::new(headerhelpers::tags::data::build_tag_array(tags))
            }
        ]
    }
}

pub fn get_tags_from_header(header: &FHD) -> Vec<String> {
    headerhelpers::tags::data::get_tags_from_header(header)
}

pub fn get_url_from_header(header: &FHD) -> Option<String> {
    headerhelpers::data::get_url_from_header(header)
}

pub fn rebuild_header_with_tags(header: &FHD, tags: Vec<String>) -> Option<FHD> {
    get_url_from_header(header).map(|url| build_header(url, tags))
}
