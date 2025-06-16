# GPU Info

A cross-platform Rust library for retrieving GPU information and monitoring metrics.

## Features

- Support for multiple GPU vendors (NVIDIA, AMD, Intel)
- Real-time GPU metrics monitoring
- Caching support for performance optimization
- Cross-platform compatibility (Windows, Linux, macOS)
- Safe error handling with Result types
- Formatted output for all metrics

## Supported Metrics

- Vendor and model information
- Temperature
- GPU utilization
- Core and memory clock speeds
- Power usage and limits
- Memory usage and total memory
- Active state
- Driver version

## Usage

```rust
use gpu_info::GpuInfo;

// Get GPU information
let gpu = gpu_info::get()?;

// Access raw metrics
println!("Vendor: {:?}", gpu.vendor);
println!("Name: {:?}", gpu.name_gpu);
println!("Utilization: {:?}", gpu.utilization);
println!("Temperature: {:?}", gpu.temperature);
println!("Clock Speed: {:?}", gpu.core_clock);
println!("Power Usage: {:?}", gpu.power_usage);
println!("Memory Usage: {:?}", gpu.memory_util);
println!("Memory Total: {:?}", gpu.memory_total);
println!("Is active: {:?}", gpu.active);

// Use formatted output
println!("Formatted Name: {}", gpu.format_name_gpu());
println!("Formatted Temperature: {}", gpu.format_temperature());
println!("Formatted Power Usage: {}", gpu.format_power_usage());
println!("Formatted Core Clock: {}", gpu.format_core_clock());
println!("Formatted Memory Usage: {}", gpu.format_memory_util());
println!("Formatted Active State: {}", gpu.format_active());
println!("Formatted Power Limit: {}", gpu.format_power_limit());
println!("Formatted Memory Total: {}", gpu.format_memory_total());
println!("Formatted Driver Version: {}", gpu.format_driver_version());
println!("Formatted Max Clock Speed: {}", gpu.format_max_clock_speed());
```

## Examples

Run the detailed example:
```bash
cargo run --example detailed
```

Run the caching example:
```bash
cargo run --example cache
```

## Dependencies

- Windows: NVIDIA NVML, AMD ADL, or Intel WMI
- Linux: NVIDIA NVML or AMD ADL
- macOS: Apple Metal API (support currently suspended)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.