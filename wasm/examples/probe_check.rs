// Checks the port's sprite sampling against what Java does, as measured by parity/ScaleProbe.java.
// Usage: cargo run --release --example probe_check -- <ScaleProbe output>

fn main() {
    let text = std::fs::read_to_string(std::env::args().nth(1).expect("the probe's output")).unwrap();
    let (mut cases, mut wrong) = (0, 0);
    for line in text.lines() {
        let (head, values) = line.split_once(':').unwrap();
        let n: Vec<i32> = head.split_whitespace().map(|v| v.parse().unwrap()).collect();
        let (src, size, at) = (n[0], n[1], n[2]);
        let java: Vec<i32> = values.split_whitespace().map(|v| v.parse().unwrap()).collect();
        let mut port = vec![-1; 800];
        if let Some((first, _, columns)) = frogger::sample_columns(at, size, src, 800) {
            for (i, c) in columns.iter().enumerate() {
                port[first as usize + i] = c.unwrap_or(-1);
            }
        }
        cases += 1;
        if port != java {
            wrong += 1;
            if wrong <= 5 {
                let x = (0..800).find(|&x| port[x] != java[x]).unwrap();
                println!("source {src}, drawn {size} at {at}: screen x {x} is {} in Java, {} here", java[x], port[x]);
            }
        }
    }
    println!("{cases} cases, {wrong} differ");
}
