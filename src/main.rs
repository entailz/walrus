use clap::{Arg, Command};
use std::fs;
use std::path::PathBuf;

mod color;
mod haishoku;
mod generator;
mod sequences;
mod templates;
mod parser;

use generator::PywalGenerator;
use sequences::SequenceGenerator;
use templates::TemplateGenerator;

fn initialize_walrus() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let config_dir = home_dir.join(".config/walrus");
    let templates_dir = config_dir.join("templates");
    
    fs::create_dir_all(&templates_dir)?;
    
    let exe_dir = std::env::current_exe()?
        .parent()
        .ok_or("Could not determine executable directory")?
        .to_path_buf();
    
    let possible_template_dirs = vec![
        PathBuf::from("templates"),
        PathBuf::from("/usr/share/walrus/templates"),
        exe_dir.join("templates"), 
        exe_dir.parent().unwrap_or(&exe_dir).join("templates"),
    ];
    
    let mut source_templates_dir = None;
    for dir in possible_template_dirs {
        if dir.exists() && dir.is_dir() {
            source_templates_dir = Some(dir);
            break;
        }
    }
    
    let source_dir = source_templates_dir
        .ok_or("Could not find templates directory. Make sure templates/ exists in the project directory.")?;
    
    let mut copied_files = Vec::new();
    for entry in fs::read_dir(&source_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            let dest_path = templates_dir.join(file_name);
            
            fs::copy(&path, &dest_path)?;
            copied_files.push(file_name.to_string_lossy().to_string());
        }
    }
    
    println!("✓ Created configuration directory: {}", config_dir.display());
    println!("✓ Created templates directory: {}", templates_dir.display());
    
    if !copied_files.is_empty() {
        println!("✓ Copied template files:");
        for file in copied_files {
            println!("  - {}", file);
        }
    }
    
    println!("\nWalrus has been initialized!");
    println!("You can now:");
    println!("  - Add custom templates to: {}", templates_dir.display());
    println!("  - Run walrus with an image: walrus <image_path>");
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("pywal-haishoku")
        .about("A minimal pywal-style color generator using haishoku algorithm")
        .version("1.0")
        .arg(
            Arg::new("image")
                .help("The image file to process")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output directory for template files")
                .default_value("~/.cache/walrus"),
        )
        .arg(
            Arg::new("saturation")
                .short('s')
                .long("saturation")
                .help("Saturation factor (1.0 = normal, 0.2 = 20% saturation)")
                .default_value("1.0"),
        )
        .arg(
            Arg::new("strip")
                .long("strip")
                .help("Strip # from hex colors")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("light")
                .short('l')
                .long("light")
                .help("Generate a light colorscheme")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("templates")
                .short('t')
                .long("templates")
                .help("Directory containing template files to process")
                .value_name("TEMPLATES_DIR"),
        )
        .arg(
            Arg::new("init")
                .short('u')
                .long("init")
                .help("Initialize walrus by creating ~/.config/walrus and copying templates")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("init") {
        return initialize_walrus();
    }

    let image_path = matches.get_one::<String>("image")
        .ok_or("Image path is required. Use --help for usage information.")?;
    let output_dir_str = matches.get_one::<String>("output").unwrap();
    let saturation: f32 = matches.get_one::<String>("saturation").unwrap().parse()?;
    let strip_hash = matches.get_flag("strip");
    let light = matches.get_flag("light");

    // Expand tilde in output directory path
    let output_dir = if output_dir_str.starts_with("~/") {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(&output_dir_str[2..])
    } else if output_dir_str == "~" {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
    } else {
        PathBuf::from(output_dir_str)
    };

    fs::create_dir_all(&output_dir)?;

    let mut generator = PywalGenerator::new();
    let colors = generator.generate_from_image(image_path, saturation, light)?;

    let template_gen = TemplateGenerator::new(colors.clone(), strip_hash);

    // Generate terminal sequences
    let sequence_gen = SequenceGenerator::new(colors.clone(), 100); // 100 is the default alpha value

    fs::write(
        output_dir.join("colors.sh"),
        template_gen.generate_shell_template(),
    )?;

    fs::write(
        output_dir.join("colors.css"),
        template_gen.generate_css_template(),
    )?;

    fs::write(
        output_dir.join("colors.json"),
        template_gen.generate_json_template(),
    )?;

    fs::write(
        output_dir.join("colors.Xresources"),
        template_gen.generate_xresources_template(),
    )?;

    fs::write(
        output_dir.join("colors.scss"),
        template_gen.generate_scss_template(image_path),
    )?;

    let sequences = sequence_gen.generate_sequences(false);
    fs::write(
        output_dir.join("sequences"),
        &sequences,
    )?;
    
    // Send sequences to all open terminals
    if let Err(e) = sequence_gen.send_sequences_to_terminals(false) {
        eprintln!("Warning: Failed to send sequences to terminals: {}", e);
    } else {
        println!("Applied colors to open terminals");
    }
    
    // Process template files
    // Check for template directory in the following order:
    // 1. Command line argument
    // 2. User config directory (~/.config/walrus/templates)
    // 3. Local templates directory (development)
    // 4. System-wide templates (/usr/share/walrus/templates)
    let mut template_dirs = Vec::new();
    
    if let Some(template_path) = matches.get_one::<String>("templates") {
        template_dirs.push(PathBuf::from(template_path));
    }
    
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    template_dirs.push(home_dir.join(".config/walrus/templates"));
    template_dirs.push(PathBuf::from("templates"));
    template_dirs.push(PathBuf::from("/usr/share/walrus/templates"));
    
    let mut processed_any = false;
    for template_dir in template_dirs {
        if template_dir.exists() && template_dir.is_dir() {
            let template_parser = parser::TemplateParser::new(colors.clone());
            let processed_files = template_parser.process_template_directory(
                &template_dir,
                &output_dir
            )?;
            
            if !processed_files.is_empty() {
                println!("Processed template files from {}:", template_dir.display());
                for file in processed_files {
                    println!("  - {}", file.file_name().unwrap().to_string_lossy());
                }
                processed_any = true;
                break;
            }
        }
    }
    
    if !processed_any {
        println!("No template files found. Templates can be placed in:");
        println!("  - ~/.config/walrus/templates (user templates)");
        println!("  - ./templates (development)");
        println!("  - /usr/share/walrus/templates (system-wide)");
        println!("  - Or specify with --templates <dir>");
    }

    println!(
        "Colors extracted using haishoku algorithm and templates generated in: {}",
        output_dir.display()
    );
    println!("Saturation factor: {}", saturation);
    println!("Strip hash: {}", strip_hash);
    println!("Generated files:");
    println!("  - colors.sh (shell variables)");
    println!("  - colors.css (CSS variables)");
    println!("  - colors.json (JSON format)");
    println!("  - colors.Xresources (X11 resources)");
    println!("  - colors.scss (SCSS variables)");
    println!("  - sequences (terminal escape sequences)");

    Ok(())
}


