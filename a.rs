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

fn init_dist(x: &[usize], y: &[usize], cities: &[usize]) -> f64 {
    let n = x.len();
    let mut tot = 0.0;
    for i in 0..n - 1 {
        let dist = ((x[cities[i]] as f64 - x[cities[i + 1]] as f64).powi(2) +
                    (y[cities[i]] as f64 - y[cities[i + 1]] as f64).powi(2)).sqrt();
        tot += dist;
    }
    tot
}

fn init_mo(x: &[usize], y: &[usize], w: usize) -> Vec<usize> {
    let n = x.len();
    let mut cities: Vec<usize> = (0..n).collect();
    let stripe_width = w / 2;
    cities.sort_by_key(|&i| {
        let qx = x[i] / stripe_width;
        let qy = y[i] / stripe_width;
        let r = if qx % 2 == 0 {
            qy
        } else {
            w - qy
        };
        let rx = x[i] % stripe_width;
        let ry = y[i] % stripe_width;
        let ry = if qx % 2 == 0 {
            ry
        } else {
            stripe_width - ry
        };
        (qx, r, ry / (stripe_width / 2), rx / (stripe_width / 2))
    });
    cities
}

fn init_greedy(x: &[usize], y: &[usize], _w: usize) -> Vec<usize> {
    let mut cities = vec![0];
    let mut rem: Vec<_> = (1..x.len()).collect();
    let dist = |a: usize, b: usize| {
        ((x[a] as f64 - x[b] as f64).powi(2) + (y[a] as f64 - y[b] as f64).powi(2)).sqrt()
    };
    while !rem.is_empty() {
        let mut best_dist = f64::MAX;
        let mut best_i = 0;
        let mut best_j = 0;
        for i in 0..=cities.len() {
            for j in 0..rem.len() {
                let mut diff = 0.0;
                if i < cities.len() {
                    diff += dist(cities[i], rem[j]);
                }
                if i > 0 {
                    diff += dist(cities[i - 1], rem[j]);
                }
                if i > 0 && i < cities.len() {
                    diff -= dist(cities[i - 1], cities[i]);
                }
                if best_dist > diff {
                    best_dist = diff;
                    best_i = i;
                    best_j = j;
                }
            }
        }
        cities.insert(best_i, rem[best_j]);
        rem.remove(best_j);
    }
    cities
}


const CLIMB0_COUNT: i32 = 0;

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

fn score0_diff(
    x: &[usize], y: &[usize], groups: &[Vec<usize>],
    i: usize, ii: usize, j: usize, jj: usize,
) -> f64 {
    let mut score = 0.0;
    for (g, idx, old, new) in [(&groups[i], ii, groups[j][jj], groups[i][ii]),
                              (&groups[j], jj, groups[i][ii], groups[j][jj])] {
        let mut checked = vec![];
        if idx < g.len() - 1 {
            checked.push(idx + 1);
        }
        if idx > 0 {
            checked.push(idx - 1);
        }
        for other in checked {
            assert_eq!(new, g[idx]);
            let dist_new = (x[new] as f64 - x[g[other]] as f64).powi(2) + (y[new] as f64 - y[g[other]] as f64).powi(2);
            let dist_old = (x[old] as f64 - x[g[other]] as f64).powi(2) + (y[old] as f64 - y[g[other]] as f64).powi(2);
            score += dist_new.sqrt() - dist_old.sqrt();
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
            let diff = score0_diff(x, y, groups, i, ii, j, jj);
            let new_score = diff + score;
            if diff > 0.0 {
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

fn find_edges_by_oracle(
    groups: &[Vec<usize>], _x: &[usize], _y: &[usize],
    l: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut edges = Vec::new();
    for group in groups {
        let mut group_edges = Vec::new();
        let group_size = group.len();
        let mut i = 0;
        while i + 1 < group_size {
            if i + 2 < group_size {
                let ret = query(&group[i..group_size.min(i + l)]);
                group_edges.extend(ret);
                i = group_size.min(i + l - 1);
            } else {
                group_edges.push((group[i], group[i + 1]));
                i += 2;
            }
        }
        edges.push(group_edges);
    }
    edges
}

fn find_edges_by_uf(
    groups: &[Vec<usize>], x: &[usize], y: &[usize],
    _l: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut edges = Vec::new();
    let mut uf = UnionFind::new(x.len());
    for group in groups {
        let mut group_edges = Vec::new();
        let group_size = group.len();
        let mut sorted_edges = Vec::new();
        for i in 0..group_size {
            for j in i + 1..group_size {
                let dist = (x[group[i]] as f64 - x[group[j]] as f64).powi(2) +
                   (y[group[i]] as f64 - y[group[j]] as f64).powi(2);
                sorted_edges.push((dist, group[i], group[j]));
            }
        }
        sorted_edges.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for (_, i, j) in sorted_edges {
            if uf.is_same_set(i, j) {
                continue;
            }
            uf.unite(i, j);
            group_edges.push((i, j));
        }
        edges.push(group_edges);
    }
    edges
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
    let (n, _m, _q, l, w) = (first_line[0], first_line[1], first_line[2], first_line[3], first_line[4]);

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

    let cities0: Vec<usize> = init_mo(&x, &y, w);
    let dist0 = init_dist(&x, &y, &cities0);
    eprintln!("dist0 = {dist0}");
    let cities1 = init_greedy(&x, &y, w);
    let dist1 = init_dist(&x, &y, &cities1);
    eprintln!("dist1 = {dist1}");
    let cities = if dist0 < dist1 {
        cities0
    } else {
        cities1
    };

    let mut groups = Vec::new();
    let mut start_idx = 0;
    for &group_size in &g {
        groups.push(cities[start_idx..start_idx + group_size].to_vec());
        start_idx += group_size;
    }
    climb0(conf, &x, &y, &mut rng, &mut groups);

    let edges_oracle = find_edges_by_oracle(&groups, &x, &y, l);
    let edges_uf = find_edges_by_uf(&groups, &x, &y, l);
    let edges = edges_uf;
    let edges = edges_oracle;

    answer(&groups, &edges);
}
