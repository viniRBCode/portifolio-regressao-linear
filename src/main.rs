mod regression;
mod metrics;
mod prediction;

use regression::linear_regression;
use metrics::{calcular_mse, calcular_r2};
use prediction::prever;

fn main() {
    // Dados de entrada para a regressão.
    let dados = vec![3.0, 4.5, 6.0, 7.5];

    // Realiza a regressão linear e exibe os resultados.
    match linear_regression(&dados) {
        Ok(modelo) => {
            println!("Modelo: y = {:.2}x + {:.2}", modelo.slope, modelo.intercept); // Exibe o modelo.
            println!("MSE: {:.2}", calcular_mse(&modelo, &dados).unwrap()); // Calcula e exibe o MSE.
            println!("R²: {:.2}", calcular_r2(&modelo, &dados).unwrap_or(0.0)); // Calcula e exibe o R².
            println!("Previsões para os próximos períodos: {:?}", prever(&modelo, 3)); // Exibe previsões.
        }
        Err(erro) => println!("Erro ao calcular a regressão: {}", erro), // Trata erros.
    }
}
