use megastore_search::product::Product;
use megastore_search::search::SearchEngine;

fn main() {
    // Criar uma instância do SearchEngine
    let mut engine = SearchEngine::new();

    // Adicionar produtos
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

    // Buscar por nome
    let results_by_name = engine.search_by_name("Teclado");
    println!("Busca por nome 'Teclado': {:?}", results_by_name);

    // Buscar por categoria
    let results_by_category = engine.search_by_category("Eletrônicos");
    println!("Busca por categoria 'Eletrônicos': {:?}", results_by_category);

    // Buscar por faixa de preço
    let results_by_price = engine.search_by_price_range(0.0, 100.0);
    println!("Busca por preço entre 0 e 100: {:?}", results_by_price);

    // Buscar por termo (case-insensitive)
    let results_by_term = engine.search("mouse");
    println!("Busca por termo 'mouse': {:?}", results_by_term);
}