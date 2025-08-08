# System Monitor

A powerful, configurable system monitoring tool built in Rust that provides real-time insights into your system's performance.

## Features

- **Real-time Monitoring**: CPU, memory, disk, and process monitoring
- **Color-coded Progress Bars**: Visual representation of system usage
- **Configurable Thresholds**: Custom warning and critical levels
- **Alert System**: Automatic alerts with logging when thresholds are exceeded
- **Multiple Display Modes**: Continuous monitoring, one-time summary, or single run
- **Professional Output**: Clean, organized display with timestamps
- **Cross-platform**: Works on Linux, macOS, and Windows

## Installation

### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

### Build from Source
```bash
git clone https://github.com/yourusername/system-monitor.git
cd system-monitor
cargo build --release
```

### Run Directly
```bash
cargo run
```

## Usage

### Basic Commands

```bash
# Default continuous monitoring (30-second intervals)
cargo run

# Generate configuration file
cargo run -- generate-config

# One-time system summary
cargo run -- summary

# Explicit monitor mode
cargo run -- monitor

# Run once and exit
cargo run -- --once

# Custom config file
cargo run -- --config custom.toml

# Help and version
cargo run -- --help
cargo run -- --version
```

### Configuration

Generate a configuration file to customize thresholds and display options:

```bash
cargo run -- generate-config
```

This creates a `config.toml` file with the following structure:

```toml
[general]
refresh_interval = 30
log_alerts = true
log_file = "system_monitor.log"

[display]
show_progress_bars = true
use_colors = true
show_per_core_cpu = true
max_processes_to_display = 10

[thresholds]
cpu_warning = 70.0
cpu_critical = 90.0
memory_warning = 70.0
memory_critical = 90.0
disk_warning = 70.0
disk_critical = 90.0
swap_warning = 70.0
swap_critical = 90.0

[alerts]
enable_desktop_notifications = false
enable_email_alerts = false
enable_sound_alerts = false
```

## Sample Output

```
================================================
       System Monitor - 2025-08-08 15:30:45
================================================

CPU INFORMATION
Overall Usage: [####----------------]   21.5%
Per Core:
  Core  0: [########------------]   40.9%
  Core  1: [#######-------------]   38.1%
  Core  2: [######--------------]   33.5%
  Core  3: [######--------------]   30.5%

MEMORY INFORMATION
RAM Usage:  [################----]   80.2%
Total:      8.0 GB
Used:       6.4 GB
Available:  1.6 GB

DISK INFORMATION
Mount: / [##############------]   73.3%
  Total: 460.4 GB | Used: 337.6 GB | Free: 122.9 GB

TOP PROCESSES
PID      NAME                 CPU%     MEMORY    
--------------------------------------------------
1234     chrome               15.2%    512.3MB
5678     firefox              8.1%     256.1MB

SYSTEM INFORMATION
OS:         Darwin
Hostname:   MacBookAir
Uptime:     1647h 1m
Load Avg:   2.56, 3.57, 3.71

Press Ctrl+C to exit...
```

## Color Coding

- **Green**: Normal usage (below warning threshold)
- **Yellow**: Warning level (above warning, below critical)
- **Red**: Critical level (above critical threshold)

## Alert System

When thresholds are exceeded, alerts are:
- Logged to console
- Written to log file (if enabled)
- Can trigger desktop notifications (configurable)

## Dependencies

- `sysinfo` - System information gathering
- `clap` - Command-line argument parsing
- `serde` & `toml` - Configuration file handling
- `colored` - Terminal color output
- `chrono` - Date and time handling
- `log` & `env_logger` - Logging functionality
- `crossterm` - Cross-platform terminal manipulation

## Development

### Project Structure
```
system-monitor/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Project dependencies
├── config.toml          # Configuration file (generated)
├── README.md            # This file
└── .gitignore          # Git ignore rules
```

### Building
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Network bandwidth monitoring
- [ ] Docker container monitoring
- [ ] Web dashboard interface
- [ ] Email alert notifications
- [ ] Historical data logging
- [ ] Plugin system for custom metrics
- [ ] JSON/CSV export functionality

## Troubleshooting

### Common Issues

**High memory usage alerts on macOS**: This is often normal due to macOS memory management. Adjust thresholds in config.toml if needed.

**Permission denied errors**: Some system information requires elevated privileges on certain platforms.

**Missing dependencies**: Ensure you have the latest Rust toolchain installed.

## Acknowledgments

- Built with [sysinfo](https://github.com/GuillaumeGomez/sysinfo) for cross-platform system information
- Inspired by traditional monitoring tools like `htop` and `top`
- Thanks to the Rust community for excellent crates and documentation