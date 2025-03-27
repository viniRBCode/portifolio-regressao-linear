use portifolio_regressao_linear::regression::Modelo;
use portifolio_regressao_linear::prediction::prever;

#[test]
fn test_prever() {
    // Testa se a função de previsão gera os valores esperados.
    let modelo = Modelo { slope: 2.0, intercept: 3.0 };
    let previsoes = prever(&modelo, 3);
    let valores_esperados = vec![9.0, 11.0, 13.0]; // Valores calculados manualmente.
    assert_eq!(previsoes, valores_esperados); // Compara os valores previstos com os esperados.
}
