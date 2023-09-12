use wasm_rs::Table;

#[test]
fn main() {
    let mut rows: Vec<Vec<bool>> = (0..5).map(|_| (0..10).map(|_| false).collect()).collect();

    rows[0][0] = true;
    rows[1][0] = true;
    rows[2][0] = true;
    rows[3][0] = true;
    rows[4][0] = true;

    let mut table = Table::from(rows);

    for _ in 0..10 {
        println!("{}", table);
        table.tick();
    }
}
