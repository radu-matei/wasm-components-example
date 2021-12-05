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
  pub type PayloadParam<'a,> = &'a [u8];
  /// Payload for cache values.
  pub type PayloadResult = Vec<u8>;
  /// Set the payload for the given key.
  pub fn set(key: & str,value: PayloadParam<'_,>,ttl: Option<u32>,) -> Result<(),Error>{
    unsafe {
      let vec0 = key;
      let ptr0 = vec0.as_ptr() as i32;
      let len0 = vec0.len() as i32;
      let vec1 = value;
      let ptr1 = vec1.as_ptr() as i32;
      let len1 = vec1.len() as i32;
      let (result2_0,result2_1,) = match ttl{
        None => { (0i32, 0i32)}
        Some(e) => { (1i32, wit_bindgen_rust::rt::as_i32(e))}
      };
      let ptr3 = RET_AREA.as_mut_ptr() as i32;
      #[link(wasm_import_module = "cache")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "set")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "cache_set")]
        fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
      }
      wit_import(ptr0, len0, ptr1, len1, result2_0, result2_1, ptr3);
      match *((ptr3 + 0) as *const i32) {
        0 => Ok(()),
        1 => Err(match *((ptr3 + 8) as *const i32) {
          0 => Error::RuntimeError,
          1 => Error::NotFoundError,
          _ => panic!("invalid enum discriminant"),
        }),
        _ => panic!("invalid enum discriminant"),
      }
    }
  }
  /// Get the payload stored in the cache for the given key.
  pub fn get(key: & str,) -> Result<PayloadResult,Error>{
    unsafe {
      let vec0 = key;
      let ptr0 = vec0.as_ptr() as i32;
      let len0 = vec0.len() as i32;
      let ptr1 = RET_AREA.as_mut_ptr() as i32;
      #[link(wasm_import_module = "cache")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "get")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "cache_get")]
        fn wit_import(_: i32, _: i32, _: i32, );
      }
      wit_import(ptr0, len0, ptr1);
      match *((ptr1 + 0) as *const i32) {
        0 => Ok({
          let len2 = *((ptr1 + 16) as *const i32) as usize;
          
          Vec::from_raw_parts(*((ptr1 + 8) as *const i32) as *mut _, len2, len2)
        }),
        1 => Err(match *((ptr1 + 8) as *const i32) {
          0 => Error::RuntimeError,
          1 => Error::NotFoundError,
          _ => panic!("invalid enum discriminant"),
        }),
        _ => panic!("invalid enum discriminant"),
      }
    }
  }
  /// Delete the cache entry for the given key.
  pub fn delete(key: & str,) -> Result<(),Error>{
    unsafe {
      let vec0 = key;
      let ptr0 = vec0.as_ptr() as i32;
      let len0 = vec0.len() as i32;
      let ptr1 = RET_AREA.as_mut_ptr() as i32;
      #[link(wasm_import_module = "cache")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "delete")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "cache_delete")]
        fn wit_import(_: i32, _: i32, _: i32, );
      }
      wit_import(ptr0, len0, ptr1);
      match *((ptr1 + 0) as *const i32) {
        0 => Ok(()),
        1 => Err(match *((ptr1 + 8) as *const i32) {
          0 => Error::RuntimeError,
          1 => Error::NotFoundError,
          _ => panic!("invalid enum discriminant"),
        }),
        _ => panic!("invalid enum discriminant"),
      }
    }
  }
  static mut RET_AREA: [i64; 3] = [0; 3];
}
