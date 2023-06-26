use rand::Rng;

#[derive(Debug)]
struct Marble {
    r: f64,
    p: [f64; 2],
    v: [f64; 2],
}

fn dot(x: [f64; 2], y: [f64; 2]) -> f64 {
    x[0]*y[0] + x[1]*y[1]
}

struct Decomposition {
    d: [f64; 2],  // distance vector
    o: [f64; 2],  // orthogonal vector
    v_d: [f64; 2],
    v_o: [f64; 2],
}
impl Decomposition {
    fn new(x: &Marble, y: &Marble) -> Option<Decomposition> {
        let d = [y.p[0] - x.p[0], y.p[1] - x.p[1]];
        let o = [d[1], -d[0]];

        let dsqr = dot(d, d);
        let min_d = x.r + y.r;
        if dsqr > min_d*min_d {
            return None
        }

        let dsqrinv = 1.0/dsqr;
        let osqrinv = 1.0/dot(o, o);
        Some(Decomposition {
            d: d,
            o: o,
            v_d: [dot(d, x.v)*dsqrinv, dot(d, y.v)*dsqrinv],
            v_o: [dot(o, x.v)*osqrinv, dot(o, y.v)*osqrinv],
        })
    }
    fn restore(&self, idx: usize) -> [f64; 2] {
        [
            self.v_d[idx]*self.d[0] + self.v_o[idx]*self.o[0],
            self.v_d[idx]*self.d[1] + self.v_o[idx]*self.o[1],
        ]
    }
}

impl Marble {
    fn new(rng: &mut rand::rngs::ThreadRng, radius: f64, width: f64, height: f64) -> Marble {
        Marble {
            r: radius,
            p: [(width-20.0)*rng.gen::<f64>() + 10.0, (height-20.0)*rng.gen::<f64>() + 10.0],
            v: [0.5*rng.gen::<f64>()-0.25, 0.5*rng.gen::<f64>()-0.25],
        }
    }
    fn step(&mut self, diff: f64, width: f64, height: f64) {
        let dim = [width, height];
        for i in 0..2 {
            self.p[i] = self.p[i] + self.v[i] * diff;
            if self.p[i] > dim[i]-self.r && self.v[i] > 0.0 {
                self.p[i] = dim[i]-self.r;
                self.v[i] = -self.v[i];
            }
            if self.p[i] < self.r && self.v[i] < 0.0 {
                self.p[i] = self.r;
                self.v[i] = -self.v[i];
            }
        }
    }
    fn check_collision(&mut self, othr: &mut Self) {
        let mut decomp = match Decomposition::new(&self, &othr) {
            None => return,
            Some(d) => d,
        };

        if decomp.v_d[1] > decomp.v_d[0] {
            return
        }
        // Flip parallel parts
        decomp.v_d = [decomp.v_d[1], decomp.v_d[0]];
        self.v = decomp.restore(0);
        othr.v = decomp.restore(1);
    }

    fn check_collision_fixed(&mut self, othr: &Self) {
        let mut decomp = match Decomposition::new(&self, &othr) {
            None => return,
            Some(d) => d,
        };

        // flip parallel part
        if decomp.v_d[0] > 0.0 {
            // flip parallel part
            decomp.v_d[0] = -decomp.v_d[0];
            self.v = decomp.restore(0);
        }
    }
}

pub struct World {
    width: f64,
    height: f64,
    m: Vec<Marble>,
    fixed: Vec<Marble>,
}

impl World {
    pub fn new(width: f64, height: f64) -> World{
        let mut rng = rand::thread_rng();
        let mut m = Vec::new();
        for _ in 0..25 {
            m.push(Marble::new(&mut rng, 10.0, width, height));
        }
        let mut f = Vec::new();
        for _ in 0..5 {
            f.push(Marble::new(&mut rng, 50.0, width, height));
        }
        World{
            width: width,
            height: height,
            m: m,
            fixed: f,
        }
    }
    pub fn step(&mut self, dt: f64) {
        if dt > 100.0 {
            for _ in 0..10 {
                self.step(dt/10.0)
            }
            return;
        }
        for marble in self.m.iter_mut() {
            marble.step(dt, self.width, self.height);
            for fixed in self.fixed.iter() {
                marble.check_collision_fixed(&fixed);
            }
        }
        for i in 0..self.m.len() {
            let (left, right) = self.m.split_at_mut(i);
            for j in 0..i {
                left[j].check_collision(&mut right[0])
            }
        }
    }

    pub fn draw<F>(&self, mut f: F) -> Result<(), String>
        where F: FnMut(u32, u32, u32, [u8; 3]) -> Result<(), String> {
            for marble in self.m.iter() {
                f(marble.p[0] as u32, marble.p[1] as u32, marble.r as u32, [200, 0, 0])?;
            }
            for fixed in self.fixed.iter() {
                f(fixed.p[0] as u32, fixed.p[1] as u32, fixed.r as u32, [0,0,0])?;
            }
            Ok(())
        }

    #[allow(dead_code)]
    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}