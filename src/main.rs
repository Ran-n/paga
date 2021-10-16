//#+ Autor:	Ran#
//#+ Creado:	16/10/2021 13:10:43
//#+ Editado:	16/10/2021 13:10:43

extern crate separator;

use std::io;
use std::io::Write;

use separator::Separatable;

fn meu_round(num: f32, decimais: i8) -> f32 {
    match decimais {
        1 => return (num * 10.0).round() / 10.0,
        2 => return (num * 100.0).round() / 100.0,
        3 => return (num * 1000.0).round() / 1000.0,
        4 => return (num * 10000.0).round() / 10000.0,
        5 => return (num * 100000.0).round() / 100000.0,
        _ => return num,
    }

}

fn main() {
    const DECIMAIS: i8 = 2;

    let moeda = '€';
    let cartos: f32;
    let mut cartos_str = String::new();
    let mut rango = String::new();
    let horas_lab: f32;
    let mut horas_lab_str = String::new();
    let pagas: f32;
    let mut pagas_str = String::new();
    let dias_lab: f32;
    let mut dias_lab_str = String::new();

    let mut cartos_hora: f32 = 0.0;
    let cartos_dia: f32;
    let mut cartos_mes: f32 = 0.0;
    let mut cartos_ano: f32 = 0.0;

    // CARTOS
    // fago print sen ln para poder poñer a entrada xusto detrás 
    print!("Mete os cartos [1300]: ");
    // para que se mostre o print de xeito instantáneo e non tras facer o input
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut cartos_str) {
        Ok(_) => {
        },
        Err(e) => println!("Erro {}", e)
    }

    // se non me pon nada nos cartos 1300 por defecto
    if cartos_str.trim().is_empty() {
        cartos = 1300.0;
    } else {
        cartos = cartos_str.trim().parse::<f32>().unwrap();
    }

    // RANGO
    // mostra do que se precisa
    print!("Cada canto son estes cartos? (h)ora/[(m)es]/(a)no: ");
    io::stdout().flush().unwrap();

    // lectura da entrada
    match io::stdin().read_line(&mut rango) {
        Ok(_) => {
        },
        Err(e) => println!("Erro {}", e)
    }

    // mirar que o que metes está dentro do esperado
    rango = rango.to_lowercase().trim().to_string();
    if rango != "h" && rango != "m" && rango != "a" {
        rango = "m".to_string();
    }

    // HORAS
    // mostra do que se precisa
    print!("Mete as horas laborables diarias [8]: ");
    io::stdout().flush().unwrap();

    // lectura da entrada
    match io::stdin().read_line(&mut horas_lab_str) {
        Ok(_) => {
        },
        Err(e) => println!("Erro {}", e)
    }

    // se non me pon nada nas horas 8 por defecto
    if horas_lab_str.trim().is_empty() {
        horas_lab = 8.0;
    } else {
        horas_lab = horas_lab_str.trim().parse::<f32>().unwrap();
    }

    // PAGAS
    // mostra do que se precisa
    print!("Mete o número de pagas [14]: ");
    io::stdout().flush().unwrap();

    // lectura da entrada
    match io::stdin().read_line(&mut pagas_str) {
        Ok(_) => {
        },
        Err(e) => println!("Erro {}", e)
    }

    // se non me pon nada nas horas 8 por defecto
    if pagas_str.trim().is_empty() {
        pagas = 14.0;
    } else {
        pagas = pagas_str.trim().parse::<f32>().unwrap();
    }
    
    // Dias laborables
    // mostra do que se precisa
    print!("Mete o número de días laborables por semana [5]: ");
    io::stdout().flush().unwrap();

    // lectura da entrada
    match io::stdin().read_line(&mut dias_lab_str) {
        Ok(_) => {
        },
        Err(e) => println!("Erro {}", e)
    }

    // se non me pon nada nos días son 5
    if dias_lab_str.trim().is_empty() {
        dias_lab = 5.0;
    } else {
        dias_lab = dias_lab_str.trim().parse::<f32>().unwrap();
    }


    let dias_lab_x_mes = 365.0 / 12.0 / 7.0 * dias_lab;
    let horas_lab_x_mes = dias_lab_x_mes * horas_lab;
    // let char_vec: Vec<char> = rango.chars().collect();
    // let char = rango.chars().nth(0).unwrap();
    match  rango.as_str() {
        "a" => {
            cartos_ano = cartos;
            cartos_mes = cartos_ano / pagas;
            cartos_hora = cartos_mes / horas_lab_x_mes;
        },
        "m" => {
            cartos_mes = cartos;
            cartos_ano = cartos_mes * pagas;
            cartos_hora = cartos_mes / horas_lab_x_mes;
        },
        "h" => {
            cartos_hora = cartos;
            cartos_mes = cartos_hora * horas_lab_x_mes;
            cartos_ano = cartos_mes * pagas;
        },
        _ => println!("Erro"),
    }
    cartos_dia = cartos_hora * horas_lab;


    // ---------------------------------------------------------
    println!("");
    println!("Paga ({}) -------------------", pagas);
    println!("xAno:\t{:} {}", meu_round(cartos_ano, DECIMAIS).separated_string(), moeda);
    println!("xMes:\t{} {}", meu_round(cartos_mes, DECIMAIS).separated_string(), moeda);
    println!("xDía:\t{} {}", meu_round(cartos_dia, DECIMAIS).separated_string(), moeda);
    println!("xHora:\t{} {}", meu_round(cartos_hora, DECIMAIS).separated_string(), moeda);
    println!("------------------------------");
    // ---------------------------------------------------------
}
