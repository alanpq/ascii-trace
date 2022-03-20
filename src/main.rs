extern crate nalgebra_glm as glm;
extern crate pancurses;

use glm::{vec3, vec2, vec4, TVec3};
use pancurses::{endwin, initscr, noecho, Input, resize_term, Window};
use rand::{thread_rng, Rng};
use std::rc::Rc;
use std::cell::{RefCell};

use std::sync::{Arc, RwLock};
use crate::material::Material;
use crate::renderables::{Plane, Sphere};
use crate::scene::{Scene};

use std::time::Instant;
use rayon::prelude::*;

mod material;
mod matrixes;
mod renderables;
mod intersect;
mod scene;
// use crate::matrixes;

const LUT: &[u8] = " .,-~:;=!*#$@".as_bytes();


fn cast_ray(source: &TVec3<f32>, dir: &TVec3<f32>, scene: Arc<Scene>) -> f32 {
  // let scene = scene.unwrap();
  match &scene.intersect(source, dir) {
    Some(result) => {
      let light_dir = glm::normalize(&vec3(1., 1., 1.));
      let dot: f32 = glm::dot(&result.normal, &light_dir);
      // depth render
      // 1.-(result.dist/50.)

      // normal render
      0.1 + dot.max(0_f32) * result.obj.material().albedo
      * match &scene.intersect(&(result.hit + result.normal * 0.001), &light_dir) {
        Some(_) => 0.,
        None => 1.,
      }
    },
    None => 0.,
  }
  //1.
}

// currently using view_angles also for target_pos (cos im lazy)
fn render(win: Arc<pancurses::Window>, scene: Arc<Scene>, view_pos: &TVec3<f32>, view_angles: &TVec3<f32>) {
  let size = win.get_max_yx();
  win.mv(0, 0);
  // let fov: f32 = std::f32::consts::PI/1.2; // ortho fov
  let fov: f32 = std::f32::consts::PI/3.;
  let area = size.0 * size.1;
  let chars: Vec<char> = (0..area).into_par_iter().chunks(1).map(|idxs: Vec<i32>| -> Vec<char> {
    idxs.into_iter().map(|i| {
      pixel(scene.clone(), *view_pos, *view_angles, size.1, size.0, fov, i)
    }).collect()
  }).flatten().collect();
  chars.into_iter().for_each(|c| {
    win.addch(c);
  });
}

fn pixel(scene: Arc<Scene>, view_pos: TVec3<f32>, view_angles: TVec3<f32>, w: i32, h: i32, fov: f32, i: i32) -> char {
  let x = i % w;
  let y = i / w;

  let px = x as f32;
  let py = y as f32;
  let w = w as f32;
  let h = h as f32;

  // normalized device coords (screenspace [0,1] both axes)
  let ndcx = (px + 0.5) / w;
  let ndcy = (py + 0.5) / h;

  // screen space coords ([-1,1] both axes, origin at center)
  let fov_fac = (fov / 2.).tan();
  let ssx = (2. * ndcx - 1.) * (w / h) * fov_fac;
  // inverted to flip y axis
  let ssy = (1. - 2. * ndcy) * fov_fac * 2.; // * 2, as 1 character is twice as tall as it is wide (roughly)

  let c2w = matrixes::look_at_matrix(&view_pos, &view_angles);

  // normal perspective rays
  let mut lum = cast_ray(&view_pos, &glm::normalize(&(c2w * vec4(ssx, ssy, -1., 0.)).xyz()), scene);
  if lum < 0. {
    lum = 0.;
  }
  if lum > 1. {
    lum = 1.;
  }
  LUT[(lum * 12.) as usize] as char
}

fn main() {
  let window = initscr();
  window.printw("Hello Rust");
  window.refresh();
  window.nodelay(true);
  resize_term(30, 100);
  noecho();
  let mut scene = Scene {
    objects: Vec::new()
  };
  scene.add_object(Plane {
    center: vec3(0.0, -5.0, 0.0),
    size: vec2(100.0, 100.0),
    material: Material {
      albedo: 0.5,
    }
  });
  //let mut balls: Vec<Box<Sphere>> = Vec::new();
  let mut rng = thread_rng();
  for _ in 0..3 {
    scene.add_object(Sphere {
      center: vec3(rng.gen_range(-10_f32..10_f32), rng.gen_range(-5_f32..5_f32), rng.gen_range(-10_f32..10_f32)),
      radius: rng.gen_range(2_f32..4_f32),
      material: Material {
        albedo: 1.,
      }
    });
  }
  let sphere = Sphere {
    center: vec3(0., 0., 0.),
    radius: 4.,
    material: Material {
      albedo: 1.,
    }
  };
  scene.add_object(sphere);

  let scene = Arc::new(scene);
  let window = Arc::new(window);

  let mut t: f32 = 0.;
  let mut then: Instant = Instant::now();
  let radius:f32 = 12.;
  loop {
    let now = Instant::now();
    let dt = now.duration_since(then).as_secs_f32() * 1000.0;
    then = now;
    render(window.clone(), scene.clone(), &vec3(t.sin()*radius, 3., t.cos()*radius), &vec3(0., 0., 0.));
    // render(&window, Arc::clone(&scene), &vec3(10.0, 10.0, 10.0), &vec3(0., 0., 0.));

    // render(&window, Arc::clone(&scene), &vec3(7.0, 7.0, 10.0), &vec3(0., 0., 0.));
    window.mvaddstr(0, 0, format!("{}ms",dt));

    // sphere.borrow_mut().center = vec3(0.0,0.0,time.sin());
    // sphere.borrow_mut().radius = (3. + (time*2.).sin()) * 2.0;
    t += 0.04;
    match window.getch() {
      Some(Input::KeyDC) => break,
      Some(Input::KeyResize) => {resize_term(0,0);},
      _ => {},
    }
  }
  endwin();
}
