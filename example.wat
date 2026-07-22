(module

    (import "host" "cpu_count" (func $cpu_count (result i32)))
  
    (import "host" "cpu_usage_pcnt_by_index"
      (func $cpu_usage_pcnt_by_index (param i32) (result f32)))
  
    (import "host" "cpu_freq_mhz_by_index"
      (func $cpu_freq_mhz_by_index (param i32) (result i64)))
  
    (import "host" "print_cpu_count" (func $print_cpu_count))
    (import "host" "print_cpu_ix" (func $print_cpu_ix (param i32)))
  
    (import "host" "print_cpu_usage_pcnt_by_index"
      (func $print_cpu_usage_pcnt_by_index (param i32)))
  
    (import "host" "print_cpu_freq_mhz_by_index"
      (func $print_cpu_freq_mhz_by_index (param i32)))
  
    (import "host" "print_cpu_brand_by_index"
      (func $print_cpu_brand_by_index (param i32)))
  
    (import "host" "print_cpu_name_by_index"
      (func $print_cpu_name_by_index (param i32)))
  
    (import "host" "print_cpu_vendor_id_by_index"
      (func $print_cpu_vendor_id_by_index (param i32)))
  
    (import "host" "print_f32" (func $print_f32 (param f32)))
    (import "host" "print_u64" (func $print_u64 (param i64)))
  
    (func $io_main (export "io_main")
        (local $cpu_count i32)
    
        (local $cpu_index i32)
    
        (local $usage_pcnt f32)
        (local $freq_mhz i64)
    
        (local $cpuix i32)
    
        call $print_cpu_count
    
        call $cpu_count local.set $cpu_count
    
        i32.const 0 local.set $cpuix
    
        loop
            local.get $cpuix call $print_cpu_ix
    
            local.get $cpuix call $cpu_usage_pcnt_by_index local.set $usage_pcnt
            local.get $cpuix call $cpu_freq_mhz_by_index   local.set $freq_mhz
    
            local.get $cpuix call $print_cpu_usage_pcnt_by_index
            local.get $cpuix call $print_cpu_freq_mhz_by_index
    
            local.get $cpuix call $print_cpu_brand_by_index
            local.get $cpuix call $print_cpu_name_by_index
            local.get $cpuix call $print_cpu_vendor_id_by_index
    
            local.get $cpuix i32.const 1 i32.add
            local.tee $cpuix
            local.get $cpu_count
            i32.lt_u br_if 0
        end
    
    )

)
