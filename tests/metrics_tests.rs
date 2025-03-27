use portifolio_regressao_linear::regression::linear_regression;
use portifolio_regressao_linear::metrics::{calcular_mse, calcular_r2};

#[test]
fn test_calcular_mse() {
    // Testa o cálculo do erro quadrático médio (MSE).
    let data = vec![2.0, 4.0, 6.0, 8.0];
    let modelo = linear_regression(&data).unwrap();
    let mse = calcular_mse(&modelo, &data).unwrap();
    assert!((mse - 0.0).abs() < 1e-6); // MSE esperado é 0, pois os dados são perfeitos.
}

#[test]
fn test_calcular_r2() {
    // Testa o cálculo do coeficiente de determinação (R²).
    let data = vec![2.0, 4.0, 6.0, 8.0];
    let modelo = linear_regression(&data).unwrap();
    let r2 = calcular_r2(&modelo, &data);

    // Verifica se o R² foi calculado corretamente.
    assert!(r2.is_some()); // Certifica-se de que o valor não é None.
    assert!((r2.unwrap() - 1.0).abs() < 1e-6); // R² esperado é 1, indicando ajuste perfeito.
}
