# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore 

 

## Descrição do Projeto 

Este projeto implementa um sistema de busca eficiente para um catálogo de produtos utilizando a linguagem Rust. O objetivo é permitir buscas rápidas e escaláveis em grandes volumes de dados, simulando um cenário real de e-commerce. 

 

## Tecnologias Utilizadas 

• Rust 

• std::collections::HashMap 

 

## Como Executar 

1. Instale o Rust: https://rustup.rs  

2. Execute o projeto: 

cargo run 

 

## Como Executar os Testes 

cargo test 

 

## Exemplos de Uso 

- Buscar por nome 

- Buscar por categoria 

- Buscar por marca 

Exemplo: 

Buscar por categoria "Eletrônicos": 

Saída: 

Teclado Mecânico - MegaGamer 

Mouse Gamer - MegaGamer 

  

## Arquitetura do Sistema 

O sistema utiliza múltiplos índices baseados em tabelas hash: 

- Índice por nome 

- Índice por categoria 

- Índice por marca 

Cada índice armazena uma lista de produtos, permitindo múltiplos resultados por busca. 

  

## Estruturas de Dados e Algoritmos 

Foi utilizada a estrutura `HashMap`, que permite: 

- Inserção em tempo médio O(1) 

- Busca em tempo médio O(1) 

Isso garante alta performance mesmo com grandes volumes de dados. 

  

## Desempenho e Escalabilidade 

O sistema é altamente escalável devido ao uso de tabelas hash. Ele pode lidar com milhões de produtos mantendo tempo de resposta baixo. 

  

## Métricas de Avaliação 

- Tempo de resposta das buscas 

- Uso de memória 

- Tempo de inserção 

  

## Considerações 

O sistema pode ser expandido para: 

- Suporte a busca parcial 

- Integração com banco de dados 

- API REST 

  

## Contribuições 

Contribuições são bem-vindas! 

 

## Licença 

Projeto acadêmico. 