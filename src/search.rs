use std::collections::HashMap;
use crate::product::Product;

pub struct SearchEngine {
    products_by_name: HashMap<String, Vec<Product>>,
    products_by_category: HashMap<String, Vec<Product>>,
    products: Vec<Product>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            products_by_name: HashMap::new(),
            products_by_category: HashMap::new(),
            products: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product.clone());

        self.products_by_name
            .entry(product.name.clone())
            .or_default()
            .push(product.clone());

        self.products_by_category
            .entry(product.category.clone())
            .or_default()
            .push(product);
    }

    pub fn search_by_name(&self, query: &str) -> Vec<&Product> {
        self.products_by_name
            .get(query)
            .map(|products| products.iter().collect())
            .unwrap_or_default()
    }

    pub fn search_by_category(&self, query: &str) -> Vec<&Product> {
        self.products_by_category
            .get(query)
            .map(|products| products.iter().collect())
            .unwrap_or_default()
    }

    pub fn search_by_price_range(&self, min_price: f64, max_price: f64) -> Vec<&Product> {
        self.products
            .iter()
            .filter(|p| p.price >= min_price && p.price <= max_price)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Product> {
        self.products
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}