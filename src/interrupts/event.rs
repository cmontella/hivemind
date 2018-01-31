use x86_64::instructions::rdtsc;

pub static mut TIME: u64 = 0;

pub fn update_time() -> u64 {

  unsafe {
    let new_time = rdtsc();
    let dt = new_time - TIME;
    TIME = new_time;
    new_time
  }

}