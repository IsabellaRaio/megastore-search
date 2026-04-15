use std::collections::HashMap; 

#[derive(Debug, Clone)] 

pub struct Produto { 

    pub id: u32, 

    pub nome: String, 

    pub categoria: String, 

    pub marca: String, 

} 

pub struct Catalogo { 

    por_nome: HashMap<String, Vec<Produto>>, 

    por_categoria: HashMap<String, Vec<Produto>>, 

    por_marca: HashMap<String, Vec<Produto>>, 

} 

impl Catalogo { 

    pub fn new() -> Self { 

        Catalogo { 

            por_nome: HashMap::new(), 

            por_categoria: HashMap::new(), 

            por_marca: HashMap::new(), 

        } 

    } 

    pub fn adicionar_produto(&mut self, produto: Produto) { 

        let nome = produto.nome.to_lowercase(); 

        let categoria = produto.categoria.to_lowercase(); 

        let marca = produto.marca.to_lowercase(); 

        self.por_nome.entry(nome).or_insert(Vec::new()).push(produto.clone()); 

        self.por_categoria.entry(categoria).or_insert(Vec::new()).push(produto.clone()); 

        self.por_marca.entry(marca).or_insert(Vec::new()).push(produto); 

    } 

    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Vec<Produto>> { 

        self.por_nome.get(&nome.to_lowercase()) 

    } 

    pub fn buscar_por_categoria(&self, categoria: &str) -> Option<&Vec<Produto>> { 

        self.por_categoria.get(&categoria.to_lowercase()) 

    } 

    pub fn buscar_por_marca(&self, marca: &str) -> Option<&Vec<Produto>> { 

        self.por_marca.get(&marca.to_lowercase()) 

    } 

} 

fn main() { 

    let mut catalogo = Catalogo::new(); 

    catalogo.adicionar_produto(Produto { 

        id: 1, 

        nome: "Teclado Mecânico".to_string(), 

        categoria: "Eletrônicos".to_string(), 

        marca: "MegaGamer".to_string(), 

    }); 

    catalogo.adicionar_produto(Produto { 

        id: 2, 

        nome: "Mouse Gamer".to_string(), 

        categoria: "Eletrônicos".to_string(), 

        marca: "MegaGamer".to_string(), 

    }); 

    if let Some(produtos) = catalogo.buscar_por_categoria("Eletrônicos") { 

        for p in produtos { 

            println!("{} - {}", p.nome.as_str(), p.marca.as_str()); 

        } 

    } 

} 

#[cfg(test)] 

mod tests { 

    use super::*; 

    #[test] 

    fn test_busca_categoria() { 

        let mut cat = Catalogo::new(); 

        cat.adicionar_produto(Produto { 

            id: 1, 

            nome: "Cadeira".to_string(), 

            categoria: "Móveis".to_string(), 

            marca: "MegaHome".to_string(), 

        }); 

        assert!(cat.buscar_por_categoria("Móveis").is_some()); 

    } 

} 