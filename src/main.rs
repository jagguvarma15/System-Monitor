use chrono::Local;
use clap::{Parser, Subcommand};
use colored::*;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use log::warn;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path, thread, time::Duration};
use sysinfo::{Disks, System};

#[derive(Parser)]
#[command(name = "system_monitor")]
#[command(about = "Monitor system resources and performance")]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    /// Run once and exit (no continuous monitoring)
    #[arg(short, long)]
    once: bool,

    /// Quiet mode (minimal output)
    #[arg(short, long)]
    quiet: bool,

    /// Log file path
    #[arg(short, long, default_value = "system_monitor.log")]
    log_file: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the monitor
    Monitor,
    /// Show system summary
    Summary,
    /// Generate sample config file
    GenerateConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    general: GeneralConfig,
    display: DisplayConfig,
    thresholds: ThresholdsConfig,
    alerts: AlertsConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeneralConfig {
    refresh_interval: u64,
    log_alerts: bool,
    log_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DisplayConfig {
    show_progress_bars: bool,
    use_colors: bool,
    show_per_core_cpu: bool,
    max_processes_to_display: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct ThresholdsConfig {
    cpu_warning: f32,
    cpu_critical: f32,
    memory_warning: f32,
    memory_critical: f32,
    disk_warning: f32,
    disk_critical: f32,
    swap_warning: f32,
    swap_critical: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AlertsConfig {
    enable_desktop_notifications: bool,
    enable_email_alerts: bool,
    enable_sound_alerts: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                refresh_interval: 30,
                log_alerts: true,
                log_file: "system_monitor.log".to_string(),
            },

            display: DisplayConfig {
                show_progress_bars: true,
                use_colors: true,
                show_per_core_cpu: true,
                max_processes_to_display: 10,
            },

            thresholds: ThresholdsConfig {
                cpu_warning: 70.0,
                cpu_critical: 90.0,
                memory_warning: 70.0,
                memory_critical: 90.0,
                disk_warning: 70.0,
                disk_critical: 90.0,
                swap_warning: 70.0,
                swap_critical: 90.0,
            },

            alerts: AlertsConfig {
                enable_desktop_notifications: false,
                enable_email_alerts: false,
                enable_sound_alerts: false,
            },
        }
    }
}

fn load_config(path: &str) -> Config {
    if Path::new(path).exists() {
        let file = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&file).expect("Failed to parse config file")
    } else {
        println!("Config file not found. Using default settings.");
        Config::default()
    }
}

fn generate_config(path: &str) {
    let config = Config::default();
    let toml_config = toml::to_string_pretty(&config).expect("Failed to convert config to string");
    fs::write(path, toml_config).expect("Failed to write config file");
    println!("Generated config file at {}", path);
}

fn get_usage_color(
    usage: f32,
    warning_threshold: f32,
    critical_threshold: f32,
    use_colors: bool,
) -> colored::Color {
    if !use_colors {
        return colored::Color::White;
    }

    if usage >= critical_threshold {
        colored::Color::Red
    } else if usage >= warning_threshold {
        colored::Color::Yellow
    } else {
        colored::Color::Green
    }
}

fn create_progress_bar(usage: f32, warning: f32, critical: f32, use_colors: bool) -> String {
    if !use_colors {
        return format!("[{:>6.1}%]", usage);
    }

    let width = 20;
    let filled_length = (usage / 100.0 * width as f32) as usize;
    let empty_length = width - filled_length;

    let color = get_usage_color(usage, warning, critical, use_colors);
    let bar = "#".repeat(filled_length) + &"-".repeat(empty_length);
    format!("[{}] {:>6.1}%", bar.color(color), usage)
}

fn log_alert(message: &str, config: &Config) {
    if config.general.log_alerts {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("[{}] ALERT: {}\n", timestamp, message);

        if let Err(e) = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&config.general.log_file)
            .and_then(|mut file| file.write_all(log_entry.as_bytes()))
        {
            eprintln!("Failed to write to log file: {}", e);
        }
    }

    warn!("{}", message);
}

fn check_and_alert(name: &str, usage: f32, warning: f32, critical: f32, config: &Config) {
    if usage >= critical {
        let message = format!("{} usage is critical: {:.1}%", name, usage);
        log_alert(&message, config);
    } else if usage >= warning {
        let message = format!("{} usage is high: {:.1}%", name, usage);
        log_alert(&message, config);
    }
}

fn display_system_info(sys: &System, disks: &Disks, config: &Config) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    if config.display.use_colors {
        print!("\x1B[2J\x1B[1;1H");
    } else {
        execute!(std::io::stdout(), Clear(ClearType::All)).ok();
    }

    println!(
        "{}",
        "================================================"
            .blue()
            .bold()
    );
    println!(
        "{}",
        format!("       System Monitor - {}", timestamp)
            .blue()
            .bold()
    );
    println!(
        "{}",
        "================================================"
            .blue()
            .bold()
    );

    // CPU Information - ALWAYS SHOW
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    check_and_alert(
        "CPU",
        cpu_usage,
        config.thresholds.cpu_warning,
        config.thresholds.cpu_critical,
        config,
    );

    println!("\n{}", "CPU INFORMATION".cyan().bold());
    println!(
        "Overall Usage: {}",
        create_progress_bar(
            cpu_usage,
            config.thresholds.cpu_warning,
            config.thresholds.cpu_critical,
            config.display.use_colors
        )
    );

    if config.display.show_per_core_cpu {
        println!("Per Core:");
        for (i, cpu) in sys.cpus().iter().enumerate() {
            let core_usage = cpu.cpu_usage();
            println!(
                "  Core {:2}: {}",
                i,
                create_progress_bar(
                    core_usage,
                    config.thresholds.cpu_warning,
                    config.thresholds.cpu_critical,
                    config.display.use_colors
                )
            );
        }
    }

    // Memory Information - ALWAYS SHOW
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage = (used_memory as f64 / total_memory as f64) * 100.0;
    check_and_alert(
        "Memory",
        memory_usage as f32,
        config.thresholds.memory_warning,
        config.thresholds.memory_critical,
        config,
    );

    println!("\n{}", "MEMORY INFORMATION".cyan().bold());
    println!(
        "RAM Usage:  {}",
        create_progress_bar(
            memory_usage as f32,
            config.thresholds.memory_warning,
            config.thresholds.memory_critical,
            config.display.use_colors
        )
    );
    println!(
        "Total:      {:.1} GB",
        total_memory as f64 / 1024.0 / 1024.0 / 1024.0
    );
    println!(
        "Used:       {:.1} GB",
        used_memory as f64 / 1024.0 / 1024.0 / 1024.0
    );
    println!(
        "Available:  {:.1} GB",
        sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0
    );

    // Swap Information - ALWAYS SHOW (even if 0)
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let swap_usage = if total_swap > 0 {
        (used_swap as f64 / total_swap as f64) * 100.0
    } else {
        0.0
    };

    if total_swap > 0 {
        check_and_alert(
            "Swap",
            swap_usage as f32,
            config.thresholds.swap_warning,
            config.thresholds.swap_critical,
            config,
        );
        println!(
            "Swap Usage: {}",
            create_progress_bar(
                swap_usage as f32,
                config.thresholds.swap_warning,
                config.thresholds.swap_critical,
                config.display.use_colors
            )
        );
    } else {
        println!("Swap Usage: Not Available");
    }

    // Disk Information - ALWAYS SHOW ALL DISKS
    println!("\n{}", "DISK INFORMATION".cyan().bold());
    for disk in disks {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;
        let disk_usage = if total_space > 0 {
            (used_space as f64 / total_space as f64) * 100.0
        } else {
            0.0
        };

        check_and_alert(
            &format!("Disk {}", disk.mount_point().display()),
            disk_usage as f32,
            config.thresholds.disk_warning,
            config.thresholds.disk_critical,
            config,
        );

        println!(
            "Mount: {} {}",
            disk.mount_point().display(),
            create_progress_bar(
                disk_usage as f32,
                config.thresholds.disk_warning,
                config.thresholds.disk_critical,
                config.display.use_colors
            )
        );
        println!(
            "  Total: {:.1} GB | Used: {:.1} GB | Free: {:.1} GB",
            total_space as f64 / 1024.0 / 1024.0 / 1024.0,
            used_space as f64 / 1024.0 / 1024.0 / 1024.0,
            available_space as f64 / 1024.0 / 1024.0 / 1024.0
        );
    }

    // Process Information - ALWAYS SHOW TOP PROCESSES
    println!("\n{}", "TOP PROCESSES".cyan().bold());
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());

    println!("{:<8} {:<20} {:<8} {:<10}", "PID", "NAME", "CPU%", "MEMORY");
    println!("{}", "-".repeat(50));

    for process in processes
        .iter()
        .take(config.display.max_processes_to_display)
    {
        let cpu_color = get_usage_color(process.cpu_usage(), 50.0, 80.0, config.display.use_colors);
        println!(
            "{:<8} {:<20} {:<8} {:<10.1}MB",
            process.pid().to_string().color(cpu_color),
            process.name().chars().take(20).collect::<String>(),
            format!("{:.1}%", process.cpu_usage()).color(cpu_color),
            process.memory() as f64 / 1024.0 / 1024.0
        );
    }

    // System Information - ALWAYS SHOW
    println!("\n{}", "SYSTEM INFORMATION".cyan().bold());
    if let Some(name) = System::name() {
        println!("OS:         {}", name);
    }
    if let Some(hostname) = System::host_name() {
        println!("Hostname:   {}", hostname);
    }

    let uptime = System::uptime();
    let uptime_hours = uptime / 3600;
    let uptime_minutes = (uptime % 3600) / 60;
    println!("Uptime:     {}h {}m", uptime_hours, uptime_minutes);

    let load_avg = System::load_average();
    println!(
        "Load Avg:   {:.2}, {:.2}, {:.2}",
        load_avg.one, load_avg.five, load_avg.fifteen
    );

    println!("\n{}", "Press Ctrl+C to exit...".bright_black());
}

fn run_monitor(config: &Config, once: bool) {
    let mut sys = System::new_all();
    let mut disks = Disks::new_with_refreshed_list();

    loop {
        sys.refresh_all();
        disks.refresh();

        display_system_info(&sys, &disks, config);

        if once {
            break;
        }

        thread::sleep(Duration::from_secs(config.general.refresh_interval));
    }
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let config = load_config(&cli.config);

    match cli.command {
        Some(Commands::GenerateConfig) => {
            generate_config(&cli.config);
        }
        Some(Commands::Summary) => {
            let mut sys = System::new_all();
            let mut disks = Disks::new_with_refreshed_list();
            sys.refresh_all();
            disks.refresh();
            display_system_info(&sys, &disks, &config);
        }
        Some(Commands::Monitor) => {
            run_monitor(&config, cli.once);
        }
        None => {
            // Default behavior - run monitor
            run_monitor(&config, cli.once);
        }
    }
}
