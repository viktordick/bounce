use wasm_bindgen::prelude::*;
use js_sys::Math::random;

struct Marble {
    p: [f64; 2],
    v: [f64; 2],
}

fn sqr(x: f64) -> f64 { x*x }
fn dot(x: [f64; 2], y: [f64; 2]) -> f64 {
    x[0]*y[0] + x[1]*y[1]
}

impl Marble {
    fn new() -> Marble {
        Marble {
            p: [480.0*random(), 480.0*random()],
            v: [0.5*random()-0.25, 0.5*random()-0.25],
        }
    }
    fn step(&mut self, diff: f64) {
        for i in 0..2 {
            self.p[i] = self.p[i] + self.v[i] * diff;
            if self.p[i] > 490.0 {
                self.p[i] = 490.0;
                self.v[i] = -self.v[i];
            }
            if self.p[i] < 10.0 {
                self.p[i] = 10.0;
                self.v[i] = -self.v[i];
            }
        }
    }
    fn check_collision(&mut self, other: &mut Self) {
        if sqr(self.p[0]-other.p[0]) + sqr(self.p[1] - other.p[1]) > 400.0 {
            return;
        }
        // distance vector and one perpendicular to it
        let d = [other.p[0] - self.p[0], other.p[1] - self.p[1]];
        let e = [d[1], -d[0]];

        // decompose velocities into parallel and perpendicular parts
        let v1 = [dot(d, self.v)/dot(d, d), dot(e, self.v)/dot(e, e)];
        let v2 = [dot(d, other.v)/dot(d, d), dot(e, other.v)/dot(e, e)];

        // Recompose with swapped parallel parts
        self.v = [v2[0]*d[0] + v1[1]*e[0], v2[0]*d[1] + v1[1]*e[1]];
        other.v = [v1[0]*d[0] + v2[1]*e[0], v1[0]*d[1] + v2[1]*e[1]];
    }
}

#[wasm_bindgen]
pub struct World {
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
            marble.step(diff);
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
    pub fn new() -> World{
        let mut m = Vec::new();
        for _ in 0..15 {
            m.push(Marble::new());
        }
        World{
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
}
