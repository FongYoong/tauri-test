#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![rust_function])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn rust_function() -> String {
  "Hello from Rust!".into()
  /* 
  let result =  trivial();
  match result {
    Ok(str) => {
      return str
    },
    _ => {
      return "Error!".into()
    }
  }
  */
}
/*
fn trivial() -> ocl::Result<String> {
  let src = r#"
      __kernel void add(__global float* buffer, float scalar) {
          buffer[get_global_id(0)] += scalar;
      }
  "#;

  let pro_que = ProQue::builder()
      .src(src)
      .dims(1 << 20)
      .build()?;

  let buffer = pro_que.create_buffer::<f32>()?;

  let kernel = pro_que.kernel_builder("add")
      .arg(&buffer)
      .arg(10.0f32)
      .build()?;

  unsafe { kernel.enq()?; }

  let mut vec = vec![0.0f32; buffer.len()];
  buffer.read(&mut vec).enq()?;
  Ok(format!("The value at index [{}] is now '{}'!", 200007, vec[200007]))
}
 */