use portifolio_regressao_linear::regression::linear_regression;

#[test]
fn test_linear_regression_valida() {
    // Testa se a regressão linear funciona corretamente com dados válidos.
    let data = vec![2.0, 4.0, 6.0, 8.0];
    let model = linear_regression(&data).unwrap();
    assert!((model.slope - 2.0).abs() < 1e-6); // Verifica o coeficiente angular.
    assert!((model.intercept - 2.0).abs() < 1e-6); // Verifica o coeficiente linear.
}

#[test]
fn test_array_vazio() {
    // Testa se a função retorna erro ao receber um vetor vazio.
    let data: Vec<f64> = vec![];
    let result = linear_regression(&data);
    assert!(result.is_err()); // Espera-se um erro.
}
