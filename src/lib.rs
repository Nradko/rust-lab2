use std::path::PathBuf;

pub fn head(path: &PathBuf, n: u32) -> std::io::Result<()> {
    let mut line_count: u32 = 0;
    for _ in read_lines(path)? {
        line_count += 1;
    }

    let first_not_to_print: u32 = std::cmp::min(n, line_count);

    let mut at_line: u32 = 0;
    for line in read_lines(path)? {
        if at_line >= first_not_to_print {
            break;
        }
        println!("{}", line.expect("failed to read line"));
        at_line += 1;
    }
    Ok(())
}

pub fn tail(path: &PathBuf, n: u32) -> std::io::Result<()> {
    let mut line_count: u32 = 0;
    for _ in read_lines(path)? {
        line_count += 1;
    }

    let first_to_print: u32 = line_count.saturating_sub(n);

    let mut at_line: u32 = 0;
    for line in read_lines(path)? {
        if at_line >= first_to_print {
            println!("{}", line.expect("failed to read line"));
        }
        at_line += 1;
    }
    Ok(())
}

use std::io::BufRead;

fn read_lines(
    filename: &PathBuf,
) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_math() {}
}
