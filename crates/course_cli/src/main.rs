use agent_core::{
    benchmarks, context, course_manifest, experience, interaction, retrieval, tools, training,
};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    match args.as_slice() {
        [] => print_help(),
        [command] if command == "help" || command == "--help" || command == "-h" => print_help(),
        [command] if command == "list" => print_course_list(),
        [command] if command == "map" => print_course_map(),
        [command, target] if command == "run" => run_target(target),
        [command, target] if command == "show" => show_week(target),
        _ => {
            eprintln!("Unknown command: {:?}", args);
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!(
        "\
agent-courses - Rust edition

Usage:
  cargo run -p course_cli -- list
  cargo run -p course_cli -- map
  cargo run -p course_cli -- show week1
  cargo run -p course_cli -- run week1
  cargo run -p course_cli -- run all

Weeks:
  week1  learning from experience
  week2  context engineering
  week3  retrieval and memory
  week4  tool ecosystem
  week5  coding agent
  week6  evaluation
  week7  post-training
  week8  live multimodal interaction"
    );
}

fn print_course_list() {
    for week in course_manifest() {
        println!("week{} - {}: {}", week.number, week.title, week.goal);
    }
}

fn print_course_map() {
    for week in course_manifest() {
        println!("\nweek{} - {}", week.number, week.title);
        println!("  goal: {}", week.goal);
        for project in week.projects {
            println!("  - {}: {}", project.slug, project.outcome);
        }
    }
}

fn show_week(target: &str) {
    let Some(week) = agent_core::course::find_week(target) else {
        eprintln!("Could not find week '{target}'");
        std::process::exit(1);
    };

    println!("week{} - {}", week.number, week.title);
    println!("{}", week.goal);
    println!();
    for project in week.projects {
        println!("- {} ({})", project.title, project.slug);
        println!("  outcome: {}", project.outcome);
        println!("  concepts: {}", project.concepts.join(", "));
    }
}

fn run_target(target: &str) {
    if target == "all" {
        for week in 1..=8 {
            run_week(week);
            if week < 8 {
                println!("\n{}\n", "=".repeat(72));
            }
        }
        return;
    }

    let Some(week) = agent_core::course::find_week(target) else {
        eprintln!("Could not find runnable target '{target}'");
        std::process::exit(1);
    };
    run_week(week.number);
}

fn run_week(number: u8) {
    let output = match number {
        1 => experience::demo_learning_from_experience(),
        2 => context::demo_context_ablation(),
        3 => retrieval::demo_retrieval(),
        4 => tools::demo_tool_ecosystem(),
        5 => tools::demo_coding_agent(),
        6 => benchmarks::demo_benchmarks(),
        7 => training::demo_post_training(),
        8 => interaction::demo_live_interaction(),
        _ => format!("week{number} is not available"),
    };
    println!("{output}");
}
