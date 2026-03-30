use megastore_search::product::Product;
use megastore_search::search::SearchEngine;

#[test]
fn test_search_by_name() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product {
        id: 1,
        name: "Teclado".to_string(),
        category: "Eletrônicos".to_string(),
        price: 200.0,
    });

    let result = engine.search_by_name("Teclado");
    assert_eq!(result.len(), 1);
}

#[test]
fn test_search_by_category() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product {
        id: 1,
        name: "Teclado".to_string(),
        category: "Eletrônicos".to_string(),
        price: 200.0,
    });

    let result = engine.search_by_category("Eletrônicos");
    assert_eq!(result.len(), 1);
}

#[test]
fn test_search_by_price_range() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product {
        id: 1,
        name: "Teclado".to_string(),
        category: "Eletrônicos".to_string(),
        price: 200.0,
    });

    engine.add_product(Product {
        id: 2,
        name: "Mouse".to_string(),
        category: "Eletrônicos".to_string(),
        price: 50.0,
    });

    let result = engine.search_by_price_range(0.0, 100.0);
    assert_eq!(result.len(), 1);
}

#[test]
fn test_search_case_insensitive() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product {
        id: 1,
        name: "Teclado".to_string(),
        category: "Eletrônicos".to_string(),
        price: 200.0,
    });

    let result = engine.search("teclado");
    assert_eq!(result.len(), 1);
}