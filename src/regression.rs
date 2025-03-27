#[derive(Debug)]
pub struct Modelo {
    pub slope: f64, // Coeficiente angular da reta.
    pub intercept: f64, // Coeficiente linear da reta.
}

pub fn linear_regression(data: &Vec<f64>) -> Result<Modelo, String> {
    let n = data.len();
    if n == 0 {
        return Err("Dados vazios não são aceitos.".to_string()); // Validação de entrada.
    }

    // Gera os valores de x como índices do vetor.
    let x_vals: Vec<f64> = (0..n).map(|x| x as f64).collect();
    let y_vals = data;

    // Calcula somatórios necessários para a fórmula da regressão.
    let sum_x: f64 = x_vals.iter().sum();
    let sum_y: f64 = y_vals.iter().sum();
    let sum_xx: f64 = x_vals.iter().map(|x| x * x).sum();
    let sum_xy: f64 = x_vals.iter().zip(y_vals.iter()).map(|(x, y)| x * y).sum();

    // Calcula o divisor para evitar divisão por zero.
    let divisor = n as f64 * sum_xx - sum_x * sum_x;
    if divisor == 0.0 {
        return Err("Divisão por zero na computação da regressão.".to_string());
    }

    // Calcula os coeficientes da reta.
    let slope = (n as f64 * sum_xy - sum_x * sum_y) / divisor;
    let intercept = (sum_y * sum_xx - sum_x * sum_xy) / divisor;

    Ok(Modelo { slope, intercept }) // Retorna o modelo calculado.
}
