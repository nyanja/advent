use std::time::Instant;

use std::time::Duration;
use pixels::{Pixels, SurfaceTexture};
use winit::{
  dpi::LogicalSize,
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

// const H: i32 = 7;
// const W: i32 = 11;
const H: i32 = 103;
const W: i32 = 101;

pub fn solve_1(input: &str) -> i32 {
  let steps = 100;
  let mut res = vec![0, 0, 0, 0];
  
  for line in input.lines() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let p: Vec<i32> = parts[0][2..].split(',').map(|x| x.parse().unwrap()).collect();
    let v: Vec<i32> = parts[1][2..].split(',').map(|x| x.parse().unwrap()).collect();
    
    let new_p = ((p[0] + v[0] * steps).rem_euclid(W), (p[1] + v[1] * steps).rem_euclid(H));
    match new_p {
      (x, y) if x < W / 2 && y < H / 2 => res[0] += 1,
      (x, y) if x > W / 2 && y < H / 2 => res[1] += 1,
      (x, y) if x < W / 2 && y > H / 2 => res[2] += 1,
      (x, y) if x > W / 2 && y > H / 2 => res[3] += 1,
      _ => (),
    }
  }
  res.iter().product()
}

pub fn solve_2(input: &str) {
  let mut rules: Vec<Vec<Vec<i32>>> = Vec::new();
  
  for line in input.lines() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let p: Vec<i32> = parts[0][2..].split(',').map(|x| x.parse().unwrap()).collect();
    let v: Vec<i32> = parts[1][2..].split(',').map(|x| x.parse().unwrap()).collect();
    rules.push(vec![p, v]);
  }
  
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
  .with_inner_size(LogicalSize::new(W * 4, H * 4))
  .build(&event_loop)
  .unwrap();
  let window_size = window.inner_size();
  let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
  
  let mut pixels = Pixels::new(W as u32, H as u32, surface_texture).unwrap();
  let mut last_update = Instant::now();
  let update_interval = Duration::from_secs_f64(0.02);
  
  let mut sec = 0;
  
  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;
    
    match event {
      Event::RedrawRequested(_) => {
        let frame = pixels.get_frame_mut();

        println!("sec: {}", sec);
        sec += 1;
        
        for pixel in frame.chunks_exact_mut(4) {
          pixel.copy_from_slice(&[0, 0, 0, 255]);
        }
        
        for i in 0..rules.len() {
          let p = &rules[i][0];
          let v = &rules[i][1];
          let x = p[0];
          let y = p[1];
          
          let index = ((y * W + x) * 4) as usize;
          frame[index..index + 4].copy_from_slice(&[255, 255, 255, 255]);

          let new_p = ((p[0] + v[0]).rem_euclid(W), (p[1] + v[1]).rem_euclid(H));
          rules[i] = vec![vec![new_p.0, new_p.1], v.clone()];
        }

        if pixels.render().is_err() {
          *control_flow = ControlFlow::Exit;
          return;
        }
      }

      Event::MainEventsCleared => {
        if last_update.elapsed() >= update_interval {
          last_update = Instant::now();
          window.request_redraw();
        }
      }

      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        _ => {}
      },
      _ => {}
    }
  });
}

#[cfg(test)]
mod tests {
  use aoc_2024::read_example;
  
  use super::*;
  
  #[test]
  fn test_solve_1() {
    let input = read_example!(14);
    assert_eq!(solve_1(&input), 12);
  }
}