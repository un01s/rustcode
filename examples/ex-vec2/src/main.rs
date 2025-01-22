use std::os::raw::c_void;

#[repr(C)]
pub struct FfiVector {
  data: *mut c_void,
  len: usize,
}

impl From<Vec<i32>> for FfiVector {
  fn from(vec: Vec<i32>) -> Self {
    let len = vec.len();
    let data = vec.as_ptr() as *mut c_void;
    std::mem::forget(vec); // Prevent the vector from being dropped
    FfiVector { data, len }
  }
}

extern "C" {
  fn use_vector(vec: FfiVector);
}

fn main() {
  let mut my_vec: Vec<i32> = Vec::new(); // Creation of an empty vector
  my_vec.push(42); // Adding an element to the vector
  my_vec.push(7);
  println!("{:?}", my_vec);

  //
  let myvec = vec![1, 2, 3, 4, 5];
  let ffi_vec = FfiVector::from(myvec); 
  unsafe {
    use_vector(ffi_vec);
  }
}
