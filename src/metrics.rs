use crate::regression::Modelo;

pub fn calcular_mse(modelo: &Modelo, data: &Vec<f64>) -> Result<f64, String> {
    let n = data.len();
    if n == 0 {
        return Err("Dados vazios não são aceitos para cálculo de MSE.".to_string()); // Validação de entrada.
    }

    // Calcula os erros quadráticos.
    let erros: Vec<f64> = data.iter().enumerate().map(|(i, &y_real)| {
        let x = i as f64;
        let y_pred = modelo.slope * x + modelo.intercept;
        (y_real - y_pred).powi(2)
    }).collect();

    Ok(erros.iter().sum::<f64>() / n as f64) // Retorna o MSE.
}

pub fn calcular_r2(modelo: &Modelo, data: &Vec<f64>) -> Option<f64> {
    let n = data.len();
    if n == 0 {
        return None; // Retorna None para dados vazios.
    }

    // Calcula a média de y.
    let media_y = data.iter().sum::<f64>() / n as f64;
    let ss_total: f64 = data.iter().map(|y| (y - media_y).powi(2)).sum(); // Soma dos quadrados totais.
    let ss_residual: f64 = data.iter().enumerate().map(|(i, &y_real)| {
        let x = i as f64;
        let y_pred = modelo.slope * x + modelo.intercept;
        (y_real - y_pred).powi(2)
    }).sum(); // Soma dos quadrados residuais.

    if ss_total == 0.0 {
        return Some(1.0); // R² é 1 se não houver variação em y.
    }

    Some(1.0 - (ss_residual / ss_total)) // Retorna o R².
}
