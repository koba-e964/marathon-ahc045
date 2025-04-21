use std::io::{self, BufRead, Write};

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn query(c: &[usize]) -> Vec<(usize, usize)> {
    print!("? {}", c.len());
    for &ci in c {
        print!(" {}", ci);
    }
    println!();

    let stdin = io::stdin();
    let mut res = vec![];
    for _ in 0..c.len() - 1 {
        let line = getline().trim().to_string();
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        res.push((nums[0], nums[1]));
    }
    res
}

fn answer(groups: &[Vec<usize>], edges: &[Vec<(usize, usize)>]) {
    println!("!");
    for (group, edge_set) in groups.iter().zip(edges.iter()) {
        let mut first = true;
        for city in group {
            if !first {
                print!(" ");
            } else {
                first = false;
            }
            print!("{}", city);
        }
        println!();
        for &(a, b) in edge_set {
            println!("{} {}", a, b);
        }
    }
}

fn main() {
    let first_line = getline().trim().to_string();
    let first_line: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, m, _q, _l, _w) = (first_line[0], first_line[1], first_line[2], first_line[3], first_line[4]);

    let g: Vec<usize> = getline().trim().to_string()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (mut lx, mut rx, mut ly, mut ry) = (vec![0; n], vec![0; n], vec![0; n], vec![0; n]);

    for i in 0..n {
        let rect: Vec<usize> = getline().trim().to_string()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        lx[i] = rect[0];
        rx[i] = rect[1];
        ly[i] = rect[2];
        ry[i] = rect[3];
    }

    let x: Vec<usize> = lx.iter().zip(&rx).map(|(l, r)| (l + r) / 2).collect();
    let y: Vec<usize> = ly.iter().zip(&ry).map(|(l, r)| (l + r) / 2).collect();

    let mut cities: Vec<usize> = (0..n).collect();
    cities.sort_by_key(|&i| (x[i], y[i]));

    let mut groups = Vec::new();
    let mut start_idx = 0;
    for &group_size in &g {
        groups.push(cities[start_idx..start_idx + group_size].to_vec());
        start_idx += group_size;
    }

    let mut edges = Vec::new();
    for (k, &group_size) in g.iter().enumerate() {
        let mut group_edges = Vec::new();
        let group = &groups[k];
        let mut i = 0;
        while i + 1 < group_size {
            if i < group_size - 2 {
                let ret = query(&group[i..i + 3]);
                group_edges.extend(ret);
                i += 2;
            } else {
                group_edges.push((group[i], group[i + 1]));
                i += 2;
            }
        }
        edges.push(group_edges);
    }

    answer(&groups, &edges);
}
