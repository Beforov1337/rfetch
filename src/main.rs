use std::fs;

fn get_os() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("ID=") {
                return val.trim_matches('"').to_string();
            }
        }
    }
    "linux".to_string()
}

fn get_kernel() -> String {
    if let Ok(content) = fs::read_to_string("/proc/sys/kernel/osrelease") {
        let trimmed = content.trim();
        if let Some((ver, _)) = trimmed.split_once('-') {
            return ver.to_string();
        }
        return trimmed.to_string();
    }
    "unknown".to_string()
}

fn get_uptime() -> String {
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(first) = content.split_whitespace().next() {
            if let Ok(secs) = first.parse::<f64>() {
                let total_secs = secs as u64;
                let days = total_secs / 86400;
                let hours = (total_secs % 86400) / 3600;
                let mins = (total_secs % 3600) / 60;

                let mut parts = Vec::new();
                if days > 0 {
                    parts.push(format!("{}d", days));
                }
                if hours > 0 || days > 0 {
                    parts.push(format!("{}h", hours));
                }
                parts.push(format!("{}m", mins));
                return parts.join(" ");
            }
        }
    }
    "unknown".to_string()
}

fn count_xbps_packages() -> Option<String> {
    // Void Linux / XBPS: каждый пакет имеет файл .<pkgname>-files.plist
    let entries = fs::read_dir("/var/db/xbps").ok()?;
    let count = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.starts_with('.') && name.ends_with("-files.plist")
        })
        .count();
    Some(count.to_string())
}

fn count_pacman_packages() -> Option<String> {
    // Arch Linux / Pacman: каждая папка в local = один пакет
    let entries = fs::read_dir("/var/lib/pacman/local").ok()?;
    let count = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name() != "ALPM_DB_VERSION"
                && e.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
        })
        .count();
    Some(count.to_string())
}

fn count_dpkg_packages() -> Option<String> {
    // Debian / Ubuntu: считаем блоки со статусом installed
    let content = fs::read_to_string("/var/lib/dpkg/status").ok()?;
    let count = content
        .split("\n\n")
        .filter(|block| block.contains("Status: install ok installed"))
        .count();
    Some(count.to_string())
}

fn count_rpm_packages() -> Option<String> {
    // Fedora / RHEL: считаем файлы Packages* в /var/lib/rpm
    let entries = fs::read_dir("/var/lib/rpm").ok()?;
    let count = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("Packages")
        })
        .count();
    Some(count.to_string())
}

fn get_packages() -> String {
    let os = get_os();
    let os_lower = os.to_lowercase();

    // Void Linux / XBPS
    if os_lower == "void" || os_lower.contains("void") {
        if let Some(n) = count_xbps_packages() {
            return n;
        }
    }

    // Arch / Manjaro / Pacman
    if matches!(
        os_lower.as_str(),
        "arch" | "manjaro" | "endeavouros" | "garuda" | "artix"
    ) {
        if let Some(n) = count_pacman_packages() {
            return n;
        }
    }

    // Debian / Ubuntu
    if matches!(
        os_lower.as_str(),
        "debian" | "ubuntu" | "linuxmint" | "pop" | "kali" | "raspbian"
    ) {
        if let Some(n) = count_dpkg_packages() {
            return n;
        }
    }

    // Fedora / RHEL
    if matches!(
        os_lower.as_str(),
        "fedora" | "rhel" | "centos" | "rocky" | "almalinux"
    ) {
        if let Some(n) = count_rpm_packages() {
            return n;
        }
    }

    // Фолбэк: пробуем все известные источники по очереди
    count_xbps_packages()
        .or_else(count_pacman_packages)
        .or_else(count_dpkg_packages)
        .or_else(count_rpm_packages)
        .unwrap_or_else(|| "unknown".to_string())
}

// Раскрашиваем строчку пингвина по символам:
// Черно-серый / темный для контура/спинки, белый для глаз и пузика, желтый для клюва и лапок
fn colorize_tux_line(line_idx: usize, raw_line: &str) -> String {
    let dark = "\x1b[38;5;244m"; // серый/темный контур перьев
    let yellow = "\x1b[1;33m";   // ярко-желтый для клюва и лапок
    let white = "\x1b[1;37m";    // ярко-белый для пузика и глаз
    let reset = "\x1b[0m";

    match line_idx {
        // "    .--."
        0 => format!("{dark}{raw_line}{reset}"),

        // "   |o_o |" -> | (dark), o (white), _ (yellow beak top), o (white), | (dark)
        1 => {
            let prefix_spaces = " ".repeat(raw_line.chars().take_while(|c| *c == ' ').count());
            format!("{prefix_spaces}{dark}|{white}o{yellow}_{white}o {dark}|{reset}")
        }

        // "   |:_/ |" -> | (dark), :_/ (yellow beak), | (dark)
        2 => {
            let prefix_spaces = " ".repeat(raw_line.chars().take_while(|c| *c == ' ').count());
            format!("{prefix_spaces}{dark}|{yellow}:_/{reset} {dark}|{reset}")
        }

        // "  //   \ \" -> крылья темные, между ними пузико белое
        3 => {
            let prefix_spaces = " ".repeat(raw_line.chars().take_while(|c| *c == ' ').count());
            format!("{prefix_spaces}{dark}//{white}   {dark}\\ \\{reset}")
        }

        // " (|     | )" -> крылья/бока темные, внутри пузико белое
        4 => {
            let prefix_spaces = " ".repeat(raw_line.chars().take_while(|c| *c == ' ').count());
            format!("{prefix_spaces}{dark}(|{white}     {dark}| ){reset}")
        }

        // "/'\_   _/`\" -> перья темные, лапки желтые \_ и _/
        5 => {
            let prefix_spaces = " ".repeat(raw_line.chars().take_while(|c| *c == ' ').count());
            format!("{prefix_spaces}{dark}/'{yellow}\\_   _/{dark}`\\{reset}")
        }

        // "\___)=(___/ " -> лапки целиком желтые
        6 => {
            let prefix_spaces = " ".repeat(raw_line.chars().take_while(|c| *c == ' ').count());
            format!("{prefix_spaces}{yellow}\\___)=(___/{reset}")
        }

        // Любая другая строка
        _ => format!("{dark}{raw_line}{reset}"),
    }
}

fn main() {
    let os = get_os();
    let kernel = get_kernel();
    let uptime = get_uptime();
    let packages = get_packages();

    const EMBEDDED_ASCII: &str = include_str!("../ascii.txt");
    let ascii_content =
        fs::read_to_string("ascii.txt").unwrap_or_else(|_| EMBEDDED_ASCII.to_string());

    let mut logo_lines: Vec<String> = ascii_content
        .lines()
        .map(|s| s.trim_end().to_string())
        .collect();

    while logo_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
        logo_lines.pop();
    }

    let max_logo_width = logo_lines
        .iter()
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(0);

    let key = "\x1b[1m";
    let reset = "\x1b[0m";

    let info_items = [
        format!("{key}os{reset}        {os}"),
        format!("{key}kernel{reset}    {kernel}"),
        format!("{key}uptime{reset}    {uptime}"),
        format!("{key}packages{reset}  {packages}"),
    ];

    let total_lines = logo_lines.len().max(info_items.len());
    let info_start = if total_lines > info_items.len() {
        (total_lines - info_items.len()) / 2
    } else {
        0
    };

    println!();
    for i in 0..total_lines {
        let logo_part = if i < logo_lines.len() {
            let raw_line = &logo_lines[i];
            let raw_len = raw_line.chars().count();
            let colored_line = colorize_tux_line(i, raw_line);
            let padding = " ".repeat(max_logo_width.saturating_sub(raw_len));
            format!("{colored_line}{padding}")
        } else {
            " ".repeat(max_logo_width)
        };

        let info_part = if i >= info_start && (i - info_start) < info_items.len() {
            &info_items[i - info_start]
        } else {
            ""
        };

        if info_part.is_empty() {
            println!("  {}", logo_part.trim_end());
        } else {
            println!("  {}   {}", logo_part, info_part);
        }
    }
    println!();
}
