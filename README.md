# Portfólio: Regressão Linear

Este projeto implementa uma biblioteca em Rust para realizar regressão linear, prever valores futuros e calcular métricas estatísticas como MSE (Erro Quadrático Médio) e R² (Coeficiente de Determinação). Ele foi desenvolvido como parte do curso de Análise e Desenvolvimento de Sistemas, na UniFECAF.

## 📋 Funcionalidades

- **Regressão Linear**: Geração de um modelo linear a partir de um conjunto de dados.
- **Previsão**: Cálculo de valores futuros com base no modelo gerado.
- **Métricas Estatísticas**:
  - Cálculo do MSE (Erro Quadrático Médio).
  - Cálculo do R² (Coeficiente de Determinação).

## 🗂️ Estrutura do Projeto

- `src/regression.rs`: Implementação da lógica de regressão linear.
- `src/prediction.rs`: Função para prever valores futuros com base no modelo.
- `src/metrics.rs`: Cálculo de métricas estatísticas (MSE e R²).
- `src/main.rs`: Exemplo de uso da biblioteca.
- `tests/`: Testes unitários para validar as funcionalidades.

## 🚀 Como Usar

### Pré-requisitos

- Rust instalado. Caso não tenha, instale através do [Rustup](https://rustup.rs/).

### Clonar o Repositório

```bash
git clone https://github.com/seu-usuario/portifolio-regressao-linear.git
cd portifolio-regressao-linear
```

### Executar o Projeto

Para rodar o exemplo no arquivo `main.rs`:

```bash
cargo run
```

### Executar os Testes

Para garantir que todas as funcionalidades estão funcionando corretamente:

```bash
cargo test
```

## 💡 Exemplo de Uso

### Entrada

```rust
let dados = vec![3.0, 4.5, 6.0, 7.5];
```

### Saída Esperada

```plaintext
Modelo: y = 1.50x + 3.00
MSE: 0.00
R²: 1.00
Previsões para os próximos períodos: [9.0, 10.5, 12.0]
```

## 🛠️ Estrutura do Modelo

O modelo gerado pela regressão linear é representado pela struct `Modelo`:

```rust
pub struct Modelo {
    pub slope: f64, // Coeficiente angular.
    pub intercept: f64, // Coeficiente linear.
}
```

## 📊 Aplicações Práticas

- **Análise de Tendências**: Identificar padrões em séries temporais.
- **Previsões Financeiras**: Estimar valores futuros com base em dados históricos.
- **Ciência de Dados**: Aplicação em modelos preditivos simples.

## 🤝 Contribuição

Contribuições são bem-vindas! Siga os passos abaixo para contribuir:

1. Faça um fork do repositório.
2. Crie uma branch para sua feature ou correção: `git checkout -b minha-feature`.
3. Faça commit das suas alterações: `git commit -m 'Minha nova feature'`.
4. Envie para o repositório remoto: `git push origin minha-feature`.
5. Abra um Pull Request.

## 📜 Licença

Este projeto está licenciado sob a licença MIT. Consulte o arquivo `LICENSE` para mais detalhes.

## 📞 Contato

Desenvolvido por **Vinicius Rabelo Barbosa**. Para dúvidas ou sugestões, entre em contato:

- **Email**: v.rabelobarbosa@proton.me
- **LinkedIn**: https://www.linkedin.com/in/viniciusrabelobarbosa/