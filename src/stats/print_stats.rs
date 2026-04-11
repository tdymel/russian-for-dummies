use std::collections::HashSet;
use std::fmt::Write;

use crate::{
    model::WordId,
    stats::word_lists::{A1, A2, B1, B2, C1, C2},
};

struct LevelStat {
    pub level: &'static str,
    pub covered: usize,
    pub total: usize,
    pub percent: f64,
}

pub fn print_stats() {
    let stats = calculate_stats();
    println!("{}", format_coverage_table(&stats));
}

struct CoverageStats {
    pub per_level: Vec<LevelStat>,
    pub uncategorized_known: usize,
}

fn calculate_stats() -> CoverageStats {
    let known_ids: HashSet<usize> = WordId::all_ids().into_iter().collect();

    let levels: [(&str, &Vec<usize>); 6] = [
        ("A1", &A1),
        ("A2", &A2),
        ("B1", &B1),
        ("B2", &B2),
        ("C1", &C1),
        ("C2", &C2),
    ];

    let mut categorized_ids = HashSet::new();
    let mut per_level = Vec::new();

    for (level_name, level_ids) in levels {
        let level_set: HashSet<usize> = level_ids.iter().copied().collect();
        categorized_ids.extend(level_set.iter().copied());

        let covered = level_set.intersection(&known_ids).count();
        let total = level_set.len();
        let percent = if total == 0 {
            0.0
        } else {
            (covered as f64 / total as f64) * 100.0
        };

        per_level.push(LevelStat {
            level: level_name,
            covered,
            total,
            percent,
        });
    }

    let uncategorized_known = known_ids.difference(&categorized_ids).count();

    CoverageStats {
        per_level,
        uncategorized_known,
    }
}

fn format_coverage_table(stats: &CoverageStats) -> String {
    let mut out = String::new();

    writeln!(out, "+-------+---------+-------+----------+").unwrap();
    writeln!(out, "| Level | Covered | Total | Percent  |").unwrap();
    writeln!(out, "+-------+---------+-------+----------+").unwrap();

    for s in &stats.per_level {
        writeln!(
            out,
            "| {:<5} | {:>7} | {:>5} | {:>7.2}% |",
            s.level, s.covered, s.total, s.percent
        )
        .unwrap();
    }

    writeln!(out, "+-------+---------+-------+----------+").unwrap();
    writeln!(
        out,
        "| Additional uncategorized known words: {:>10} |",
        stats.uncategorized_known
    )
    .unwrap();

    out
}
