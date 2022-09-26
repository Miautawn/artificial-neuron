use rand::Rng;
use std::io::{self, Write};

// Aprašomi duomenys ir klasės
const X: [[f64; 3]; 4] =[
    [1.0, -0.3, 0.6],
    [1.0, 0.3, 0.6],
    [1.0, 1.2, -1.2],
    [1.0, 1.2, 1.2]
];

const Y:[i8; 4] = [0, 0, 1, 1];

// Aprašomos aktyvacijos funkcijos
fn step_activation(x: f64) -> i8 {
    return if x >= 0.0 {1} else {0}
}

fn sigmoid_activation(x: f64) -> i8 {
    return (1.0 / (1.0 + (-x).exp())).round() as i8
}

// Aprašoma perceptrono "objektas"
struct Perceptron {
    activation_function: fn(f64) -> i8,
    weights: [f64; 3]
}

impl Perceptron {
    fn vector_multiply(a: [f64; 3], b: [f64; 3]) -> f64 {
        let mut product: f64 = 0.0;
        for index in 0..a.len() {
            product += a[index] * b[index];
        }

        return product
    }

    fn forward(&self, x: [f64; 3]) -> i8 {
        let y_pred = Self::vector_multiply(x, self.weights);
        return (self.activation_function)(y_pred)
    }
}

// pagalbinės funkcijos
fn print_and_read(question: &str) -> String {
    let mut value = String::new();

    print!("{}", question);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut value).expect("Nesupratau :(");

    return value
}

fn main() {
    println!("Sveiki atvykę į dirbtinį perceptroną!\n\
    Ši programa buvo įgyvendinta su Rust programavimo kalba.\n\
    \n\
    'Mokymo duomenys': {X:?}\n\
    'Mokymo klasės': {Y:?}\n");

    let lower_bound: f64 = print_and_read(
        "Svorių ieškojimo apatinė riba: "
    ).trim().parse().expect("Įveskite skaičių!");

    let upper_bound: f64 = print_and_read(
        "Svorių ieškojimo viršutinė riba: "
    ).trim().parse().expect("Įveskite skaičių!");

    let activation_function_selection: i32 = print_and_read(
        "Aktyvacijos funkcijas:\n\
        1 - Slenksitė funkija,\n\
        2 - Sigmoidinė funkcija\n\
        Įveskite pasirinkimą: "
    ).trim().parse().expect("Įveskite skaičių!");

    println!("\nPradedamas atsitiktinis svorių ieškojimas!!");
    let mut rng = rand::thread_rng();
    let mut perceptron = Perceptron{
        weights: [0.0, 0.0, 0.0],
        activation_function: match activation_function_selection {
            1 => step_activation,
            2 => sigmoid_activation,
            _ => panic!("Tokios aktyvavymo funkcijos nera!")
        }
    };

    loop {
        let mut found: bool = true;

        for index in 0..perceptron.weights.len() {
            perceptron.weights[index] = rng.gen_range(lower_bound..upper_bound);
        }
        println!("Bandome tokius svorius: {:?}", perceptron.weights);

        for index in 0..X.len() {
            let predicted_class: i8 = perceptron.forward(X[index]);
            if predicted_class != Y[index] {
                found = false;
                break;
            }
        }

        if found {
            println!("Pavyko rasti gerus svorius! {:?}", perceptron.weights);
            break;
        }
    }

    println!("\nTestuojame 'optimalius' svorius!");
    println!("Orginalios klasės == Gautos klasės");
    for index in 0..X.len() {
        println!(
            "{} == {} - {}",
            Y[index], perceptron.forward(X[index]),
            Y[index] == perceptron.forward(X[index]),
        ); 
    }
}
