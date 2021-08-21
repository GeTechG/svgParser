use crate::svg_parse;

pub struct Point {
    path: Vec<Vec<String>>
}

impl Point {

    pub fn new(path:&str) -> Point {
        Point {
            path: svg_parse::parse(path)
        }
    }

    pub(crate) fn at(&self, pos:f64) -> [f64; 2] {
        let mut cur:[f64;2] = [ 0.0, 0.0 ];
        let mut prev:[f64;3] = [ 0.0, 0.0, 0.0 ];
        let mut p0:[f64;2] = [ 0.0, 0.0 ];
        let mut len = 0.0;


        for i in 0..self.path.len() {
            let p = self.path.get(i).unwrap();
            if p[0] == "M".to_string() {
                cur[0] = p[1].parse().unwrap();
                cur[1] = p[2].parse().unwrap();
                if pos == 0.0 {
                    return cur;
                }
            }
            else if p[0] == "C".to_string() {
                p0[0] = cur[0];
                prev[0] = p0[0];

                p0[1] = cur[1];
                prev[1] = p0[1];
                prev[2] = len;

                let n = 100;
                for j in  0..=n {
                    let t = j as f64 / n as f64;
                    let x = (1.0-t).powf(2.0) * p0[0] + 2.0 * (1.0-t) * t * p[1].parse::<f64>().unwrap() + t.powf(2.0) * p[3].parse::<f64>().unwrap();
                    let y = (1.0-t).powf(3.0) * p0[1] + 3.0 * (1.0-t).powf(2.0) * t * p[2].parse::<f64>().unwrap() + 3.0 * (1.0-t) * t.powf(2.0) * p[4].parse::<f64>().unwrap() + t.powf(3.0) * p[6].to_string().parse::<f64>().unwrap();
                    len += dist(cur[0], cur[1], x, y);

                    cur[0] = x;
                    cur[1] = y;

                    if len >= pos {
                        let dv = (len - pos) / (len - prev[2]);

                        let npos = [
                            cur[0] * (1.0 - dv) + prev[0] * dv,
                            cur[1] * (1.0 - dv) + prev[1] * dv
                        ];
                        return npos;
                    }
                    prev[0] = cur[0];
                    prev[1] = cur[1];
                    prev[2] = len;
                }
            }
            else if p[0].eq("Q") {
                p0[0] = cur[0];
                prev[0] = p0[0];


                p0[1] = cur[1];
                prev[1] = p0[1];
                prev[2] = len;

                let n = 100;
                for j in 0..=n {
                    let t = j as f64 / n as f64;
                    let x = (1.0-t).powf(2.0) * p0[0] + 2.0 * (1.0-t) * t * p[1].parse::<f64>().unwrap() + t.powf(2.0) * p[3].parse::<f64>().unwrap();
                    let y = (1.0-t).powf(2.0) * p0[1] + 2.0 * (1.0-t) * t * p[2].parse::<f64>().unwrap() + t.powf(2.0) * p[4].parse::<f64>().unwrap();
                    len += dist(cur[0] as f64, cur[1] as f64, x, y);

                    cur[0] = x;
                    cur[1] = y;

                    if len >= pos {
                        let dv = (len - pos) / (len - prev[2]);

                        let npos = [
                            cur[0] * (1.0 - dv) + prev[0] * dv,
                            cur[1] * (1.0 - dv) + prev[1] * dv
                        ];
                        return npos;
                    }
                    prev[0] = cur[0];
                    prev[1] = cur[1];
                    prev[2] = len;
                }
            }
            else if p[0].eq("L") {
                prev[0] = cur[0];
                prev[1] = cur[1];
                prev[2] = len;

                len += dist(cur[0] as f64, cur[1] as f64, p[1].parse::<f64>().unwrap(), p[2].parse::<f64>().unwrap());
                cur[0] = p[1].parse::<f64>().unwrap();
                cur[1] = p[2].parse::<f64>().unwrap();

                if len >= pos {
                    let dv = (len - pos) / (len - prev[2]);
                    let npos = [
                        cur[0] * (1.0 - dv) + prev[0] * dv,
                        cur[1] * (1.0 - dv) + prev[1] * dv
                    ];
                    return npos;
                }
                prev[0] = cur[0];
                prev[1] = cur[1];
                prev[2] = len;
            }
        }
        [-1.0,-1.0]
    }

    fn at_v2(&self, pos:f64) -> [f64; 2] {
        [0.0,0.0]
    }
}

fn dist(ax:f64, ay:f64, bx:f64, by:f64) -> f64 {
    let x = ax - bx;
    let y = ay - by;
    return (x*x + y*y).sqrt();
}