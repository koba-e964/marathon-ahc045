#[derive(Clone, Copy)]
struct Conf {
    debug: bool,
    climb0_count: i32,
}

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

// Union-Find tree.
// Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

#[allow(unused)]
impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] += self.rank[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
}

struct Rng {
    x: u64,
}

impl Rng {
    fn next(&mut self) -> u32 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        ((x ^ x << 10) >> 32) as _
    }
}

fn query(c: &[usize]) -> Vec<(usize, usize)> {
    print!("? {}", c.len());
    for &ci in c {
        print!(" {}", ci);
    }
    println!();

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

const CLIMB0_COUNT: i32 = 400000;

fn score0(x: &[usize], y: &[usize], groups: &[Vec<usize>]) -> f64 {
    let mut score = 0.0;
    for group in groups {
        for i in 0..group.len() - 1 {
            let j = i + 1;
            let dist = (x[group[i]] as f64 - x[group[j]] as f64).powi(2) + (y[group[i]] as f64 - y[group[j]] as f64).powi(2);
            score += dist.sqrt();
        }
    }
    score
}

fn climb0(conf: Conf, x: &[usize], y: &[usize], rng: &mut Rng, groups: &mut [Vec<usize>]) {
    let mut score = score0(x, y, groups);
    for _ in 0..conf.climb0_count {
        let i = rng.next() as usize % groups.len();
        let j = rng.next() as usize % groups.len();
        if i == j {
            continue;
        }
        if groups[i].len() > 1 && groups[j].len() > 1 {
            let ii = rng.next() as usize % groups[i].len();
            let jj = rng.next() as usize % groups[j].len();
            let tmp = groups[i][ii];
            groups[i][ii] = groups[j][jj];
            groups[j][jj] = tmp;
            let new_score = score0(x, y, &groups);
            if new_score > score {
                // revert
                let tmp = groups[i][ii];
                groups[i][ii] = groups[j][jj];
                groups[j][jj] = tmp;
            } else {
                if conf.debug {
                    eprintln!("score improvement: {score} -> {new_score}");
                }
                score = new_score;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut conf = Conf {
        debug: false,
        climb0_count: CLIMB0_COUNT,
    };
    for arg in args.iter().skip(1) {
        if arg == "debug" {
            conf.debug = true;
        } else if arg.starts_with("climb0_count=") {
            let val = arg.split('=').nth(1).unwrap().parse::<i32>().unwrap();
            conf.climb0_count = val;
        }
    }
    let mut rng = Rng { x: 0xc0ba_e964 };

    let first_line = getline().trim().to_string();
    let first_line: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, _m, _q, _l, _w) = (first_line[0], first_line[1], first_line[2], first_line[3], first_line[4]);

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
    climb0(conf, &x, &y, &mut rng, &mut groups);

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
