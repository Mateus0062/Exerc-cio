use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;

fn main() {
    let tamanho = 12;
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
    let mut soma_total = 0;

    while linha_inicio <= linha_fim {
        let mut soma = 0;

        for linha in linha_inicio..=linha_fim {
            soma += matriz[linha][coluna];
        }

        soma_total += soma;

        coluna -= 1;
        linha_inicio += 1;
        linha_fim -= 1;
    }

    if o == "M" {
        println!("Média total da soma: {}", soma_total / 30);
    } else {
        println!("Soma total: {}", soma_total);
    }
}
