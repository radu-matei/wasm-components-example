mod cache {
  /// A simple cache interface.
  /// Type for cache errors.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum Error{
    RuntimeError,
    NotFoundError,
  }
  impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Error::RuntimeError => {
          f.debug_tuple("Error::RuntimeError").finish()
        }
        Error::NotFoundError => {
          f.debug_tuple("Error::NotFoundError").finish()
        }
      }
    }
  }
  /// Payload for cache values.
  pub type Payload = Vec<u8>;
  #[export_name = "set"]
  unsafe extern "C" fn __wit_bindgen_set(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, ) -> i32{
    let len0 = arg1 as usize;
    let len1 = arg3 as usize;
    let result2 = <super::Cache as Cache>::set(String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(), Vec::from_raw_parts(arg2 as *mut _, len1, len1), match arg4 {
      0 => None,
      1 => Some(arg5 as u32),
      _ => panic!("invalid enum discriminant"),
    });
    let (result3_0,result3_1,) = match result2{
      Ok(()) => { (0i32, 0i32)}
      Err(e) => { (1i32, e as i32)}
    };
    let ptr4 = RET_AREA.as_mut_ptr() as i32;
    *((ptr4 + 8) as *mut i32) = result3_1;
    *((ptr4 + 0) as *mut i32) = result3_0;
    ptr4
  }
  #[export_name = "get"]
  unsafe extern "C" fn __wit_bindgen_get(arg0: i32, arg1: i32, ) -> i32{
    let len0 = arg1 as usize;
    let result1 = <super::Cache as Cache>::get(String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap());
    let (result3_0,result3_1,result3_2,) = match result1{
      Ok(e) => { {
        let vec2 = (e).into_boxed_slice();
        let ptr2 = vec2.as_ptr() as i32;
        let len2 = vec2.len() as i32;
        core::mem::forget(vec2);
        
        (0i32, ptr2, len2)
      }}
      Err(e) => { (1i32, e as i32, 0i32)}
    };
    let ptr4 = RET_AREA.as_mut_ptr() as i32;
    *((ptr4 + 16) as *mut i32) = result3_2;
    *((ptr4 + 8) as *mut i32) = result3_1;
    *((ptr4 + 0) as *mut i32) = result3_0;
    ptr4
  }
  #[export_name = "delete"]
  unsafe extern "C" fn __wit_bindgen_delete(arg0: i32, arg1: i32, ) -> i32{
    let len0 = arg1 as usize;
    let result1 = <super::Cache as Cache>::delete(String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap());
    let (result2_0,result2_1,) = match result1{
      Ok(()) => { (0i32, 0i32)}
      Err(e) => { (1i32, e as i32)}
    };
    let ptr3 = RET_AREA.as_mut_ptr() as i32;
    *((ptr3 + 8) as *mut i32) = result2_1;
    *((ptr3 + 0) as *mut i32) = result2_0;
    ptr3
  }
  pub trait Cache {
    /// Set the payload for the given key.
    fn set(key: String,value: Payload,ttl: Option<u32>,) -> Result<(),Error>;
    /// Get the payload stored in the cache for the given key.
    fn get(key: String,) -> Result<Payload,Error>;
    /// Delete the cache entry for the given key.
    fn delete(key: String,) -> Result<(),Error>;
  }
  static mut RET_AREA: [i64; 3] = [0; 3];
}
