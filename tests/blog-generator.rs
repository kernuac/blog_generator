#[cfg(test)]
use blog_generator;
#[test]
fn intizalizing_with_blank() {
    let meta = blog_generator::metadata::meta::Metadata::new("".to_string(), "".to_string(), "".to_string());
    assert_eq!("", format!("{}{}{}", meta.title, meta.author, meta.description ));
}
