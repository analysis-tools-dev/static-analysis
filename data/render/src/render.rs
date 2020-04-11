use crate::types::{Catalog, Categories};
use std::error::Error;
use tera::{Context, Tera};

pub fn render(
    template: &str,
    catalog: Catalog,
    categories: Categories,
) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    context.insert("catalog", &catalog);
    context.insert("categories", &categories);
    Ok(Tera::one_off(template, &context, true)?)
}
