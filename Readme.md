# System Monitor

[![Crates.io](https://img.shields.io/crates/v/system-monitor.svg)](https://crates.io/crates/system-monitor)
[![Crates.io Downloads](https://img.shields.io/crates/d/system-monitor.svg)](https://crates.io/crates/system-monitor)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/jagguvarma15/system-monitor/workflows/Rust%20CI/badge.svg)](https://github.com/yourusername/system-monitor/actions)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey.svg)](https://github.com/yourusername/system-monitor)

A powerful, configurable system monitoring tool built in Rust that provides real-time insights into your system's performance.

## Features

- **Real-time Monitoring**: CPU, memory, disk, and process monitoring
- **Color-coded Progress Bars**: Visual representation of system usage
- **Configurable Thresholds**: Custom warning and critical levels
- **Alert System**: Automatic alerts with logging when thresholds are exceeded
- **Multiple Display Modes**: Continuous monitoring, one-time summary, or single run
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Professional Output**: Clean, organized display with timestamps

## Installation

### Using Cargo (Recommended)

```bash
cargo install system-monitor
```

### Using Homebrew (macOS/Linux)

```bash
brew install system-monitor
```

### Using Chocolatey (Windows)

```bash
choco install system-monitor
```

### From Source

```bash
git clone https://github.com/yourusername/system-monitor.git
cd system-monitor
cargo install --path .
```

### Download Pre-built Binaries

Download the latest release for your platform from [GitHub Releases](https://github.com/yourusername/system-monitor/releases).

## Quick Start

```bash
# Run continuous monitoring
system-monitor

# One-time system summary
system-monitor summary

# Generate configuration file
system-monitor generate-config

# Show help
system-monitor --help
```

## Usage

### Basic Commands

| Command | Description |
|---------|-------------|
| `system-monitor` | Default continuous monitoring (30-second intervals) |
| `system-monitor summary` | One-time system summary |
| `system-monitor generate-config` | Generate configuration file |
| `system-monitor monitor` | Explicit monitor mode |
| `system-monitor --once` | Run once and exit |
| `system-monitor --help` | Show help information |
| `system-monitor --version` | Show version information |

### Configuration

Generate a configuration file to customize thresholds and display options:

```bash
system-monitor generate-config
```

This creates a `config.toml` file:

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

| Color | Meaning | Usage Range |
|-------|---------|-------------|
| Green | Normal | Below warning threshold |
| Yellow | Warning | Warning to critical threshold |
| Red | Critical | Above critical threshold |

## Configuration Options

### General Settings
- `refresh_interval`: Update frequency in seconds
- `log_alerts`: Enable/disable alert logging
- `log_file`: Path to log file

### Display Options
- `show_progress_bars`: Visual progress bars
- `use_colors`: Colored output
- `show_per_core_cpu`: Individual CPU core display
- `max_processes_to_display`: Number of top processes

### Threshold Settings
Customize warning and critical levels for:
- CPU usage
- Memory usage  
- Disk usage
- Swap usage

## Alert System

When thresholds are exceeded, alerts are:
- Logged to console with timestamps
- Written to log file (if enabled)
- Can trigger desktop notifications (future feature)

## Development

### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

### Building
```bash
git clone https://github.com/yourusername/system-monitor.git
cd system-monitor
cargo build --release
```

### Testing
```bash
cargo test
cargo clippy
cargo fmt -- --check
```

## Performance

- **Low Resource Usage**: Minimal CPU and memory footprint
- **Fast Startup**: Sub-second initialization
- **Efficient Updates**: Optimized refresh cycles
- **Cross-platform**: Native performance on all platforms

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

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
- [ ] Interactive terminal UI mode

## Acknowledgments

- Built with [sysinfo](https://github.com/GuillaumeGomez/sysinfo) for cross-platform system information
- Inspired by traditional monitoring tools like `htop` and `top`
- Thanks to the Rust community for excellent crates and documentation

## Support

- [Documentation](https://docs.rs/system-monitor)
- [Issue Tracker](https://github.com/jagguvarma15/system-monitor/issues)
- [Discussions](https://github.com/jagguvarma15/system-monitor/discussions)
- [crates.io](https://crates.io/crates/system-monitor)

---
