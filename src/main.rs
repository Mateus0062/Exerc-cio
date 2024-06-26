use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;

fn main() {
    let mut o = String::new();
    let path = Path::new("./data/matriz.json");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let matriz: Vec<Vec<i32>> = serde_json::from_reader(reader).unwrap();

    println!("Matriz: {:?}", matriz);

    println!("Digite a operação desejada: ");
    print!("Soma S ou Média M => ");
    stdin().read_line(&mut o).unwrap();
    let o = o.trim();

    let mut coluna = 11;
    let mut linha_inicio = 1;
    let mut linha_fim = 10;
    let mut soma = 0;

    while linha_inicio <= linha_fim {
        for linha in linha_inicio..=linha_fim {
            soma += matriz[linha][coluna];
        }
        coluna -= 1;
        linha_inicio += 1;
        linha_fim -= 1;
    }

    if o == "M" {
        println!("Média total da soma: {}", soma / 30);
    } else if o == "S" {
        println!("Soma total: {}", soma);
    } else {
        println!("informe uma entrada válida");
        return;
    }
}
