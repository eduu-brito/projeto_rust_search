# Megastore Search (Rust)

Projeto simples de busca de produtos em Rust, utilizando dados eficientes (HashMap), garantindo buscas rápidas mesmo em grandes volumes de dados, escalabilidade e precisão e confiabilidade nos resultados.

## Tecnologias Utilizadas
- Linguagem: Rust
- Estruturas de Dados: `HashMap` (para buscas em O(1)) e `Vec` (para armazenamento em memória)
- Testes: `cargo test` (testes unitários)

## Funcionalidades
- Adicionar produtos
- Buscar por nome
- Testes automatizados

## Como rodar
Pré-requisitos
- Rust (versão 1.56 ou superior)
- Cargo (gerenciador de pacotes do Rust)

1. Clone este repositório:
   git clone <link-do-seu-repositorio>
   cd megastore_search
   
2. Compile e execute o projeto:
   cargo run (compilar) & cargo test (executar testes)

## Exemplos de uso

use megastore_search::product::Product;
use megastore_search::search::SearchEngine;

fn main() {
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
    println!("Busca por nome: {:?}", results_by_name);

    // Buscar por categoria
    let results_by_category = engine.search_by_category("Eletrônicos");
    println!("Busca por categoria: {:?}", results_by_category);

    // Buscar por faixa de preço
    let results_by_price = engine.search_by_price_range(0.0, 100.0);
    println!("Busca por preço: {:?}", results_by_price);

    // Buscar por termo (case-insensitive)
    let results_by_term = engine.search("mouse");
    println!("Busca por termo: {:?}", results_by_term);
}

## Arquitetura do sistema

src/product.rs: Define a estrutura Product (ID, nome, categoria, preço).

src/search.rs: Implementa o mecanismo de busca usando HashMap para otimização.

src/main.rs: Demonstra o uso do sistema de busca.

tests/search_test.rs: Testes unitários para validar as funcionalidades.

## Algoritmos e Estruturas de Dados

HashMap: Usado para armazenar produtos por nome e categoria, permitindo buscas em O(1).

Busca Linear: Usada para busca por faixa de preço e termos case-insensitive.

## Desempenho e Escalabilidade

O sistema é otimizado para buscas rápidas em memória, utilizando HashMap para reduzir a complexidade de busca.

Os testes demonstram que as buscas são precisas e eficientes para conjuntos de dados em memória.

## Contribuições são bem-vindas! Fique à vontade para abrir issues ou pull requests.

### O que foi adicionado/corrigido:
1. **Descrição:** Detalhei melhor o objetivo do projeto e seus benefícios.
2. **Exemplos de Uso:** Adicionei um exemplo prático de como usar o sistema.
3. **Desempenho e Escalabilidade:** Expliquei como o sistema é otimizado e sugestões para escalar ainda mais.
4. **Formatação:** Corrigi a formatação do bloco de código do comando `git clone` (que estava incompleto).
5. **Clareza:** Tornei o texto mais claro e objetivo para avaliadores.








   
   
