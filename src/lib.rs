use wasm_bindgen::prelude::*;
use js_sys::Math::random;

struct Marble {
    p: [f64; 2],
    v: [f64; 2],
}

fn dot(x: [f64; 2], y: [f64; 2]) -> f64 {
    x[0]*y[0] + x[1]*y[1]
}

impl Marble {
    fn new(width: f64, height: f64) -> Marble {
        Marble {
            p: [(width-20.0)*random() + 10.0, (height-20.0)*random() + 10.0],
            v: [0.5*random()-0.25, 0.5*random()-0.25],
        }
    }
    fn step(&mut self, diff: f64, width: f64, height: f64) {
        let dim = [width, height];
        for i in 0..2 {
            self.p[i] = self.p[i] + self.v[i] * diff;
            if self.p[i] > dim[i]-10.0 && self.v[i] > 0.0 {
                self.p[i] = dim[i]-10.0;
                self.v[i] = -self.v[i];
            }
            if self.p[i] < 10.0 && self.v[i] < 0.0 {
                self.p[i] = 10.0;
                self.v[i] = -self.v[i];
            }
        }
    }
    fn check_collision(&mut self, othr: &mut Self) {
        // distance vector and one perpendicular to it
        let d = [othr.p[0] - self.p[0], othr.p[1] - self.p[1]];
        let e = [d[1], -d[0]];

        let dsqr = dot(d, d);
        if dsqr > 400.0 {
            return;
        }

        // decompose velocities into parallel and orthogonal parts
        let dsqrinv = 1.0/dsqr;
        let esqrinv = 1.0/dot(e, e);
        let v1 = [dot(d, self.v)*dsqrinv, dot(e, self.v)*esqrinv];
        let v2 = [dot(d, othr.v)*dsqrinv, dot(e, othr.v)*esqrinv];

        // Recompose with swapped parallel parts
        self.v = [v2[0]*d[0] + v1[1]*e[0], v2[0]*d[1] + v1[1]*e[1]];
        othr.v = [v1[0]*d[0] + v2[1]*e[0], v1[0]*d[1] + v2[1]*e[1]];

        // shift positions so the distance becomes 20.0
        let scale = 10.0*dsqrinv.sqrt()-0.5;
        self.p = [self.p[0]-scale*d[0], self.p[1]-scale*d[1]];
        othr.p = [othr.p[0]+scale*d[0], othr.p[1]+scale*d[1]];
    }
}

#[wasm_bindgen]
pub struct World {
    width: f64,
    height: f64,
    m: Vec<Marble>,
    last_time: f64,
}

impl World {
    fn step_eps(&mut self, diff: f64) {
        if diff > 100.0 {
            for _ in 0..10 {
                self.step_eps(diff/10.0)
            }
            return;
        }
        for marble in self.m.iter_mut() {
            marble.step(diff, self.width, self.height);
        }
        for i in 0..self.m.len() {
            let (left, right) = self.m.split_at_mut(i);
            for j in 0..i {
                left[j].check_collision(&mut right[0])
            }
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: f64, height: f64) -> World{
        let mut m = Vec::new();
        for _ in 0..25 {
            m.push(Marble::new(width, height));
        }
        World{
            width: width,
            height: height,
            m: m,
            last_time: 0.0,
        }
    }
    pub fn step(&mut self, t: f64) {
        self.step_eps(t - self.last_time);
        self.last_time = t;
    }
    pub fn draw(&self, f: &js_sys::Function) {
        let this = JsValue::null();
        for marble in self.m.iter() {
            let x = JsValue::from(marble.p[0]-10.0);
            let y = JsValue::from(marble.p[1]-10.0);
            let _ = f.call2(&this, &x, &y);
        }
    }
    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}
